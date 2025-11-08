use super::provider::OcrProvider;
use super::types::{OcrRequest, OcrResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::time::Instant;

/// DeepSeek-OCR provider implementation
pub struct DeepSeekOcr {
    /// API key (HuggingFace token)
    api_key: Option<String>,

    /// API endpoint
    endpoint: String,

    /// HTTP client
    client: reqwest::Client,
}

impl DeepSeekOcr {
    pub fn new() -> Self {
        Self {
            api_key: None,
            endpoint: "https://api-inference.huggingface.co/models/deepseek-ai/Deepseek-Ocr"
                .to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = endpoint;
        self
    }

    /// Convert image to base64-encoded PNG
    fn image_to_base64(&self, image: &image::DynamicImage) -> Result<String> {
        let mut buffer = Cursor::new(Vec::new());
        image
            .write_to(&mut buffer, ImageFormat::Png)
            .context("Failed to encode image as PNG")?;

        Ok(BASE64.encode(buffer.into_inner()))
    }

    /// Build the request payload for HuggingFace Inference API
    fn build_request_payload(
        &self,
        request: &OcrRequest,
    ) -> Result<HuggingFaceRequest> {
        let image_base64 = self.image_to_base64(&request.image)?;

        // Build the prompt
        let mut prompt = request
            .system_prompt
            .clone()
            .unwrap_or_else(|| "Extract all text from this image.".to_string());

        if !request.hints.is_empty() {
            prompt.push_str("\n\nHints:\n");
            for hint in &request.hints {
                prompt.push_str(&format!("- {}\n", hint));
            }
        }

        Ok(HuggingFaceRequest {
            inputs: HuggingFaceInputs {
                image: image_base64,
                prompt,
            },
            parameters: HuggingFaceParameters {
                max_new_tokens: 2048,
                temperature: 0.1,
            },
        })
    }
}

#[async_trait]
impl OcrProvider for DeepSeekOcr {
    async fn process(&self, request: OcrRequest) -> Result<OcrResult> {
        let start = Instant::now();

        let payload = self.build_request_payload(&request)?;

        let mut req = self.client.post(&self.endpoint).json(&payload);

        if let Some(api_key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req
            .send()
            .await
            .context("Failed to send request to OCR API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "OCR API request failed with status {}: {}",
                status,
                error_text
            );
        }

        let response_data: Vec<HuggingFaceResponse> = response
            .json()
            .await
            .context("Failed to parse OCR API response")?;

        let text = response_data
            .first()
            .and_then(|r| r.generated_text.clone())
            .unwrap_or_default();

        let processing_time = start.elapsed().as_millis() as u64;

        Ok(OcrResult {
            text,
            confidence: None,
            processing_time_ms: processing_time,
            metadata: serde_json::json!({
                "provider": "deepseek-ocr",
                "endpoint": self.endpoint,
            }),
        })
    }

    fn name(&self) -> &str {
        "DeepSeek-OCR"
    }

    fn is_ready(&self) -> bool {
        self.api_key.is_some()
    }

    fn status_message(&self) -> Option<String> {
        if self.api_key.is_none() {
            Some("API key not configured. Please set your HuggingFace token in Settings.".to_string())
        } else {
            None
        }
    }
}

impl Default for DeepSeekOcr {
    fn default() -> Self {
        Self::new()
    }
}

// HuggingFace API types

#[derive(Debug, Serialize)]
struct HuggingFaceRequest {
    inputs: HuggingFaceInputs,
    parameters: HuggingFaceParameters,
}

#[derive(Debug, Serialize)]
struct HuggingFaceInputs {
    image: String,
    prompt: String,
}

#[derive(Debug, Serialize)]
struct HuggingFaceParameters {
    max_new_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceResponse {
    generated_text: Option<String>,
}

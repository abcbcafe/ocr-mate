use image::DynamicImage;
use serde::{Deserialize, Serialize};

/// Request to perform OCR on an image
#[derive(Clone)]
pub struct OcrRequest {
    /// The image to process
    pub image: DynamicImage,

    /// Optional system prompt/instructions
    pub system_prompt: Option<String>,

    /// Optional page-specific hints
    pub hints: Vec<String>,

    /// Language code (e.g., "en", "es", "fr")
    pub language: Option<String>,
}

impl OcrRequest {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            image,
            system_prompt: None,
            hints: Vec::new(),
            language: None,
        }
    }

    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = Some(prompt);
        self
    }

    pub fn with_hints(mut self, hints: Vec<String>) -> Self {
        self.hints = hints;
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }
}

/// Result from OCR processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    /// Extracted text
    pub text: String,

    /// Confidence score (0.0 to 1.0), if available
    pub confidence: Option<f32>,

    /// Processing time in milliseconds
    pub processing_time_ms: u64,

    /// Metadata from the OCR provider
    pub metadata: serde_json::Value,
}

impl OcrResult {
    pub fn new(text: String) -> Self {
        Self {
            text,
            confidence: None,
            processing_time_ms: 0,
            metadata: serde_json::Value::Null,
        }
    }
}

/// Response from OCR API
#[derive(Debug, Deserialize)]
pub struct OcrResponse {
    pub text: String,
    #[serde(default)]
    pub confidence: Option<f32>,
}

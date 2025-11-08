use super::types::{OcrRequest, OcrResult};
use anyhow::Result;
use async_trait::async_trait;

/// Trait for OCR providers
/// Allows for multiple OCR backends (DeepSeek, Tesseract, cloud services, etc.)
#[async_trait]
pub trait OcrProvider: Send + Sync {
    /// Process an OCR request and return the extracted text
    async fn process(&self, request: OcrRequest) -> Result<OcrResult>;

    /// Get the name of this OCR provider
    fn name(&self) -> &str;

    /// Check if the provider is configured and ready to use
    fn is_ready(&self) -> bool;

    /// Get any error or status message
    fn status_message(&self) -> Option<String> {
        None
    }
}

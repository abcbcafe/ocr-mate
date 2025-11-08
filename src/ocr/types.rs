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

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    fn create_test_image() -> DynamicImage {
        let img = RgbaImage::new(10, 10);
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_ocr_request_new() {
        let img = create_test_image();
        let request = OcrRequest::new(img);

        assert!(request.system_prompt.is_none());
        assert!(request.hints.is_empty());
        assert!(request.language.is_none());
    }

    #[test]
    fn test_ocr_request_builder_pattern() {
        let img = create_test_image();
        let request = OcrRequest::new(img)
            .with_system_prompt("Extract text".to_string())
            .with_hints(vec!["Hint 1".to_string(), "Hint 2".to_string()])
            .with_language("en".to_string());

        assert_eq!(request.system_prompt, Some("Extract text".to_string()));
        assert_eq!(request.hints.len(), 2);
        assert_eq!(request.language, Some("en".to_string()));
    }

    #[test]
    fn test_ocr_result_new() {
        let result = OcrResult::new("Sample text".to_string());

        assert_eq!(result.text, "Sample text");
        assert!(result.confidence.is_none());
        assert_eq!(result.processing_time_ms, 0);
        assert!(result.metadata.is_null());
    }

    #[test]
    fn test_ocr_result_with_confidence() {
        let result = OcrResult {
            text: "Sample text".to_string(),
            confidence: Some(0.95),
            processing_time_ms: 1000,
            metadata: serde_json::json!({"key": "value"}),
        };

        assert_eq!(result.confidence, Some(0.95));
        assert_eq!(result.processing_time_ms, 1000);
        assert!(!result.metadata.is_null());
    }

    #[test]
    fn test_ocr_result_serialization() {
        let result = OcrResult::new("Test".to_string());
        let json = serde_json::to_string(&result).unwrap();

        assert!(json.contains("\"text\":\"Test\""));
        assert!(json.contains("\"processing_time_ms\":0"));
    }

    #[test]
    fn test_ocr_result_deserialization() {
        let json = r#"{
            "text": "Deserialized text",
            "confidence": 0.85,
            "processing_time_ms": 500,
            "metadata": null
        }"#;

        let result: OcrResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.text, "Deserialized text");
        assert_eq!(result.confidence, Some(0.85));
        assert_eq!(result.processing_time_ms, 500);
    }

    #[test]
    fn test_ocr_response_deserialization() {
        let json = r#"{"text": "Response text", "confidence": 0.9}"#;
        let response: OcrResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.text, "Response text");
        assert_eq!(response.confidence, Some(0.9));
    }

    #[test]
    fn test_ocr_response_without_confidence() {
        let json = r#"{"text": "Response without confidence"}"#;
        let response: OcrResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.text, "Response without confidence");
        assert!(response.confidence.is_none());
    }
}

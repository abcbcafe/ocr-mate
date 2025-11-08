// Integration tests for OCR workflow

use ocr_mate::ocr::{OcrConfig, OcrRequest, OcrResult};
use image::RgbaImage;

#[path = "../common/mod.rs"]
mod common;

#[test]
fn test_ocr_request_creation() {
    let img = common::create_test_image();
    let request = OcrRequest::new(img)
        .with_system_prompt("Extract all text".to_string())
        .with_language("en".to_string());

    assert_eq!(request.system_prompt, Some("Extract all text".to_string()));
    assert_eq!(request.language, Some("en".to_string()));
}

#[test]
fn test_ocr_result_creation() {
    let result = OcrResult {
        text: "Sample extracted text".to_string(),
        confidence: Some(0.92),
        processing_time_ms: 1500,
        metadata: serde_json::json!({"provider": "test"}),
    };

    assert!(!result.text.is_empty());
    assert!(result.confidence.unwrap() > 0.9);
    assert!(result.processing_time_ms > 0);
}

#[test]
fn test_ocr_config_validation() {
    let mut config = OcrConfig::default();

    // Default config is invalid (no API key)
    assert!(!config.is_valid());

    // Make it valid
    config.api_key = "test_api_key".to_string();
    assert!(config.is_valid());
}

#[test]
fn test_ocr_request_with_hints() {
    let img = common::create_test_image();
    let hints = vec!["Contains table".to_string(), "Mathematical notation".to_string()];

    let request = OcrRequest::new(img).with_hints(hints.clone());

    assert_eq!(request.hints.len(), 2);
    assert_eq!(request.hints, hints);
}

#[test]
fn test_multiple_ocr_requests() {
    let img1 = common::create_test_image();
    let img2 = RgbaImage::new(50, 50);
    let img2_dynamic = image::DynamicImage::ImageRgba8(img2);

    let request1 = OcrRequest::new(img1);
    let request2 = OcrRequest::new(img2_dynamic);

    assert_eq!(request1.image.width(), 100);
    assert_eq!(request2.image.width(), 50);
}

// Note: Actual API tests require a valid API key and network access
// These are marked as ignored and should be run manually with proper setup
#[test]
#[ignore = "Requires valid HuggingFace API key and network access"]
fn test_deepseek_ocr_integration() {
    // This test would make an actual API call
    // Skipped by default to avoid network dependencies in CI/CD
    use ocr_mate::ocr::DeepSeekOcr;

    let api_key = std::env::var("HUGGINGFACE_API_KEY").ok();
    if api_key.is_none() {
        println!("Skipping: HUGGINGFACE_API_KEY not set");
        return;
    }

    // Integration test code would go here
}

#[test]
fn test_ocr_result_serialization() {
    let result = OcrResult {
        text: "Test text".to_string(),
        confidence: Some(0.88),
        processing_time_ms: 2000,
        metadata: serde_json::json!({
            "model": "test-model",
            "version": "1.0"
        }),
    };

    let json = serde_json::to_string(&result).unwrap();
    let deserialized: OcrResult = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.text, result.text);
    assert_eq!(deserialized.confidence, result.confidence);
    assert_eq!(deserialized.processing_time_ms, result.processing_time_ms);
}

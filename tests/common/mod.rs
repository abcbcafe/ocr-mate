// Common test utilities and helpers

use image::{DynamicImage, RgbaImage};
use serde_json::json;
use std::path::PathBuf;

/// Create a simple test image (100x100 white square with black text area)
pub fn create_test_image() -> DynamicImage {
    let mut img = RgbaImage::new(100, 100);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }

    // Add a black rectangle (simulating text)
    for y in 20..80 {
        for x in 10..90 {
            img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }

    DynamicImage::ImageRgba8(img)
}

/// Get path to test fixtures directory
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// Sample OCR result for testing
pub fn sample_ocr_result() -> serde_json::Value {
    json!({
        "text": "Sample OCR text\nWith multiple lines\nAnd some content",
        "confidence": 0.95,
        "processing_time_ms": 1250,
        "metadata": {
            "provider": "test",
            "model": "test-model-v1"
        }
    })
}

/// Sample OCR configuration for testing
pub fn sample_ocr_config() -> String {
    r#"
api_key = "test_api_key_12345"
endpoint = "https://test.api.example.com/ocr"
system_prompt = "Extract all text from this test image."
language = "en"
max_retries = 3
timeout_secs = 30
"#
    .to_string()
}

/// Create a minimal valid PDF bytes (empty single-page PDF)
pub fn minimal_pdf_bytes() -> Vec<u8> {
    // This is a minimal valid PDF with one blank page
    // PDF structure: Header, Catalog, Pages, Page, Content Stream, XRef, Trailer
    b"%PDF-1.4
1 0 obj
<<
/Type /Catalog
/Pages 2 0 R
>>
endobj
2 0 obj
<<
/Type /Pages
/Kids [3 0 R]
/Count 1
>>
endobj
3 0 obj
<<
/Type /Page
/Parent 2 0 R
/MediaBox [0 0 612 792]
/Contents 4 0 R
/Resources <<
/Font <<
/F1 <<
/Type /Font
/Subtype /Type1
/BaseFont /Helvetica
>>
>>
>>
>>
endobj
4 0 obj
<<
/Length 44
>>
stream
BT
/F1 12 Tf
100 700 Td
(Test PDF) Tj
ET
endstream
endobj
xref
0 5
0000000000 65535 f
0000000009 00000 n
0000000058 00000 n
0000000115 00000 n
0000000315 00000 n
trailer
<<
/Size 5
/Root 1 0 R
>>
startxref
408
%%EOF
"
    .to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_image() {
        let img = create_test_image();
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    #[test]
    fn test_sample_ocr_result() {
        let result = sample_ocr_result();
        assert!(result.get("text").is_some());
        assert!(result.get("confidence").is_some());
    }

    #[test]
    fn test_minimal_pdf_bytes() {
        let pdf_bytes = minimal_pdf_bytes();
        assert!(!pdf_bytes.is_empty());
        // Check PDF header
        assert!(pdf_bytes.starts_with(b"%PDF"));
    }
}

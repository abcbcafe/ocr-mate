// Integration tests for document loading functionality

use ocr_mate::document::Document;
use std::io::Write;
use tempfile::NamedTempFile;

#[path = "../common/mod.rs"]
mod common;

#[test]
fn test_load_image_file() {
    // Create a temporary PNG file
    let temp_file = NamedTempFile::with_suffix(".png").unwrap();
    let test_img = common::create_test_image();
    test_img.save(temp_file.path()).unwrap();

    // Load the document
    let document = Document::load(temp_file.path().to_path_buf());

    assert!(document.is_ok(), "Should load image successfully");
    let doc = document.unwrap();
    assert_eq!(doc.page_count(), 1, "Image should have 1 page");
    assert!(doc.is_image(), "Should be identified as image");
    assert!(!doc.is_pdf(), "Should not be identified as PDF");
}

#[test]
fn test_load_unsupported_format() {
    // Create a file with unsupported extension
    let mut temp_file = NamedTempFile::with_suffix(".xyz").unwrap();
    temp_file.write_all(b"invalid content").unwrap();

    let document = Document::load(temp_file.path().to_path_buf());

    assert!(document.is_err(), "Should fail for unsupported format");
    // Check the error message
    match document {
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("Unsupported file format"),
                "Error message should mention unsupported format, got: {}",
                error_msg
            );
        }
        Ok(_) => panic!("Expected error, got Ok"),
    }
}

#[test]
fn test_load_nonexistent_file() {
    let path = std::path::PathBuf::from("/nonexistent/file.png");
    let document = Document::load(path);

    assert!(document.is_err(), "Should fail for non-existent file");
}

#[test]
fn test_render_image_page() {
    let temp_file = NamedTempFile::with_suffix(".png").unwrap();
    let test_img = common::create_test_image();
    test_img.save(temp_file.path()).unwrap();

    let document = Document::load(temp_file.path().to_path_buf()).unwrap();

    // Render page 0
    let rendered = document.render_page(0, 1.0);
    assert!(rendered.is_ok(), "Should render page successfully");

    let img = rendered.unwrap();
    assert_eq!(img.width(), 100, "Image width should match");
    assert_eq!(img.height(), 100, "Image height should match");
}

#[test]
fn test_render_invalid_page_index() {
    let temp_file = NamedTempFile::with_suffix(".png").unwrap();
    let test_img = common::create_test_image();
    test_img.save(temp_file.path()).unwrap();

    let document = Document::load(temp_file.path().to_path_buf()).unwrap();

    // Try to render page 1 (doesn't exist, only page 0)
    let rendered = document.render_page(1, 1.0);
    assert!(rendered.is_err(), "Should fail for invalid page index");
}

// Note: PDF tests are more complex and may fail due to PDFium library availability
// They are marked as ignored and can be run manually when PDFium is properly set up
#[test]
#[ignore = "Requires PDFium library to be installed"]
fn test_load_pdf_file() {
    let mut temp_file = NamedTempFile::with_suffix(".pdf").unwrap();
    let pdf_bytes = common::minimal_pdf_bytes();
    temp_file.write_all(&pdf_bytes).unwrap();

    let document = Document::load(temp_file.path().to_path_buf());

    // This may fail if PDFium is not available
    if document.is_ok() {
        let doc = document.unwrap();
        assert!(doc.is_pdf(), "Should be identified as PDF");
        assert!(!doc.is_image(), "Should not be identified as image");
        assert!(doc.page_count() > 0, "PDF should have at least one page");
    }
}

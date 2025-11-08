// Integration tests for the complete export pipeline

use ocr_mate::export::{export_to_file, ExportFormat};
use tempfile::TempDir;

#[path = "../common/mod.rs"]
mod common;

#[test]
fn test_export_pipeline_plain_text() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.txt");

    let pages = vec![
        "First page content".to_string(),
        "Second page content".to_string(),
        "Third page content".to_string(),
    ];

    let result = export_to_file(&output_path, ExportFormat::PlainText, &pages, None);

    assert!(result.is_ok(), "Export should succeed");
    assert!(output_path.exists(), "Output file should exist");

    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("First page content"));
    assert!(content.contains("Second page content"));
    assert!(content.contains("Third page content"));
}

#[test]
fn test_export_pipeline_markdown() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.md");

    let pages = vec![
        "# Document Title\nSome content".to_string(),
        "More content on page 2".to_string(),
    ];

    let result = export_to_file(&output_path, ExportFormat::Markdown, &pages, None);

    assert!(result.is_ok(), "Export should succeed");

    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("## Page 1"));
    assert!(content.contains("## Page 2"));
    assert!(content.contains("Document Title"));
}

#[test]
fn test_export_pipeline_json_with_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.json");

    let pages = vec!["Page content".to_string()];
    let metadata = Some(serde_json::json!({
        "source_file": "test.pdf",
        "ocr_provider": "deepseek-ocr",
        "processing_date": "2024-01-01"
    }));

    let result = export_to_file(&output_path, ExportFormat::Json, &pages, metadata);

    assert!(result.is_ok(), "Export should succeed");

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(json["version"], "1.0");
    assert_eq!(json["page_count"], 1);
    assert_eq!(json["pages"][0]["text"], "Page content");
    assert_eq!(json["metadata"]["source_file"], "test.pdf");
    assert_eq!(json["metadata"]["ocr_provider"], "deepseek-ocr");
}

#[test]
fn test_export_all_formats() {
    let temp_dir = TempDir::new().unwrap();
    let pages = vec!["Test content".to_string()];

    let formats = vec![
        (ExportFormat::PlainText, "test.txt"),
        (ExportFormat::Markdown, "test.md"),
        (ExportFormat::Json, "test.json"),
    ];

    for (format, filename) in formats {
        let output_path = temp_dir.path().join(filename);
        let result = export_to_file(&output_path, format, &pages, None);

        assert!(result.is_ok(), "Export for {:?} should succeed", format);
        assert!(output_path.exists(), "File {} should exist", filename);

        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(
            !content.is_empty(),
            "File {} should not be empty",
            filename
        );
    }
}

#[test]
fn test_export_empty_content() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("empty.txt");

    let pages: Vec<String> = vec![];

    let result = export_to_file(&output_path, ExportFormat::PlainText, &pages, None);

    assert!(result.is_ok(), "Export should succeed even with empty content");
    assert!(output_path.exists(), "Output file should exist");
}

#[test]
fn test_export_large_content() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("large.txt");

    // Create many pages with substantial content
    let pages: Vec<String> = (0..100)
        .map(|i| format!("Page {} content with lots of text here.\n{}", i + 1, "Lorem ipsum ".repeat(100)))
        .collect();

    let result = export_to_file(&output_path, ExportFormat::PlainText, &pages, None);

    assert!(result.is_ok(), "Export should handle large content");

    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.len() > 10000, "Large content should be written");
}

#[test]
fn test_export_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("special.txt");

    let pages = vec![
        "Content with special chars: @#$%^&*()".to_string(),
        "Unicode: 日本語 中文 한국어 العربية".to_string(),
        "Emojis: 🎉 🚀 📄 ✨".to_string(),
    ];

    let result = export_to_file(&output_path, ExportFormat::PlainText, &pages, None);

    assert!(result.is_ok(), "Export should handle special characters");

    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("@#$%^&*()"));
    assert!(content.contains("日本語"));
    assert!(content.contains("🎉"));
}

#[test]
fn test_export_overwrite_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("overwrite.txt");

    // Write first content
    let pages1 = vec!["First content".to_string()];
    export_to_file(&output_path, ExportFormat::PlainText, &pages1, None).unwrap();

    // Overwrite with second content
    let pages2 = vec!["Second content".to_string()];
    let result = export_to_file(&output_path, ExportFormat::PlainText, &pages2, None);

    assert!(result.is_ok(), "Should overwrite existing file");

    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Second content"));
    assert!(!content.contains("First content"));
}

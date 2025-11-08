mod formats;

pub use formats::ExportFormat;

use anyhow::Result;
use std::path::Path;

/// Export OCR results to a file
pub fn export_to_file(
    path: &Path,
    format: ExportFormat,
    pages: &[String],
    metadata: Option<serde_json::Value>,
) -> Result<()> {
    match format {
        ExportFormat::PlainText => export_plain_text(path, pages),
        ExportFormat::Markdown => export_markdown(path, pages),
        ExportFormat::Json => export_json(path, pages, metadata),
    }
}

fn export_plain_text(path: &Path, pages: &[String]) -> Result<()> {
    let content = pages.join("\n\n---\n\n");
    std::fs::write(path, content)?;
    Ok(())
}

fn export_markdown(path: &Path, pages: &[String]) -> Result<()> {
    let mut content = String::new();

    for (i, page) in pages.iter().enumerate() {
        if i > 0 {
            content.push_str("\n\n---\n\n");
        }
        content.push_str(&format!("## Page {}\n\n", i + 1));
        content.push_str(page);
    }

    std::fs::write(path, content)?;
    Ok(())
}

fn export_json(path: &Path, pages: &[String], metadata: Option<serde_json::Value>) -> Result<()> {
    let export_data = serde_json::json!({
        "version": "1.0",
        "page_count": pages.len(),
        "pages": pages.iter().enumerate().map(|(i, text)| {
            serde_json::json!({
                "page_number": i + 1,
                "text": text,
            })
        }).collect::<Vec<_>>(),
        "metadata": metadata.unwrap_or(serde_json::Value::Null),
        "exported_at": chrono::Utc::now().to_rfc3339(),
    });

    let content = serde_json::to_string_pretty(&export_data)?;
    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_plain_text_single_page() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec!["This is page one".to_string()];

        export_to_file(temp_file.path(), ExportFormat::PlainText, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(content, "This is page one");
    }

    #[test]
    fn test_export_plain_text_multiple_pages() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec![
            "Page 1 content".to_string(),
            "Page 2 content".to_string(),
            "Page 3 content".to_string(),
        ];

        export_to_file(temp_file.path(), ExportFormat::PlainText, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("Page 1 content"));
        assert!(content.contains("Page 2 content"));
        assert!(content.contains("Page 3 content"));
        assert!(content.contains("\n\n---\n\n"));
    }

    #[test]
    fn test_export_markdown_single_page() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec!["Markdown page content".to_string()];

        export_to_file(temp_file.path(), ExportFormat::Markdown, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("## Page 1"));
        assert!(content.contains("Markdown page content"));
    }

    #[test]
    fn test_export_markdown_multiple_pages() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec![
            "First page".to_string(),
            "Second page".to_string(),
        ];

        export_to_file(temp_file.path(), ExportFormat::Markdown, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("## Page 1"));
        assert!(content.contains("## Page 2"));
        assert!(content.contains("First page"));
        assert!(content.contains("Second page"));
    }

    #[test]
    fn test_export_json_structure() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec!["JSON page content".to_string()];

        export_to_file(temp_file.path(), ExportFormat::Json, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(json["version"], "1.0");
        assert_eq!(json["page_count"], 1);
        assert!(json["pages"].is_array());
        assert_eq!(json["pages"][0]["page_number"], 1);
        assert_eq!(json["pages"][0]["text"], "JSON page content");
        assert!(json.get("exported_at").is_some());
    }

    #[test]
    fn test_export_json_with_metadata() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec!["Content".to_string()];
        let metadata = Some(serde_json::json!({
            "document": "test.pdf",
            "processor": "test"
        }));

        export_to_file(temp_file.path(), ExportFormat::Json, &pages, metadata).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(json["metadata"]["document"], "test.pdf");
        assert_eq!(json["metadata"]["processor"], "test");
    }

    #[test]
    fn test_export_json_without_metadata() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages = vec!["Content".to_string()];

        export_to_file(temp_file.path(), ExportFormat::Json, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert!(json["metadata"].is_null());
    }

    #[test]
    fn test_export_empty_pages() {
        let temp_file = NamedTempFile::new().unwrap();
        let pages: Vec<String> = vec![];

        export_to_file(temp_file.path(), ExportFormat::PlainText, &pages, None).unwrap();

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn test_format_extensions() {
        assert_eq!(ExportFormat::PlainText.extension(), "txt");
        assert_eq!(ExportFormat::Markdown.extension(), "md");
        assert_eq!(ExportFormat::Json.extension(), "json");
    }

    #[test]
    fn test_format_names() {
        assert_eq!(ExportFormat::PlainText.name(), "Plain Text");
        assert_eq!(ExportFormat::Markdown.name(), "Markdown");
        assert_eq!(ExportFormat::Json.name(), "JSON");
    }
}

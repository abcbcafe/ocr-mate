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

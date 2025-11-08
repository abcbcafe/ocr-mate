/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// Plain text (.txt)
    PlainText,

    /// Markdown (.md)
    Markdown,

    /// JSON with metadata (.json)
    Json,
}

impl ExportFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::PlainText => "txt",
            ExportFormat::Markdown => "md",
            ExportFormat::Json => "json",
        }
    }

    /// Get a human-readable name for this format
    pub fn name(&self) -> &str {
        match self {
            ExportFormat::PlainText => "Plain Text",
            ExportFormat::Markdown => "Markdown",
            ExportFormat::Json => "JSON",
        }
    }
}

// Library exports for OCR-Mate
// This allows modules to be tested and potentially used as a library

pub mod document;
pub mod export;
pub mod ocr;
pub mod utils;

// Re-export commonly used types
pub use document::{Document, Page, PageRenderer};
pub use export::{export_to_file, ExportFormat};
pub use ocr::{DeepSeekOcr, OcrConfig, OcrProvider, OcrRequest, OcrResult};

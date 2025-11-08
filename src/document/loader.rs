use anyhow::{Context, Result};
use image::DynamicImage;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

/// Represents a loaded document (PDF or image)
pub struct Document {
    /// Path to the document file
    path: PathBuf,

    /// Document type
    doc_type: DocumentType,

    /// PDF document (if applicable)
    pdf: Option<PdfDocument<'static>>,

    /// Single image (if applicable)
    image: Option<DynamicImage>,
}

enum DocumentType {
    Pdf,
    Image,
}

impl Document {
    /// Load a document from a file path
    pub fn load(path: PathBuf) -> Result<Self> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "pdf" => Self::load_pdf(path),
            "png" | "jpg" | "jpeg" | "tiff" | "tif" | "webp" => Self::load_image(path),
            _ => anyhow::bail!("Unsupported file format: {}", extension),
        }
    }

    fn load_pdf(path: PathBuf) -> Result<Self> {
        // Initialize pdfium and leak it to get 'static lifetime
        // This is necessary because PdfDocument borrows from Pdfium
        let pdfium = Box::leak(Box::new(Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .context("Failed to load PDFium library")?,
        )));

        // Load PDF document
        let pdf = pdfium
            .load_pdf_from_file(&path, None)
            .context("Failed to load PDF file")?;

        Ok(Self {
            path,
            doc_type: DocumentType::Pdf,
            pdf: Some(pdf),
            image: None,
        })
    }

    fn load_image(path: PathBuf) -> Result<Self> {
        let image = image::open(&path).context("Failed to load image file")?;

        Ok(Self {
            path,
            doc_type: DocumentType::Image,
            pdf: None,
            image: Some(image),
        })
    }

    /// Get the number of pages in the document
    pub fn page_count(&self) -> usize {
        match self.doc_type {
            DocumentType::Pdf => {
                if let Some(pdf) = &self.pdf {
                    pdf.pages().len() as usize
                } else {
                    0
                }
            }
            DocumentType::Image => 1,
        }
    }

    /// Render a page to an image
    pub fn render_page(&self, page_index: usize, scale: f32) -> Result<DynamicImage> {
        match self.doc_type {
            DocumentType::Pdf => {
                let pdf = self
                    .pdf
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("PDF not loaded"))?;

                let page = pdf
                    .pages()
                    .get(page_index as u16)
                    .context("Page index out of bounds")?;

                // Render at specified scale
                let render_config = PdfRenderConfig::new()
                    .set_target_width((page.width().value * scale) as i32)
                    .set_maximum_height((page.height().value * scale) as i32)
                    .rotate_if_landscape(PdfPageRenderRotation::None, true);

                let bitmap = page
                    .render_with_config(&render_config)
                    .context("Failed to render PDF page")?;

                // Convert to DynamicImage
                let image = bitmap.as_image();
                Ok(image)
            }
            DocumentType::Image => {
                if page_index != 0 {
                    anyhow::bail!("Image documents only have one page");
                }

                self.image
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Image not loaded"))
            }
        }
    }

    /// Get the document path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if this is a PDF document
    pub fn is_pdf(&self) -> bool {
        matches!(self.doc_type, DocumentType::Pdf)
    }

    /// Check if this is an image document
    pub fn is_image(&self) -> bool {
        matches!(self.doc_type, DocumentType::Image)
    }
}

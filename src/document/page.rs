use image::DynamicImage;

/// Represents a single page in a document
#[derive(Clone)]
pub struct Page {
    /// Page number (1-based)
    pub number: usize,

    /// Rendered image of the page
    pub image: DynamicImage,

    /// Page width in points (PDF) or pixels (image)
    pub width: f32,

    /// Page height in points (PDF) or pixels (image)
    pub height: f32,
}

impl Page {
    pub fn new(number: usize, image: DynamicImage) -> Self {
        let width = image.width() as f32;
        let height = image.height() as f32;

        Self {
            number,
            image,
            width,
            height,
        }
    }

    /// Get the aspect ratio of the page
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }
}

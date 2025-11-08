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

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = RgbaImage::new(width, height);
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_page_new() {
        let img = create_test_image(100, 200);
        let page = Page::new(1, img);

        assert_eq!(page.number, 1);
        assert_eq!(page.width, 100.0);
        assert_eq!(page.height, 200.0);
    }

    #[test]
    fn test_page_aspect_ratio_portrait() {
        let img = create_test_image(100, 200);
        let page = Page::new(1, img);

        assert_eq!(page.aspect_ratio(), 0.5);
    }

    #[test]
    fn test_page_aspect_ratio_landscape() {
        let img = create_test_image(200, 100);
        let page = Page::new(1, img);

        assert_eq!(page.aspect_ratio(), 2.0);
    }

    #[test]
    fn test_page_aspect_ratio_square() {
        let img = create_test_image(100, 100);
        let page = Page::new(1, img);

        assert_eq!(page.aspect_ratio(), 1.0);
    }

    #[test]
    fn test_page_clone() {
        let img = create_test_image(50, 75);
        let page1 = Page::new(5, img);
        let page2 = page1.clone();

        assert_eq!(page2.number, 5);
        assert_eq!(page2.width, 50.0);
        assert_eq!(page2.height, 75.0);
    }

    #[test]
    fn test_page_different_numbers() {
        let img1 = create_test_image(100, 100);
        let img2 = create_test_image(100, 100);

        let page1 = Page::new(1, img1);
        let page2 = Page::new(2, img2);

        assert_eq!(page1.number, 1);
        assert_eq!(page2.number, 2);
    }
}

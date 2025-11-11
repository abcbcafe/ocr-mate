/// Page renderer for converting document pages to images
/// (Simplified for iced - no texture caching needed)
pub struct PageRenderer;

impl PageRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PageRenderer {
    fn default() -> Self {
        Self::new()
    }
}

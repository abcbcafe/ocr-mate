// Split view implementation
// This module provides utilities for creating resizable split panels
// Currently handled directly in app.rs, but can be extracted here for reusability

pub struct SplitView {
    /// Split ratio (0.0 to 1.0)
    ratio: f32,

    /// Minimum size for each panel
    min_size: f32,

    /// Is the divider being dragged
    dragging: bool,
}

impl SplitView {
    pub fn new() -> Self {
        Self {
            ratio: 0.5,
            min_size: 200.0,
            dragging: false,
        }
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.clamp(0.1, 0.9);
    }
}

impl Default for SplitView {
    fn default() -> Self {
        Self::new()
    }
}

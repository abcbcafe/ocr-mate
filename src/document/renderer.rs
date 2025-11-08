use egui::Context;

/// Page renderer for converting document pages to egui textures
pub struct PageRenderer {
    /// egui context
    ctx: Context,
}

impl PageRenderer {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    /// Get the egui context
    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
}

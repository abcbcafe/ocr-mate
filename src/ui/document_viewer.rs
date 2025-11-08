use crate::document::Document;
use egui::{ColorImage, TextureHandle, Vec2};

pub struct DocumentViewer {
    /// Zoom level (1.0 = 100%)
    zoom: f32,

    /// Pan offset
    offset: Vec2,

    /// Cached texture for current page
    current_texture: Option<TextureHandle>,

    /// Last rendered page index
    last_page_index: Option<usize>,

    /// egui context for texture management
    ctx: egui::Context,
}

impl DocumentViewer {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
            current_texture: None,
            last_page_index: None,
            ctx: egui::Context::default(),
        }
    }

    pub fn ctx(&self) -> &egui::Context {
        &self.ctx
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, document: &Option<Document>, current_page: usize) {
        // Store context
        self.ctx = ui.ctx().clone();

        let available_size = ui.available_size();

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(doc) = document {
                    // Check if we need to render a new page
                    if self.last_page_index != Some(current_page) {
                        self.render_page(ui, doc, current_page);
                    }

                    // Display the texture
                    if let Some(texture) = &self.current_texture {
                        let size = texture.size_vec2() * self.zoom;
                        let image = egui::Image::new(texture).fit_to_exact_size(size);
                        ui.add(image);
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.spinner();
                            ui.label("Rendering page...");
                        });
                    }
                } else {
                    // No document loaded - show placeholder
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("No document loaded");
                            ui.add_space(10.0);
                            ui.label("Click 'Open Document' to get started");
                        });
                    });
                }
            });

        // Zoom controls overlay
        if document.is_some() {
            egui::Window::new("Zoom Controls")
                .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -10.0])
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("−").clicked() {
                            self.zoom = (self.zoom * 0.8).max(0.1);
                        }
                        ui.label(format!("{:.0}%", self.zoom * 100.0));
                        if ui.button("+").clicked() {
                            self.zoom = (self.zoom * 1.25).min(5.0);
                        }
                        if ui.button("Fit").clicked() {
                            self.zoom = 1.0;
                        }
                    });
                });
        }
    }

    fn render_page(&mut self, ui: &mut egui::Ui, document: &Document, page_index: usize) {
        // Render page to image
        match document.render_page(page_index, 2.0) {
            Ok(image) => {
                let size = [image.width() as usize, image.height() as usize];
                let pixels = image.to_rgba8().into_raw();

                let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

                let texture = ui.ctx().load_texture(
                    format!("page_{}", page_index),
                    color_image,
                    egui::TextureOptions::LINEAR,
                );

                self.current_texture = Some(texture);
                self.last_page_index = Some(page_index);
            }
            Err(e) => {
                tracing::error!("Failed to render page {}: {}", page_index, e);
            }
        }
    }
}

impl Default for DocumentViewer {
    fn default() -> Self {
        Self::new()
    }
}

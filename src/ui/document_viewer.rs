use crate::document::Document;
use egui::{ColorImage, TextureHandle, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomMode {
    Manual,
    FitHeight,
    FitWidth,
}

pub struct DocumentViewer {
    /// Zoom level (1.0 = 100%)
    zoom: f32,

    /// Zoom mode
    zoom_mode: ZoomMode,

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
            zoom_mode: ZoomMode::FitHeight,
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

        if let Some(doc) = document {
            // Check if we need to render a new page
            if self.last_page_index != Some(current_page) {
                self.render_page(ui, doc, current_page);
            }

            // Display the texture
            if let Some(texture) = &self.current_texture {
                let texture_size = texture.size_vec2();

                // Calculate zoom based on mode with division-by-zero protection
                let actual_zoom = match self.zoom_mode {
                    ZoomMode::FitHeight => {
                        if texture_size.y > 0.0 {
                            available_size.y / texture_size.y
                        } else {
                            1.0
                        }
                    }
                    ZoomMode::FitWidth => {
                        if texture_size.x > 0.0 {
                            available_size.x / texture_size.x
                        } else {
                            1.0
                        }
                    }
                    ZoomMode::Manual => self.zoom,
                };

                let size = texture_size * actual_zoom;

                // Configure scroll area based on zoom mode
                let (scroll_horizontal, scroll_vertical) = match self.zoom_mode {
                    ZoomMode::FitHeight => (true, false),   // Only horizontal scroll
                    ZoomMode::FitWidth => (false, true),    // Only vertical scroll
                    ZoomMode::Manual => (true, true),       // Both directions
                };

                egui::ScrollArea::new([scroll_horizontal, scroll_vertical])
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Center vertically when in FitHeight mode and horizontally when in FitWidth mode
                        match self.zoom_mode {
                            ZoomMode::FitHeight => {
                                // Center the image vertically if it's smaller than viewport
                                let vertical_padding = (available_size.y - size.y).max(0.0) / 2.0;
                                ui.add_space(vertical_padding);

                                // Center horizontally
                                ui.vertical_centered(|ui| {
                                    ui.add(egui::Image::new(texture).fit_to_exact_size(size));
                                });
                            }
                            ZoomMode::FitWidth => {
                                // Center horizontally if it's smaller than viewport
                                ui.horizontal_centered(|ui| {
                                    ui.vertical(|ui| {
                                        let horizontal_padding = (available_size.x - size.x).max(0.0) / 2.0;
                                        ui.add_space(horizontal_padding);
                                        ui.add(egui::Image::new(texture).fit_to_exact_size(size));
                                    });
                                });
                            }
                            ZoomMode::Manual => {
                                // No automatic centering in manual mode - scroll areas handle it
                                ui.vertical_centered(|ui| {
                                    ui.add(egui::Image::new(texture).fit_to_exact_size(size));
                                });
                            }
                        }
                    });

                // Store the actual zoom for display
                self.zoom = actual_zoom;
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
                            self.zoom_mode = ZoomMode::Manual;
                            self.zoom = (self.zoom * 0.8).max(0.1);
                        }
                        ui.label(format!("{:.0}%", self.zoom * 100.0));
                        if ui.button("+").clicked() {
                            self.zoom_mode = ZoomMode::Manual;
                            self.zoom = (self.zoom * 1.25).min(5.0);
                        }
                        ui.separator();
                        if ui.button("Fit Height").clicked() {
                            self.zoom_mode = ZoomMode::FitHeight;
                        }
                        if ui.button("Fit Width").clicked() {
                            self.zoom_mode = ZoomMode::FitWidth;
                        }
                        if ui.button("100%").clicked() {
                            self.zoom_mode = ZoomMode::Manual;
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

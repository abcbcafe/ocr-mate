use crate::app::OcrMateApp;
use crate::export::ExportFormat;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut OcrMateApp) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;

            // File operations
            if ui.button("📁 Open Document").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Documents", &["pdf", "png", "jpg", "jpeg", "tiff", "webp"])
                    .pick_file()
                {
                    if let Err(e) = app.load_document(path) {
                        tracing::error!("Failed to load document: {}", e);
                    }
                }
            }

            ui.separator();

            // OCR operations
            let has_document = app.document.is_some();
            ui.add_enabled_ui(has_document && !app.ocr_in_progress, |ui| {
                if ui.button("🔍 OCR Current Page").clicked() {
                    app.ocr_current_page();
                }

                if ui.button("🔍 OCR All Pages").clicked() {
                    app.ocr_all_pages();
                }
            });

            ui.separator();

            // Settings
            if ui.button("⚙️ Settings").clicked() {
                app.show_settings = true;
            }

            ui.separator();

            // Export
            ui.add_enabled_ui(has_document, |ui| {
                ui.menu_button("💾 Export", |ui| {
                    if ui.button("Plain Text (.txt)").clicked() {
                        if let Err(e) = app.export(ExportFormat::PlainText) {
                            tracing::error!("Export failed: {}", e);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Markdown (.md)").clicked() {
                        if let Err(e) = app.export(ExportFormat::Markdown) {
                            tracing::error!("Export failed: {}", e);
                        }
                        ui.close_menu();
                    }
                    if ui.button("JSON (.json)").clicked() {
                        if let Err(e) = app.export(ExportFormat::Json) {
                            tracing::error!("Export failed: {}", e);
                        }
                        ui.close_menu();
                    }
                });
            });

            // Page navigation
            if has_document {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let page_count = app.document.as_ref().unwrap().page_count();

                    if ui
                        .add_enabled(app.current_page < page_count - 1, egui::Button::new("→"))
                        .clicked()
                    {
                        app.current_page += 1;
                    }

                    ui.label(format!("{} / {}", app.current_page + 1, page_count));

                    if ui
                        .add_enabled(app.current_page > 0, egui::Button::new("←"))
                        .clicked()
                    {
                        app.current_page = app.current_page.saturating_sub(1);
                    }

                    ui.label("Page:");
                });
            }
        });
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

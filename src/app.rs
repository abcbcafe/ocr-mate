use crate::document::{Document, PageRenderer};
use crate::ocr::{DeepSeekOcr, OcrConfig};
use crate::ui::{DocumentViewer, EditorPanel, Toolbar};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main application state
pub struct OcrMateApp {
    /// Currently loaded document
    pub document: Option<Document>,

    /// Page renderer for converting pages to textures
    pub page_renderer: PageRenderer,

    /// Currently selected page index (0-based)
    pub current_page: usize,

    /// OCR results for each page
    pub ocr_results: Vec<String>,

    /// OCR provider (DeepSeek by default)
    pub ocr_provider: Arc<RwLock<DeepSeekOcr>>,

    /// OCR configuration (instructions, hints)
    pub ocr_config: OcrConfig,

    /// OCR processing state
    pub ocr_in_progress: bool,
    pub ocr_progress: f32,
    pub ocr_status: String,

    /// UI components
    pub document_viewer: DocumentViewer,
    pub editor_panel: EditorPanel,

    /// Show settings dialog
    pub show_settings: bool,

    /// Tokio runtime for async operations
    pub runtime: tokio::runtime::Runtime,
}

impl OcrMateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure egui style
        configure_style(&cc.egui_ctx);

        // Create tokio runtime
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

        // Initialize OCR provider
        let ocr_provider = Arc::new(RwLock::new(DeepSeekOcr::new()));

        Self {
            document: None,
            page_renderer: PageRenderer::new(cc.egui_ctx.clone()),
            current_page: 0,
            ocr_results: Vec::new(),
            ocr_provider,
            ocr_config: OcrConfig::default(),
            ocr_in_progress: false,
            ocr_progress: 0.0,
            ocr_status: String::new(),
            document_viewer: DocumentViewer::new(),
            editor_panel: EditorPanel::new(),
            show_settings: false,
            runtime,
        }
    }

    /// Load a document from a file path
    pub fn load_document(&mut self, path: std::path::PathBuf) -> Result<()> {
        tracing::info!("Loading document: {:?}", path);

        let document = Document::load(path)?;
        let page_count = document.page_count();

        self.document = Some(document);
        self.current_page = 0;
        self.ocr_results = vec![String::new(); page_count];

        tracing::info!("Document loaded with {} pages", page_count);
        Ok(())
    }

    /// Run OCR on the current page
    pub fn ocr_current_page(&mut self) {
        if let Some(doc) = &self.document {
            self.ocr_in_progress = true;
            self.ocr_status = format!("Processing page {}...", self.current_page + 1);

            let page_index = self.current_page;
            let image = match doc.render_page(page_index, 2.0) {
                Ok(img) => img,
                Err(e) => {
                    tracing::error!("Failed to render page: {}", e);
                    self.ocr_in_progress = false;
                    self.ocr_status = format!("Error: {}", e);
                    return;
                }
            };

            let ocr_provider = Arc::clone(&self.ocr_provider);
            let ocr_config = self.ocr_config.clone();

            // Spawn async task
            let ctx = self.document_viewer.ctx().clone();
            self.runtime.spawn(async move {
                // This is a placeholder - actual implementation will call the OCR API
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                ctx.request_repaint();
            });
        }
    }

    /// Run OCR on all pages
    pub fn ocr_all_pages(&mut self) {
        // TODO: Implement batch OCR processing
        tracing::info!("OCR all pages requested");
    }

    /// Export OCR results
    pub fn export(&self, format: crate::export::ExportFormat) -> Result<()> {
        // TODO: Implement export functionality
        tracing::info!("Export requested: {:?}", format);
        Ok(())
    }
}

impl eframe::App for OcrMateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            Toolbar::new().ui(ui, self);
        });

        // Settings dialog
        if self.show_settings {
            egui::Window::new("OCR Settings")
                .collapsible(false)
                .resizable(true)
                .default_width(500.0)
                .show(ctx, |ui| {
                    self.render_settings(ui);
                });
        }

        // Main split view
        egui::CentralPanel::default().show(ctx, |ui| {
            // Split panel: document viewer on left, editor on right
            let available_width = ui.available_width();
            let available_height = ui.available_height();
            let split_ratio = 0.5;

            // Use horizontal top layout that expands to fill available space
            ui.allocate_ui_with_layout(
                egui::vec2(available_width, available_height),
                egui::Layout::left_to_right(egui::Align::Min),
                |ui| {
                    // Left panel: Document viewer
                    ui.allocate_ui(
                        egui::vec2(available_width * split_ratio, available_height),
                        |ui| {
                            egui::Frame::none()
                                .fill(ui.style().visuals.extreme_bg_color)
                                .show(ui, |ui| {
                                    self.document_viewer.ui(ui, &self.document, self.current_page);
                                });
                        },
                    );

                    // Separator
                    ui.separator();

                    // Right panel: Editor
                    ui.allocate_ui(
                        egui::vec2(available_width * split_ratio - 20.0, available_height),
                        |ui| {
                            if let Some(text) = self.ocr_results.get_mut(self.current_page) {
                                self.editor_panel.ui(ui, text);
                            } else {
                                self.editor_panel.ui(ui, &mut String::new());
                            }
                        },
                    );
                },
            );
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.ocr_in_progress {
                    ui.spinner();
                    ui.label(&self.ocr_status);
                    ui.add(egui::ProgressBar::new(self.ocr_progress).show_percentage());
                } else if let Some(doc) = &self.document {
                    ui.label(format!(
                        "Page {} of {} | {}",
                        self.current_page + 1,
                        doc.page_count(),
                        doc.path().display()
                    ));
                } else {
                    ui.label("No document loaded");
                }
            });
        });
    }
}

impl OcrMateApp {
    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("OCR Configuration");
        ui.separator();

        ui.label("System Instructions:");
        ui.text_edit_multiline(&mut self.ocr_config.system_prompt);
        ui.add_space(10.0);

        ui.label("API Key (HuggingFace):");
        ui.text_edit_singleline(&mut self.ocr_config.api_key);
        ui.add_space(10.0);

        ui.label("Model Endpoint:");
        ui.text_edit_singleline(&mut self.ocr_config.endpoint);
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                self.show_settings = false;
                // TODO: Save config to file
            }
            if ui.button("Cancel").clicked() {
                self.show_settings = false;
            }
        });
    }
}

fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    ctx.set_style(style);
}

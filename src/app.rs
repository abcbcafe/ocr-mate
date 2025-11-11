use crate::document::Document;
use crate::ocr::{DeepSeekOcr, OcrConfig};
use crate::export::ExportFormat;
use anyhow::Result;
use iced::{
    widget::{button, column, container, pane_grid, row, scrollable, text, text_editor, text_input},
    Alignment, Element, Length, Task, Theme,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main application state
pub struct OcrMateApp {
    /// Currently loaded document
    pub document: Option<Document>,

    /// Currently selected page index (0-based)
    pub current_page: usize,

    /// OCR results for each page
    pub ocr_results: Vec<String>,

    /// OCR provider (DeepSeek by default)
    pub ocr_provider: Arc<RwLock<DeepSeekOcr>>,

    /// OCR configuration
    pub ocr_config: OcrConfig,

    /// OCR processing state
    pub ocr_in_progress: bool,
    pub ocr_progress: f32,
    pub ocr_status: String,

    /// Zoom state
    pub zoom: f32,
    pub zoom_mode: ZoomMode,

    /// Editor state
    pub editor_content: text_editor::Content,
    pub show_line_numbers: bool,
    pub word_wrap: bool,

    /// UI state
    pub show_settings: bool,
    pub panes: pane_grid::State<PaneContent>,

    /// Cached document image for current page
    pub current_page_image: Option<iced::widget::image::Handle>,

    /// Settings modal inputs
    pub settings_system_prompt: String,
    pub settings_api_key: String,
    pub settings_endpoint: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomMode {
    Manual,
    FitHeight,
    FitWidth,
}

#[derive(Debug, Clone)]
pub enum PaneContent {
    DocumentViewer,
    Editor,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Document operations
    OpenDocument,
    PageRendered(Result<Vec<u8>, String>),

    // Page navigation
    NextPage,
    PreviousPage,

    // OCR operations
    OcrCurrentPage,
    OcrAllPages,
    OcrCompleted(Result<String, String>),
    OcrProgress(f32, String),

    // Export
    ExportAs(ExportFormat),

    // Settings
    OpenSettings,
    CloseSettings,
    SaveSettings,
    UpdateSystemPrompt(String),
    UpdateApiKey(String),
    UpdateEndpoint(String),

    // Zoom
    ZoomIn,
    ZoomOut,
    SetZoomMode(ZoomMode),
    ResetZoom,

    // Editor
    EditorAction(text_editor::Action),
    ToggleLineNumbers,
    ToggleWordWrap,

    // Split pane
    PaneResized(pane_grid::ResizeEvent),
}

impl OcrMateApp {
    pub fn new() -> (Self, Task<Message>) {
        // Initialize pane grid with two panes
        let (mut panes, _first_pane) = pane_grid::State::new(PaneContent::DocumentViewer);
        let _split_result = panes.split(
            pane_grid::Axis::Vertical,
            _first_pane,
            PaneContent::Editor,
        );

        let app = Self {
            document: None,
            current_page: 0,
            ocr_results: Vec::new(),
            ocr_provider: Arc::new(RwLock::new(DeepSeekOcr::new())),
            ocr_config: OcrConfig::default(),
            ocr_in_progress: false,
            ocr_progress: 0.0,
            ocr_status: String::new(),
            zoom: 1.0,
            zoom_mode: ZoomMode::FitHeight,
            editor_content: text_editor::Content::new(),
            show_line_numbers: true,
            word_wrap: true,
            show_settings: false,
            panes,
            current_page_image: None,
            settings_system_prompt: OcrConfig::default().system_prompt,
            settings_api_key: String::new(),
            settings_endpoint: OcrConfig::default().endpoint,
        };

        (app, Task::none())
    }

    /// Load a document from a file path (synchronous - PdfDocument is not Send)
    pub fn load_document_sync(&mut self, path: std::path::PathBuf) -> Result<()> {
        tracing::info!("Loading document: {:?}", path);

        let document = Document::load(path)?;
        let page_count = document.page_count();

        // Render first page
        if let Ok(image) = document.render_page(0, 2.0) {
            let image_bytes = image.to_rgba8().into_raw();
            // TODO: Store image bytes for display
            self.current_page_image = Some(create_image_handle(&image_bytes, image.width(), image.height()));
        }

        self.document = Some(document);
        self.current_page = 0;
        self.ocr_results = vec![String::new(); page_count];

        tracing::info!("Document loaded with {} pages", page_count);
        Ok(())
    }

    pub fn render_current_page_sync(&mut self) {
        if let Some(doc) = &self.document {
            let page_index = self.current_page;

            if let Ok(image) = doc.render_page(page_index, 2.0) {
                let image_bytes = image.to_rgba8().into_raw();
                self.current_page_image = Some(create_image_handle(&image_bytes, image.width(), image.height()));
            }
        }
    }

    /// Run OCR on the current page
    pub fn ocr_current_page(&mut self) -> Task<Message> {
        if let Some(doc) = &self.document {
            let page_index = self.current_page;

            // Render page to image (synchronously, since PdfDocument is not Send)
            let image_result = doc.render_page(page_index, 2.0);

            match image_result {
                Ok(image) => {
                    let image_bytes = image.to_rgba8().into_raw();
                    let ocr_provider = Arc::clone(&self.ocr_provider);
                    let ocr_config = self.ocr_config.clone();

                    self.ocr_in_progress = true;
                    self.ocr_status = format!("Processing page {}...", page_index + 1);

                    Task::perform(
                        async move {
                            // TODO: Call actual OCR API
                            // For now, simulate processing
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                            Ok(format!("OCR result for page {}", page_index + 1))
                        },
                        Message::OcrCompleted,
                    )
                }
                Err(e) => {
                    tracing::error!("Failed to render page for OCR: {}", e);
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }
}

// Helper function to create iced image handle from raw bytes
fn create_image_handle(bytes: &[u8], width: u32, height: u32) -> iced::widget::image::Handle {
    // Create a temporary file or use memory-based handle
    // For now, return a placeholder
    // TODO: Properly implement image handle creation
    iced::widget::image::Handle::from_rgba(width, height, bytes.to_vec())
}

impl OcrMateApp {
    pub fn title(&self) -> String {
        String::from("OCR-Mate - Intelligent Document OCR")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenDocument => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Documents", &["pdf", "png", "jpg", "jpeg", "tiff", "webp"])
                    .pick_file()
                {
                    if let Err(e) = self.load_document_sync(path) {
                        tracing::error!("Failed to load document: {}", e);
                    }
                }
                Task::none()
            }

            Message::PageRendered(result) => {
                match result {
                    Ok(image_bytes) => {
                        // Image already rendered and stored
                        tracing::info!("Page rendered successfully");
                    }
                    Err(e) => {
                        tracing::error!("Failed to render page: {}", e);
                    }
                }
                Task::none()
            }

            Message::NextPage => {
                if let Some(doc) = &self.document {
                    if self.current_page < doc.page_count() - 1 {
                        self.current_page += 1;
                        self.render_current_page_sync();
                    }
                }
                Task::none()
            }

            Message::PreviousPage => {
                if self.current_page > 0 {
                    self.current_page -= 1;
                    self.render_current_page_sync();
                }
                Task::none()
            }

            Message::OcrCurrentPage => self.ocr_current_page(),

            Message::OcrAllPages => {
                // TODO: Implement batch OCR
                tracing::info!("OCR all pages requested");
                Task::none()
            }

            Message::OcrCompleted(result) => {
                self.ocr_in_progress = false;
                match result {
                    Ok(text) => {
                        self.ocr_results[self.current_page] = text.clone();
                        self.editor_content = text_editor::Content::with_text(&text);
                        self.ocr_status = "OCR completed".to_string();
                    }
                    Err(e) => {
                        self.ocr_status = format!("OCR failed: {}", e);
                        tracing::error!("OCR failed: {}", e);
                    }
                }
                Task::none()
            }

            Message::OcrProgress(progress, status) => {
                self.ocr_progress = progress;
                self.ocr_status = status;
                Task::none()
            }

            Message::ExportAs(format) => {
                // TODO: Implement export with file dialog
                tracing::info!("Export requested: {:?}", format);
                Task::none()
            }

            Message::OpenSettings => {
                self.show_settings = true;
                self.settings_system_prompt = self.ocr_config.system_prompt.clone();
                self.settings_api_key = self.ocr_config.api_key.clone();
                self.settings_endpoint = self.ocr_config.endpoint.clone();
                Task::none()
            }

            Message::CloseSettings => {
                self.show_settings = false;
                Task::none()
            }

            Message::SaveSettings => {
                self.ocr_config.system_prompt = self.settings_system_prompt.clone();
                self.ocr_config.api_key = self.settings_api_key.clone();
                self.ocr_config.endpoint = self.settings_endpoint.clone();
                self.show_settings = false;

                // TODO: Save config to disk
                tracing::info!("Settings saved");
                Task::none()
            }

            Message::UpdateSystemPrompt(value) => {
                self.settings_system_prompt = value;
                Task::none()
            }

            Message::UpdateApiKey(value) => {
                self.settings_api_key = value;
                Task::none()
            }

            Message::UpdateEndpoint(value) => {
                self.settings_endpoint = value;
                Task::none()
            }

            Message::ZoomIn => {
                self.zoom_mode = ZoomMode::Manual;
                self.zoom = (self.zoom * 1.25).min(5.0);
                Task::none()
            }

            Message::ZoomOut => {
                self.zoom_mode = ZoomMode::Manual;
                self.zoom = (self.zoom * 0.8).max(0.1);
                Task::none()
            }

            Message::SetZoomMode(mode) => {
                self.zoom_mode = mode;
                Task::none()
            }

            Message::ResetZoom => {
                self.zoom_mode = ZoomMode::Manual;
                self.zoom = 1.0;
                Task::none()
            }

            Message::EditorAction(action) => {
                self.editor_content.perform(action);

                // Update the OCR results for current page
                self.ocr_results[self.current_page] = self.editor_content.text();
                Task::none()
            }

            Message::ToggleLineNumbers => {
                self.show_line_numbers = !self.show_line_numbers;
                Task::none()
            }

            Message::ToggleWordWrap => {
                self.word_wrap = !self.word_wrap;
                Task::none()
            }

            Message::PaneResized(event) => {
                self.panes.resize(event.split, event.ratio);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![
            // Toolbar
            self.view_toolbar(),
            // Main content with paned grid
            self.view_main_content(),
            // Status bar
            self.view_status_bar(),
        ]
        .spacing(0);

        // If settings dialog is open, show it as an overlay
        if self.show_settings {
            iced::widget::stack![
                content,
                self.view_settings_modal()
            ].into()
        } else {
            content.into()
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl OcrMateApp {
    fn view_toolbar(&self) -> Element<Message> {
        let has_document = self.document.is_some();

        let mut toolbar = row![
            button("📁 Open Document").on_press(Message::OpenDocument),
            text(" | ").size(20),
        ]
        .spacing(10)
        .padding(10)
        .align_y(Alignment::Center);

        // OCR buttons (only enabled when document is loaded)
        if has_document && !self.ocr_in_progress {
            toolbar = toolbar
                .push(button("🔍 OCR Current Page").on_press(Message::OcrCurrentPage))
                .push(button("🔍 OCR All Pages").on_press(Message::OcrAllPages));
        } else if has_document {
            toolbar = toolbar
                .push(button("🔍 OCR Current Page"))
                .push(button("🔍 OCR All Pages"));
        } else {
            toolbar = toolbar
                .push(button("🔍 OCR Current Page"))
                .push(button("🔍 OCR All Pages"));
        }

        toolbar = toolbar
            .push(text(" | ").size(20))
            .push(button("⚙️ Settings").on_press(Message::OpenSettings));

        // Export menu (simplified for now)
        if has_document {
            toolbar = toolbar
                .push(text(" | ").size(20))
                .push(button("💾 Export TXT").on_press(Message::ExportAs(ExportFormat::PlainText)))
                .push(button("💾 Export MD").on_press(Message::ExportAs(ExportFormat::Markdown)))
                .push(button("💾 Export JSON").on_press(Message::ExportAs(ExportFormat::Json)));
        }

        // Page navigation
        if let Some(doc) = &self.document {
            let page_count = doc.page_count();
            toolbar = toolbar
                .push(text("     "))  // Spacer
                .push(
                    if self.current_page > 0 {
                        button("←").on_press(Message::PreviousPage)
                    } else {
                        button("←")
                    }
                )
                .push(text(format!("Page {} / {}", self.current_page + 1, page_count)))
                .push(
                    if self.current_page < page_count - 1 {
                        button("→").on_press(Message::NextPage)
                    } else {
                        button("→")
                    }
                );
        }

        container(toolbar)
            .width(Length::Fill)
            .style(container::bordered_box)
            .into()
    }

    fn view_main_content(&self) -> Element<Message> {
        let pane_grid_widget = pane_grid::PaneGrid::new(&self.panes, |_id, pane, _is_maximized| {
            let content = match pane {
                PaneContent::DocumentViewer => self.view_document_viewer(),
                PaneContent::Editor => self.view_editor_panel(),
            };
            pane_grid::Content::new(content)
        })
        .on_resize(10, Message::PaneResized)
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill);

        container(pane_grid_widget)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_document_viewer(&self) -> Element<Message> {
        if let Some(_doc) = &self.document {
            let mut content = column![].spacing(10).padding(10);

            // Document display
            if let Some(image_handle) = &self.current_page_image {
                let img = iced::widget::image(image_handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill);
                content = content.push(img);
            } else {
                content = content.push(text("Rendering page..."));
            }

            // Zoom controls
            let zoom_controls = row![
                button("−").on_press(Message::ZoomOut),
                text(format!("{:.0}%", self.zoom * 100.0)),
                button("+").on_press(Message::ZoomIn),
                text(" | "),
                button("Fit Height").on_press(Message::SetZoomMode(ZoomMode::FitHeight)),
                button("Fit Width").on_press(Message::SetZoomMode(ZoomMode::FitWidth)),
                button("100%").on_press(Message::ResetZoom),
            ]
            .spacing(5);

            content = content.push(zoom_controls);

            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            container(
                column![
                    text("No document loaded").size(20),
                    text("Click 'Open Document' to get started"),
                ]
                .spacing(10)
                .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        }
    }

    fn view_editor_panel(&self) -> Element<Message> {
        let editor_toolbar = row![
            text("OCR Results:"),
            text("     "), // Spacer
            button(if self.show_line_numbers { "☑ Line Numbers" } else { "☐ Line Numbers" })
                .on_press(Message::ToggleLineNumbers),
            button(if self.word_wrap { "☑ Word Wrap" } else { "☐ Word Wrap" })
                .on_press(Message::ToggleWordWrap),
        ]
        .spacing(10)
        .padding(5);

        let editor = text_editor(&self.editor_content)
            .on_action(Message::EditorAction)
            .height(Length::Fill);

        let text_content = self.editor_content.text();
        let stats = row![
            text(format!("Characters: {}", text_content.len())),
            text(" | "),
            text(format!("Lines: {}", text_content.lines().count())),
            text(" | "),
            text(format!("Words: {}", text_content.split_whitespace().count())),
        ]
        .spacing(5)
        .padding(5);

        column![
            editor_toolbar,
            container(scrollable(editor))
                .width(Length::Fill)
                .height(Length::Fill),
            stats,
        ]
        .spacing(5)
        .into()
    }

    fn view_status_bar(&self) -> Element<Message> {
        let status_text = if self.ocr_in_progress {
            format!("⟳ {} ({:.0}%)", self.ocr_status, self.ocr_progress * 100.0)
        } else if let Some(doc) = &self.document {
            format!(
                "Page {} of {} | {}",
                self.current_page + 1,
                doc.page_count(),
                doc.path().display()
            )
        } else {
            "No document loaded".to_string()
        };

        container(text(status_text))
            .width(Length::Fill)
            .padding(5)
            .style(container::bordered_box)
            .into()
    }

    fn view_settings_modal(&self) -> Element<Message> {
        let modal_content = column![
            text("OCR Settings").size(24),
            text("System Instructions:"),
            text_input("Enter system instructions...", &self.settings_system_prompt)
                .on_input(Message::UpdateSystemPrompt),
            text("API Key (HuggingFace):"),
            text_input("Enter API key...", &self.settings_api_key)
                .on_input(Message::UpdateApiKey)
                .secure(true),
            text("Model Endpoint:"),
            text_input("Enter endpoint URL...", &self.settings_endpoint)
                .on_input(Message::UpdateEndpoint),
            row![
                button("Save").on_press(Message::SaveSettings),
                button("Cancel").on_press(Message::CloseSettings),
            ]
            .spacing(10),
        ]
        .spacing(15)
        .padding(20)
        .max_width(600);

        container(modal_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.8))),
                    ..container::Style::default()
                }
            })
            .into()
    }
}

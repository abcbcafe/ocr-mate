mod app;
mod document;
mod export;
mod ocr;
mod ui;
mod utils;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ocr_mate=info,warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting OCR-Mate...");

    // Run the app with settings
    iced::application("OCR-Mate - Intelligent Document OCR", app::OcrMateApp::update, app::OcrMateApp::view)
        .window(iced::window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        })
        .theme(app::OcrMateApp::theme)
        .run_with(app::OcrMateApp::new)
}

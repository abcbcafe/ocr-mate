mod config;
mod deepseek;
mod provider;
mod types;

pub use config::OcrConfig;
pub use deepseek::DeepSeekOcr;
pub use provider::OcrProvider;
pub use types::{OcrRequest, OcrResponse, OcrResult};

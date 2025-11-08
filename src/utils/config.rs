use serde::{Deserialize, Serialize};

/// Application-wide configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Window settings
    pub window: WindowConfig,

    /// UI preferences
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width
    pub width: f32,

    /// Window height
    pub height: f32,

    /// Remember window position
    pub remember_position: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Split panel ratio (0.0 to 1.0)
    pub split_ratio: f32,

    /// Show line numbers in editor
    pub show_line_numbers: bool,

    /// Word wrap in editor
    pub word_wrap: bool,

    /// Theme (light/dark)
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 1400.0,
                height: 900.0,
                remember_position: true,
            },
            ui: UiConfig {
                split_ratio: 0.5,
                show_line_numbers: true,
                word_wrap: true,
                theme: "dark".to_string(),
            },
        }
    }
}

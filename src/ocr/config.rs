use serde::{Deserialize, Serialize};

/// Configuration for OCR processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    /// API key for the OCR service (e.g., HuggingFace token)
    pub api_key: String,

    /// API endpoint URL
    pub endpoint: String,

    /// System prompt/instructions for the OCR model
    pub system_prompt: String,

    /// Default language
    pub language: String,

    /// Maximum retries on API failure
    pub max_retries: u32,

    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            endpoint: "https://api-inference.huggingface.co/models/deepseek-ai/Deepseek-Ocr"
                .to_string(),
            system_prompt: "Extract all text from this image accurately. Preserve formatting, tables, and structure.".to_string(),
            language: "en".to_string(),
            max_retries: 3,
            timeout_secs: 30,
        }
    }
}

impl OcrConfig {
    /// Load configuration from file
    pub fn load() -> anyhow::Result<Self> {
        let config_dir = directories::ProjectDirs::from("com", "ocr-mate", "OCR-Mate")
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;

        let config_path = config_dir.config_dir().join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: OcrConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> anyhow::Result<()> {
        let config_dir = directories::ProjectDirs::from("com", "ocr-mate", "OCR-Mate")
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;

        std::fs::create_dir_all(config_dir.config_dir())?;

        let config_path = config_dir.config_dir().join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }

    /// Check if the configuration is valid
    pub fn is_valid(&self) -> bool {
        !self.api_key.is_empty() && !self.endpoint.is_empty()
    }
}

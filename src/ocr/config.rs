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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_config_default() {
        let config = OcrConfig::default();

        assert_eq!(config.api_key, "");
        assert!(config.endpoint.contains("deepseek-ai"));
        assert_eq!(config.language, "en");
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_secs, 30);
        assert!(!config.system_prompt.is_empty());
    }

    #[test]
    fn test_ocr_config_is_valid() {
        let mut config = OcrConfig::default();

        // Default config is not valid (no API key)
        assert!(!config.is_valid());

        // Add API key
        config.api_key = "test_key".to_string();
        assert!(config.is_valid());

        // Empty endpoint makes it invalid
        config.endpoint = String::new();
        assert!(!config.is_valid());
    }

    #[test]
    fn test_ocr_config_serialization() {
        let config = OcrConfig {
            api_key: "test_key_123".to_string(),
            endpoint: "https://test.api.com".to_string(),
            system_prompt: "Test prompt".to_string(),
            language: "es".to_string(),
            max_retries: 5,
            timeout_secs: 60,
        };

        let toml_str = toml::to_string(&config).unwrap();

        assert!(toml_str.contains("test_key_123"));
        assert!(toml_str.contains("Test prompt"));
        assert!(toml_str.contains("es"));
        assert!(toml_str.contains("max_retries = 5"));
        assert!(toml_str.contains("timeout_secs = 60"));
    }

    #[test]
    fn test_ocr_config_deserialization() {
        let toml_str = r#"
            api_key = "deserialized_key"
            endpoint = "https://example.com/ocr"
            system_prompt = "Deserialize test"
            language = "fr"
            max_retries = 2
            timeout_secs = 45
        "#;

        let config: OcrConfig = toml::from_str(toml_str).unwrap();

        assert_eq!(config.api_key, "deserialized_key");
        assert_eq!(config.endpoint, "https://example.com/ocr");
        assert_eq!(config.system_prompt, "Deserialize test");
        assert_eq!(config.language, "fr");
        assert_eq!(config.max_retries, 2);
        assert_eq!(config.timeout_secs, 45);
    }

    #[test]
    fn test_ocr_config_clone() {
        let config1 = OcrConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.api_key, config2.api_key);
        assert_eq!(config1.endpoint, config2.endpoint);
        assert_eq!(config1.language, config2.language);
    }

    // Note: save() and load() tests require filesystem access and are tested in integration tests
}

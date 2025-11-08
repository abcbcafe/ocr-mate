// Integration tests for configuration persistence

use ocr_mate::ocr::OcrConfig;
use std::io::Write;
use tempfile::NamedTempFile;

#[path = "../common/mod.rs"]
mod common;

#[test]
fn test_config_serialization_deserialization() {
    let config = OcrConfig {
        api_key: "test_key_12345".to_string(),
        endpoint: "https://test.api.com/ocr".to_string(),
        system_prompt: "Test system prompt".to_string(),
        language: "es".to_string(),
        max_retries: 5,
        timeout_secs: 45,
    };

    // Serialize to TOML
    let toml_string = toml::to_string(&config).unwrap();

    // Deserialize back
    let deserialized: OcrConfig = toml::from_str(&toml_string).unwrap();

    assert_eq!(config.api_key, deserialized.api_key);
    assert_eq!(config.endpoint, deserialized.endpoint);
    assert_eq!(config.system_prompt, deserialized.system_prompt);
    assert_eq!(config.language, deserialized.language);
    assert_eq!(config.max_retries, deserialized.max_retries);
    assert_eq!(config.timeout_secs, deserialized.timeout_secs);
}

#[test]
fn test_config_load_from_toml_file() {
    let mut temp_file = NamedTempFile::with_suffix(".toml").unwrap();

    let config_content = r#"
api_key = "file_test_key"
endpoint = "https://file.test.com"
system_prompt = "File test prompt"
language = "fr"
max_retries = 4
timeout_secs = 50
"#;

    temp_file.write_all(config_content.as_bytes()).unwrap();

    // Read and parse the file
    let file_content = std::fs::read_to_string(temp_file.path()).unwrap();
    let config: OcrConfig = toml::from_str(&file_content).unwrap();

    assert_eq!(config.api_key, "file_test_key");
    assert_eq!(config.endpoint, "https://file.test.com");
    assert_eq!(config.language, "fr");
    assert_eq!(config.max_retries, 4);
}

#[test]
fn test_config_default_values() {
    let config = OcrConfig::default();

    // Check default values
    assert!(config.api_key.is_empty());
    assert!(config.endpoint.contains("huggingface"));
    assert!(config.endpoint.contains("deepseek"));
    assert_eq!(config.language, "en");
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.timeout_secs, 30);
    assert!(!config.system_prompt.is_empty());
}

#[test]
fn test_config_partial_deserialization() {
    // TOML with only some fields (should fail without defaults)
    let partial_toml = r#"
api_key = "partial_key"
endpoint = "https://partial.test.com"
"#;

    let result: Result<OcrConfig, _> = toml::from_str(partial_toml);

    // Should fail because required fields are missing
    assert!(
        result.is_err(),
        "Partial config should fail without all required fields"
    );
}

#[test]
fn test_config_validation_empty_api_key() {
    let mut config = OcrConfig::default();
    config.endpoint = "https://test.com".to_string();

    assert!(!config.is_valid(), "Config with empty API key should be invalid");
}

#[test]
fn test_config_validation_empty_endpoint() {
    let mut config = OcrConfig::default();
    config.api_key = "test_key".to_string();
    config.endpoint = String::new();

    assert!(!config.is_valid(), "Config with empty endpoint should be invalid");
}

#[test]
fn test_config_validation_valid() {
    let config = OcrConfig {
        api_key: "valid_key".to_string(),
        endpoint: "https://valid.endpoint.com".to_string(),
        system_prompt: "Test".to_string(),
        language: "en".to_string(),
        max_retries: 3,
        timeout_secs: 30,
    };

    assert!(config.is_valid(), "Config with API key and endpoint should be valid");
}

#[test]
fn test_config_round_trip() {
    let original = OcrConfig {
        api_key: "round_trip_key".to_string(),
        endpoint: "https://roundtrip.com".to_string(),
        system_prompt: "Round trip test".to_string(),
        language: "de".to_string(),
        max_retries: 7,
        timeout_secs: 60,
    };

    // Serialize
    let toml_str = toml::to_string_pretty(&original).unwrap();

    // Deserialize
    let restored: OcrConfig = toml::from_str(&toml_str).unwrap();

    // Compare all fields
    assert_eq!(original.api_key, restored.api_key);
    assert_eq!(original.endpoint, restored.endpoint);
    assert_eq!(original.system_prompt, restored.system_prompt);
    assert_eq!(original.language, restored.language);
    assert_eq!(original.max_retries, restored.max_retries);
    assert_eq!(original.timeout_secs, restored.timeout_secs);
}

// Note: Tests for OcrConfig::load() and save() would require filesystem access
// and are better suited for manual/system tests since they depend on directories crate
#[test]
#[ignore = "Requires filesystem access to user config directory"]
fn test_config_save_and_load() {
    // This test would use OcrConfig::save() and load()
    // Skipped by default to avoid filesystem side effects
}

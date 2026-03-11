// Exercise 4: Config Files
// 
// Implement configuration management for the Textio SMS system.
// Complete the functions marked with TODO.
//
// Add to Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// toml = "0.8"
// serde_json = "1.0"

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

/// Main Textio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextioConfig {
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub messaging: MessagingConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub contacts_file: Option<String>,
}

/// API-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_api_url")]
    pub base_url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            base_url: default_api_url(),
            api_key: None,
            timeout: default_timeout(),
            max_retries: default_retries(),
        }
    }
}

/// Messaging-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingConfig {
    #[serde(default = "default_sender_id")]
    pub default_sender: String,
    #[serde(default = "default_max_length")]
    pub max_message_length: usize,
}

impl Default for MessagingConfig {
    fn default() -> Self {
        MessagingConfig {
            default_sender: default_sender_id(),
            max_message_length: default_max_length(),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub file: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            level: default_log_level(),
            file: None,
        }
    }
}

fn default_api_url() -> String {
    "https://api.textio.com/v1".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_retries() -> u32 {
    3
}

fn default_sender_id() -> String {
    "Textio".to_string()
}

fn default_max_length() -> usize {
    160
}

fn default_log_level() -> String {
    "info".to_string()
}

/// Load configuration from a TOML file
pub fn load_config(path: &Path) -> Result<TextioConfig, ConfigError> {
    // TODO: Implement this function
    // 1. Read the file content using fs::read_to_string
    // 2. Parse the TOML content using toml::from_str
    // 3. Return the parsed configuration
    // 4. Handle both IO errors and parse errors
    
    todo!()
}

/// Load configuration from a JSON file
pub fn load_config_json(path: &Path) -> Result<TextioConfig, ConfigError> {
    // TODO: Implement this function
    // Similar to load_config but parse as JSON
    
    todo!()
}

/// Save configuration to a TOML file
pub fn save_config(path: &Path, config: &TextioConfig) -> Result<(), ConfigError> {
    // TODO: Implement this function
    // 1. Serialize config to TOML string (pretty format)
    // 2. Write to file using fs::write
    
    todo!()
}

/// Save configuration to a JSON file
pub fn save_config_json(path: &Path, config: &TextioConfig) -> Result<(), ConfigError> {
    // TODO: Implement this function
    
    todo!()
}

/// Merge a partial config with defaults
/// Fields that are None or empty in the overlay keep values from base
pub fn merge_configs(base: &TextioConfig, overlay: &TextioConfig) -> TextioConfig {
    // TODO: Implement this function
    // Create a new config where:
    // - Non-empty/non-zero overlay values take precedence
    // - Empty/zero overlay values fall back to base
    
    todo!()
}

/// Get the default configuration
pub fn default_config() -> TextioConfig {
    // TODO: Implement this function
    
    todo!()
}

/// Validate configuration values
pub fn validate_config(config: &TextioConfig) -> Result<(), ConfigError> {
    // TODO: Implement this function
    // Validate:
    // - base_url is not empty and starts with http:// or https://
    // - timeout > 0
    // - max_retries < 10
    // - max_message_length > 0 and <= 1600
    // - log level is one of: trace, debug, info, warn, error
    
    todo!()
}

/// Get a configuration value by key path (e.g., "api.timeout")
pub fn get_config_value(config: &TextioConfig, key: &str) -> Option<String> {
    // TODO: Implement this function
    // Support keys like:
    // - "api.base_url"
    // - "api.timeout"
    // - "messaging.default_sender"
    // - "logging.level"
    // - "contacts_file"
    
    todo!()
}

/// Set a configuration value by key path
pub fn set_config_value(config: &mut TextioConfig, key: &str, value: &str) -> Result<(), ConfigError> {
    // TODO: Implement this function
    // Parse the value based on the key type
    
    todo!()
}

/// Configuration error type
#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(String),
    Validation(String),
    NotFound(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "IO error: {}", e),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ConfigError::NotFound(key) => write!(f, "Config key not found: {}", key),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_test_config(path: &Path, content: &str) {
        fs::write(path, content).expect("Failed to write test config");
    }

    #[test]
    fn test_load_config() {
        let path = Path::new("test_config.toml");
        let content = r#"
[api]
base_url = "https://custom.api.com"
timeout = 60

[messaging]
default_sender = "MyApp"

[logging]
level = "debug"
"#;
        write_test_config(path, content);
        
        let config = load_config(path).expect("Failed to load config");
        assert_eq!(config.api.base_url, "https://custom.api.com");
        assert_eq!(config.api.timeout, 60);
        assert_eq!(config.messaging.default_sender, "MyApp");
        assert_eq!(config.logging.level, "debug");
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_load_config_with_defaults() {
        let path = Path::new("test_config_minimal.toml");
        let content = r#"
[api]
"#;
        write_test_config(path, content);
        
        let config = load_config(path).expect("Failed to load config");
        assert_eq!(config.api.base_url, default_api_url());
        assert_eq!(config.api.timeout, default_timeout());
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_save_and_load_config() {
        let path = Path::new("test_save_config.toml");
        
        let config = TextioConfig {
            api: ApiConfig {
                base_url: "https://test.com".to_string(),
                api_key: Some("secret".to_string()),
                timeout: 45,
                max_retries: 5,
            },
            messaging: MessagingConfig::default(),
            logging: LoggingConfig::default(),
            contacts_file: Some("contacts.toml".to_string()),
        };
        
        save_config(path, &config).expect("Failed to save");
        let loaded = load_config(path).expect("Failed to load");
        
        assert_eq!(loaded.api.base_url, "https://test.com");
        assert_eq!(loaded.api.api_key, Some("secret".to_string()));
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_validate_config_valid() {
        let config = default_config();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_invalid_url() {
        let mut config = default_config();
        config.api.base_url = "invalid-url".to_string();
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_config_invalid_timeout() {
        let mut config = default_config();
        config.api.timeout = 0;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_config_invalid_log_level() {
        let mut config = default_config();
        config.logging.level = "invalid".to_string();
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_get_config_value() {
        let config = default_config();
        
        assert_eq!(
            get_config_value(&config, "api.base_url"),
            Some(default_api_url())
        );
        assert_eq!(
            get_config_value(&config, "api.timeout"),
            Some(default_timeout().to_string())
        );
        assert_eq!(get_config_value(&config, "invalid.key"), None);
    }

    #[test]
    fn test_set_config_value() {
        let mut config = default_config();
        
        set_config_value(&mut config, "api.timeout", "60").expect("Failed to set");
        assert_eq!(config.api.timeout, 60);
        
        set_config_value(&mut config, "logging.level", "debug").expect("Failed to set");
        assert_eq!(config.logging.level, "debug");
    }

    #[test]
    fn test_merge_configs() {
        let base = default_config();
        let mut overlay = TextioConfig::default();
        overlay.api.timeout = 60;
        
        let merged = merge_configs(&base, &overlay);
        assert_eq!(merged.api.timeout, 60);
        assert_eq!(merged.api.base_url, default_api_url());
    }
}

fn main() {
    println!("Run tests with: cargo test");
}

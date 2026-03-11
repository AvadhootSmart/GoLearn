// Exercise 4: Config Files - Complete Solution
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

pub fn load_config(path: &Path) -> Result<TextioConfig, ConfigError> {
    let content = fs::read_to_string(path)?;
    let config: TextioConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn load_config_json(path: &Path) -> Result<TextioConfig, ConfigError> {
    let content = fs::read_to_string(path)?;
    let config: TextioConfig = serde_json::from_str(&content)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    Ok(config)
}

pub fn save_config(path: &Path, config: &TextioConfig) -> Result<(), ConfigError> {
    let content = toml::to_string_pretty(config)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    fs::write(path, content)?;
    Ok(())
}

pub fn save_config_json(path: &Path, config: &TextioConfig) -> Result<(), ConfigError> {
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    fs::write(path, content)?;
    Ok(())
}

pub fn merge_configs(base: &TextioConfig, overlay: &TextioConfig) -> TextioConfig {
    TextioConfig {
        api: ApiConfig {
            base_url: if overlay.api.base_url.is_empty() || overlay.api.base_url == default_api_url() {
                base.api.base_url.clone()
            } else {
                overlay.api.base_url.clone()
            },
            api_key: overlay.api.api_key.clone().or(base.api.api_key.clone()),
            timeout: if overlay.api.timeout == 0 || overlay.api.timeout == default_timeout() {
                base.api.timeout
            } else {
                overlay.api.timeout
            },
            max_retries: overlay.api.max_retries,
        },
        messaging: MessagingConfig {
            default_sender: if overlay.messaging.default_sender.is_empty() {
                base.messaging.default_sender.clone()
            } else {
                overlay.messaging.default_sender.clone()
            },
            max_message_length: overlay.messaging.max_message_length,
        },
        logging: LoggingConfig {
            level: if overlay.logging.level.is_empty() || overlay.logging.level == default_log_level() {
                base.logging.level.clone()
            } else {
                overlay.logging.level.clone()
            },
            file: overlay.logging.file.clone().or(base.logging.file.clone()),
        },
        contacts_file: overlay.contacts_file.clone().or(base.contacts_file.clone()),
    }
}

pub fn default_config() -> TextioConfig {
    TextioConfig {
        api: ApiConfig::default(),
        messaging: MessagingConfig::default(),
        logging: LoggingConfig::default(),
        contacts_file: None,
    }
}

pub fn validate_config(config: &TextioConfig) -> Result<(), ConfigError> {
    if config.api.base_url.is_empty() {
        return Err(ConfigError::Validation("api.base_url cannot be empty".to_string()));
    }
    
    if !config.api.base_url.starts_with("http://") && !config.api.base_url.starts_with("https://") {
        return Err(ConfigError::Validation(
            "api.base_url must start with http:// or https://".to_string()
        ));
    }
    
    if config.api.timeout == 0 {
        return Err(ConfigError::Validation("api.timeout must be greater than 0".to_string()));
    }
    
    if config.api.max_retries >= 10 {
        return Err(ConfigError::Validation("api.max_retries must be less than 10".to_string()));
    }
    
    if config.messaging.max_message_length == 0 {
        return Err(ConfigError::Validation("messaging.max_message_length must be greater than 0".to_string()));
    }
    
    if config.messaging.max_message_length > 1600 {
        return Err(ConfigError::Validation("messaging.max_message_length must be at most 1600".to_string()));
    }
    
    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&config.logging.level.as_str()) {
        return Err(ConfigError::Validation(
            format!("logging.level must be one of: {}", valid_levels.join(", "))
        ));
    }
    
    Ok(())
}

pub fn get_config_value(config: &TextioConfig, key: &str) -> Option<String> {
    match key {
        "api.base_url" => Some(config.api.base_url.clone()),
        "api.timeout" => Some(config.api.timeout.to_string()),
        "api.max_retries" => Some(config.api.max_retries.to_string()),
        "api.api_key" => config.api.api_key.clone(),
        "messaging.default_sender" => Some(config.messaging.default_sender.clone()),
        "messaging.max_message_length" => Some(config.messaging.max_message_length.to_string()),
        "logging.level" => Some(config.logging.level.clone()),
        "logging.file" => config.logging.file.clone(),
        "contacts_file" => config.contacts_file.clone(),
        _ => None,
    }
}

pub fn set_config_value(config: &mut TextioConfig, key: &str, value: &str) -> Result<(), ConfigError> {
    match key {
        "api.base_url" => config.api.base_url = value.to_string(),
        "api.timeout" => {
            config.api.timeout = value.parse()
                .map_err(|_| ConfigError::Validation("Invalid timeout value".to_string()))?;
        }
        "api.max_retries" => {
            config.api.max_retries = value.parse()
                .map_err(|_| ConfigError::Validation("Invalid max_retries value".to_string()))?;
        }
        "api.api_key" => config.api.api_key = Some(value.to_string()),
        "messaging.default_sender" => config.messaging.default_sender = value.to_string(),
        "messaging.max_message_length" => {
            config.messaging.max_message_length = value.parse()
                .map_err(|_| ConfigError::Validation("Invalid max_message_length value".to_string()))?;
        }
        "logging.level" => config.logging.level = value.to_string(),
        "logging.file" => config.logging.file = Some(value.to_string()),
        "contacts_file" => config.contacts_file = Some(value.to_string()),
        _ => return Err(ConfigError::NotFound(key.to_string())),
    }
    Ok(())
}

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

fn main() -> Result<(), ConfigError> {
    let config_path = Path::new("textio.toml");
    
    let config = TextioConfig {
        api: ApiConfig {
            base_url: "https://api.textio.com/v1".to_string(),
            api_key: Some("your-api-key".to_string()),
            timeout: 30,
            max_retries: 3,
        },
        messaging: MessagingConfig {
            default_sender: "Textio".to_string(),
            max_message_length: 160,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            file: Some("textio.log".to_string()),
        },
        contacts_file: Some("contacts.toml".to_string()),
    };
    
    save_config(config_path, &config)?;
    println!("Config saved to {:?}", config_path);
    
    let loaded = load_config(config_path)?;
    println!("\nLoaded config:");
    println!("  API URL: {}", loaded.api.base_url);
    println!("  Timeout: {}s", loaded.api.timeout);
    println!("  Default Sender: {}", loaded.messaging.default_sender);
    println!("  Log Level: {}", loaded.logging.level);
    
    validate_config(&loaded)?;
    println!("\nConfig validation: PASSED");
    
    println!("\nConfig values:");
    for key in &["api.base_url", "api.timeout", "messaging.default_sender", "logging.level"] {
        if let Some(value) = get_config_value(&loaded, key) {
            println!("  {}: {}", key, value);
        }
    }
    
    let json_path = Path::new("textio.json");
    save_config_json(json_path, &loaded)?;
    println!("\nConfig also saved as JSON to {:?}", json_path);
    
    let from_json = load_config_json(json_path)?;
    assert_eq!(loaded.api.base_url, from_json.api.base_url);
    println!("JSON round-trip: OK");
    
    fs::remove_file(config_path).ok();
    fs::remove_file(json_path).ok();
    
    Ok(())
}

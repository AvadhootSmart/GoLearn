# Exercise 4: Config Files

## Overview

Configuration files allow applications to store settings persistently. Using formats like TOML and JSON for configuration is a common pattern in Rust applications. This exercise covers reading, writing, and managing configuration files for the Textio SMS system.

## Learning Objectives

By the end of this exercise, you will be able to:

- Read configuration from TOML files
- Read configuration from JSON files
- Write configuration back to files
- Handle missing configuration with defaults
- Validate configuration values
- Implement layered configuration (defaults < file < environment)

## Core Concepts

### TOML Format

TOML (Tom's Obvious Minimal Language) is a configuration file format that's easy to read and write:

```toml
# textio.toml
[api]
base_url = "https://api.textio.com"
timeout = 30

[defaults]
sender_id = "Textio"
country_code = "+1"

[logging]
level = "info"
file = "textio.log"

[[contacts]]
name = "John Doe"
phone = "+1234567890"

[[contacts]]
name = "Jane Smith"
phone = "+0987654321"
```

### TOML with Rust

The `toml` crate works with Serde for serialization:

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api: ApiConfig,
    defaults: DefaultsConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiConfig {
    base_url: String,
    timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DefaultsConfig {
    sender_id: String,
    country_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoggingConfig {
    level: String,
    file: String,
}

fn load_config(path: &str) -> io::Result<Config> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
```

### JSON Configuration

JSON is also commonly used for configuration:

```json
{
  "api": {
    "base_url": "https://api.textio.com",
    "timeout": 30
  },
  "defaults": {
    "sender_id": "Textio",
    "country_code": "+1"
  },
  "logging": {
    "level": "info",
    "file": "textio.log"
  }
}
```

### Default Values

Handle missing fields with defaults:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default = "default_api_url")]
    api_url: String,
    
    #[serde(default)]
    timeout: u64,  // Uses Default trait (0)
    
    #[serde(default = "default_timeout")]
    retry_timeout: u64,
}

fn default_api_url() -> String {
    "https://api.textio.com".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_url: default_api_url(),
            timeout: 30,
            retry_timeout: default_timeout(),
        }
    }
}
```

### Configuration Validation

Always validate configuration after loading:

```rust
impl Config {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.timeout == 0 {
            return Err(ConfigError::InvalidValue(
                "timeout must be greater than 0".to_string()
            ));
        }
        
        if !self.api_url.starts_with("http") {
            return Err(ConfigError::InvalidValue(
                "api_url must start with http:// or https://".to_string()
            ));
        }
        
        Ok(())
    }
}

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    ParseError(toml::de::Error),
    InvalidValue(String),
}
```

### Layered Configuration

Combine multiple configuration sources:

```rust
fn load_full_config() -> Config {
    // Start with defaults
    let mut config = Config::default();
    
    // Override with file config
    if let Ok(file_config) = load_config("textio.toml") {
        config = merge_configs(config, file_config);
    }
    
    // Override with environment variables
    if let Ok(api_url) = env::var("TEXTIO_API_URL") {
        config.api_url = api_url;
    }
    
    config
}

fn merge_configs(base: Config, overlay: Config) -> Config {
    Config {
        api_url: overlay.api_url,
        timeout: if overlay.timeout > 0 { overlay.timeout } else { base.timeout },
        // ... merge other fields
    }
}
```

### Environment Variables

```rust
use std::env;

fn load_from_env() -> PartialConfig {
    PartialConfig {
        api_url: env::var("TEXTIO_API_URL").ok(),
        api_key: env::var("TEXTIO_API_KEY").ok(),
        timeout: env::var("TEXTIO_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok()),
    }
}

struct PartialConfig {
    api_url: Option<String>,
    api_key: Option<String>,
    timeout: Option<u64>,
}
```

### Saving Configuration

```rust
fn save_config(path: &str, config: &Config) -> io::Result<()> {
    let content = toml::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    fs::write(path, content)
}
```

### Config File Discovery

Look for config in multiple locations:

```rust
fn find_config_file() -> Option<PathBuf> {
    let locations = vec![
        PathBuf::from("textio.toml"),
        PathBuf::from(".textio.toml"),
        dirs::config_dir().map(|p| p.join("textio/config.toml")),
        PathBuf::from("/etc/textio/config.toml"),
    ];
    
    locations.into_iter()
        .flatten()
        .find(|p| p.exists())
}

// Or use xdg crate for proper XDG Base Directory specification
```

## Textio Context

### Textio Configuration Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
struct TextioConfig {
    #[serde(default)]
    api: ApiConfig,
    
    #[serde(default)]
    messaging: MessagingConfig,
    
    #[serde(default)]
    logging: LoggingConfig,
    
    #[serde(default)]
    contacts_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ApiConfig {
    #[serde(default = "default_api_url")]
    base_url: String,
    
    #[serde(default)]
    api_key: Option<String>,
    
    #[serde(default = "default_timeout")]
    timeout: u64,
    
    #[serde(default = "default_retries")]
    max_retries: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct MessagingConfig {
    #[serde(default = "default_sender_id")]
    default_sender: String,
    
    #[serde(default)]
    max_message_length: usize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct LoggingConfig {
    #[serde(default = "default_log_level")]
    level: String,
    
    #[serde(default)]
    file: Option<String>,
}
```

### Sample textio.toml

```toml
[api]
base_url = "https://api.textio.com/v1"
api_key = "your-api-key-here"
timeout = 30
max_retries = 3

[messaging]
default_sender = "Textio"
max_message_length = 160

[logging]
level = "info"
file = "textio.log"

# Optional path to contacts file
contacts_file = "contacts.toml"
```

### Configuration CLI

```bash
# View all config
textio config

# View specific key
textio config api.api_key

# Set a value
textio config api.timeout 60

# Use custom config file
textio --config /path/to/config.toml send +123 "Hello"
```

## Best Practices

1. **Use sensible defaults**: Application should work without a config file
2. **Validate early**: Check config values on startup
3. **Support multiple formats**: Allow TOML and JSON
4. **Layer configuration**: Defaults < file < env < CLI
5. **Document options**: Comment your config files
6. **Handle errors gracefully**: Provide clear error messages
7. **Use typed config**: Don't use stringly-typed values

## Exercise Instructions

You will implement configuration management for Textio:

1. **load_config**: Load configuration from TOML file
2. **save_config**: Save configuration to TOML file
3. **merge_with_defaults**: Fill in missing values
4. **validate_config**: Ensure configuration is valid

## Key Takeaways

- TOML is human-friendly for configuration files
- Use Serde derive macros for automatic parsing
- Always provide default values for optional settings
- Validate configuration before using it
- Support layered configuration for flexibility

## Further Reading

- [toml crate documentation](https://docs.rs/toml/)
- [TOML specification](https://toml.io/en/)
- [The config crate](https://docs.rs/config/) - Advanced config management
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)

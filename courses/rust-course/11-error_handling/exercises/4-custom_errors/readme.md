# Exercise 4: Custom Error Types in Rust

## Introduction

While Rust's standard library provides basic error types, real applications benefit from custom error types that capture domain-specific failure modes. Custom errors improve debugging, enable better error handling, and provide clearer APIs.

## Why Custom Errors?

1. **Domain specificity**: Express exactly what can go wrong in your domain
2. **Type safety**: Different error types for different modules
3. **Better messages**: Provide detailed, actionable error information
4. **Error recovery**: Enable callers to handle specific error cases
5. **API documentation**: Errors become part of your API contract

## Creating a Custom Error Type

### Step 1: Define an Error Enum

```rust
#[derive(Debug)]
pub enum SmsError {
    InvalidPhone(String),
    MessageTooLong { max: usize, actual: usize },
    CarrierUnavailable,
    RateLimitExceeded { retry_after: u64 },
}
```

### Step 2: Implement Display

The `Display` trait provides human-readable error messages:

```rust
use std::fmt;

impl fmt::Display for SmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmsError::InvalidPhone(phone) => {
                write!(f, "Invalid phone number: {}", phone)
            }
            SmsError::MessageTooLong { max, actual } => {
                write!(f, "Message too long: {} characters (max: {})", actual, max)
            }
            SmsError::CarrierUnavailable => {
                write!(f, "SMS carrier is currently unavailable")
            }
            SmsError::RateLimitExceeded { retry_after } => {
                write!(f, "Rate limit exceeded, retry after {} seconds", retry_after)
            }
        }
    }
}
```

### Step 3: Implement std::error::Error

```rust
impl std::error::Error for SmsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            // Return underlying error if wrapping one
            SmsError::CarrierUnavailable => None,
            _ => None,
        }
    }
}
```

For simple cases, the default implementation is sufficient:

```rust
impl std::error::Error for SmsError {}
```

## The Error Trait

The `std::error::Error` trait requires:
- `Debug` (for developer output)
- `Display` (for user output)

And optionally provides:
- `source()` - Returns the underlying error
- `description()` - Deprecated, use Display instead
- `chain()` - Returns an iterator over the error chain

## Complete Example

```rust
use std::fmt;

#[derive(Debug)]
pub enum TextioError {
    Validation(String),
    Database(String),
    Network(String),
    NotFound { resource: String, id: u64 },
}

impl fmt::Display for TextioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextioError::Validation(msg) => write!(f, "Validation error: {}", msg),
            TextioError::Database(msg) => write!(f, "Database error: {}", msg),
            TextioError::Network(msg) => write!(f, "Network error: {}", msg),
            TextioError::NotFound { resource, id } => {
                write!(f, "{} with id {} not found", resource, id)
            }
        }
    }
}

impl std::error::Error for TextioError {}
```

## Wrapping Other Errors

Custom errors can wrap underlying errors:

```rust
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    ParseError(String),
    InvalidValue { field: String, value: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::InvalidValue { field, value } => {
                write!(f, "Invalid value '{}' for field '{}'", value, field)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(e) => Some(e),
            _ => None,
        }
    }
}
```

## The From Trait for Error Conversion

The `From` trait enables automatic conversion between error types:

```rust
impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::IoError(error)
    }
}
```

This allows using `?` seamlessly:

```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)?;  // Auto-converts io::Error
    parse_config(&contents)
}
```

## Implementing From for Multiple Types

```rust
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Json(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Http(e)
    }
}
```

## Error with Context

Add context to errors for better debugging:

```rust
impl TextioError {
    pub fn with_context(self, context: &str) -> Self {
        match self {
            TextioError::Database(msg) => {
                TextioError::Database(format!("{}: {}", context, msg))
            }
            other => other,
        }
    }
}

// Usage
let result = query_database()
    .map_err(|e| TextioError::Database(format!("Failed to load user: {}", e)))?;
```

## Structured Error Data

Errors can carry structured data for programmatic handling:

```rust
#[derive(Debug)]
pub enum ValidationError {
    InvalidPhone { value: String, reason: String },
    InvalidEmail { value: String },
    OutOfRange { field: String, min: i32, max: i32, actual: i32 },
}

fn handle_error(error: ValidationError) {
    match error {
        ValidationError::OutOfRange { field, min, max, actual } => {
            println!("{} must be between {} and {}, got {}", field, min, max, actual);
        }
        _ => println!("Validation failed"),
    }
}
```

## Textio SMS API Error Design

```rust
#[derive(Debug)]
pub enum SmsApiError {
    Validation(ValidationError),
    Database(DatabaseError),
    Carrier(CarrierError),
    RateLimit { retry_after_seconds: u64 },
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidPhoneFormat { phone: String },
    MessageTooLong { length: usize, max: usize },
    EmptyField { field: String },
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed(String),
    NotFound { table: String, id: u64 },
}

#[derive(Debug)]
pub enum CarrierError {
    Timeout,
    Unavailable,
    Rejected { code: u16, reason: String },
}
```

## Best Practices

1. **Use enums** for different error categories
2. **Include relevant data** in error variants
3. **Implement Display** with helpful messages
4. **Implement Error** for compatibility
5. **Use source()** when wrapping errors
6. **Implement From** for automatic conversion
7. **Keep errors public** but implementation details private
8. **Document what errors** functions can return

## Exercise Task

In this exercise, you'll create a comprehensive error system for Textio:

1. `TextioError` - Main error enum with multiple categories
2. `ValidationError` - Input validation errors with data
3. Implement `Display` for all error types
4. Implement `std::error::Error` for all error types
5. Implement `From` for error conversion
6. Functions that return your custom errors

You'll practice:
- Designing error hierarchies
- Implementing Display and Error traits
- Using From for automatic conversion
- Wrapping errors with context

Run your code with:
```bash
rustc code.rs && ./code
```

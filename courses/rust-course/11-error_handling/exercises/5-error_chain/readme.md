# Exercise 5: Error Chains and Error Libraries

## Introduction

As applications grow, errors become more complex. You often need to chain multiple errors together, preserve the error chain for debugging, and handle multiple error types uniformly. This exercise explores error chaining patterns and popular error handling libraries.

## Box<dyn Error>

The simplest way to handle multiple error types is `Box<dyn std::error::Error>`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string("config.txt")?;
    let data: Config = serde_json::from_str(&config)?;
    send_request(&data)?;
    Ok(())
}
```

### Pros
- Simple to use
- Works with any error type implementing `std::error::Error`
- Great for prototyping and simple applications

### Cons
- Loses type information at compile time
- Harder to handle specific errors programmatically
- No automatic error context

## Error Chaining

Error chaining preserves the history of what went wrong:

```rust
Error: Failed to send SMS campaign
  Caused by:
    0: Failed to load contact
    1: Database connection failed
    2: Connection refused (os error 61)
```

## The anyhow Crate

`anyhow` is a popular crate for application-level error handling:

```rust
use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .context(format!("Failed to read {}", path))?;
    
    let config: Config = toml::from_str(&contents)
        .context("Failed to parse config")?;
    
    Ok(config)
}

fn main() -> Result<()> {
    let config = load_config("config.toml")?;
    Ok(())
}
```

### Key Features
- `context()` adds descriptive context to errors
- `with_context()` for lazy context evaluation
- `anyhow!` macro for creating errors
- `bail!` macro for early return with error
- Automatic error chaining

### anyhow::Error

```rust
use anyhow::{anyhow, bail, Error, Result};

fn validate_input(input: &str) -> Result<()> {
    if input.is_empty() {
        bail!("Input cannot be empty");
    }
    if input.len() > 100 {
        bail!(anyhow!("Input too long: {} characters", input.len()));
    }
    Ok(())
}

fn process() -> Result<String> {
    validate_input("")
        .context("Validation failed")?;
    Ok("success".to_string())
}
```

### Error Chains with anyhow

```rust
use anyhow::{Context, Result};

fn step1() -> Result<i32> {
    Ok(42)
}

fn step2(n: i32) -> Result<String> {
    if n > 100 {
        bail!("Number too large");
    }
    Ok(n.to_string())
}

fn process() -> Result<()> {
    let n = step1()
        .context("Step 1 failed")?;
    
    let s = step2(n)
        .context("Step 2 failed")?;
    
    println!("Result: {}", s);
    Ok(())
}
```

## The thiserror Crate

`thiserror` is for library-level error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Data store disconnected")]
    Disconnect(#[from] io::Error),
    
    #[error("The data for key `{0}` is not available")]
    Redaction(String),
    
    #[error("Invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    
    #[error("Unknown data store error")]
    Unknown,
}
```

### Key Features
- Derive macro for `std::error::Error`
- Automatic `Display` implementation
- `#[from]` for automatic `From` implementations
- Attribute-based error messages

### thiserror vs anyhow

| thiserror | anyhow |
|-----------|--------|
| For libraries | For applications |
| Custom error types | Generic error handling |
| Library's public API | Internal error handling |
| More boilerplate | Less boilerplate |

## Using Both Together

```rust
// In your library (lib.rs)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

// In your application (main.rs)
use anyhow::{Context, Result};
use my_library::LibraryError;

fn main() -> Result<()> {
    let result = library_function()
        .context("Library operation failed")?;
    Ok(())
}
```

## Textio Error Handling Pattern

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextioError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Rate limited, retry after {retry_after}s")]
    RateLimited { retry_after: u64 },
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid phone number: {phone}")]
    InvalidPhone { phone: String },
    
    #[error("Message too long: {length}/{max}")]
    MessageTooLong { length: usize, max: usize },
}
```

## Error Handling Best Practices

### 1. Use the Right Tool

```rust
// Library code: Define specific error types
#[derive(Error, Debug)]
pub enum ApiError { ... }

// Application code: Use anyhow for flexibility
fn main() -> anyhow::Result<()> { ... }
```

### 2. Add Context

```rust
// Bad
let config = read_config(path)?;

// Good
let config = read_config(path)
    .with_context(|| format!("Failed to load config from {}", path))?;
```

### 3. Preserve Error Chains

```rust
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Parse(e) => Some(e),
            _ => None,
        }
    }
}
```

### 4. Make Errors Actionable

```rust
#[derive(Error, Debug)]
pub enum TextioError {
    #[error("Rate limited. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },
    
    #[error("Invalid API key. Check your credentials")]
    InvalidApiKey,
    
    #[error("Service temporarily unavailable. Status page: status.textio.io")]
    ServiceUnavailable,
}
```

### 5. Document Errors

```rust
/// Sends an SMS message.
///
/// # Errors
///
/// Returns `TextioError::InvalidPhone` if the phone number format is invalid.
/// Returns `TextioError::RateLimited` if rate limit is exceeded.
/// Returns `TextioError::Network` if the carrier API is unreachable.
pub fn send_sms(to: &str, message: &str) -> Result<MessageId, TextioError> {
    // ...
}
```

## Exercise Task

This exercise demonstrates error handling patterns conceptually:

1. `Box<dyn Error>` - Multiple error types
2. Manual error chaining with `source()`
3. Context patterns for error messages
4. Error reporting patterns

Note: This exercise uses standard library only to demonstrate concepts.
In production, consider using `anyhow` and `thiserror` crates.

Run your code with:
```bash
rustc code.rs && ./code
```

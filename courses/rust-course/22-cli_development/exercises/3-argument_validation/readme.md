# Exercise 3: Argument Validation and Custom Parsers

## Overview

User input is inherently unreliable. Users make typos, provide invalid data, and sometimes try to use your CLI in unexpected ways. Robust argument validation ensures your program fails gracefully with helpful error messages. This exercise covers custom parsers, validation, and creating user-friendly error messages with clap.

## Learning Objectives

By the end of this exercise, you will be able to:
- Implement custom type parsing with `ValueParser`
- Validate arguments using the `value_parser` attribute
- Create custom error messages
- Use regex for pattern validation
- Parse complex types like dates and phone numbers
- Handle validation errors gracefully

## Built-in Validation

Clap provides several built-in validators:

```rust
#[derive(Parser)]
struct Cli {
    /// Port number (1-65535)
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
    
    /// Number of retries (0-10)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=10))]
    retries: u8,
}
```

## Custom Value Parser Functions

Create custom parser functions for complex validation:

```rust
fn parse_phone_number(s: &str) -> Result<String, String> {
    // Remove any spaces or dashes
    let cleaned: String = s.chars()
        .filter(|c| c.is_numeric() || *c == '+')
        .collect();
    
    // Validate format: should start with + and have 10-15 digits
    if !cleaned.starts_with('+') {
        return Err("Phone number must start with '+' and country code".to_string());
    }
    
    let digits: String = cleaned.chars().filter(|c| c.is_numeric()).collect();
    if digits.len() < 10 || digits.len() > 15 {
        return Err("Phone number must have 10-15 digits".to_string());
    }
    
    Ok(cleaned)
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_parser = parse_phone_number)]
    phone: String,
}
```

## Using ValueParser Trait

For more complex types, implement the `ValueParserFactory` trait:

```rust
use clap::builder::TypedValueParser;
use std::ops::RangeInclusive;

#[derive(Clone)]
struct Percentage(u8);

impl TypedValueParser for Percentage {
    type Value = Percentage;
    
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u8);
        let value = inner.parse_ref(cmd, arg, value)?;
        
        if value > 100 {
            return Err(clap::Error::raw(
                clap::error::ErrorKind::ValueValidation,
                "Percentage must be between 0 and 100",
            ));
        }
        
        Ok(Percentage(value))
    }
}
```

## Validation with Regex

For pattern-based validation:

```rust
use regex::Regex;

fn parse_email(s: &str) -> Result<String, String> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    
    if !email_regex.is_match(s) {
        return Err(format!("'{}' is not a valid email address", s));
    }
    
    Ok(s.to_string())
}
```

Add regex to `Cargo.toml`:
```toml
[dependencies]
regex = "1.10"
```

## Custom Error Messages

Create helpful error messages:

```rust
fn parse_api_key(s: &str) -> Result<String, String> {
    if s.len() < 16 {
        return Err(
            "API key must be at least 16 characters.\n\
             Get your API key from: https://textio.example.com/dashboard/api-keys"
                .to_string(),
        );
    }
    
    if !s.starts_with("tx_") {
        return Err(
            "Invalid API key format. API keys start with 'tx_'.\n\
             Example: tx_live_abc123..."
                .to_string(),
        );
    }
    
    Ok(s.to_string())
}
```

## Possible Values

Restrict arguments to a set of allowed values:

```rust
#[derive(Parser)]
struct Cli {
    /// Log level
    #[arg(long, value_enum, default_value = "info")]
    level: LogLevel,
}

#[derive(Clone, clap::ValueEnum)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
```

Usage: `app --level debug`

## Mutual Exclusivity and Conflicts

Ensure arguments don't conflict:

```rust
#[derive(Parser)]
struct Cli {
    /// Send immediately
    #[arg(short, long, conflicts_with = "schedule")]
    now: bool,
    
    /// Schedule for later
    #[arg(short, long, conflicts_with = "now")]
    schedule: Option<String>,
    
    /// Required unless --schedule is provided
    #[arg(long, required_unless_present = "schedule")]
    message: Option<String>,
}
```

## Validation Groups

Require at least one from a group:

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long, group = "recipient")]
    phone: Option<String>,
    
    #[arg(short, long, group = "recipient")]
    email: Option<String>,
    
    #[arg(short, long, requires = "recipient")]
    message: String,
}
```

## Phone Number Validation for Textio

A robust phone number parser:

```rust
fn parse_phone_number(s: &str) -> Result<PhoneNumber, String> {
    // Clean the input
    let cleaned: String = s.chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();
    
    // Check for + prefix
    if !cleaned.starts_with('+') {
        return Err(
            "Phone number must include country code with '+' prefix.\n\
             Example: +1 for US, +44 for UK"
                .to_string(),
        );
    }
    
    // Extract digits
    let digits: String = cleaned[1..].chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    // Validate length
    if digits.len() < 10 || digits.len() > 15 {
        return Err(format!(
            "Phone number must have 10-15 digits (excluding '+').\n\
             Your number has {} digits.",
            digits.len()
        ));
    }
    
    Ok(PhoneNumber(cleaned))
}

#[derive(Debug, Clone)]
struct PhoneNumber(String);

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

## Message Length Validation

Validate SMS message length:

```rust
fn parse_message(s: &str) -> Result<Message, String> {
    let len = s.chars().count();
    
    if len == 0 {
        return Err("Message cannot be empty".to_string());
    }
    
    if len > 160 {
        return Err(format!(
            "Message too long: {} characters (max 160).\n\
             Consider splitting into multiple messages.",
            len
        ));
    }
    
    Ok(Message(s.to_string()))
}

#[derive(Debug)]
struct Message(String);

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Message {
    fn len(&self) -> usize {
        self.0.chars().count()
    }
}
```

## Combining Multiple Validations

```rust
#[derive(Parser)]
struct Cli {
    /// Recipient phone number (e.g., +1234567890)
    #[arg(short, long, value_parser = parse_phone_number)]
    to: PhoneNumber,
    
    /// Message content (max 160 characters)
    #[arg(short, long, value_parser = parse_message)]
    message: Message,
    
    /// API key (starts with 'tx_')
    #[arg(long, value_parser = parse_api_key, env = "TEXTIO_API_KEY")]
    api_key: String,
}
```

## Error Display

When validation fails, clap displays the error:

```bash
$ textio --to 1234567890 --message "Hello"
error: Invalid value "1234567890" for '--to <TO>': Phone number must include 
country code with '+' prefix.
Example: +1 for US, +44 for UK

For more information, try '--help'.
```

## Exercise Task

Create a Textio CLI with validation:
1. Phone number validation (requires + prefix, 10-15 digits)
2. Message validation (non-empty, max 160 chars)
3. API key validation (starts with 'tx_', min 16 chars)
4. Custom error messages with helpful guidance
5. Optional scheduled time in ISO 8601 format

## Best Practices

1. **Fail fast**: Validate as early as possible
2. **Be helpful**: Error messages should guide users to the solution
3. **Show examples**: Include format examples in error messages
4. **Preserve user input**: Show what the user provided in errors
5. **Use appropriate types**: Custom types make validation results clear

## Resources

- [Clap ValueParser Documentation](https://docs.rs/clap/latest/clap/builder/struct.ValueParser.html)
- [Clap Error Handling](https://docs.rs/clap/latest/clap/error/index.html)
- [Rust Regex Crate](https://docs.rs/regex/)

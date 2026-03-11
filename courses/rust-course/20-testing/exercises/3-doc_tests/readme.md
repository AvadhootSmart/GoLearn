# Documentation Tests in Rust

Documentation tests (doc tests) are a powerful feature that allows you to write tests within your documentation comments. They ensure your code examples stay up-to-date and actually compile.

## Doc Comments

Use `///` for doc comments on items (functions, structs, etc.):

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = mylib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

For module-level documentation, use `//!`:

```rust
//! # Textio SMS Library
//!
//! A library for sending SMS messages.
//!
//! ## Getting Started
//!
//! ```
//! use textio::Client;
//!
//! let client = Client::new("your_api_key");
//! let msg = client.send("+1234567890", "Hello!").unwrap();
//! ```
```

## Running Doc Tests

```bash
# Run all tests including doc tests
cargo test

# Run only doc tests
cargo test --doc
```

## Doc Test Structure

A typical doc test includes:

```rust
/// Short description of the function.
///
/// More detailed explanation of what the function does,
/// any important behavior, and when to use it.
///
/// # Arguments
///
/// * `phone` - The phone number in E.164 format
/// * `body` - The message content (max 160 characters)
///
/// # Returns
///
/// Returns `Ok(Message)` on success, or `Err(Error)` if validation fails.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use textio::Client;
///
/// let client = Client::new("test_key");
/// let msg = client.send("+1234567890", "Hello")?;
/// assert_eq!(msg.body(), "Hello");
/// # Ok::<(), textio::Error>(())
/// ```
///
/// With error handling:
///
/// ```
/// use textio::Client;
///
/// let client = Client::new("test_key");
/// let result = client.send("invalid", "Test");
/// assert!(result.is_err());
/// ```
pub fn send(&self, phone: &str, body: &str) -> Result<Message, Error> {
    // ...
}
```

## Textio Example: Message Parsing

```rust
/// Parses a raw message string into a Message struct.
///
/// The input format is: `to:body` where:
/// - `to` is the recipient's phone number (must start with +)
/// - `body` is the message content
///
/// # Examples
///
/// ```
/// use textio::parse_message;
///
/// let msg = parse_message("+1234567890:Hello World")?;
/// assert_eq!(msg.to, "+1234567890");
/// assert_eq!(msg.body, "Hello World");
/// # Ok::<(), String>(())
/// ```
///
/// Invalid format returns an error:
///
/// ```
/// use textio::parse_message;
///
/// let result = parse_message("invalid");
/// assert!(result.is_err());
/// ```
pub fn parse_message(input: &str) -> Result<Message, String> {
    let parts: Vec<&str> = input.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }
    Ok(Message {
        to: parts[0].to_string(),
        body: parts[1].to_string(),
    })
}
```

## Hiding Code from Documentation

Use `#` to hide lines that are needed for compilation but shouldn't appear in docs:

```rust
/// ```
/// use textio::Client;
///
/// # let client = Client::new("test_key");  // Hidden from docs
/// let msg = client.send("+1234567890", "Hello")?;
/// assert!(msg.id().len() > 0);
/// # Ok::<(), textio::Error>(())
/// ```
```

This renders in docs without the hidden lines, but they're still compiled and tested.

## Handling Results in Doc Tests

Doc tests that return `Result` need explicit type annotation:

```rust
/// ```
/// use textio::Client;
///
/// let client = Client::new("test_key");
/// let msg = client.send("+1234567890", "Hello")?;
/// # Ok::<(), textio::Error>(())
/// ```
```

The `# Ok::<(), textio::Error>(())` line tells Rust the test returns a Result.

## Common Sections

### # Examples

Code examples showing typical usage:

```rust
/// # Examples
///
/// ```
/// let result = calculate_cost(150);
/// assert_eq!(result, 5);
/// ```
```

### # Panics

Document when the function panics:

```rust
/// # Panics
///
/// Panics if `index` is out of bounds.
```

### # Errors

Document what errors can be returned:

```rust
/// # Errors
///
/// Returns `Error::InvalidPhoneNumber` if the phone number format is invalid.
/// Returns `Error::RateLimited` if the rate limit is exceeded.
```

### # Safety

For unsafe functions:

```rust
/// # Safety
///
/// The caller must ensure the pointer is valid.
```

## No-Run Tests

Use `no_run` to compile without executing:

```rust
/// ```no_run
/// use textio::Client;
///
/// let client = Client::new("production_key");
/// client.send("+1234567890", "Hello")?;  // Might fail without real API
/// # Ok::<(), textio::Error>(())
/// ```
```

## Compile-Fail Tests

Use `compile_fail` to test that code doesn't compile:

```rust
/// ```compile_fail
/// use textio::Client;
///
/// let client = Client::new("key");
/// client.send(123, "Hello");  // Wrong type - won't compile
/// ```
```

## Ignoring Doc Tests

Use `ignore` to skip a test:

```rust
/// ```ignore
/// // Requires external service
/// let client = Client::connect("ws://production-server").await?;
/// ```
```

## Best Practices

### 1. Show Real Usage

```rust
/// Send an SMS message.
///
/// # Examples
///
/// ```
/// use textio::{Client, Message};
///
/// let client = Client::new("your_api_key");
/// let message = client.send("+15551234567", "Hello from Textio!")?;
///
/// println!("Message sent with ID: {}", message.id());
/// # Ok::<(), textio::Error>(())
/// ```
pub fn send(&self, to: &str, body: &str) -> Result<Message, Error> {
    // ...
}
```

### 2. Show Error Handling

```rust
/// # Errors
///
/// ```
/// use textio::{Client, Error};
///
/// let client = Client::new("test_key");
///
/// // Invalid phone number
/// let err = client.send("123", "Test").unwrap_err();
/// assert!(matches!(err, Error::InvalidPhoneNumber));
///
/// // Empty message
/// let err = client.send("+1234567890", "").unwrap_err();
/// assert!(matches!(err, Error::EmptyMessage));
/// ```
```

### 3. Show Edge Cases

```rust
/// Calculate message cost.
///
/// First 100 messages are free, then $5 per 100 messages.
///
/// # Examples
///
/// ```
/// use textio::calculate_cost;
///
/// // Free tier
/// assert_eq!(calculate_cost(0), 0);
/// assert_eq!(calculate_cost(100), 0);
///
/// // Paid tier
/// assert_eq!(calculate_cost(101), 5);
/// assert_eq!(calculate_cost(200), 5);
/// assert_eq!(calculate_cost(201), 10);
/// ```
pub fn calculate_cost(count: u32) -> u32 {
    // ...
}
```

### 4. Test Builder Patterns

```rust
/// Create a client with custom configuration.
///
/// # Examples
///
/// ```
/// use textio::ClientBuilder;
/// use std::time::Duration;
///
/// let client = ClientBuilder::new()
///     .api_key("your_key")
///     .timeout(Duration::from_secs(30))
///     .retry_attempts(3)
///     .build()?;
///
/// # Ok::<(), textio::Error>(())
/// ```
```

## Running Specific Doc Tests

```bash
# Run doc tests for specific module
cargo test --doc -- message

# Run doc tests for specific file
cargo test --doc --lib
```

## Viewing Documentation

```bash
# Build and open documentation
cargo doc --open
```

This shows how your doc tests appear in the rendered documentation.

## Summary

- Use `///` for item documentation, `//!` for module documentation
- Code blocks in doc comments become tests
- Use `#` to hide setup code
- Add `?` and return type for Result-returning tests
- Use `no_run`, `compile_fail`, `ignore` for special cases
- Run with `cargo test --doc`
- View with `cargo doc --open`

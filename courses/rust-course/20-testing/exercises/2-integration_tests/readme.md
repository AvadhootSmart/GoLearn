# Integration Testing in Rust

Integration tests are external to your library and test its public API as a consumer would use it. They help verify that different parts of your code work together correctly.

## The tests/ Directory

Integration tests live in the `tests/` directory at the root of your Cargo project. Each file in this directory is compiled as a separate crate.

```
textio/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs
    ├── integration_test.rs
    └── api_test.rs
```

## Basic Integration Test

Create a file `tests/integration_test.rs`:

```rust
use textio::*;

#[test]
fn test_send_and_deliver() {
    let mut client = Client::new("api_key");
    let message = client.send("+1234567890", "Hello").unwrap();
    assert_eq!(message.status, Status::Sent);
}
```

Unlike unit tests, integration tests don't need `#[cfg(test)]` because the entire `tests/` directory is only compiled during testing.

## Testing the Public API

Integration tests can only access your library's public items. This is intentional—it ensures you're testing the API that users will consume.

```rust
// In src/lib.rs
pub fn public_function() -> i32 { 42 }
fn private_helper() -> i32 { 10 }

// In tests/integration_test.rs
use mylib::*;

#[test]
fn test_public_api() {
    assert_eq!(public_function(), 42);
    // private_helper(); // This would fail to compile!
}
```

## Shared Test Utilities

If you have helper code used across multiple integration tests, place it in `tests/common/mod.rs`:

```rust
// tests/common/mod.rs
use textio::*;

pub fn create_test_client() -> Client {
    Client::new("test_api_key")
}

pub fn create_test_message() -> Message {
    Message::new("+1234567890", "Test message")
}
```

Use it in your tests:

```rust
// tests/integration_test.rs
mod common;

use common::*;

#[test]
fn test_with_helpers() {
    let client = create_test_client();
    // ...
}
```

The `common/` directory approach prevents `common` from being treated as an integration test file (files in `tests/` that aren't named `mod.rs` become separate test binaries).

## Multiple Test Files

For larger projects, split integration tests by feature:

```
tests/
├── common/
│   └── mod.rs
├── messaging.rs      # Tests for message operations
├── contacts.rs       # Tests for contact management
├── billing.rs        # Tests for billing
└── webhook.rs        # Tests for webhooks
```

Run specific test files:

```bash
cargo test --test messaging
cargo test --test billing
```

## Textio Example: Testing the Client API

```rust
// tests/client_test.rs
use textio::{Client, Message, Error};

#[test]
fn test_client_creation() {
    let client = Client::new("test_key");
    assert!(!client.api_key().is_empty());
}

#[test]
fn test_send_message() {
    let client = Client::new("test_key");
    let result = client.send("+1234567890", "Hello, World!");
    
    assert!(result.is_ok());
    let message = result.unwrap();
    assert_eq!(message.to(), "+1234567890");
    assert_eq!(message.body(), "Hello, World!");
}

#[test]
fn test_send_invalid_number() {
    let client = Client::new("test_key");
    let result = client.send("invalid", "Hello");
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::InvalidPhoneNumber => (),
        _ => panic!("Expected InvalidPhoneNumber error"),
    }
}

#[test]
fn test_get_message_status() {
    let client = Client::new("test_key");
    let message = client.send("+1234567890", "Test").unwrap();
    let id = message.id();
    
    let status = client.get_status(&id).unwrap();
    assert!(status.is_delivered() || status.is_pending());
}
```

## Testing Error Handling

Integration tests should verify error conditions:

```rust
#[test]
fn test_rate_limiting() {
    let client = Client::new("test_key");
    
    // Send many messages quickly
    for _ in 0..100 {
        let _ = client.send("+1234567890", "Spam");
    }
    
    // Should hit rate limit
    let result = client.send("+1234567890", "One more");
    assert!(matches!(result, Err(Error::RateLimited)));
}

#[test]
fn test_authentication_failure() {
    let client = Client::new("invalid_key");
    let result = client.send("+1234567890", "Test");
    
    assert!(result.is_err());
}
```

## Testing Async Code

For async APIs, use `tokio::test`:

```rust
// Cargo.toml needs tokio and tokio-test
#[tokio::test]
async fn test_async_send() {
    let client = Client::new("test_key");
    let result = client.send_async("+1234567890", "Async message").await;
    
    assert!(result.is_ok());
}
```

## Organization Best Practices

### 1. Test Real-World Scenarios

```rust
#[test]
fn test_complete_message_flow() {
    let client = create_test_client();
    
    // Create contact
    let contact = client.add_contact("Alice", "+1234567890").unwrap();
    
    // Send message
    let message = client.send(&contact.phone, "Hello Alice!").unwrap();
    
    // Check status
    let status = client.get_status(message.id()).unwrap();
    assert!(status.is_sent());
    
    // Get history
    let history = client.get_message_history(&contact.id).unwrap();
    assert_eq!(history.len(), 1);
}
```

### 2. Test Configuration Options

```rust
#[test]
fn test_client_with_custom_endpoint() {
    let client = Client::builder()
        .api_key("test_key")
        .endpoint("https://custom.api.com")
        .timeout(Duration::from_secs(30))
        .build();
    
    assert!(client.send("+1234567890", "Test").is_ok());
}
```

### 3. Test Backward Compatibility

```rust
#[test]
fn test_legacy_api() {
    let client = Client::new("test_key");
    
    // Old API should still work
    let msg = client.send_message("+1234567890", "Test");
    assert!(msg.is_ok());
}
```

## Integration vs Unit Tests

| Aspect | Unit Tests | Integration Tests |
|--------|-----------|-------------------|
| Location | `src/` with `#[cfg(test)]` | `tests/` directory |
| Scope | Individual functions | Whole modules/APIs |
| Access | Public and private | Public only |
| Speed | Very fast | Slower |
| Purpose | Correctness of logic | Component interaction |

## Running Integration Tests

```bash
# Run all tests (unit + integration)
cargo test

# Run only integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test client_test

# Run specific test within file
cargo test --test client_test test_send_message
```

## Debugging Integration Tests

Use `println!` with `--nocapture`:

```bash
cargo test --test client_test -- --nocapture
```

Add debug output:

```rust
#[test]
fn test_with_debug() {
    let client = Client::new("test_key");
    println!("Client created: {:?}", client);
    
    let result = client.send("+1234567890", "Test");
    println!("Send result: {:?}", result);
    
    assert!(result.is_ok());
}
```

## Test Fixtures in Integration Tests

For complex setup, use helper functions:

```rust
// tests/common/mod.rs
use textio::*;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize_test_env() {
    INIT.call_once(|| {
        // One-time setup
        std::env::set_var("TEXTIO_TEST_MODE", "1");
    });
}

pub fn create_test_client() -> Client {
    initialize_test_env();
    Client::new("test_key")
}

pub fn create_test_contacts(client: &Client) -> Vec<Contact> {
    vec![
        client.add_contact("Alice", "+1111111111").unwrap(),
        client.add_contact("Bob", "+2222222222").unwrap(),
        client.add_contact("Charlie", "+3333333333").unwrap(),
    ]
}
```

## Summary

- Integration tests go in `tests/` directory
- Each file compiles as a separate crate
- Tests can only access public API
- Use `tests/common/mod.rs` for shared utilities
- Run with `cargo test` or `cargo test --test <file>`
- Test real-world scenarios and error handling
- Use `--nocapture` for debugging output

# Test Organization in Rust

Well-organized tests are crucial for maintainable code. This exercise covers test modules, conditional test attributes, fixtures, and mocking concepts.

## Test Modules

Organize related tests into modules within your test file:

```rust
#[cfg(test)]
mod tests {
    mod message_validation {
        use super::*;
        
        #[test]
        fn test_valid_phone() { /* ... */ }
        
        #[test]
        fn test_invalid_phone() { /* ... */ }
    }
    
    mod cost_calculation {
        use super::*;
        
        #[test]
        fn test_free_tier() { /* ... */ }
    }
}
```

## #[should_panic]

Some functions are expected to panic under certain conditions. Use `#[should_panic]` to test this:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_on_invalid_input() {
        panic!("This test expects a panic");
    }
}
```

### Expected Panic Messages

Use `expected` to verify the panic message:

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds() {
    let v = vec![1, 2, 3];
    v[10];
}

#[test]
#[should_panic(expected = "Phone number must start")]
fn test_invalid_phone_format() {
    validate_and_panic("1234567890");
}

fn validate_and_panic(phone: &str) {
    if !phone.starts_with('+') {
        panic!("Phone number must start with '+'");
    }
}
```

The `expected` string is matched as a substring of the panic message.

## #[ignore]

Some tests are slow or require special setup. Use `#[ignore]` to skip them by default:

```rust
#[test]
#[ignore]
fn slow_integration_test() {
    // Takes 30 seconds
}

#[test]
#[ignore = "Requires external API key"]
fn test_with_real_api() {
    // Needs real credentials
}
```

Run ignored tests explicitly:

```bash
# Run only ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

## Test Fixtures

Fixtures provide consistent test data. In Rust, we typically use functions or lazy_static:

### Function-Based Fixtures

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message() -> Message {
        Message {
            to: "+1234567890".to_string(),
            body: "Test message".to_string(),
            status: Status::Pending,
        }
    }

    fn create_test_client() -> Client {
        Client::new("test_api_key")
    }

    #[test]
    fn test_send_message() {
        let client = create_test_client();
        let msg = create_test_message();
        
        let result = client.send(&msg.to, &msg.body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_another_scenario() {
        let client = create_test_client();
        // Reuse the fixture
    }
}
```

### Using lazy_static

For expensive fixtures that should be created once:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();
    static mut TEST_DATA: Option<Vec<Message>> = None;

    fn initialize() {
        INIT.call_once(|| {
            let data = vec![
                Message::new("+111", "First"),
                Message::new("+222", "Second"),
            ];
            unsafe {
                TEST_DATA = Some(data);
            }
        });
    }
}
```

### Using lazy_static crate

Add to Cargo.toml:

```toml
[dev-dependencies]
lazy_static = "1.4"
```

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_MESSAGES: Vec<Message> = vec![
            Message::new("+1111111111", "First test message"),
            Message::new("+2222222222", "Second test message"),
            Message::new("+3333333333", "Third test message"),
        ];
    }

    #[test]
    fn test_with_shared_data() {
        assert_eq!(TEST_MESSAGES.len(), 3);
    }
}
```

## Setup and Teardown

Rust doesn't have built-in setup/teardown, but you can simulate it:

### RAII Pattern

```rust
struct TestContext {
    temp_dir: std::path::PathBuf,
}

impl TestContext {
    fn new() -> Self {
        let temp_dir = std::env::temp_dir().join(format!("test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        TestContext { temp_dir }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Teardown - cleanup temp directory
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

#[test]
fn test_with_context() {
    let ctx = TestContext::new();
    // Use ctx.temp_dir
    // Automatic cleanup when ctx goes out of scope
}
```

### Using a Test Function Wrapper

```rust
fn with_test_client<F>(f: F)
where
    F: FnOnce(&mut Client),
{
    let mut client = Client::new("test_key");
    f(&mut client);
    // Cleanup happens here
}

#[test]
fn test_using_wrapper() {
    with_test_client(|client| {
        let result = client.send("+1234567890", "Test");
        assert!(result.is_ok());
    });
}
```

## Mocking Concepts

Mocking replaces real dependencies with test doubles. Rust's approach differs from languages with reflection.

### Trait-Based Mocking

Define traits for external dependencies:

```rust
pub trait SmsGateway {
    fn send(&self, to: &str, body: &str) -> Result<String, Error>;
}

pub struct RealGateway {
    api_key: String,
}

impl SmsGateway for RealGateway {
    fn send(&self, to: &str, body: &str) -> Result<String, Error> {
        // Real API call
    }
}

// In tests
struct MockGateway {
    should_succeed: bool,
}

impl SmsGateway for MockGateway {
    fn send(&self, to: &str, body: &str) -> Result<String, Error> {
        if self.should_succeed {
            Ok("msg_123".to_string())
        } else {
            Err(Error::NetworkError)
        }
    }
}

#[test]
fn test_send_with_mock() {
    let gateway = MockGateway { should_succeed: true };
    let service = MessageService::new(Box::new(gateway));
    
    let result = service.send_message("+1234567890", "Test");
    assert!(result.is_ok());
}
```

### Using mockall crate

Add to Cargo.toml:

```toml
[dev-dependencies]
mockall = "0.11"
```

```rust
use mockall::automock;

#[automock]
pub trait SmsGateway {
    fn send(&self, to: &str, body: &str) -> Result<String, Error>;
}

#[test]
fn test_with_mockall() {
    let mut mock = MockSmsGateway::new();
    mock.expect_send()
        .with(eq("+1234567890"), eq("Hello"))
        .times(1)
        .returning(|_, _| Ok("msg_123".to_string()));
    
    let service = MessageService::new(Box::new(mock));
    let result = service.send_message("+1234567890", "Hello");
    
    assert!(result.is_ok());
}
```

## Test Organization Patterns

### Arrange-Act-Assert Pattern

```rust
#[test]
fn test_aaa_pattern() {
    // Arrange
    let mut client = Client::new("test_key");
    let phone = "+1234567890";
    let body = "Test message";
    
    // Act
    let result = client.send(phone, body);
    
    // Assert
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert_eq!(msg.to, phone);
    assert_eq!(msg.body, body);
}
```

### Parameterized Tests

```rust
#[test]
fn test_cost_calculation_table() {
    let test_cases = vec![
        (0, 0),
        (50, 0),
        (100, 0),
        (101, 5),
        (150, 5),
        (200, 5),
        (201, 10),
    ];
    
    for (count, expected_cost) in test_cases {
        assert_eq!(
            calculate_cost(count),
            expected_cost,
            "Failed for count={}",
            count
        );
    }
}
```

## Textio Example: Comprehensive Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test fixtures
    mod fixtures {
        use super::*;
        
        pub fn valid_phone() -> &'static str {
            "+1234567890"
        }
        
        pub fn test_message() -> Message {
            Message::new("msg_1", "+1234567890", "Test")
        }
        
        pub fn test_client() -> Client {
            Client::new("test_key")
        }
    }

    // Validation tests
    mod validation {
        use super::*;
        
        #[test]
        fn test_valid_phone() {
            let result = validate_phone("+1234567890");
            assert!(result.is_ok());
        }
        
        #[test]
        #[should_panic(expected = "must start with '+'")]
        fn test_panic_on_invalid_phone() {
            validate_and_panic("invalid");
        }
    }

    // Cost calculation tests
    mod cost {
        use super::*;
        
        #[test]
        fn test_free_tier() {
            assert_eq!(calculate_cost(0), 0);
            assert_eq!(calculate_cost(100), 0);
        }
        
        #[test]
        fn test_paid_tier() {
            assert_eq!(calculate_cost(101), 5);
        }
    }

    // Slow tests
    mod integration {
        use super::*;
        
        #[test]
        #[ignore = "Requires database connection"]
        fn test_database_integration() {
            // ...
        }
    }
}
```

## Summary

- Use `#[should_panic]` for panic testing, with `expected` for message matching
- Use `#[ignore]` for slow or conditional tests
- Create fixtures with functions or lazy_static
- Use RAII for setup/teardown
- Mock external dependencies with traits
- Organize tests into modules
- Use AAA (Arrange-Act-Assert) pattern
- Run ignored tests with `--ignored` or `--include-ignored`

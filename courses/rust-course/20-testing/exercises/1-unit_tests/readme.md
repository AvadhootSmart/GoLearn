# Unit Testing in Rust

Unit tests are the foundation of a robust testing strategy. In Rust, unit tests are typically placed in the same file as the code they test, within a module annotated with `#[cfg(test)]`. This module is only compiled when running tests, keeping your production binary lean.

## The #[test] Attribute

Any function annotated with `#[test]` becomes a test function. Rust's test harness will discover and execute these functions when you run `cargo test`.

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

## Test Module Structure

By convention, unit tests are placed in a `tests` module at the bottom of your source file:

```rust
// Your production code here
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

The `#[cfg(test)]` attribute tells Rust to compile this module only when running tests. The `use super::*;` brings the parent module's items into scope.

## Assertions

Rust provides three main assertion macros:

### assert!

The most basic assertion. It checks that a boolean expression is true:

```rust
#[test]
fn test_is_valid() {
    let message = "Hello";
    assert!(!message.is_empty());
}
```

### assert_eq!

Asserts that two expressions are equal:

```rust
#[test]
fn test_equality() {
    let result = calculate_fee(100);
    assert_eq!(result, 5);
}
```

### assert_ne!

Asserts that two expressions are not equal:

```rust
#[test]
fn test_not_equal() {
    let id1 = generate_id();
    let id2 = generate_id();
    assert_ne!(id1, id2, "IDs should be unique");
}
```

## Custom Failure Messages

All assertion macros accept optional format strings for custom messages:

```rust
#[test]
fn test_with_message() {
    let balance = 50;
    assert!(
        balance > 100,
        "Insufficient balance: {} (minimum required: 100)",
        balance
    );
}
```

## Using panic! in Tests

Sometimes you want a test to fail with a specific message:

```rust
#[test]
fn test_unimplemented() {
    panic!("This feature is not yet implemented");
}
```

## Tests Returning Result<T, E>

Tests can return a `Result<(), E>` instead of panicking. This allows using the `?` operator:

```rust
#[test]
fn test_with_result() -> Result<(), String> {
    let value = parse_number("42")?;
    assert_eq!(value, 42);
    Ok(())
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("Parse error: {}", e))
}
```

This approach is cleaner when testing functions that return `Result` types.

## Running Tests

### Basic Test Run

```bash
cargo test
```

### Running Specific Tests

Filter tests by name:

```bash
cargo test test_add
cargo test integration
```

### Showing Output

By default, Rust captures stdout. Use `--nocapture` to see output:

```bash
cargo test -- --nocapture
```

### Running in Release Mode

For long-running tests:

```bash
cargo test --release
```

### Running a Single Test

```bash
cargo test test_name -- --exact
```

## Test Isolation

Each test runs in its own thread. Tests should not depend on each other or share mutable state. If you need shared setup, use fixtures (covered in a later exercise).

## Textio Example: Testing Message Parsing

In Textio, we parse incoming SMS messages. Here's how we might test this:

```rust
pub struct Message {
    pub to: String,
    pub body: String,
}

pub fn parse_message(input: &str) -> Result<Message, String> {
    let parts: Vec<&str> = input.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err("Invalid format. Expected 'to:body'".to_string());
    }
    
    let to = parts[0].trim().to_string();
    let body = parts[1].trim().to_string();
    
    if to.is_empty() {
        return Err("Recipient cannot be empty".to_string());
    }
    
    if body.is_empty() {
        return Err("Message body cannot be empty".to_string());
    }
    
    Ok(Message { to, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_message() {
        let msg = parse_message("+1234567890:Hello World").unwrap();
        assert_eq!(msg.to, "+1234567890");
        assert_eq!(msg.body, "Hello World");
    }

    #[test]
    fn test_parse_trims_whitespace() {
        let msg = parse_message("  +1234567890  :  Hello  ").unwrap();
        assert_eq!(msg.to, "+1234567890");
        assert_eq!(msg.body, "Hello");
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = parse_message("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_recipient() {
        let result = parse_message(":Hello");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Recipient"));
    }
}
```

## Testing Edge Cases

Good tests cover:
- Happy path (valid inputs)
- Boundary conditions
- Invalid inputs
- Empty inputs
- Extreme values

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost_free_tier() {
        assert_eq!(calculate_cost(0), 0);
        assert_eq!(calculate_cost(99), 0);
    }

    #[test]
    fn test_calculate_cost_paid_tier() {
        assert_eq!(calculate_cost(100), 5);
        assert_eq!(calculate_cost(1000), 50);
    }

    #[test]
    fn test_calculate_cost_large_volume() {
        assert_eq!(calculate_cost(10000), 400); // Bulk discount
    }
}
```

## Best Practices

1. **One assertion per test** - Makes failures easier to diagnose
2. **Descriptive test names** - `test_parse_rejects_empty_body` is better than `test_parse_3`
3. **Test behavior, not implementation** - Focus on inputs and outputs
4. **Keep tests simple** - Complex test logic can hide bugs
5. **Test failures too** - Ensure error cases work correctly

## Common Patterns

### Testing Option Types

```rust
#[test]
fn test_find_user() {
    let user = find_user_by_id(1);
    assert!(user.is_some());
    
    let user = user.unwrap();
    assert_eq!(user.name, "Alice");
}

#[test]
fn test_find_nonexistent_user() {
    let user = find_user_by_id(9999);
    assert!(user.is_none());
}
```

### Testing Result Types

```rust
#[test]
fn test_send_message() -> Result<(), String> {
    let result = send_sms("+1234567890", "Test")?;
    assert!(result.success);
    Ok(())
}
```

### Testing Collections

```rust
#[test]
fn test_filter_messages() {
    let messages = vec![
        Message::new("a", "pending"),
        Message::new("b", "sent"),
        Message::new("c", "pending"),
    ];
    
    let pending: Vec<_> = messages.iter()
        .filter(|m| m.status == "pending")
        .collect();
    
    assert_eq!(pending.len(), 2);
}
```

## Summary

- Use `#[test]` to mark test functions
- Place unit tests in `#[cfg(test)] mod tests`
- Use `assert!`, `assert_eq!`, and `assert_ne!` for assertions
- Tests can return `Result<(), E>` to use `?`
- Run tests with `cargo test`
- Use `--nocapture` to see println! output
- Filter tests by passing name patterns

In the next exercise, we'll explore integration tests, which test your library's public API from an external perspective.

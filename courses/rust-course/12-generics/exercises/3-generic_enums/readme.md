# Generic Enums in Rust

## Introduction

Generic enums combine the power of algebraic data types with type parameters, enabling you to create flexible, type-safe abstractions. The standard library's `Option<T>` and `Result<T, E>` are prime examples of how powerful generic enums can be. In this exercise, you'll learn to create and use generic enums in Textio's SMS API.

## Why Generic Enums?

### The Problem

Imagine Textio needs to represent optional values and operation results. Without generics:

```rust
enum OptionalInt {
    Some(i32),
    None,
}

enum OptionalString {
    Some(String),
    None,
}

enum IntResult {
    Ok(i32),
    Err(String),
}

enum MessageResult {
    Ok(Message),
    Err(String),
}
```

This quickly becomes unmanageable.

### The Generic Solution

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

let maybe_number: Option<i32> = Some(42);
let maybe_text: Option<String> = Some("hello".to_string());
let result: Result<Message, String> = Ok(message);
```

## Option<T>: Representing Presence or Absence

`Option<T>` is Rust's way of handling nullable values safely:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Textio Examples

```rust
// Finding a message by ID
fn find_message(id: u32) -> Option<Message> {
    // Returns Some(message) if found, None otherwise
}

// Getting a configuration value
fn get_config(key: &str) -> Option<String> {
    // Returns Some(value) if key exists, None otherwise
}

// Parsing a phone number
fn parse_phone(input: &str) -> Option<PhoneNumber> {
    // Returns Some(number) if valid, None if invalid
}
```

### Working with Option

```rust
// Unwrapping with default
let count: i32 = maybe_count.unwrap_or(0);

// Mapping the contained value
let length: Option<usize> = maybe_string.map(|s| s.len());

// Chaining operations
let domain: Option<String> = email
    .and_then(|e| extract_domain(e))
    .map(|d| d.to_lowercase());
```

## Result<T, E>: Representing Success or Failure

`Result<T, E>` is for operations that can fail:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Textio Examples

```rust
// Sending a message
fn send_sms(message: Message) -> Result<DeliveryReport, SendError> {
    // Returns Ok(report) on success, Err(error) on failure
}

// Validating a phone number
fn validate_phone(phone: &str) -> Result<PhoneNumber, ValidationError> {
    // Returns Ok(valid_number) or Err(validation_error)
}

// Parsing JSON
fn parse_webhook(body: &str) -> Result<WebhookEvent, JsonError> {
    // Returns Ok(event) or Err(json_error)
}
```

### Working with Result

```rust
// Propagating errors with ?
let report = send_sms(message)?;

// Mapping success value
let id: Result<u32, Error> = result.map(|r| r.id);

// Mapping error
let result = operation.map_err(|e| format!("Failed: {}", e));

// Providing default
let count = result.unwrap_or(0);
```

## Creating Custom Generic Enums

### Textio's Message Status

```rust
#[derive(Debug)]
enum MessageStatus<T> {
    Pending,
    Processing(T),
    Delivered { at: u64, details: T },
    Failed(String),
}

let pending: MessageStatus<()> = MessageStatus::Pending;
let processing: MessageStatus<f32> = MessageStatus::Processing(0.5);
let delivered: MessageStatus<String> = MessageStatus::Delivered {
    at: 1630000000,
    details: "via_twillio".to_string(),
};
```

### Generic Event Types

```rust
enum Event<T, M> {
    MessageReceived { payload: T, metadata: M },
    MessageSent { id: u64, payload: T },
    Error { code: u16, message: String },
}

let sms_event: Event<SmsMessage, SmsMetadata> = Event::MessageReceived {
    payload: sms,
    metadata: meta,
};
```

## Pattern Matching with Generic Enums

### Basic Matching

```rust
fn process_option<T>(opt: Option<T>) -> String {
    match opt {
        Some(value) => format!("Got: {:?}", value),
        None => "Nothing".to_string(),
    }
}
```

### Matching with Guards

```rust
fn categorize_result<T, E: Debug>(result: Result<T, E>) -> String {
    match result {
        Ok(value) => format!("Success: {:?}", value),
        Err(e) if format!("{:?}", e).contains("timeout") => "Timeout error".to_string(),
        Err(e) => format!("Error: {:?}", e),
    }
}
```

### Binding Values

```rust
fn handle_status<T: Debug>(status: MessageStatus<T>) {
    match status {
        MessageStatus::Pending => println!("Still waiting..."),
        MessageStatus::Processing(progress) => println!("Progress: {:?}", progress),
        MessageStatus::Delivered { at, details } => {
            println!("Delivered at {} with {:?}", at, details);
        }
        MessageStatus::Failed(msg) => println!("Failed: {}", msg),
    }
}
```

## Methods on Generic Enums

```rust
impl<T> Option<T> {
    fn is_some(&self) -> bool {
        match self {
            Some(_) => true,
            None => false,
        }
    }
    
    fn unwrap_or(self, default: T) -> T {
        match self {
            Some(value) => value,
            None => default,
        }
    }
}

impl<T, E> Result<T, E> {
    fn is_ok(&self) -> bool {
        match self {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Result<U, E> {
        match self {
            Ok(value) => Ok(f(value)),
            Err(e) => Err(e),
        }
    }
}
```

## Combining Option and Result

Real-world code often chains these:

```rust
fn send_message(id: u32) -> Result<DeliveryReport, String> {
    let message = find_message(id)
        .ok_or_else(|| format!("Message {} not found", id))?;
    
    let validated = validate_message(message)
        .map_err(|e| format!("Validation failed: {}", e))?;
    
    let report = deliver(validated)
        .map_err(|e| format!("Delivery failed: {}", e))?;
    
    Ok(report)
}
```

## Textio's Custom Result Types

### Domain-Specific Results

```rust
#[derive(Debug)]
enum SmsResult<T> {
    Success(T),
    InvalidNumber(String),
    RateLimited { retry_after: u64 },
    NetworkError(String),
}

impl<T> SmsResult<T> {
    fn is_success(&self) -> bool {
        matches!(self, SmsResult::Success(_))
    }
    
    fn unwrap_success(self) -> T {
        match self {
            SmsResult::Success(value) => value,
            _ => panic!("Called unwrap_success on non-success"),
        }
    }
}
```

### Error Aggregation

```rust
#[derive(Debug)]
enum BatchResult<T, E> {
    AllSuccess(Vec<T>),
    PartialSuccess { 
        succeeded: Vec<T>, 
        failed: Vec<(usize, E)> 
    },
    AllFailed(Vec<E>),
}
```

## Converting Between Types

### Option to Result

```rust
// Option<T> -> Result<T, E>
let result = option.ok_or(error);
let result = option.ok_or_else(|| make_error());

// Result<T, E> -> Option<T>
let option = result.ok();
```

### Result Transformations

```rust
// Swap Ok and Err
let swapped = result.flip();

// Convert error type
let converted = result.map_err(|e| NewError::from(e));
```

## Best Practices

1. **Use Option for nullable values** - Never use null references
2. **Use Result for fallible operations** - Always include meaningful error types
3. **Prefer combinators over match** when appropriate:
```rust
// Verbose
match option {
    Some(v) => Some(v * 2),
    None => None,
}

// Concise
option.map(|v| v * 2)
```

4. **Use the ? operator** for error propagation
5. **Create domain-specific enums** for complex states

## Exercise Overview

In this exercise, you will:
1. Create a custom `Option<T>` enum with methods
2. Build a `Result<T, E>` enum with transformations
3. Implement a `MessageStatus<T>` enum for Textio
4. Create a `WebResponse<T>` enum for API responses
5. Implement error handling patterns with generic enums

## Key Takeaways

- `Option<T>` represents presence or absence of a value
- `Result<T, E>` represents success or failure
- Generic enums can have multiple type parameters
- Pattern matching works naturally with generic variants
- Methods can be implemented on generic enums
- Combinators like `map`, `and_then` enable elegant chaining
- The `?` operator works with `Result` for error propagation

## Next Steps

Now that you understand generic enums, you'll learn about multiple type parameters, constraints, and PhantomData for advanced generic patterns.

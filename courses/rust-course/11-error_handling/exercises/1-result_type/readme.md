# Exercise 1: Result Type in Rust

## Introduction

In Rust, error handling is explicit and type-safe. The `Result<T, E>` enum is the primary way to handle recoverable errors. Unlike exceptions in other languages, Rust forces you to acknowledge and handle potential failures at compile time.

## The Result Enum

`Result<T, E>` is a generic enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),   // Contains the success value
    Err(E),  // Contains the error value
}
```

- `T` represents the type of the success value
- `E` represents the type of the error value

## Why Result Over Exceptions?

1. **Explicit Error Handling**: You must handle errors; they cannot be silently ignored
2. **Type Safety**: The compiler ensures you handle both success and failure cases
3. **No Hidden Control Flow**: Errors don't cause unexpected jumps in your code
4. **Composable**: Results can be chained and transformed

## Creating Results

```rust
let success: Result<i32, &str> = Ok(42);
let failure: Result<i32, &str> = Err("Something went wrong");
```

## Pattern Matching Results

The most explicit way to handle a `Result`:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

## Checking Result State

### is_ok() and is_err()

```rust
let result = divide(10.0, 2.0);
if result.is_ok() {
    println!("Success!");
}
if result.is_err() {
    println!("Failed!");
}
```

These methods return booleans and are useful for conditional logic without extracting the value.

## Unwrapping Values

### unwrap()

Extracts the value if `Ok`, panics if `Err`:

```rust
let result: Result<i32, &str> = Ok(42);
let value = result.unwrap(); // Returns 42

let bad_result: Result<i32, &str> = Err("error");
// bad_result.unwrap(); // PANICS!
```

Use `unwrap()` only when you're certain the result is `Ok`, or in prototypes/tests.

### expect()

Like `unwrap()`, but lets you provide a custom panic message:

```rust
let result: Result<i32, &str> = Err("error");
// result.expect("Division should not fail"); // PANICS with your message
```

### unwrap_or()

Provides a default value if the result is `Err`:

```rust
let result: Result<i32, &str> = Err("error");
let value = result.unwrap_or(0); // Returns 0 instead of panicking
```

### unwrap_or_else()

Computes a default value using a closure:

```rust
let result: Result<i32, &str> = Err("error");
let value = result.unwrap_or_else(|e| {
    println!("Error occurred: {}", e);
    0
});
```

## Transforming Results

### map()

Transforms the `Ok` value, leaves `Err` unchanged:

```rust
let result: Result<i32, &str> = Ok(5);
let doubled = result.map(|x| x * 2); // Ok(10)

let err_result: Result<i32, &str> = Err("error");
let unchanged = err_result.map(|x| x * 2); // Err("error")
```

### map_err()

Transforms the `Err` value, leaves `Ok` unchanged:

```rust
let result: Result<i32, &str> = Err("error");
let with_context = result.map_err(|e| format!("Context: {}", e));
// Err("Context: error")
```

### and_then()

Chains operations that return `Result`:

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

fn double_if_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n * 2)
    } else {
        Err("Number must be positive".to_string())
    }
}

let result = parse_number("5")
    .and_then(double_if_positive); // Ok(10)
```

## Textio SMS API Example

In our Textio SMS API, we use `Result` extensively:

```rust
fn send_sms(to: &str, message: &str) -> Result<MessageId, SmsError> {
    // Validate phone number
    if !is_valid_phone(to) {
        return Err(SmsError::InvalidPhone);
    }
    
    // Validate message length
    if message.len() > 160 {
        return Err(SmsError::MessageTooLong);
    }
    
    // Send the message
    let id = transmit_to_carrier(to, message)?;
    Ok(id)
}
```

## Common Patterns

### Early Return on Error

```rust
fn process() -> Result<String, Error> {
    let data = read_file("config.txt")?;  // Returns early if Err
    let parsed = parse_config(&data)?;    // Returns early if Err
    Ok(parsed)
}
```

### Providing Context

```rust
result.map_err(|e| format!("Failed to process: {}", e))?;
```

### Default Values for Errors

```rust
let timeout = parse_timeout(config)
    .unwrap_or(DEFAULT_TIMEOUT);
```

## Best Practices

1. **Use pattern matching** for comprehensive error handling
2. **Avoid unwrap()** in production code; use `?` or `unwrap_or()`
3. **Provide context** with `map_err()` when propagating errors
4. **Use descriptive error types** rather than strings
5. **Document what errors** a function can return

## When to Use Result vs panic!

- Use `Result` for **recoverable errors** (file not found, invalid input, network timeout)
- Use `panic!` for **unrecoverable errors** (logic bugs, invariant violations)

## Exercise Task

In this exercise, you'll implement SMS parsing functions for Textio:

1. `parse_phone_number` - Parse and validate phone numbers
2. `parse_message_length` - Parse message length from string
3. `send_sms` - Simulate sending an SMS with validation

You'll practice:
- Creating `Result` values
- Pattern matching on results
- Using `unwrap()`, `expect()`, and `unwrap_or()`
- Using `is_ok()` and `is_err()`

Run your code with:
```bash
rustc code.rs && ./code
```

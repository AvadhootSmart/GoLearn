# Exercise 3: Match Expressions

## Overview

The `match` expression is Rust's powerful pattern matching construct. It compares a value against patterns and executes code for the first matching pattern. Match expressions are exhaustive - you must handle every possible case.

## Basic Match Syntax

```rust
let status = MessageStatus::Sent;

match status {
    MessageStatus::Pending => println!("Waiting..."),
    MessageStatus::Sent => println!("On its way!"),
    MessageStatus::Delivered => println!("Arrived!"),
    MessageStatus::Failed => println!("Oops!"),
}
```

## Match is an Expression

Match returns a value, making it an expression:

```rust
let description = match status {
    MessageStatus::Pending => "pending",
    MessageStatus::Sent => "sent",
    MessageStatus::Delivered => "delivered",
    MessageStatus::Failed => "failed",
};
```

All arms must return the same type.

## Pattern Binding

Capture values from enum variants:

```rust
match message {
    Message::Text(body) => println!("Text: {}", body),
    Message::Media { url, caption } => {
        println!("Media at: {}", url);
        if let Some(cap) = caption {
            println!("Caption: {}", cap);
        }
    }
    Message::Empty => println!("No content"),
}
```

## Destructuring

### Tuples

```rust
let coords = (10, 20);

match coords {
    (0, y) => println!("On y-axis at {}", y),
    (x, 0) => println!("On x-axis at {}", x),
    (x, y) => println!("At ({}, {})", x, y),
}
```

### Structs

```rust
struct Point { x: i32, y: i32 }

match point {
    Point { x: 0, y: 0 } => println!("Origin"),
    Point { x: 0, y } => println!("On y-axis: {}", y),
    Point { x, y: 0 } => println!("On x-axis: {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

### Enums

```rust
match result {
    Result::Ok(value) => println!("Success: {}", value),
    Result::Err(code) => println!("Error: {}", code),
}
```

## The `_` Placeholder

Catch-all for unhandled patterns:

```rust
match status {
    MessageStatus::Delivered => println!("Success!"),
    MessageStatus::Failed => println!("Failed!"),
    _ => println!("In progress..."),
}
```

## Match Guards

Add conditions to patterns:

```rust
match message {
    Message::Text(body) if body.len() > 160 => {
        println!("Long message ({} chars)", body.len());
    }
    Message::Text(body) => {
        println!("Normal message: {}", body);
    }
    _ => println!("Not a text message"),
}
```

Guards can use variables from the pattern:

```rust
match (x, y) {
    (a, b) if a == b => println!("Equal: {}", a),
    (a, b) if a > b => println!("{} > {}", a, b),
    (a, b) => println!("{} < {}", a, b),
}
```

## Binding with `@`

Bind a value while also matching it:

```rust
match age {
    n @ 0..=12 => println!("Child: {} years old", n),
    n @ 13..=19 => println!("Teenager: {} years old", n),
    n @ 20..=29 => println!("Young adult: {} years old", n),
    n => println!("Adult: {} years old", n),
}
```

## Multiple Patterns

```rust
match status {
    MessageStatus::Pending | MessageStatus::Sent => {
        println!("In progress");
    }
    MessageStatus::Delivered | MessageStatus::Failed => {
        println!("Terminal state");
    }
}
```

## Range Patterns

For numeric and char types:

```rust
match grade {
    90..=100 => 'A',
    80..=89 => 'B',
    70..=79 => 'C',
    60..=69 => 'D',
    _ => 'F',
}
```

## Nested Patterns

```rust
match nested_option {
    Some(Some(value)) => println!("Double-wrapped: {}", value),
    Some(None) => println!("Inner is None"),
    None => println!("Outer is None"),
}
```

## Destructuring References

```rust
match &message {
    Message::Text(body) => println!("Body: {}", body),
    // body is &String here
}

// Or explicitly:
match message {
    Message::Text(ref body) => println!("Body: {}", body),
    // body is &String, message not moved
}
```

## Match with Slices

```rust
match numbers.as_slice() {
    [] => println!("Empty"),
    [one] => println!("Single: {}", one),
    [first, second] => println!("Pair: {} and {}", first, second),
    [first, .., last] => println!("First: {}, Last: {}", first, last),
    [first, rest @ ..] => println!("First: {}, rest: {:?}", first, rest),
}
```

## Exhaustiveness

Match must cover all possibilities:

```rust
// This won't compile!
match option {
    Some(value) => println!("{}", value),
    // error: non-exhaustive patterns: `None` not covered
}
```

## Real-World Textio Example

```rust
enum DeliveryResult {
    Delivered { id: String, segments: u8 },
    Failed { code: u16, reason: String },
    Pending { retry_count: u8 },
}

fn handle_result(result: DeliveryResult) -> String {
    match result {
        DeliveryResult::Delivered { id, segments } if segments > 1 => {
            format!("Multi-part message {} delivered in {} segments", id, segments)
        }
        DeliveryResult::Delivered { id, .. } => {
            format!("Message {} delivered", id)
        }
        DeliveryResult::Failed { code, reason } if code >= 500 => {
            format!("Server error {}: {}", code, reason)
        }
        DeliveryResult::Failed { code, reason } => {
            format!("Client error {}: {}", code, reason)
        }
        DeliveryResult::Pending { retry_count } if retry_count >= 3 => {
            String::from("Max retries exceeded")
        }
        DeliveryResult::Pending { retry_count } => {
            format!("Retry {} in progress", retry_count)
        }
    }
}
```

## Match vs If-Else

Match is preferred when:
- Multiple conditions on the same value
- Pattern matching is needed
- Exhaustiveness checking is valuable

```rust
// Prefer match
match status {
    Status::Active => ...,
    Status::Inactive => ...,
    Status::Suspended => ...,
}

// If-else is fine for simple conditions
if user.is_admin {
    ...
} else {
    ...
}
```

## Common Patterns in Textio

### Message Type Handling

```rust
match message_type {
    MessageType::Sms { to, body } => send_sms(to, body),
    MessageType::Mms { to, body, media } => send_mms(to, body, media),
    MessageType::Scheduled { to, body, time } => schedule_sms(to, body, time),
}
```

### Error Classification

```rust
match error {
    Error::Network(_) => ErrorClass::Temporary,
    Error::InvalidNumber(_) => ErrorClass::Permanent,
    Error::RateLimited { .. } => ErrorClass::Retryable,
}
```

### Response Parsing

```rust
match response.status {
    200..=299 => Ok(response.body),
    400 => Err(Error::BadRequest),
    401 => Err(Error::Unauthorized),
    429 => Err(Error::RateLimited),
    500..=599 => Err(Error::Server),
    _ => Err(Error::Unknown),
}
```

## Exercise

You'll implement match expressions for Textio:
1. Message type routing
2. Status code classification
3. Error handling with guards
4. Nested pattern matching
5. Range patterns for validation

## Key Takeaways

1. Match is exhaustive - all cases must be handled
2. Match is an expression that returns a value
3. Patterns can bind values with `=>`
4. Use `_` for catch-all patterns
5. Guards add conditions: `pattern if condition`
6. `@` binds while matching: `name @ pattern`
7. Multiple patterns: `A | B =>`
8. Ranges work on numbers and chars: `1..=10`
9. Match on references with `&` or `ref`
10. Nested patterns for complex data

## Next Steps

Next, we'll learn `if let` - a concise syntax for when you only care about one pattern and want to ignore the rest.

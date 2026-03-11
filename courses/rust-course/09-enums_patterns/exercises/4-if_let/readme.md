# Exercise 4: if let and let else

## Overview

When you only care about one pattern and want to ignore all others, `if let` provides a more concise syntax than `match`. The `let else` pattern (Rust 1.65+) handles early returns when patterns don't match.

## The Problem with Match

When you only care about one variant:

```rust
// Verbose with match
match some_option {
    Some(value) => println!("{}", value),
    _ => (),  // Do nothing for None
}
```

## if let Syntax

```rust
// Concise with if let
if let Some(value) = some_option {
    println!("{}", value);
}
```

### General Form

```rust
if let PATTERN = EXPRESSION {
    // Run if pattern matches
} else {
    // Optional: run if pattern doesn't match
}
```

## When to Use if let

Use `if let` when:
- You care about one pattern
- Other patterns should be ignored or handled uniformly
- You want more readable code

Use `match` when:
- You need to handle multiple patterns differently
- Exhaustiveness checking is important
- You're dealing with public enums that might gain variants

## if let with else

```rust
if let Some(user) = find_user(user_id) {
    println!("Found: {}", user.name);
} else {
    println!("User not found");
}
```

## if let with Enums

```rust
enum Message {
    Text(String),
    Binary(Vec<u8>),
}

let msg = Message::Text(String::from("Hello"));

if let Message::Text(body) = msg {
    println!("Text message: {}", body);
}
```

## The let else Pattern (Rust 1.65+)

For early returns when a pattern doesn't match:

```rust
fn process_message(id: u64) -> Result<String, Error> {
    let message = find_message(id).ok_or(Error::NotFound)?;
    
    // Before let else, you might write:
    let Message::Text(body) = message else {
        return Err(Error::NotTextMessage);
    };
    
    Ok(body)
}
```

### Syntax

```rust
let PATTERN = EXPRESSION else {
    // Must diverge (return, break, continue, or panic)
};
// Pattern is bound here
```

### Why let else?

Without let else:

```rust
fn get_message_body(msg: Message) -> Option<String> {
    match msg {
        Message::Text(body) => Some(body),
        _ => None,
    }
}
```

With let else:

```rust
fn get_message_body(msg: Message) -> Option<String> {
    let Message::Text(body) = msg else {
        return None;
    };
    Some(body)
}
```

The let else version is often clearer when you have guard clauses.

## Combining with Other Patterns

### Nested Patterns

```rust
if let Some(Some(inner)) = nested_option {
    println!("Found: {}", inner);
}
```

### Struct Destructuring

```rust
struct Config {
    name: String,
    value: Option<i32>,
}

if let Config { value: Some(v), .. } = config {
    println!("Value is: {}", v);
}
```

### With Match Guards

```rust
if let Some(n) = number if n > 0 {
    println!("Positive: {}", n);
}
```

## if let in Loops

Iterate while a pattern matches:

```rust
let mut iter = vec![Some(1), Some(2), None, Some(3)].into_iter();

while let Some(Some(value)) = iter.next() {
    println!("{}", value);
}
// Prints: 1, 2 (stops at None)
```

## Real-World Textio Examples

### Processing Optional Sender

```rust
fn send_sms(to: &str, body: &str, from: Option<&str>) {
    let sender = from.unwrap_or("TEXTIO");
    
    if let Some(custom_sender) = from {
        if !validate_sender_id(custom_sender) {
            panic!("Invalid sender ID");
        }
    }
    
    // Send message...
}
```

### Handling API Responses

```rust
fn handle_response(response: ApiResponse) -> Result<MessageId, Error> {
    let ApiResponse::Success { id, .. } = response else {
        return Err(Error::ApiFailed);
    };
    
    Ok(id)
}
```

### Extracting Error Details

```rust
fn log_error(error: &ApiError) {
    if let ApiError::RateLimited { retry_after } = error {
        warn!("Rate limited, retry after {}s", retry_after);
    }
}
```

### let else Chain

```rust
fn process_scheduled_message(msg: Message) -> Result<(), Error> {
    let Message::Scheduled { to, body, send_at } = msg else {
        return Err(Error::NotScheduled);
    };
    
    let user = find_user_by_phone(&to).ok_or(Error::UserNotFound)?;
    
    if send_at < current_timestamp() {
        return Err(Error::ScheduledTimePassed);
    }
    
    queue_message(to, body, send_at);
    Ok(())
}
```

## Comparison: match vs if let vs let else

| Situation | Best Choice |
|-----------|-------------|
| One pattern, ignore rest | `if let` |
| One pattern, early return | `let else` |
| Multiple patterns, different handling | `match` |
| Exhaustiveness important | `match` |
| Simple Option handling | `if let` |
| Complex guard logic | `let else` |

## Common Patterns

### Option Handling

```rust
// Quick check
if let Some(value) = optional {
    use_value(value);
}

// With default
let value = if let Some(v) = optional {
    v
} else {
    default_value()
};

// Better: use unwrap_or
let value = optional.unwrap_or(default_value());
```

### Result Handling

```rust
if let Ok(data) = parse_json(&input) {
    process(data);
}
```

### Enum Variant Check

```rust
if matches!(status, MessageStatus::Delivered { .. }) {
    mark_complete();
}
```

## The matches! Macro

For when you just need a boolean:

```rust
// Instead of:
if let MessageStatus::Delivered { .. } = status { true } else { false }

// Use:
if matches!(status, MessageStatus::Delivered { .. }) {
    // ...
}

// Or just get the bool:
let is_done = matches!(status, MessageStatus::Delivered { .. });
```

## Best Practices

### 1. Use if let for Single Pattern Focus

```rust
// Good
if let Some(user) = find_user(id) {
    greet(user);
}

// Overkill
match find_user(id) {
    Some(user) => greet(user),
    _ => (),
}
```

### 2. Use let else for Early Returns

```rust
// Good
let user = find_user(id).ok_or(Error::NotFound)?;
let User { phone: Some(number), .. } = user else {
    return Err(Error::NoPhone);
};

// Less clear
match find_user(id) {
    Some(User { phone: Some(number), .. }) => number,
    Some(_) => return Err(Error::NoPhone),
    None => return Err(Error::NotFound),
}
```

### 3. Don't Overuse if let

```rust
// Bad - should use match
if let Message::Text(body) = msg {
    handle_text(body);
}
if let Message::Binary(data) = msg {
    handle_binary(data);
}
// What about other variants?
```

### 4. Consider match for Public APIs

```rust
// If Message might get new variants, match is safer
match msg {
    Message::Text(body) => handle_text(body),
    Message::Binary(data) => handle_binary(data),
    _ => unimplemented!(),  // Explicitly handle unknowns
}
```

## Exercise

You'll implement:
1. if let for optional value handling
2. let else for early returns
3. while let for iteration
4. Combining patterns with guards
5. matches! macro usage

## Key Takeaways

1. `if let PATTERN = EXPR { }` - concise single pattern matching
2. Optional `else` block for non-matching case
3. `let PATTERN = EXPR else { return/break/panic };` - early return on non-match
4. Use when you care about one pattern
5. `while let` for iterating while pattern matches
6. `matches!` macro returns boolean
7. Don't overuse - `match` is better for multiple patterns
8. Consider API evolution when choosing between match and if let

## Next Steps

In the final exercise, we'll explore exhaustive matching and how the Rust compiler ensures you handle all cases, making your code more robust.

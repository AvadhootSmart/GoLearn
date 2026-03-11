# Exercise 5: Exhaustive Matching

## Overview

One of Rust's most powerful features is exhaustive pattern matching. The compiler ensures you handle every possible case, preventing bugs from unhandled edge cases. This is a cornerstone of Rust's safety guarantees.

## Why Exhaustiveness Matters

In other languages:

```c
// C - What if status is something else?
switch (status) {
    case PENDING: handle_pending(); break;
    case SENT: handle_sent(); break;
    // Forgot DELIVERED and FAILED - silent bug!
}
```

In Rust:

```rust
// Won't compile - must handle all variants
match status {
    MessageStatus::Pending => handle_pending(),
    MessageStatus::Sent => handle_sent(),
    // error[E0004]: non-exhaustive patterns: `Delivered` and `Failed` not covered
}
```

## The _ Wildcard

Catch all remaining patterns:

```rust
match status {
    MessageStatus::Delivered => handle_success(),
    MessageStatus::Failed => handle_failure(),
    _ => handle_in_progress(),
}
```

**Warning**: Using `_` can hide bugs when new enum variants are added.

## Catch-All with Binding

Capture the value while catching all:

```rust
match status {
    MessageStatus::Delivered => println!("Success!"),
    other => println!("Still working: {:?}", other),
}
```

## Best Practice: Explicit vs Catch-All

### When to Use Explicit Patterns

For public enums in your API:

```rust
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

// Good - compiler will tell you when new variant added
fn handle_status(status: MessageStatus) {
    match status {
        MessageStatus::Pending => ...,
        MessageStatus::Sent => ...,
        MessageStatus::Delivered => ...,
        MessageStatus::Failed => ...,
    }
}
```

When you add a new variant:

```rust
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Cancelled,  // New variant
}

// Now this won't compile - you're forced to handle Cancelled!
```

### When to Use _ (Underscore)

For external enums you don't control, or when all other cases truly are equivalent:

```rust
// External library might add new variants
fn handle_http_status(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Other",  // Unknown codes
    }
}
```

## The #[non_exhaustive] Attribute

Mark enums that might gain variants:

```rust
#[non_exhaustive]
pub enum ApiError {
    Network,
    Timeout,
    // Library might add more later
}
```

Users must use `_`:

```rust
// This won't compile with non_exhaustive
match error {
    ApiError::Network => ...,
    ApiError::Timeout => ...,
    // error: non-exhaustive patterns
}

// Must include catch-all
match error {
    ApiError::Network => ...,
    ApiError::Timeout => ...,
    _ => ...,
}
```

## Exhaustiveness with Structs

```rust
struct Config {
    host: String,
    port: u16,
    use_ssl: bool,
}

// Must match all fields (or use ..)
let Config { host, port, use_ssl } = config;

// Use .. to ignore remaining fields
let Config { host, .. } = config;
```

## Exhaustiveness with Tuples

```rust
let tuple = (1, 2, 3);

// Must match all elements
let (a, b, c) = tuple;

// Use .. for rest
let (first, .., last) = tuple;
```

## Exhaustiveness with Slices

```rust
fn describe_slice(slice: &[i32]) -> String {
    match slice {
        [] => String::from("Empty"),
        [one] => format!("One: {}", one),
        [a, b] => format!("Two: {} and {}", a, b),
        _ => format!("Many: {} items", slice.len()),
    }
}
```

## The matches! Macro for Boolean Checks

When you just need true/false:

```rust
let is_terminal = matches!(status, MessageStatus::Delivered | MessageStatus::Failed);

let is_success_code = matches!(code, 200..=299);
```

## Compiler Error Messages

Rust's error messages tell you exactly what's missing:

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::Up => y += 1,
        Direction::Down => y -= 1,
        // error[E0004]: non-exhaustive patterns: `Left` and `Right` not covered
    }
}
```

## Pattern: Future-Proofing

### Use Explicit Matches

```rust
// Good for internal enums
match status {
    Status::Pending => ...,
    Status::Sent => ...,
    Status::Delivered => ...,
    Status::Failed => ...,
}
```

### Use _ with Logging for External

```rust
// Good for external/evolving enums
match external_status {
    KnownVariant::A => handle_a(),
    KnownVariant::B => handle_b(),
    unknown => {
        log::warn!("Unhandled status: {:?}", unknown);
        handle_default();
    }
}
```

## Exhaustiveness in Real-World Textio

### Message Status Handling

```rust
fn process_status_update(status: MessageStatus) -> Result<(), Error> {
    match status {
        MessageStatus::Pending => {
            info!("Message queued");
            Ok(())
        }
        MessageStatus::Sent => {
            info!("Message sent to carrier");
            Ok(())
        }
        MessageStatus::Delivered { timestamp } => {
            info!("Delivered at {}", timestamp);
            notify_webhook(WebhookEvent::Delivered)?;
            Ok(())
        }
        MessageStatus::Failed { code, reason } => {
            error!("Failed with code {}: {}", code, reason);
            notify_webhook(WebhookEvent::Failed)?;
            Err(Error::DeliveryFailed(code, reason))
        }
    }
}
```

### Error Classification

```rust
fn classify_error(error: SmsError) -> ErrorAction {
    match error {
        SmsError::InvalidNumber(_) => ErrorAction::Permanent,
        SmsError::RateLimited { .. } => ErrorAction::RetryLater,
        SmsError::NetworkError(_) => ErrorAction::RetryNow,
        SmsError::CarrierError { code, .. } if code >= 500 => ErrorAction::RetryLater,
        SmsError::CarrierError { .. } => ErrorAction::Permanent,
    }
}
```

### Feature Flag Handling

```rust
#[derive(Debug, Clone, Copy)]
pub enum Feature {
    Sms,
    Mms,
    ScheduledMessages,
    Webhooks,
    Analytics,
}

fn is_enabled(feature: Feature, plan: Plan) -> bool {
    match (feature, plan) {
        (Feature::Sms, _) => true,  // All plans have SMS
        (Feature::Mms, Plan::Free) => false,
        (Feature::Mms, _) => true,
        (Feature::ScheduledMessages, Plan::Free) => false,
        (Feature::ScheduledMessages, _) => true,
        (Feature::Webhooks, Plan::Free | Plan::Basic) => false,
        (Feature::Webhooks, Plan::Pro | Plan::Enterprise) => true,
        (Feature::Analytics, Plan::Enterprise) => true,
        (Feature::Analytics, _) => false,
    }
}
```

## Handling Impossible States

Sometimes you "know" a state is impossible:

```rust
fn unwrap_delivered(status: MessageStatus) -> u64 {
    match status {
        MessageStatus::Delivered { timestamp } => timestamp,
        _ => unreachable!("Called unwrap_delivered on non-delivered status"),
    }
}
```

Better: Design types to make impossible states unrepresentable:

```rust
enum DeliveryState {
    InProgress,
    Complete { timestamp: u64 },
    Failed { reason: String },
}

fn get_timestamp(state: &DeliveryState) -> Option<u64> {
    match state {
        DeliveryState::Complete { timestamp } => Some(*timestamp),
        _ => None,
    }
}
```

## Pattern: Total Functions

Every input has a defined output - no panics:

```rust
// Total function - handles all cases
fn status_to_string(status: MessageStatus) -> &'static str {
    match status {
        MessageStatus::Pending => "pending",
        MessageStatus::Sent => "sent",
        MessageStatus::Delivered { .. } => "delivered",
        MessageStatus::Failed { .. } => "failed",
    }
}

// Partial function - can panic
fn status_to_code(status: MessageStatus) -> u16 {
    match status {
        MessageStatus::Delivered { .. } => 200,
        MessageStatus::Failed { code, .. } => code,
        _ => panic!("No code for in-progress status"),
    }
}
```

## Exercise

You'll implement exhaustive pattern matching for Textio:
1. Handling all enum variants explicitly
2. Using `_` appropriately for catch-alls
3. Handling `#[non_exhaustive]` patterns
4. Working with slice patterns
5. Using `matches!` for boolean checks

## Key Takeaways

1. Match must be exhaustive - all cases must be handled
2. The compiler catches missing patterns at compile time
3. Use `_` for catch-all, but be careful with public enums
4. `#[non_exhaustive]` forces downstream users to use `_`
5. Explicit patterns help when enums evolve
6. The `matches!` macro provides boolean pattern checks
7. Total functions handle all inputs without panicking
8. Design types to make invalid states unrepresentable
9. Use `unreachable!` sparingly - prefer Option/Result
10. Compiler error messages tell you exactly what's missing

## Conclusion

Exhaustive matching is one of Rust's superpowers. It transforms runtime bugs into compile-time errors, making your code more reliable and easier to maintain. Combined with enums and pattern matching, it enables you to model domains precisely and handle all cases correctly.

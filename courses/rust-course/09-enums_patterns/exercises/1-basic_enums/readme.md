# Exercise 1: Basic Enums

## Overview

Enums (enumerations) allow you to define a type by listing its possible variants. In Rust, enums are far more powerful than in most languages because each variant can carry data.

## Why Enums Matter for Textio

In our SMS API, we constantly deal with finite sets of possibilities:
- Message status: Pending, Sent, Delivered, Failed
- Priority levels: Low, Normal, High, Urgent
- Error types: NetworkError, InvalidNumber, RateLimited

Enums make these explicit and type-safe.

## Basic Enum Syntax

```rust
enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}
```

Each variant is a possible value. Create one with:

```rust
let status = MessageStatus::Sent;
```

## Enums with Data

Rust enums can hold data in variants:

```rust
enum Message {
    Text(String),              // Single unnamed field (tuple variant)
    Media { url: String },     // Named fields (struct variant)
    System(u32, String),       // Multiple unnamed fields
    Empty,                     // No data
}
```

Creating instances:

```rust
let msg1 = Message::Text(String::from("Hello"));
let msg2 = Message::Media { url: String::from("https://...") };
let msg3 = Message::System(1, String::from("Connected"));
let msg4 = Message::Empty;
```

## Enum Namespacing

Variants live in the enum's namespace:

```rust
let status = MessageStatus::Pending;  // Full path

use MessageStatus::*;  // Import all variants
let status = Pending;  // Now available directly
```

## The `#[derive]` Attribute

By default, enums have few capabilities. Add traits with derive:

```rust
#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

println!("{:?}", Priority::High);  // Works because of Debug
assert_eq!(Priority::High, Priority::High);  // Works because of PartialEq
```

Common derives:
- `Debug` - For `{:?}` printing
- `Clone` - For `.clone()`
- `Copy` - For stack-only types (no heap data)
- `PartialEq` - For `==` comparison
- `Eq` - For full equality
- `PartialOrd`, `Ord` - For ordering/comparison

## Enum Methods

Enums can have `impl` blocks:

```rust
impl MessageStatus {
    fn is_terminal(&self) -> bool {
        match self {
            MessageStatus::Delivered => true,
            MessageStatus::Failed => true,
            _ => false,
        }
    }
    
    fn description(&self) -> &str {
        match self {
            MessageStatus::Pending => "Message is pending",
            MessageStatus::Sent => "Message was sent",
            MessageStatus::Delivered => "Message delivered successfully",
            MessageStatus::Failed => "Message delivery failed",
        }
    }
}
```

## Associated Constants and Functions

```rust
enum SmsError {
    InvalidNumber,
    RateLimited { retry_after: u64 },
    NetworkError(String),
}

impl SmsError {
    const MAX_RETRIES: u32 = 3;
    
    fn network(msg: &str) -> Self {
        SmsError::NetworkError(msg.to_string())
    }
}
```

## Enums vs Structs

When to use enums vs structs:

**Use enums when:**
- Values are mutually exclusive
- You need to represent "one of" relationships
- The set of possibilities is finite and known

**Use structs when:**
- Values can coexist
- You need to group related data
- You're describing a thing with multiple attributes

Example - wrong choice:

```rust
struct MessageStatus {
    pending: bool,
    sent: bool,
    delivered: bool,
    failed: bool,
}
```

This is wrong because a message can't be both pending AND sent.

Correct with enum:

```rust
enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}
```

## Visibility

Enum visibility applies to variants:

```rust
pub enum ApiStatus {
    Online,
    Offline,
    Maintenance,
}
```

With `pub enum`, all variants are public by default. Control variant visibility:

```rust
pub enum InternalStatus {
    Public,
    Internal,  // Still public, enum is pub
}

// For granular control, use modules:
mod status {
    pub enum Status {
        Public,
        Internal,
    }
    
    pub fn public_status() -> Status {
        Status::Public
    }
}
```

## Generic Enums

Enums can be generic over types:

```rust
enum ApiResponse<T> {
    Success(T),
    Error { code: u16, message: String },
    Loading,
}

let text_response = ApiResponse::Success(String::from("OK"));
let count_response = ApiResponse::Success(42);
```

## Real-World Textio Example

```rust
#[derive(Debug, Clone)]
enum Message {
    Sms {
        to: String,
        body: String,
    },
    Mms {
        to: String,
        body: String,
        media_url: String,
    },
    Scheduled {
        to: String,
        body: String,
        send_at: u64,
    },
}

impl Message {
    fn recipient(&self) -> &str {
        match self {
            Message::Sms { to, .. } => to,
            Message::Mms { to, .. } => to,
            Message::Scheduled { to, .. } => to,
        }
    }
}
```

## Memory Layout

Rust enums are compact:
- No-variant enums: 1 byte minimum (for discriminant)
- With data: size of largest variant + discriminant

```rust
enum Simple {
    A,
    B,
}

std::mem::size_of::<Simple>();  // 1 byte

enum WithData {
    A,
    B(String),
}

std::mem::size_of::<WithData>();  // 24 bytes (String size + 1 byte discriminant)
```

## Common Patterns

### Unit Variants for States

```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}
```

### Tuple Variants for Wrappers

```rust
enum Recipient {
    Phone(String),
    GroupId(u64),
    ContactList(String),
}
```

### Struct Variants for Complex Data

```rust
enum SmsResult {
    Delivered { id: String, segments: u32 },
    Failed { code: u16, reason: String },
}
```

## Exercise

You'll implement a `MessageStatus` enum for Textio with:
1. Four status variants
2. Data in some variants (error codes, timestamps)
3. Methods for checking and describing status
4. Implementation of useful traits

The exercise demonstrates:
- Defining enums with multiple variant types
- Adding methods to enums
- Using derive attributes
- Pattern matching for behavior

## Key Takeaways

1. Enums model mutually exclusive possibilities
2. Variants can carry data (tuple or struct style)
3. The `use` keyword imports variants
4. `#[derive(...)]` adds common traits
5. Enums can have methods via `impl` blocks
6. Generic enums provide flexibility
7. Rust enums are memory-efficient

## Next Steps

In the next exercise, we'll explore `Option<T>`, Rust's solution to null values and how enums enable safe handling of potentially missing data.

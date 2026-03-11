# Common Standard Library Traits

## Introduction

Rust's standard library provides several essential traits that enable common functionality. Implementing these traits for your types makes them behave consistently with Rust's conventions and unlocks powerful features like debugging, comparison, and type conversion.

In Textio, implementing standard traits allows our message types to integrate seamlessly with Rust's ecosystem.

## Debug and Display

### Debug Trait

The `Debug` trait enables formatting for debugging purposes:

```rust
#[derive(Debug)]
struct Message {
    id: String,
    content: String,
}

let msg = Message { /* ... */ };
println!("{:?}", msg);  // Debug format
println!("{:#?}", msg); // Pretty debug format
```

### Display Trait

`Display` provides user-friendly formatting:

```rust
use std::fmt;

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Message[{}]: {}", self.id, self.content)
    }
}

println!("{}", msg);  // Uses Display
```

### Key Differences

| Debug | Display |
|-------|---------|
| For developers | For users |
| Can be derived | Must be implemented |
| `{:?}` format | `{}` format |
| Shows internal structure | Shows meaningful representation |

## Clone and Copy

### Clone

`Clone` allows explicit duplication:

```rust
#[derive(Clone)]
struct SmsMessage {
    to: String,
    body: String,
}

let original = SmsMessage { /* ... */ };
let duplicate = original.clone();
```

### Copy

`Copy` allows implicit duplication (bitwise copy):

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copied, not moved
```

### Copy Requirements

A type can implement `Copy` only if:
1. It implements `Clone`
2. All its fields implement `Copy`
3. It doesn't manage resources (no `String`, `Vec`, `Box`, etc.)

## PartialEq and Eq

### PartialEq

`PartialEq` provides equality comparison:

```rust
#[derive(PartialEq)]
struct User {
    id: u32,
    name: String,
}

let u1 = User { id: 1, name: "Alice".to_string() };
let u2 = User { id: 1, name: "Alice".to_string() };
assert_eq!(u1, u2);
```

### Eq

`Eq` indicates full equivalence (reflexive, symmetric, transitive):

```rust
#[derive(PartialEq, Eq)]
struct UserId(u32);
```

Use `Eq` when equality is total (all values can be compared).

## PartialOrd and Ord

### PartialOrd

For ordering comparisons:

```rust
#[derive(PartialOrd, PartialEq)]
struct Priority(u8);

let low = Priority(1);
let high = Priority(5);
assert!(low < high);
```

### Ord

`Ord` provides total ordering:

```rust
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}
```

## From and Into

### From Trait

Convert from one type to another:

```rust
impl From<&str> for SmsMessage {
    fn from(content: &str) -> Self {
        SmsMessage {
            to: String::new(),
            body: content.to_string(),
        }
    }
}

let msg = SmsMessage::from("Hello");
```

### Into Trait

`Into` is automatically implemented when `From` is implemented:

```rust
fn send_message<M: Into<SmsMessage>>(msg: M) {
    let sms: SmsMessage = msg.into();
    // send sms
}

send_message("Hello");  // &str converts via From
```

### TryFrom and TryInto

For fallible conversions:

```rust
use std::convert::TryFrom;

impl TryFrom<&str> for PhoneNumber {
    type Error = String;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.starts_with('+') {
            Ok(PhoneNumber(s.to_string()))
        } else {
            Err("Invalid phone number".to_string())
        }
    }
}
```

## Default

`Default` provides a default value:

```rust
#[derive(Default)]
struct Config {
    timeout: u32,
    retries: u32,
    debug: bool,
}

let config = Config::default();
// timeout: 0, retries: 0, debug: false
```

Custom implementation:

```rust
impl Default for SmsConfig {
    fn default() -> Self {
        SmsConfig {
            max_length: 160,
            encoding: "UTF-8".to_string(),
        }
    }
}
```

## Hash

`Hash` enables use in hash-based collections:

```rust
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct MessageId(String);

let mut sent_ids: HashSet<MessageId> = HashSet::new();
sent_ids.insert(MessageId("msg_001".to_string()));
```

## AsRef and AsMut

### AsRef

Borrow as a reference to another type:

```rust
impl AsRef<str> for SmsMessage {
    fn as_ref(&self) -> &str {
        &self.body
    }
}

fn print_content<M: AsRef<str>>(msg: &M) {
    println!("{}", msg.as_ref());
}
```

### AsMut

Mutable borrowing:

```rust
impl AsMut<String> for SmsMessage {
    fn as_mut(&mut self) -> &mut String {
        &mut self.body
    }
}
```

## Deref and DerefMut

Smart pointer behavior:

```rust
use std::ops::Deref;

struct MessageBuffer {
    data: Vec<u8>,
}

impl Deref for MessageBuffer {
    type Target = Vec<u8>;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

let buffer = MessageBuffer { data: vec![1, 2, 3] };
println!("Length: {}", buffer.len());  // Uses Vec's len
```

## Derive Macro

Many traits can be automatically derived:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MessageId {
    id: String,
    timestamp: u64,
}
```

## Supertraits

Traits that require other traits:

```rust
trait Serializable: Debug + Clone {
    fn serialize(&self) -> String;
}

// Any type implementing Serializable must also implement Debug and Clone
```

## Blanket Implementations

Implement a trait for all types meeting certain bounds:

```rust
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // Implementation
    }
}
```

## Exercise Overview

In this exercise, you will:

1. Implement `Debug` and `Display` for message types
2. Implement `Clone` and `PartialEq` for comparison
3. Implement `From` and `Into` for type conversions
4. Implement `Default` for configuration
5. Use derive macros where appropriate

## Key Takeaways

- `Debug` is for developers, `Display` is for users
- `Clone` for explicit copying, `Copy` for implicit (when possible)
- Implement `From` to get `Into` for free
- Use derive macros to reduce boilerplate
- Standard traits integrate your types with Rust's ecosystem

## Conclusion

Mastering traits is essential for writing idiomatic Rust. They enable code reuse, abstraction, and integration with the broader ecosystem. Practice implementing both custom traits and standard library traits to become proficient.

# Exercise 2: The Option Type

## Overview

`Option<T>` is Rust's solution to the null problem. Instead of having null values that cause runtime errors, Rust uses an enum to explicitly represent the presence or absence of a value.

## The Null Problem

In languages with null, Tony Hoare called null his "billion-dollar mistake":

```javascript
// JavaScript - runtime error waiting to happen
let user = getUser(123);
console.log(user.name);  // TypeError if user is null
```

The problem: You can't tell from the type whether `user` might be null. The compiler can't help you.

## Option<T> Definition

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

This is in the prelude, so you never need to import it. `Some` and `None` are also directly available.

## Why This Is Better

```rust
fn find_user(id: u32) -> Option<User> {
    // ...
}

let user = find_user(123);
// user is Option<User>, NOT User
// You MUST handle the None case
```

The type system forces you to handle absence. You can't accidentally access a null value.

## Using Option Values

### Pattern Matching

```rust
fn describe_status(status: Option<String>) -> String {
    match status {
        Some(msg) => format!("Status: {}", msg),
        None => String::from("No status available"),
    }
}
```

### if let

```rust
if let Some(msg) = status {
    println!("Status: {}", msg);
}
```

### unwrap and expect

Use only when you're certain:

```rust
let value = maybe_value.unwrap();  // Panics on None
let value = maybe_value.expect("Value must exist");  // Panics with message on None
```

### unwrap_or and unwrap_or_else

Provide defaults:

```rust
let value = maybe_value.unwrap_or(0);
let value = maybe_value.unwrap_or_else(|| compute_default());
```

### map and and_then

Transform Option values:

```rust
let length: Option<usize> = maybe_string.map(|s| s.len());

let parsed: Option<i32> = maybe_string
    .and_then(|s| s.parse().ok());
```

## Option in Textio

In our SMS API, many values are optional:

```rust
struct Message {
    to: String,           // Required
    body: String,         // Required
    from: Option<String>, // Optional sender ID
    media_url: Option<String>, // For MMS
    scheduled_at: Option<u64>, // For scheduled messages
    callback_url: Option<String>, // For delivery receipts
}
```

## Common Patterns

### Chaining Operations

```rust
fn get_message_sender(msg_id: u64) -> Option<String> {
    find_message(msg_id)
        .map(|msg| msg.sender_id)
        .and_then(find_user)
        .map(|user| user.phone_number)
}
```

### Filtering

```rust
let valid_number: Option<String> = phone_number
    .filter(|n| n.starts_with('+'));
```

### or and xor

```rust
let result = opt_a.or(opt_b);  // Some if either is Some (prefers opt_a)
let result = opt_a.xor(opt_b); // Some if exactly one is Some
```

### take and replace

```rust
let mut opt = Some(5);
let value = opt.take();  // opt is now None, value is Some(5)

let mut opt = Some(5);
let old = opt.replace(10);  // opt is now Some(10), old is Some(5)
```

### as_ref and as_mut

```rust
let opt: Option<String> = Some(String::from("hello"));
let len: Option<usize> = opt.as_ref().map(|s| s.len());

let mut opt: Option<String> = Some(String::from("hello"));
if let Some(s) = opt.as_mut() {
    s.push_str(" world");
}
```

### copied and cloned

```rust
let opt: Option<&i32> = Some(&5);
let copied: Option<i32> = opt.copied();

let opt: Option<&String> = Some(&String::from("hello"));
let cloned: Option<String> = opt.cloned();
```

## Option vs Result

- `Option<T>`: A value might be absent (None)
- `Result<T, E>`: An operation might fail (Err)

They're related - you can convert between them:

```rust
let result: Result<T, &str> = option.ok_or("No value");
let option: Option<T> = result.ok();
```

## Working with Collections

### Option<Vec<T>>

```rust
fn find_all_messages(user_id: u32) -> Option<Vec<Message>> {
    // Returns None if user doesn't exist
    // Returns Some(vec![]) if user exists but has no messages
}
```

### Iterating over Options

```rust
let numbers: Vec<Option<i32>> = vec![Some(1), None, Some(3)];

// Get only the Some values
let valid: Vec<i32> = numbers.iter().filter_map(|&x| x).collect();

// Or use flatten
let valid: Vec<i32> = numbers.into_iter().flatten().collect();
```

### find and position

```rust
let found: Option<&Message> = messages.iter().find(|m| m.id == target_id);
let position: Option<usize> = messages.iter().position(|m| m.id == target_id);
```

## Best Practices

### 1. Use Option for Nullable Values

```rust
// Good
fn find_user(id: u32) -> Option<User>

// Avoid (don't return Result for simple absence)
fn find_user(id: u32) -> Result<User, NotFoundError>
```

### 2. Prefer Combinators Over Nested Matches

```rust
// Less clean
match find_user(id) {
    Some(user) => match user.phone {
        Some(phone) => Some(format_phone(phone)),
        None => None,
    },
    None => None,
}

// Better
find_user(id)
    .and_then(|u| u.phone)
    .map(format_phone)
```

### 3. Use ? Operator for Propagation

```rust
fn get_user_phone(id: u32) -> Option<String> {
    let user = find_user(id)?;
    let phone = user.phone?;
    Some(format_phone(phone))
}
```

### 4. Avoid unwrap in Production Code

```rust
// Bad - will panic
let value = optional_value.unwrap();

// Good - provide default
let value = optional_value.unwrap_or(default);

// Good - propagate None
let value = optional_value?;

// Good - explicit handling
let value = match optional_value {
    Some(v) => v,
    None => return None,
};
```

## Real-World Textio Example

```rust
struct SmsClient {
    api_key: String,
    default_sender: Option<String>,
    rate_limit: Option<RateLimit>,
}

impl SmsClient {
    fn send(&self, to: &str, body: &str) -> Option<MessageId> {
        let sender = self.default_sender.as_deref()?;
        
        self.rate_limit
            .as_ref()
            .filter(|r| r.is_allowed())
            .map(|_| self.do_send(sender, to, body))
    }
}
```

## Exercise

You'll implement Option-based functions for Textio:
1. Finding messages by ID
2. Getting optional message properties
3. Chaining Option operations
4. Providing default values
5. Working with Option in collections

## Key Takeaways

1. `Option<T>` replaces null - use Some/None
2. The type system forces handling of None
3. Use combinators (map, and_then) for transformations
4. The ? operator propagates None elegantly
5. Avoid unwrap/expect in production
6. Option works great with iterators
7. as_ref/as_mut for borrowing inside Options

## Next Steps

Next, we'll explore match expressions in depth - the powerful pattern matching that makes working with enums and Options elegant and safe.

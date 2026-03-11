# Named Structs

In Rust, a **struct** (short for "structure") is a custom data type that groups related values together. Named structs are the most common type, where each field has a name.

## Why Structs?

Imagine building Textio's SMS system without structs:

```rust
fn send_sms(to: String, from: String, body: String, priority: i32, encoding: String) {
    // Which parameter was priority again?
}

// Easy to mix up order!
send_sms(String::from("+15550001"), String::from("+15550002"), String::from("Hello"), 1, String::from("UTF-8"));
```

With structs, we group related data:

```rust
struct Message {
    to: String,
    from: String,
    body: String,
    priority: i32,
    encoding: String,
}
```

## Defining a Named Struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

Key points:
- Use `struct` keyword followed by the name (PascalCase by convention)
- Fields go inside curly braces `{}`
- Each field has a name and type, separated by colon `:`
- Fields are separated by commas (trailing comma allowed)
- No semicolon after the closing brace

## Creating an Instance

```rust
let user1 = User {
    email: String::from("user@textio.com"),
    username: String::from("textio_user"),
    active: true,
    sign_in_count: 1,
};
```

Important rules:
- You must provide values for ALL fields (no partial initialization)
- Field order doesn't matter (unlike tuples or function arguments)
- The struct instance is immutable by default

## Accessing Fields

Use dot notation:

```rust
println!("Email: {}", user1.email);
println!("Sign in count: {}", user1.sign_in_count);
```

If the instance is mutable, you can modify fields:

```rust
let mut user2 = User {
    email: String::from("admin@textio.com"),
    username: String::from("admin"),
    active: true,
    sign_in_count: 0,
};

user2.active = false;  // Only possible because user2 is mut
```

Note: You can't mark individual fields as mutable. The entire instance is either mutable or not.

## Field Init Shorthand

When a variable has the same name as a field, use shorthand:

```rust
fn build_user(email: String, username: String) -> User {
    // Without shorthand (verbose):
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }
    
    // With shorthand (concise):
    User {
        email,      // same as email: email
        username,   // same as username: username
        active: true,
        sign_in_count: 1,
    }
}
```

This is idiomatic Rust and commonly used in constructor functions.

## Struct Update Syntax

Create a new struct from an existing one, changing only some fields:

```rust
let user3 = User {
    email: String::from("new@textio.com"),
    ..user1  // Use all other fields from user1
};
```

The `..` syntax must come last and means "take all remaining fields from this instance."

### Important: Ownership with Struct Update

```rust
let user4 = User {
    email: String::from("another@textio.com"),
    ..user1
};

// user1.email is now MOVED to user4 (String doesn't implement Copy)
// println!("{}", user1.email);  // ERROR!

// But user1.active is still valid (bool implements Copy)
println!("{}", user1.active);  // Works fine
```

If you use `..user1` and the fields implement `Copy`, the original remains valid. If they don't (like `String`), ownership moves.

## Tuple Structs vs Named Structs

Named structs are best when:
- Fields have clear, different meanings
- You want self-documenting code
- Order shouldn't matter

Tuple structs (covered in the next exercise) are better when:
- Fields are interchangeable (like x, y, z coordinates)
- You want a lightweight wrapper type

## Unit Structs

A struct with no fields:

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

Useful when you need to implement a trait but don't need to store data.

## Visibility

By default, structs and their fields are private to the module:

```rust
// Public struct, private fields
pub struct Config {
    api_key: String,      // private
    pub timeout: u32,     // public
}

// To create this struct outside its module, you'd need a constructor
// because api_key is private
```

To make a struct fully usable outside its module:
- Mark the struct with `pub`
- Mark fields that should be accessible with `pub`
- Provide constructor functions for private fields

## Debug Trait

To print structs with `{:?}` or `{:#?}`, implement `Debug`:

```rust
#[derive(Debug)]
struct Message {
    to: String,
    from: String,
    body: String,
}

let msg = Message {
    to: String::from("+15550001"),
    from: String::from("+15550002"),
    body: String::from("Hello"),
};

println!("{:?}", msg);      // Single line
println!("{:#?}", msg);     // Pretty printed
```

Output:
```
Message { to: "+15550001", from: "+15550002", body: "Hello" }

Message {
    to: "+15550001",
    from: "+15550002",
    body: "Hello",
}
```

## Structs and Ownership

Structs can store references, but this requires **lifetimes** (covered later):

```rust
// This won't compile without lifetime annotations!
struct RefMessage<'a> {
    body: &'a str,  // Reference to a string slice
}

// For now, prefer owned types like String
struct OwnedMessage {
    body: String,  // Owns its data
}
```

## Memory Layout

Named structs are stored contiguously in memory:

```rust
struct Point {
    x: f64,  // 8 bytes
    y: f64,  // 8 bytes
    z: f64,  // 8 bytes
}
// Total: 24 bytes, contiguous
```

This makes structs cache-friendly and efficient.

## Common Patterns

### Configuration Struct
```rust
struct TextioConfig {
    api_endpoint: String,
    max_retries: u32,
    timeout_seconds: u32,
    enable_logging: bool,
}
```

### Data Transfer Struct
```rust
struct SmsPayload {
    to: String,
    from: String,
    message: String,
}
```

### State Struct
```rust
struct ConnectionState {
    connected: bool,
    last_ping: u64,
    messages_sent: u64,
}
```

## Exercise: Textio Message System

In this exercise, you'll create named structs for Textio's SMS messaging system. You'll define a `Message` struct, create instances using field init shorthand and struct update syntax, and practice with the `Debug` trait.

### Tasks

1. Define a `Message` struct with fields for SMS data
2. Create instances using full syntax
3. Use field init shorthand where applicable
4. Use struct update syntax to create variations
5. Derive and use the `Debug` trait
6. Understand ownership implications of struct update

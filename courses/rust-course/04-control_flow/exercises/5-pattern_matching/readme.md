# Exercise 5: Advanced Pattern Matching

## Overview

In this exercise, you'll master advanced pattern matching techniques in Rust, including match guards, bindings with `@`, destructuring, and complex patterns. These techniques are essential for sophisticated message handling and data processing in your Textio SMS API.

## Learning Objectives

By the end of this exercise, you will:
- Use match guards for additional conditions
- Bind matched values with `@` operator
- Destructure structs, tuples, and enums
- Match nested patterns
- Apply advanced patterns to real-world scenarios

## Match Guards

Guards add additional conditions to match arms using `if`:

### Basic Guards

```rust
let number = 5;

match number {
    n if n < 0 => println!("Negative"),
    n if n == 0 => println!("Zero"),
    n if n > 0 => println!("Positive"),
    _ => println!("Impossible"),
}
```

### Guards with Multiple Variables

```rust
let (x, y) = (5, 10);

match (x, y) {
    (a, b) if a == b => println!("Equal: {}", a),
    (a, b) if a > b => println!("{} is greater", a),
    (a, b) if a < b => println!("{} is greater", b),
    _ => println!("Impossible"),
}
```

### Guards with Methods

```rust
let text = "hello@example.com";

match text {
    s if s.contains('@') => println!("Email address"),
    s if s.starts_with('+') => println!("Phone number"),
    s if s.len() > 20 => println!("Long text"),
    _ => println!("Other"),
}
```

## Bindings with @

The `@` operator lets you bind a value to a name while also testing it:

### Range Binding

```rust
let age = 25;

match age {
    n @ 0..=12 => println!("Child, age {}", n),
    n @ 13..=19 => println!("Teenager, age {}", n),
    n @ 20..=30 => println!("Young adult, age {}", n),
    n @ 31..=50 => println!("Adult, age {}", n),
    n => println!("Senior, age {}", n),
}
```

### Multiple Bindings

```rust
let point = (3, 5);

match point {
    (x @ 0..=5, y @ 0..=5) => println!("In grid: ({}, {})", x, y),
    (x, y) => println!("Outside grid: ({}, {})", x, y),
}
```

## Destructuring

### Structs

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

let user = User {
    name: String::from("Alice"),
    age: 30,
    active: true,
};

match user {
    User { name, age: 0..=17, active: true } => 
        println!("Active minor: {}", name),
    User { name, age: 18..=64, active: true } => 
        println!("Active adult: {}", name),
    User { name, active: false, .. } => 
        println!("Inactive user: {}", name),
    User { name, age, .. } => 
        println!("Other user: {}, {}", name, age),
}
```

### Tuples

```rust
let coordinates = (0, 5, 10);

match coordinates {
    (0, y, z) => println!("On YZ plane at y={}, z={}", y, z),
    (x, 0, z) => println!("On XZ plane at x={}, z={}", x, z),
    (x, y, 0) => println!("On XY plane at x={}, y={}", x, y),
    (x, y, z) => println!("3D point at ({}, {}, {})", x, y, z),
}
```

### Enums with Data

```rust
enum Message {
    Text(String),
    Number(i32),
    Coordinates(i32, i32),
    Empty,
}

let msg = Message::Coordinates(10, 20);

match msg {
    Message::Text(s) if s.len() > 100 => 
        println!("Long text: {}...", &s[..20]),
    Message::Text(s) => 
        println!("Text: {}", s),
    Message::Number(n @ 0..=100) => 
        println!("Small number: {}", n),
    Message::Number(n) => 
        println!("Large number: {}", n),
    Message::Coordinates(x, y) if x == y => 
        println!("Diagonal: {}", x),
    Message::Coordinates(x, y) => 
        println!("Point: ({}, {})", x, y),
    Message::Empty => 
        println!("Empty message"),
}
```

## Nested Patterns

Match can destructure nested data:

```rust
struct Address {
    city: String,
    country: String,
}

struct Contact {
    name: String,
    address: Address,
}

let contact = Contact {
    name: String::from("Alice"),
    address: Address {
        city: String::from("London"),
        country: String::from("UK"),
    },
};

match contact {
    Contact { 
        name, 
        address: Address { city, country: "UK" | "US" } 
    } => println!("{} lives in {} ({})", name, city, country),
    Contact { name, .. } => 
        println!("{} lives elsewhere", name),
}
```

## Textio SMS Application

### 1. Message with Guards

```rust
let (sender, recipient, message) = ("Alice", "Bob", "Hello!");

match (sender, recipient, message.len()) {
    (s, r, len) if s == r => 
        println!("Self-message from {}", s),
    (s, r, len) if len > 160 => 
        println!("Multi-part message from {} to {}", s, r),
    (s, r, _) => 
        println!("Standard message from {} to {}", s, r),
}
```

### 2. Status with Guards and Bindings

```rust
let status_code = 429;
let retry_after = Some(60);

match (status_code, retry_after) {
    (200..=299, _) => println!("Success"),
    (code @ 400..=499, _) => println!("Client error: {}", code),
    (429, Some(seconds @ 1..=60)) => 
        println!("Rate limited, retry in {}s", seconds),
    (429, Some(seconds)) => 
        println!("Rate limited, retry in {}s (long wait)", seconds),
    (code @ 500..=599, _) => println!("Server error: {}", code),
    (code, _) => println!("Unknown status: {}", code),
}
```

### 3. Subscription Validation

```rust
struct Subscription {
    tier: String,
    messages_sent: u32,
    limit: u32,
    days_remaining: u32,
}

let sub = Subscription {
    tier: String::from("gold"),
    messages_sent: 9500,
    limit: 10000,
    days_remaining: 15,
};

match sub {
    Subscription { tier, messages_sent, limit, .. } 
        if messages_sent >= limit => 
        println!("{} tier: Limit reached!", tier),
    Subscription { tier, messages_sent, limit, days_remaining } 
        if messages_sent as f64 / limit as f64 > 0.9 && days_remaining > 10 => 
        println!("{} tier: {}% used with {} days left", 
                 tier, messages_sent * 100 / limit, days_remaining),
    Subscription { tier, messages_sent, limit, .. } => 
        println!("{} tier: {}/{} messages", tier, messages_sent, limit),
}
```

## Advanced Patterns

### Slice Patterns

```rust
let numbers = [1, 2, 3, 4, 5];

match numbers {
    [first, .., last] => 
        println!("First: {}, Last: {}", first, last),
    [single] => 
        println!("Single element: {}", single),
    [] => 
        println!("Empty"),
}
```

### Multiple Guards

```rust
let (age, income, employed) = (25, 50000, true);

match (age, income, employed) {
    (age, income, true) if age >= 18 && income > 30000 => 
        println!("Qualified applicant"),
    (age, _, true) if age >= 18 => 
        println!("Employed but low income"),
    (age, _, false) if age >= 18 => 
        println!("Unemployed"),
    _ => 
        println!("Minor"),
}
```

### Ignoring Parts with ..

```rust
struct Message {
    id: u64,
    from: String,
    to: String,
    content: String,
    timestamp: u64,
    status: String,
}

let msg = Message {
    id: 1,
    from: String::from("alice"),
    to: String::from("bob"),
    content: String::from("Hello"),
    timestamp: 1234567890,
    status: String::from("sent"),
};

match msg {
    Message { from, to, content, status: "failed", .. } => 
        println!("Failed message from {} to {}: {}", from, to, content),
    Message { from, to, status: "delivered", .. } => 
        println!("Delivered: {} -> {}", from, to),
    Message { id, from, .. } => 
        println!("Message {} from {}", id, from),
}
```

## Best Practices

### 1. Order Specificity

More specific patterns first:

```rust
// Good
match value {
    n @ 90..=100 if n % 10 == 0 => println!("Round high score"),
    n @ 90..=100 => println!("High score"),
    n @ 0..=89 => println!("Regular score"),
    _ => println!("Invalid"),
}
```

### 2. Use Guards for Complex Logic

```rust
// Better with guard
match (x, y) {
    (a, b) if a * a + b * b > 100 => println!("Far from origin"),
    (a, b) => println!("Close to origin"),
}

// Harder to read without
match (x, y) {
    (a, b) if a * a + b * b > 100 => ...
}
```

### 3. Destructure for Clarity

```rust
// Clear with destructuring
let Point { x, y } = point;

// Less clear
let x = point.x;
let y = point.y;
```

## Common Patterns

### Validating Input

```rust
let input = "user@example.com";

let valid = match input {
    s if s.is_empty() => false,
    s if !s.contains('@') => false,
    s if s.len() > 100 => false,
    _ => true,
};
```

### Status with Context

```rust
let result: Result<u32, &str> = Ok(42);

match result {
    Ok(value @ 0..=100) => println!("Valid: {}", value),
    Ok(value) => println!("Out of range: {}", value),
    Err("network") => println!("Network error"),
    Err(e) => println!("Error: {}", e),
}
```

### Hierarchical Matching

```rust
let (country, region, city) = ("US", "CA", "San Francisco");

match (country, region, city) {
    ("US", "CA", _) => println!("California, USA"),
    ("US", "NY", "New York") => println!("NYC"),
    ("US", _, _) => println!("Other USA"),
    ("UK", _, "London") => println!("London, UK"),
    ("UK", _, _) => println!("Other UK"),
    _ => println!("Other location"),
}
```

## Common Mistakes

### 1. Shadowing in Guards

```rust
let x = 5;
match x {
    x if x > 3 => println!("{}", x),  // This x shadows outer x
    _ => println!("other"),
}
```

### 2. Unreachable Patterns

```rust
match x {
    n if n > 0 => println!("Positive"),
    n @ 1..=10 => println!("Small positive"),  // Never reached!
    _ => println!("Other"),
}
```

### 3. Overusing Guards

```rust
// Too complex
match x {
    n if n > 0 && n < 10 && n % 2 == 0 => ...
    n if n > 0 && n < 10 => ...
    _ => ...
}

// Simpler
match x {
    2 | 4 | 6 | 8 => ...
    1 | 3 | 5 | 7 | 9 => ...
    n @ 10..=20 => ...
    _ => ...
}
```

## Exercise Task

In this exercise, you'll implement:

1. **Message validation** - Complex guards for validation
2. **Status classification** - Guards with bindings
3. **Struct destructuring** - Match on nested data
4. **Tuple matching** - Multi-variable patterns
5. **Enum matching** - Complex enum patterns

## Summary

- Guards (`if condition`) add extra conditions to patterns
- Bindings (`@`) capture values while testing
- Destructuring extracts fields from structs and tuples
- Nested patterns match complex data structures
- `..` ignores remaining fields
- Order matters - more specific patterns first

## Next Steps

You've completed the Control Flow module! Next, you'll learn about ownership and borrowing, Rust's most distinctive feature.

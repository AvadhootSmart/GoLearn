# Exercise 1: If/Else - Conditional Logic in Rust

## Overview

In this exercise, you'll learn about Rust's conditional expressions and how they form the foundation of control flow in your Textio SMS API project. Unlike many languages, Rust treats `if` as an expression rather than a statement, which means it can return values.

## Learning Objectives

By the end of this exercise, you will:
- Understand Rust's `if`, `else if`, and `else` syntax
- Learn why Rust doesn't require parentheses around conditions
- Use `if` as an expression that returns values
- Apply conditional logic to SMS message routing
- Understand the importance of consistent types in `if` expressions

## If/Else Basics

### Basic Syntax

In Rust, the basic `if` statement looks like this:

```rust
let message_count = 150;

if message_count > 100 {
    println!("High volume detected!");
}
```

Notice there are **no parentheses** around the condition. Rust doesn't require them, making the code cleaner and more readable.

### Else If and Else

You can chain multiple conditions:

```rust
let message_count = 150;

if message_count > 200 {
    println!("Critical volume!");
} else if message_count > 100 {
    println!("High volume");
} else if message_count > 50 {
    println!("Moderate volume");
} else {
    println!("Normal volume");
}
```

### The Condition Must Be a Bool

Unlike some languages, Rust does NOT implicitly convert non-boolean values to booleans:

```rust
// This WON'T work:
let count = 5;
if count {  // Error: expected bool, found integer
    println!("Has messages");
}

// This WILL work:
if count > 0 {
    println!("Has messages");
}
```

This explicit requirement helps prevent common bugs where you might accidentally use an assignment (`=`) instead of comparison (`==`).

## If as an Expression

This is where Rust shines. The `if` construct is an **expression**, meaning it returns a value:

```rust
let message_count = 75;
let tier = if message_count > 100 {
    "premium"
} else {
    "standard"
};

println!("Customer tier: {}", tier);
```

Both branches MUST return the same type. This won't compile:

```rust
// Error: branches return different types
let result = if condition {
    42          // integer
} else {
    "hello"     // string - WRONG!
};
```

### Using Blocks

The last expression in each branch (without a semicolon) is the return value:

```rust
let base_cost = 10;
let message_count = 150;

let total = if message_count > 100 {
    let discount = 5;
    base_cost - discount  // No semicolon - this is returned
} else {
    base_cost             // No semicolon - this is returned
};
```

## Textio SMS Application

In your Textio SMS API, you'll use conditionals for:

### 1. Message Routing

```rust
let destination = "+1-555-0123";
let country_code = &destination[0..2];

let gateway = if country_code == "+1" {
    "US_GATEWAY"
} else if country_code == "+44" {
    "UK_GATEWAY"
} else if country_code == "+49" {
    "DE_GATEWAY"
} else {
    "INTERNATIONAL_GATEWAY"
};
```

### 2. Rate Limiting

```rust
let messages_sent = 95;
let max_messages = 100;

let can_send = if messages_sent >= max_messages {
    false
} else {
    true
};

// Or more idiomatically:
let can_send = messages_sent < max_messages;
```

### 3. Pricing Tiers

```rust
let monthly_messages = 5000;

let (tier, price_per_message) = if monthly_messages > 10000 {
    ("Enterprise", 0.005)
} else if monthly_messages > 1000 {
    ("Business", 0.01)
} else {
    ("Starter", 0.02)
};
```

### 4. Message Validation

```rust
let message = "Hello, World!";
let max_length = 160;

let status = if message.len() > max_length {
    "Message too long"
} else if message.len() == 0 {
    "Message is empty"
} else {
    "Valid message"
};
```

## Nested Conditions

You can nest `if` expressions:

```rust
let country = "US";
let verified = true;
let message_count = 150;

let limit = if country == "US" {
    if verified {
        1000
    } else {
        100
    }
} else {
    50
};
```

## Multiple Conditions with && and ||

Combine conditions logically:

```rust
let is_premium = true;
let has_credits = true;
let is_verified = false;

if is_premium && has_credits {
    println!("Send message with priority");
}

if is_premium || is_verified {
    println!("Access to advanced features");
}

if !is_verified {
    println!("Please verify your account");
}
```

## Short-Circuit Evaluation

Rust uses short-circuit evaluation:

```rust
fn get_message_count() -> i32 {
    println!("Getting count...");
    42
}

let cache_hit = true;
let cached_count = 100;

// If cache_hit is true, get_message_count() is never called
let count = if cache_hit {
    cached_count
} else {
    get_message_count()
};
```

## Best Practices

### 1. Keep Conditions Simple

```rust
// Hard to read
if user.account_type == "premium" && user.messages_sent < user.message_limit && user.status == "active" {
    send_message();
}

// Better - extract to variables
let is_premium = user.account_type == "premium";
let has_capacity = user.messages_sent < user.message_limit;
let is_active = user.status == "active";

if is_premium && has_capacity && is_active {
    send_message();
}
```

### 2. Use `if` Expressions for Assignment

```rust
// Verbose
let status;
if is_valid {
    status = "valid";
} else {
    status = "invalid";
}

// Concise
let status = if is_valid { "valid" } else { "invalid" };
```

### 3. Handle All Cases

```rust
// Good - always have an else for expressions
let result = if condition_a {
    "a"
} else if condition_b {
    "b"
} else {
    "unknown"  // Default case
};
```

## Common Mistakes

### 1. Semicolons in Expression Position

```rust
// Wrong - this returns ()
let result = if condition {
    42;  // Semicolon makes this ()
} else {
    0;
};

// Right
let result = if condition {
    42  // No semicolon
} else {
    0
};
```

### 2. Type Mismatches

```rust
// Wrong
let value = if condition {
    42
} else {
    "zero"  // Different type!
};

// Right
let value = if condition {
    42
} else {
    0
};
```

### 3. Shadowing Variables Incorrectly

```rust
let x = 5;
if true {
    let x = 10;  // This shadows the outer x
    println!("Inner x: {}", x);  // 10
}
println!("Outer x: {}", x);  // 5
```

## Exercise Task

In this exercise, you'll implement SMS message routing and pricing logic for Textio:

1. **Route messages** based on country codes
2. **Calculate pricing** based on message length and destination
3. **Validate messages** with multiple conditions
4. **Determine user tiers** using if expressions

The starter code provides the structure. Your task is to complete the conditional logic using `if`, `else if`, and `else` expressions.

## Summary

- `if` is an expression in Rust and can return values
- No parentheses needed around conditions
- Conditions must be boolean (no implicit conversion)
- All branches must return the same type
- The last expression in a block (without semicolon) is the return value
- Use `else if` for multiple conditions
- Always include `else` when using `if` as an expression for assignment

## Next Steps

After mastering `if`/`else`, you'll learn about `loop` for repeating code blocks indefinitely until a break condition is met.

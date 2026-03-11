# Exercise 4: Match Basics - Pattern Matching

## Overview

In this exercise, you'll learn about Rust's powerful `match` expression, which enables pattern matching against values. Match expressions are fundamental to Rust and are used extensively in the Textio SMS API for routing, status handling, and message processing.

## Learning Objectives

By the end of this exercise, you will:
- Understand the match expression syntax
- Learn why match must be exhaustive
- Use the `_` wildcard pattern
- Match against literals, variables, and ranges
- Apply match to Textio message routing and status handling

## Match Basics

### What is Match?

The `match` expression compares a value against a series of patterns and executes the code for the first matching pattern:

```rust
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"),
}
```

### Match is an Expression

Like `if`, `match` returns a value:

```rust
let status = "delivered";

let message = match status {
    "pending" => "Message is queued",
    "sent" => "Message was sent",
    "delivered" => "Message delivered!",
    _ => "Unknown status",
};

println!("{}", message);
```

### Exhaustive Matching

Rust requires that match covers **all possible values**. This prevents bugs:

```rust
let number = 7;

// This WON'T compile - not exhaustive!
match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    // Error: non-exhaustive patterns
}

// This WILL compile - covers all cases
match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other"),  // Catches everything else
}
```

## The Wildcard Pattern (_)

The `_` pattern matches any value and is commonly used as a catch-all:

### Basic Wildcard

```rust
let grade = 'B';

match grade {
    'A' => println!("Excellent!"),
    'B' => println!("Good"),
    'C' => println!("Average"),
    _ => println!("Needs improvement"),
}
```

### Ignoring Values

```rust
let coordinates = (0, 5);

match coordinates {
    (0, y) => println!("On Y-axis at {}", y),
    (x, 0) => println!("On X-axis at {}", x),
    _ => println!("Somewhere else"),
}
```

## Matching Literals

Match against exact values:

```rust
let direction = "north";

let degrees = match direction {
    "north" => 0,
    "east" => 90,
    "south" => 180,
    "west" => 270,
    _ => -1,
};
```

## Matching Multiple Patterns

Use `|` to match multiple patterns:

```rust
let day = "Saturday";

let category = match day {
    "Saturday" | "Sunday" => "Weekend",
    "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "Weekday",
    _ => "Invalid",
};
```

## Matching Ranges

Use `..=` for inclusive ranges in match:

```rust
let score = 85;

let grade = match score {
    90..=100 => 'A',
    80..=89 => 'B',
    70..=79 => 'C',
    60..=69 => 'D',
    0..=59 => 'F',
    _ => 'X',  // Invalid score
};
```

### Character Ranges

```rust
let c = 'g';

let category = match c {
    'a'..='z' => "lowercase",
    'A'..='Z' => "uppercase",
    '0'..='9' => "digit",
    _ => "other",
};
```

## Binding Values

Capture the matched value with `@`:

```rust
let age = 25;

match age {
    n @ 0..=12 => println!("Child of age {}", n),
    n @ 13..=19 => println!("Teenager of age {}", n),
    n @ 20..=30 => println!("Young adult of age {}", n),
    n => println!("Adult of age {}", n),
}
```

## Match with Enums

Match is particularly powerful with enums:

```rust
enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed(String),
}

let status = MessageStatus::Failed(String::from("Network error"));

match status {
    MessageStatus::Pending => println!("Waiting to send"),
    MessageStatus::Sent => println!("On its way"),
    MessageStatus::Delivered => println!("Arrived!"),
    MessageStatus::Failed(reason) => println!("Failed: {}", reason),
}
```

## Textio SMS Application

### 1. Message Routing by Country

```rust
let country_code = "US";

let gateway = match country_code {
    "US" => "us-gateway.textio.com",
    "UK" => "uk-gateway.textio.com",
    "DE" | "FR" | "IT" => "eu-gateway.textio.com",
    _ => "global-gateway.textio.com",
};
```

### 2. Status Code Handling

```rust
let http_status = 200;

let (status_type, retry) = match http_status {
    200 | 201 | 204 => ("Success", false),
    400 | 401 | 403 => ("Client Error", false),
    429 => ("Rate Limited", true),
    500 | 502 | 503 => ("Server Error", true),
    _ => ("Unknown", false),
};
```

### 3. Message Priority

```rust
let priority = 2;

let (name, timeout_ms) = match priority {
    1 => ("Critical", 100),
    2 => ("High", 500),
    3 => ("Normal", 2000),
    4 => ("Low", 10000),
    _ => ("Default", 5000),
};
```

### 4. Subscription Tier

```rust
let tier = "gold";

let (monthly_limit, features) = match tier {
    "free" => (100, vec!["basic"]),
    "bronze" => (500, vec!["basic", "analytics"]),
    "silver" => (2000, vec!["basic", "analytics", "priority"]),
    "gold" => (10000, vec!["basic", "analytics", "priority", "api"]),
    "platinum" => (100000, vec!["all"]),
    _ => (0, vec![]),
};
```

## Match vs If/Else Chain

### When to Use Match

Use match when:
- Comparing against multiple specific values
- Working with enums
- Pattern matching is needed
- You want exhaustive checking

```rust
// Clear and exhaustive with match
let status = match code {
    200 => "OK",
    404 => "Not Found",
    500 => "Server Error",
    _ => "Unknown",
};
```

### When to Use If/Else

Use if/else when:
- Conditions are complex boolean expressions
- You have fewer branches
- Conditions involve multiple variables

```rust
// Better as if/else
let can_send = if user.is_active && user.has_credits() {
    true
} else {
    false
};
```

## Block Syntax in Match Arms

For complex logic, use blocks:

```rust
let score = 75;

let result = match score {
    90..=100 => {
        let bonus = 10;
        format!("Excellent! +{} bonus points", bonus)
    }
    80..=89 => {
        let bonus = 5;
        format!("Good! +{} bonus points", bonus)
    }
    _ => String::from("Keep practicing!"),
};
```

## Common Patterns

### Default Values

```rust
let setting = Some(42);

let value = match setting {
    Some(v) => v,
    None => 0,  // Default
};
```

### Validation

```rust
let input = "hello";

let valid = match input.len() {
    0 => false,
    1..=160 => true,
    _ => false,
};
```

### Transformation

```rust
let command = "SEND";

let action = match command.to_lowercase().as_str() {
    "send" => Action::Send,
    "queue" => Action::Queue,
    "cancel" => Action::Cancel,
    _ => Action::Unknown,
};
```

## Best Practices

### 1. Order Matters

More specific patterns should come first:

```rust
// Good order
match value {
    0 => println!("Zero"),
    1..=10 => println!("Small"),
    _ => println!("Other"),
}

// Bad order - 0 would match the range first
match value {
    1..=10 => println!("Small"),
    0 => println!("Zero"),  // Never reached!
    _ => println!("Other"),
}
```

### 2. Use _ for Catch-All

```rust
// Always have a catch-all for non-enum matches
match http_code {
    200 => "OK",
    404 => "Not Found",
    _ => "Other",  // Important!
}
```

### 3. Be Explicit When Possible

```rust
// Better - explicit about what's expected
match status {
    "pending" | "queued" => "waiting",
    "sent" | "delivered" => "complete",
    "failed" | "bounced" => "error",
    _ => "unknown",
}

// Less clear - what are the valid statuses?
match status {
    s if s.starts_with("p") => "waiting",
    s if s.starts_with("s") => "complete",
    _ => "unknown",
}
```

## Common Mistakes

### 1. Non-Exhaustive Match

```rust
// Error: not all patterns covered
match x {
    1 => "one",
    2 => "two",
    // Missing catch-all!
}
```

### 2. Unreachable Patterns

```rust
// Warning: pattern unreachable
match x {
    _ => "anything",  // Matches everything
    1 => "one",       // Never reached!
}
```

### 3. Wrong Type in Pattern

```rust
let x: i32 = 5;

// Error: mismatched types
match x {
    "five" => println!("string"),  // x is i32, not &str
    _ => println!("other"),
}
```

## Exercise Task

In this exercise, you'll implement:

1. **HTTP status handler** - Match status codes to categories
2. **Message router** - Route messages based on destination
3. **Priority handler** - Match priority levels to timeouts
4. **Subscription manager** - Handle tier-based features
5. **Error classifier** - Categorize error types

## Summary

- `match` compares a value against patterns
- Match must be exhaustive (cover all possibilities)
- `_` is the wildcard pattern that matches anything
- Use `|` to match multiple patterns
- Use `..=` for inclusive ranges
- Match is an expression and returns a value
- Order matters - first match wins
- Use `_` as a catch-all for non-enum matches

## Next Steps

After mastering basic match, you'll learn advanced pattern matching with guards, bindings, and complex patterns.

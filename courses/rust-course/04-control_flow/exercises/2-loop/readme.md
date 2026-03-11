# Exercise 2: Loop - Infinite Loops and Control

## Overview

In this exercise, you'll master Rust's `loop` construct for creating repeating code blocks. The `loop` keyword creates an infinite loop that continues until explicitly stopped with `break`. This is essential for your Textio SMS API's message processing queue and retry mechanisms.

## Learning Objectives

By the end of this exercise, you will:
- Create infinite loops with the `loop` keyword
- Use `break` to exit loops
- Use `continue` to skip to the next iteration
- Return values from loops
- Use loop labels for nested loop control
- Implement retry logic and message queues

## The loop Keyword

### Basic Loop

The simplest form creates an infinite loop:

```rust
loop {
    println!("This runs forever!");
}
```

Without a `break`, this will run indefinitely. You'll need to interrupt it manually (Ctrl+C).

### Breaking Out of Loops

Use `break` to exit the loop:

```rust
let mut count = 0;

loop {
    println!("Count: {}", count);
    count += 1;
    
    if count == 5 {
        break;  // Exit the loop
    }
}

println!("Loop finished!");
```

### Continuing to Next Iteration

Use `continue` to skip the rest of the current iteration:

```rust
let mut i = 0;

loop {
    i += 1;
    
    if i == 3 {
        continue;  // Skip printing when i is 3
    }
    
    println!("Number: {}", i);
    
    if i == 5 {
        break;
    }
}
```

Output:
```
Number: 1
Number: 2
Number: 4
Number: 5
```

## Returning Values from Loops

One of Rust's unique features is that loops can return values:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter * 2;  // Return 20
    }
};

println!("Result: {}", result);  // Result: 20
```

The value after `break` becomes the loop's return value.

### Practical Example: Finding First Match

```rust
let messages = vec![
    ("msg1", "pending"),
    ("msg2", "pending"),
    ("msg3", "delivered"),
    ("msg4", "pending"),
];

let mut index = 0;
let delivered_id = loop {
    if index >= messages.len() {
        break "none";  // Not found
    }
    
    let (id, status) = &messages[index];
    if status == &"delivered" {
        break *id;  // Found it
    }
    
    index += 1;
};

println!("First delivered: {}", delivered_id);  // msg3
```

## Loop Labels

When working with nested loops, you can use labels to specify which loop to `break` or `continue`:

### Basic Labels

```rust
let mut x = 0;

'outer: loop {
    println!("Outer: {}", x);
    let mut y = 0;
    
    'inner: loop {
        println!("  Inner: {}", y);
        y += 1;
        
        if y == 3 {
            break 'inner;  // Break inner loop
        }
    }
    
    x += 1;
    
    if x == 2 {
        break 'outer;  // Break outer loop
    }
}
```

### Breaking Outer Loops

```rust
let matrix = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
];

let mut found = false;

'search: for row in &matrix {
    for &value in row {
        if value == 5 {
            println!("Found 5!");
            found = true;
            break 'search;  // Break out of both loops
        }
    }
}
```

### Continuing Outer Loops

```rust
let messages = vec![
    vec!["msg1", "msg2", "skip"],
    vec!["msg3", "skip", "msg4"],
    vec!["msg5", "msg6", "msg7"],
];

'outer: for batch in &messages {
    println!("Processing batch...");
    
    'inner: for msg in batch {
        if *msg == "skip" {
            println!("  Skipping batch due to: {}", msg);
            continue 'outer;  // Skip to next batch
        }
        println!("  Processing: {}", msg);
    }
}
```

## Textio SMS Application

### 1. Message Queue Processing

```rust
let mut queue: Vec<&str> = vec!["msg1", "msg2", "msg3"];

loop {
    if queue.is_empty() {
        break;
    }
    
    let message = queue.remove(0);
    println!("Processing: {}", message);
}
```

### 2. Retry Logic with Exponential Backoff

```rust
fn send_message(msg: &str, attempt: u32) -> bool {
    // Simulate success after 3 attempts
    attempt >= 3
}

let message = "Hello";
let mut attempt = 0;
let max_attempts = 5;

let success = loop {
    attempt += 1;
    println!("Attempt {} for: {}", attempt, message);
    
    if send_message(message, attempt) {
        break true;  // Success!
    }
    
    if attempt >= max_attempts {
        break false;  // Failed after max attempts
    }
    
    // In real code, you'd add a delay here
};

if success {
    println!("Message sent!");
} else {
    println!("Failed to send message");
}
```

### 3. Connection Pool Management

```rust
struct Connection {
    id: u32,
    active: bool,
}

let mut connections = vec![
    Connection { id: 1, active: true },
    Connection { id: 2, active: false },
    Connection { id: 3, active: true },
];

// Find first available connection
let available = loop {
    let conn = connections.iter_mut().find(|c| c.active);
    
    match conn {
        Some(c) => {
            c.active = false;
            break Some(c.id);
        }
        None => {
            // No available connection, wait and retry
            println!("Waiting for available connection...");
            // In real code: sleep(100ms)
            // For demo, just break
            break None;
        }
    }
};
```

### 4. Rate Limiter

```rust
let mut tokens = 5;  // 5 requests allowed
let max_tokens = 5;

loop {
    // Process requests
    if tokens > 0 {
        tokens -= 1;
        println!("Request processed. Tokens remaining: {}", tokens);
        
        if tokens == 0 {
            println!("Rate limit reached, waiting...");
            break;
        }
    }
}
```

## Conditional Break vs Loop Expression

### Multiple Break Points

```rust
let result = loop {
    let input = get_input();  // Hypothetical function
    
    if input == "quit" {
        break "User quit";
    }
    
    if input == "done" {
        break "Completed";
    }
    
    // Continue processing...
};

fn get_input() -> &'static str {
    "done"
}
```

## Best Practices

### 1. Always Have an Exit Condition

```rust
// Bad - potentially infinite
loop {
    process_message();
}

// Good - clear exit condition
loop {
    if queue.is_empty() {
        break;
    }
    process_message();
}
```

### 2. Use Loop Labels for Clarity

```rust
// Hard to follow
loop {
    loop {
        if condition {
            break;  // Which loop?
        }
    }
}

// Clear with labels
'outer: loop {
    'inner: loop {
        if condition {
            break 'outer;  // Explicit!
        }
    }
}
```

### 3. Return Values for Status

```rust
let status = loop {
    match try_operation() {
        Ok(result) => break result,
        Err(_) if retries > max_retries => break "failed",
        Err(_) => retries += 1,
    }
};
```

## Common Patterns

### Retry Pattern

```rust
let mut attempts = 0;
let max_attempts = 3;

let result = loop {
    attempts += 1;
    
    match send_sms() {
        Ok(response) => break Ok(response),
        Err(e) if attempts >= max_attempts => break Err(e),
        Err(_) => continue,
    }
};
```

### Polling Pattern

```rust
let mut checks = 0;
let max_checks = 10;

let status = loop {
    checks += 1;
    
    if check_delivery_status() == "delivered" {
        break "delivered";
    }
    
    if checks >= max_checks {
        break "timeout";
    }
    
    // wait_and_retry();
};
```

### Consumer Pattern

```rust
let mut queue = vec![1, 2, 3, 4, 5];
let mut processed = Vec::new();

loop {
    match queue.pop() {
        Some(item) => {
            processed.push(item * 2);
        }
        None => break,
    }
}
```

## Common Mistakes

### 1. Forgetting to Break

```rust
// Infinite loop!
loop {
    println!("Help!");
    // Forgot break condition
}

// Fixed
loop {
    println!("Help!");
    count += 1;
    if count > 5 {
        break;
    }
}
```

### 2. Wrong Label

```rust
'outer: loop {
    'inner: loop {
        break 'outer;  // Meant to break inner
    }
    // This code never runs
}
```

### 3. Not Returning Value

```rust
// Returns ()
let result = loop {
    if done {
        break;  // No value
    }
};

// Returns the value
let result = loop {
    if done {
        break 42;  // With value
    }
};
```

## Exercise Task

In this exercise, you'll implement:

1. **Message queue processor** - Process messages until queue is empty
2. **Retry mechanism** - Retry failed message sends with limits
3. **Nested loop processing** - Process message batches with labels
4. **Search function** - Find messages in nested data using loop return values

## Summary

- `loop` creates an infinite loop
- `break` exits the loop (optionally with a value)
- `continue` skips to the next iteration
- Loops can return values with `break value`
- Loop labels (`'label:`) control nested loops
- Use `break 'label` or `continue 'label` for nested control

## Next Steps

After mastering `loop`, you'll learn about `while` and `for` loops for more structured iteration.

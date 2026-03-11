# Exercise 3: While and For Loops - Structured Iteration

## Overview

In this exercise, you'll learn about Rust's `while` and `for` loops, which provide more structured iteration than the basic `loop`. These are essential for processing collections in your Textio SMS API, from iterating through messages to processing user data.

## Learning Objectives

By the end of this exercise, you will:
- Use `while` loops for condition-based iteration
- Master `for` loops with ranges
- Understand inclusive (`1..=5`) and exclusive (`1..5`) ranges
- Iterate over collections with different ownership patterns
- Choose between iteration methods appropriately

## While Loops

### Basic While Loop

The `while` loop continues as long as a condition is true:

```rust
let mut count = 0;

while count < 5 {
    println!("Count: {}", count);
    count += 1;
}
```

### While vs Loop

The `while` loop is more concise when you have a clear condition:

```rust
// Using loop
let mut count = 0;
loop {
    if count >= 5 {
        break;
    }
    println!("Count: {}", count);
    count += 1;
}

// Using while (cleaner)
let mut count = 0;
while count < 5 {
    println!("Count: {}", count);
    count += 1;
}
```

### Practical While Examples

```rust
// Process until queue is empty
let mut queue = vec![1, 2, 3];
while !queue.is_empty() {
    let item = queue.pop().unwrap();
    println!("Processing: {}", item);
}

// Poll until condition met
let mut attempts = 0;
while !is_connected() && attempts < 10 {
    attempts += 1;
    // try_connect();
}
```

## For Loops

The `for` loop is Rust's most common iteration construct. It works with any type that implements the `Iterator` trait.

### For with Ranges

#### Exclusive Range (`..`)

```rust
// 0, 1, 2, 3, 4 (5 is NOT included)
for i in 0..5 {
    println!("i = {}", i);
}
```

#### Inclusive Range (`..=`)

```rust
// 1, 2, 3, 4, 5 (5 IS included)
for i in 1..=5 {
    println!("i = {}", i);
}
```

#### Reverse Range

```rust
// 5, 4, 3, 2, 1
for i in (1..=5).rev() {
    println!("Countdown: {}", i);
}
```

### For with Collections

#### Iterating Over Values

```rust
let messages = vec!["Hello", "World", "Rust"];

for msg in &messages {
    println!("Message: {}", msg);
}
```

#### Iterating with Index

```rust
let messages = vec!["Hello", "World", "Rust"];

for (index, msg) in messages.iter().enumerate() {
    println!("{}: {}", index, msg);
}
```

#### Iterating Mutably

```rust
let mut numbers = vec![1, 2, 3, 4, 5];

for num in &mut numbers {
    *num *= 2;  // Double each number
}

println!("{:?}", numbers);  // [2, 4, 6, 8, 10]
```

## Ownership in Iteration

Understanding ownership is crucial when iterating:

### `for item in collection` - Takes Ownership

```rust
let messages = vec!["msg1".to_string(), "msg2".to_string()];

for msg in messages {
    println!("{}", msg);
    // msg is moved here
}

// messages is no longer valid!
// println!("{:?}", messages);  // Error!
```

### `for item in &collection` - Borrows

```rust
let messages = vec!["msg1".to_string(), "msg2".to_string()];

for msg in &messages {
    println!("{}", msg);
    // msg is a reference (&String)
}

// messages is still valid
println!("{:?}", messages);  // Works!
```

### `for item in &mut collection` - Mutably Borrows

```rust
let mut messages = vec!["msg1".to_string(), "msg2".to_string()];

for msg in &mut messages {
    msg.push_str(" - processed");
    // msg is a mutable reference (&mut String)
}

println!("{:?}", messages);
// ["msg1 - processed", "msg2 - processed"]
```

### `.iter()` vs `&collection`

These are equivalent for vectors:

```rust
let messages = vec![1, 2, 3];

// These are equivalent:
for msg in &messages { }
for msg in messages.iter() { }

// For mutable:
for msg in &mut messages { }
for msg in messages.iter_mut() { }
```

However, `.iter()` is more explicit and works with more types.

### `.into_iter()` - Takes Ownership

```rust
let messages = vec!["msg1".to_string(), "msg2".to_string()];

for msg in messages.into_iter() {
    println!("{}", msg);
}

// messages is consumed
```

## Range Details

### Step By

```rust
// 0, 2, 4, 6, 8
for i in (0..10).step_by(2) {
    println!("{}", i);
}
```

### Characters in Range

```rust
// a, b, c, d, e
for c in 'a'..='e' {
    println!("{}", c);
}
```

### Range as Slice

```rust
let numbers = [1, 2, 3, 4, 5];

// Get slice from index 1 to 3 (exclusive)
let slice = &numbers[1..3];
println!("{:?}", slice);  // [2, 3]

// Get slice from index 1 to 4 (inclusive)
let slice = &numbers[1..=4];
println!("{:?}", slice);  // [2, 3, 4, 5]
```

## Textio SMS Application

### 1. Processing Message Queue

```rust
let messages = vec!["msg1", "msg2", "msg3"];

// Simple iteration
for msg in &messages {
    println!("Sending: {}", msg);
}

// With index
for (i, msg) in messages.iter().enumerate() {
    println!("Message {}: {}", i + 1, msg);
}
```

### 2. Batch Processing

```rust
let all_messages: Vec<&str> = (0..100).map(|i| "msg").collect();
let batch_size = 10;

for batch_start in (0..all_messages.len()).step_by(batch_size) {
    let batch_end = (batch_start + batch_size).min(all_messages.len());
    let batch = &all_messages[batch_start..batch_end];
    
    println!("Processing batch of {} messages", batch.len());
}
```

### 3. Validating Messages

```rust
let messages = vec!["Short", "This is a very long message that exceeds limit", "OK"];
let max_length = 20;

for (index, msg) in messages.iter().enumerate() {
    if msg.len() > max_length {
        println!("Message {} too long: {} chars", index, msg.len());
    }
}
```

### 4. Processing User Accounts

```rust
struct User {
    name: String,
    messages_sent: u32,
    limit: u32,
}

let users = vec![
    User { name: "Alice".into(), messages_sent: 50, limit: 100 },
    User { name: "Bob".into(), messages_sent: 95, limit: 100 },
];

for user in &users {
    let remaining = user.limit - user.messages_sent;
    println!("{}: {} messages remaining", user.name, remaining);
}
```

### 5. Countdown Timer

```rust
println!("Sending messages in...");
for i in (1..=5).rev() {
    println!("{}...", i);
}
println!("Sent!");
```

## Best Practices

### 1. Choose the Right Loop

```rust
// Use for when you know the bounds
for i in 0..10 { }

// Use while for conditional loops
while has_more_data() { }

// Use loop when you need to break from multiple points
loop {
    if condition_a { break; }
    if condition_b { break value; }
}
```

### 2. Prefer Iterators Over Indexing

```rust
// Less idiomatic
let items = vec![1, 2, 3];
for i in 0..items.len() {
    println!("{}", items[i]);
}

// More idiomatic
for item in &items {
    println!("{}", item);
}
```

### 3. Use enumerate() When You Need Index

```rust
let items = vec!["a", "b", "c"];

for (i, item) in items.iter().enumerate() {
    println!("Index {}: {}", i, item);
}
```

### 4. Be Aware of Ownership

```rust
let data = vec![String::from("hello"), String::from("world")];

// Need the data later? Use reference
for s in &data {
    println!("{}", s);
}
// data still valid here

// Don't need data later? Take ownership
for s in data {
    println!("{}", s);
}
// data is now invalid
```

## Common Patterns

### Window/Chunk Processing

```rust
let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

for window in data.windows(3) {
    println!("{:?}", window);
}
// [1, 2, 3]
// [2, 3, 4]
// etc.
```

### Chunk Processing

```rust
let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

for chunk in data.chunks(3) {
    println!("{:?}", chunk);
}
// [1, 2, 3]
// [4, 5, 6]
// [7, 8]
```

### Filter and Process

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

for num in numbers.iter().filter(|&&n| n % 2 == 0) {
    println!("Even: {}", num);
}
```

## Common Mistakes

### 1. Off-by-One Errors

```rust
// Wrong: 0..5 gives 0,1,2,3,4 (not 5)
for i in 0..5 {
    println!("{}", i);
}

// Use inclusive if you need 5
for i in 0..=5 {
    println!("{}", i);  // 0,1,2,3,4,5
}
```

### 2. Modifying While Iterating

```rust
let mut v = vec![1, 2, 3, 4, 5];

// This won't compile - can't borrow mutably while iterating
for i in &v {
    if *i == 3 {
        v.push(6);  // Error!
    }
}

// Instead, collect indices or use filter
let to_remove: Vec<usize> = v.iter()
    .enumerate()
    .filter(|&(_, &x)| x == 3)
    .map(|(i, _)| i)
    .collect();
```

### 3. Using Wrong Range Type

```rust
// For array indexing, remember:
let arr = [1, 2, 3, 4, 5];

// arr.len() = 5, so 0..5 gives valid indices
for i in 0..arr.len() {
    println!("{}", arr[i]);  // OK
}

// 0..=arr.len() would include index 5 - out of bounds!
```

## Exercise Task

In this exercise, you'll implement:

1. **Message batch processing** - Using ranges and step_by
2. **User statistics** - Iterating with enumerate
3. **Message countdown** - Reverse ranges
4. **Collection transformation** - Mutable iteration
5. **Queue polling** - While loops for condition checking

## Summary

- `while` loops run while a condition is true
- `for` loops iterate over iterators
- `1..5` is exclusive (1, 2, 3, 4)
- `1..=5` is inclusive (1, 2, 3, 4, 5)
- `for item in &collection` borrows each item
- `for item in &mut collection` mutably borrows
- `for item in collection` takes ownership
- `.iter()`, `.iter_mut()`, `.into_iter()` are explicit alternatives

## Next Steps

After mastering `while` and `for`, you'll learn about pattern matching with `match`.

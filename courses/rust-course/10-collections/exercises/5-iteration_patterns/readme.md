# Iteration Patterns in Rust

## Introduction

Iterators are one of Rust's most powerful features, providing a lazy, composable way to process collections. In our Textio SMS API, iterators enable elegant data transformations - from filtering messages to aggregating delivery statistics. Understanding the difference between consuming and non-consuming iterators, and mastering patterns like `collect`, `filter`, and `map`, is essential for idiomatic Rust.

## Iterator Basics

### What is an Iterator?

An iterator is any type that implements the `Iterator` trait:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... many provided methods
}
```

### Lazy Evaluation

Iterators are lazy - they don't do anything until consumed:

```rust
let v = vec![1, 2, 3, 4, 5];
let iter = v.iter().map(|x| x * 2);  // Nothing happens yet!

for val in iter {  // Now the work happens
    println!("{}", val);
}
```

## into_iter vs iter vs iter_mut

These three methods are fundamental to understanding Rust iteration:

### iter() - Immutable Borrows

```rust
let v = vec![1, 2, 3];
let iter = v.iter();  // Iterator yields &i32

for val in iter {
    println!("{}", val);
}
// v is still valid here
```

### iter_mut() - Mutable Borrows

```rust
let mut v = vec![1, 2, 3];
let iter = v.iter_mut();  // Iterator yields &mut i32

for val in iter {
    *val *= 2;
}
// v is now [2, 4, 6]
```

### into_iter() - Takes Ownership

```rust
let v = vec![1, 2, 3];
let iter = v.into_iter();  // Iterator yields i32

for val in iter {
    println!("{}", val);
}
// v is no longer valid - ownership was moved
```

### Visual Comparison

```
Original Vector:
┌───┬───┬───┐
│ 1 │ 2 │ 3 │
└───┴───┴───┘

iter() yields:
┌─────┬─────┬─────┐
│ &1  │ &2  │ &3  │  (immutable references)
└─────┴─────┴─────┘
Original vector still accessible

iter_mut() yields:
┌─────┬─────┬─────┐
│&mut1│&mut2│&mut3│  (mutable references)
└─────┴─────┴─────┘
Original vector modified through refs

into_iter() yields:
┌───┬───┬───┐
│ 1 │ 2 │ 3 │  (owned values)
└───┴───┴───┘
Original vector consumed
```

## The collect Method

`collect` transforms an iterator into a collection:

```rust
let v = vec![1, 2, 3, 4, 5];

// Into Vec
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// Into HashSet
use std::collections::HashSet;
let unique: HashSet<i32> = v.into_iter().collect();

// Into HashMap (requires tuples)
let pairs: HashMap<i32, &str> = vec![(1, "one"), (2, "two")]
    .into_iter()
    .collect();
```

### Type Inference with collect

```rust
let v = vec![1, 2, 3];

// Explicit type annotation
let result: Vec<i32> = v.iter().map(|x| x + 1).collect();

// Turbofish syntax
let result = v.iter().map(|x| x + 1).collect::<Vec<i32>>();
```

## Common Iterator Adaptors

### map - Transform Each Element

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

### filter - Keep Matching Elements

```rust
let v = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
// [2, 4]
```

### filter_map - Filter and Transform Combined

```rust
let v = vec!["1", "two", "3"];
let numbers: Vec<i32> = v.iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
// [1, 3]
```

### flat_map - Flatten Nested Iterators

```rust
let v = vec![vec![1, 2], vec![3, 4]];
let flattened: Vec<i32> = v.into_iter().flat_map(|inner| inner).collect();
// [1, 2, 3, 4]
```

### enumerate - Add Index

```rust
let v = vec!["a", "b", "c"];
for (index, value) in v.iter().enumerate() {
    println!("{}: {}", index, value);
}
```

### zip - Combine Two Iterators

```rust
let names = vec!["Alice", "Bob"];
let scores = vec![90, 85];
let pairs: Vec<_> = names.into_iter().zip(scores.into_iter()).collect();
// [("Alice", 90), ("Bob", 85)]
```

### take and skip

```rust
let v = vec![1, 2, 3, 4, 5];

let first_three: Vec<_> = v.iter().take(3).collect();
// [1, 2, 3]

let skip_two: Vec<_> = v.iter().skip(2).collect();
// [3, 4, 5]
```

## Consuming Iterators

### fold - Accumulate Values

```rust
let v = vec![1, 2, 3, 4, 5];
let sum = v.iter().fold(0, |acc, x| acc + x);
// 15
```

### reduce - Like fold with first element as initial

```rust
let v = vec![1, 2, 3, 4, 5];
let max = v.iter().reduce(|a, b| if a > b { a } else { b });
// Some(&5)
```

### any and all

```rust
let v = vec![1, 2, 3, 4, 5];

let has_even = v.iter().any(|x| x % 2 == 0);  // true
let all_positive = v.iter().all(|x| *x > 0);   // true
```

### find and position

```rust
let v = vec![1, 2, 3, 4, 5];

let first_even = v.iter().find(|x| *x % 2 == 0);  // Some(&2)
let first_even_pos = v.iter().position(|x| x % 2 == 0);  // Some(1)
```

### count, sum, product

```rust
let v = vec![1, 2, 3, 4, 5];

let count = v.iter().count();        // 5
let sum: i32 = v.iter().sum();       // 15
let product: i32 = v.iter().product(); // 120
```

## Chaining Methods

The real power of iterators comes from chaining:

```rust
let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = v.into_iter()
    .filter(|x| x % 2 == 0)     // [2, 4, 6, 8, 10]
    .map(|x| x * x)              // [4, 16, 36, 64, 100]
    .take(3)                      // [4, 16, 36]
    .collect();
```

## Textio Example: Message Processing Pipeline

```rust
struct Message {
    id: u64,
    to: String,
    body: String,
    status: Status,
}

#[derive(PartialEq)]
enum Status {
    Pending,
    Sent,
    Delivered,
    Failed,
}

impl Message {
    fn is_delivered(&self) -> bool {
        matches!(self.status, Status::Delivered)
    }
    
    fn character_count(&self) -> usize {
        self.body.chars().count()
    }
}

fn process_messages(messages: Vec<Message>) -> MessageStats {
    let total = messages.len();
    
    let delivered_count = messages.iter()
        .filter(|m| m.is_delivered())
        .count();
    
    let total_chars: usize = messages.iter()
        .map(|m| m.character_count())
        .sum();
    
    let failed_recipients: Vec<String> = messages.iter()
        .filter(|m| matches!(m.status, Status::Failed))
        .map(|m| m.to.clone())
        .collect();
    
    let avg_message_length = if total > 0 {
        total_chars as f64 / total as f64
    } else {
        0.0
    };
    
    MessageStats {
        total,
        delivered_count,
        failed_count: failed_recipients.len(),
        total_chars,
        avg_message_length,
        failed_recipients,
    }
}
```

## Ownership Patterns

### Cloning During Iteration

```rust
let v = vec![String::from("hello"), String::from("world")];

// Without cloning - yields references
let refs: Vec<&String> = v.iter().collect();

// With cloning - yields owned Strings
let owned: Vec<String> = v.iter().cloned().collect();
```

### into_iter with Transformations

```rust
let v = vec![String::from("hello"), String::from("world")];

// Consumes v, produces uppercase owned strings
let upper: Vec<String> = v.into_iter()
    .map(|s| s.to_uppercase())
    .collect();
```

## Performance Considerations

### Zero-Cost Abstractions

Rust iterators are zero-cost abstractions - the compiled code is often as efficient as hand-written loops:

```rust
// These compile to similar code:

let sum: i32 = v.iter().sum();

let mut sum = 0;
for val in &v {
    sum += val;
}
```

### Avoiding Allocations

```rust
// Bad: Collects intermediate results
let v1: Vec<_> = v.iter().map(|x| x * 2).collect();
let v2: Vec<_> = v1.iter().filter(|x| *x > 5).collect();

// Good: Single pass, single allocation
let result: Vec<_> = v.iter()
    .map(|x| x * 2)
    .filter(|x| *x > 5)
    .collect();
```

## Common Patterns

### Partitioning

```rust
let v = vec![1, 2, 3, 4, 5, 6];
let (evens, odds): (Vec<i32>, Vec<i32>) = v.into_iter()
    .partition(|x| x % 2 == 0);
```

### Grouping

```rust
use std::collections::HashMap;

let v = vec!["apple", "banana", "apricot", "blueberry"];
let grouped: HashMap<char, Vec<&str>> = v.iter()
    .fold(HashMap::new(), |mut acc, word| {
        let first = word.chars().next().unwrap();
        acc.entry(first).or_insert_with(Vec::new).push(*word);
        acc
    });
```

### Deduplication with Order Preservation

```rust
fn dedup_preserve_order<T: Eq + Hash + Clone>(v: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    v.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}
```

## Best Practices

1. Prefer iterators over explicit loops for transformations
2. Chain methods for readability
3. Use `collect` only when you need a concrete collection
4. Choose `iter()`, `iter_mut()`, or `into_iter()` based on ownership needs
5. Use `fold` for complex accumulation logic
6. Consider `for_each` when you only need side effects

## Summary

Iterators provide a powerful, composable way to process collections in Rust. Understanding the ownership semantics of different iteration methods and mastering common patterns will help you write clean, efficient code for Textio's message processing pipelines.

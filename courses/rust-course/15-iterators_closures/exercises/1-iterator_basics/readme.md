# Iterator Basics

## Introduction

Iterators are one of Rust's most powerful features, providing a lazy, composable way to process sequences of elements. They form the backbone of idiomatic Rust code and enable expressive, efficient data transformations.

## What is an Iterator?

An iterator is an object that implements the `Iterator` trait, which defines a single required method:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The `Item` associated type specifies what type of elements the iterator produces. The `next()` method returns `Some(item)` when there's another element, and `None` when the iterator is exhausted.

## Laziness

Iterators in Rust are **lazy**. Creating an iterator doesn't do anything until you consume it:

```rust
let numbers = vec![1, 2, 3];
let iter = numbers.iter();  // Nothing happens yet
let sum: i32 = iter.sum();  // Now the iterator is consumed
```

This laziness allows you to chain multiple operations without creating intermediate collections.

## The Three Iterator Types

Rust provides three methods to create iterators from collections:

### 1. `.iter()` - Immutable References

Creates an iterator that yields immutable references to each element:

```rust
let messages = vec!["Hello", "World"];
for msg in messages.iter() {
    println!("{}", msg);  // msg is &&str
}
```

The original collection remains unchanged and accessible after iteration.

### 2. `.iter_mut()` - Mutable References

Creates an iterator that yields mutable references, allowing you to modify elements:

```rust
let mut counts = vec![1, 2, 3];
for count in counts.iter_mut() {
    *count *= 2;  // Double each element
}
// counts is now [2, 4, 6]
```

### 3. `.into_iter()` - Ownership Transfer

Creates an iterator that takes ownership of each element:

```rust
let messages = vec!["Hello".to_string(), "World".to_string()];
for msg in messages.into_iter() {
    println!("{}", msg);  // msg is String, not &String
}
// messages is no longer valid here!
```

## Understanding Ownership with Iterators

The choice between `iter()`, `iter_mut()`, and `into_iter()` has significant implications:

| Method | Yields | Original Collection | Use Case |
|--------|--------|---------------------|----------|
| `iter()` | `&T` | Still valid | Read-only access |
| `iter_mut()` | `&mut T` | Still valid | Modify in place |
| `into_iter()` | `T` | Consumed | Transfer ownership |

## The `for` Loop and `into_iter`

When you use a `for` loop directly on a collection, Rust automatically calls `into_iter()`:

```rust
let numbers = vec![1, 2, 3];
for n in numbers {  // Equivalent to numbers.into_iter()
    println!("{}", n);
}
// numbers is consumed
```

To iterate without consuming, use `&`:

```rust
let numbers = vec![1, 2, 3];
for n in &numbers {  // Equivalent to numbers.iter()
    println!("{}", n);
}
// numbers is still valid
```

## Consuming Adaptors

Iterator methods that consume the iterator and produce a result:

### `collect()`

Collects iterator results into a collection:

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

### `sum()` and `product()`

Calculate sum or product of all elements:

```rust
let sum: i32 = (1..=10).sum();      // 55
let product: i32 = (1..=5).product(); // 120
```

### `count()`

Counts the number of elements:

```rust
let count = (1..100).filter(|x| x % 2 == 0).count();
```

### `any()` and `all()`

Test if any or all elements satisfy a condition:

```rust
let has_even = (1..10).any(|x| x % 2 == 0);  // true
let all_positive = (1..10).all(|x| x > 0);    // true
```

### `find()` and `position()`

Find elements or their positions:

```rust
let nums = vec![1, 2, 3, 4, 5];
let first_even = nums.iter().find(|x| *x % 2 == 0);  // Some(&2)
let even_position = nums.iter().position(|x| x % 2 == 0);  // Some(1)
```

## Range Iterators

Ranges are iterators themselves:

```rust
1..10    // 1 through 9 (exclusive)
1..=10   // 1 through 10 (inclusive)
'a'..='z' // Characters a through z
```

## Iterator Size Hints

The `Iterator` trait also has optional methods:

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

This returns a lower and optional upper bound on the number of elements. Some iterators know their exact size, which enables optimizations.

## Common Pitfalls

### 1. Modifying Collection During Iteration

```rust
// This will panic or cause undefined behavior
let mut v = vec![1, 2, 3];
for i in v.iter() {
    v.push(*i);  // DON'T DO THIS!
}
```

### 2. Using Consumed Iterator

```rust
let numbers = vec![1, 2, 3];
let iter = numbers.iter();
let sum: i32 = iter.sum();
// iter is now consumed and cannot be used again
```

### 3. Forgetting to Consume

```rust
let numbers = vec![1, 2, 3];
numbers.iter().map(|x| println!("{}", x));
// Nothing prints! The iterator was never consumed.
```

## Textio Example: Message Processing

```rust
struct Message {
    id: u32,
    content: String,
    sent: bool,
}

let messages = vec![
    Message { id: 1, content: "Hello".to_string(), sent: true },
    Message { id: 2, content: "World".to_string(), sent: false },
];

// Count unsent messages
let unsent_count = messages.iter().filter(|m| !m.sent).count();

// Get all message IDs
let ids: Vec<u32> = messages.iter().map(|m| m.id).collect();
```

## Exercise Overview

In this exercise, you will:
1. Create iterators using `iter()`, `iter_mut()`, and `into_iter()`
2. Use consuming adaptors like `collect()`, `sum()`, and `count()`
3. Understand ownership transfer with iterators
4. Process Textio message collections

## Key Takeaways

- Iterators are lazy - they do nothing until consumed
- Choose the right iterator method based on ownership needs
- `iter()` borrows, `iter_mut()` mutably borrows, `into_iter()` takes ownership
- Consuming adaptors exhaust the iterator
- The `for` loop implicitly uses `into_iter()`

## Further Reading

- [Rust Book Chapter 13](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator Trait Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

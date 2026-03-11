# Vectors in Rust

## Introduction

Vectors (`Vec<T>`) are Rust's growable, heap-allocated arrays. They are one of the most commonly used collection types in Rust, providing dynamic sizing while maintaining type safety. In our Textio SMS API project, vectors are essential for storing lists of messages, phone numbers, and delivery statuses.

## What is a Vector?

A vector is a contiguous growable array type that stores elements of the same type on the heap. Unlike arrays, which have a fixed size determined at compile time, vectors can grow or shrink at runtime.

```
Memory Layout:
┌─────────────────────────────────────────────────────┐
│                    Stack                             │
│  ┌─────────────────────────────────────────────┐    │
│  │ Vec struct (24 bytes on 64-bit)             │    │
│  │ ┌───────────┬───────────┬───────────────┐   │    │
│  │ │  ptr      │  length   │   capacity    │   │    │
│  │ │ (8 bytes) │ (8 bytes) │  (8 bytes)    │   │    │
│  │ └─────┬─────┴───────────┴───────────────┘   │    │
│  └───────│─────────────────────────────────────┘    │
└──────────│──────────────────────────────────────────┘
           │
           ▼
┌──────────────────────────────────────────────────────┐
│                    Heap                               │
│  ┌───────┬───────┬───────┬───────┬───────┬───────┐   │
│  │  0    │  1    │  2    │  3    │  4    │  5    │   │
│  │ elem  │ elem  │ elem  │ elem  │(spare│(spare│   │
│  │       │       │       │       │space)│space)│   │
│  └───────┴───────┴───────┴───────┴───────┴───────┘   │
│         ◄──── length ──────►                        │
│         ◄────────── capacity ──────────►            │
└──────────────────────────────────────────────────────┘
```

## Creating Vectors

### Using the `new` Function

```rust
let v: Vec<i32> = Vec::new();
```

This creates an empty vector with zero capacity. The type annotation is required because Rust cannot infer the type from an empty vector.

### Using the `vec!` Macro

```rust
let v = vec![1, 2, 3, 4, 5];
let messages = vec!["Hello", "World"];
```

The `vec!` macro is the most common way to create vectors with initial values. Rust infers the type from the elements.

### With Initial Capacity

```rust
let mut v = Vec::with_capacity(100);
```

Pre-allocating capacity avoids reallocations when you know approximately how many elements you'll store.

## Capacity vs Length

Understanding the difference between capacity and length is crucial for performance optimization:

- **Length**: The number of elements currently stored in the vector
- **Capacity**: The total allocated space available without reallocation

```rust
let mut v = Vec::with_capacity(10);
println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // 0, 10

v.push(1);
v.push(2);
println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // 2, 10
```

### Memory Reallocation

When a vector's length exceeds its capacity, Rust automatically reallocates:

```
Before push (at capacity):
┌───┬───┬───┬───┐
│ 0 │ 1 │ 2 │ 3 │  length=4, capacity=4
└───┴───┴───┴───┘

After push (exceeds capacity):
┌───┬───┬───┬───┬───┬───┬───┬───┐
│ 0 │ 1 │ 2 │ 3 │ 4 │   │   │   │  length=5, capacity=8
└───┴───┴───┴───┴───┴───┴───┴───┘
         New allocation (typically doubles)
```

## Modifying Vectors

### Adding Elements with `push`

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```

### Removing Elements with `pop`

`pop` removes and returns the last element as an `Option<T>`:

```rust
let mut v = vec![1, 2, 3];
let last = v.pop();  // Some(3)
let second_last = v.pop();  // Some(2)
let empty = v.pop();  // Some(1)
let nothing = v.pop();  // None
```

## Reading Elements

### Indexing

```rust
let v = vec![10, 20, 30, 40];
let first = v[0];   // 10
let second = v[1];  // 20
// v[4] would panic! - index out of bounds
```

### The `get` Method

```rust
let v = vec![10, 20, 30];
let first = v.get(0);   // Some(&10)
let invalid = v.get(10); // None
```

The `get` method returns an `Option<&T>`, providing safe access without panicking.

## Advanced Operations

### `with_capacity` and `reserve`

```rust
let mut v = Vec::with_capacity(5);
v.reserve(10);  // Ensures capacity for at least 10 more elements
```

### `shrink_to_fit`

Reduces capacity to match length:

```rust
let mut v = Vec::with_capacity(100);
v.push(1);
v.push(2);
v.shrink_to_fit();  // Capacity now equals length (2)
```

### `drain`

Removes a range of elements and returns an iterator:

```rust
let mut v = vec![1, 2, 3, 4, 5];
let drained: Vec<_> = v.drain(1..4).collect();  // [2, 3, 4]
// v is now [1, 5]
```

### `splice`

Replaces a range with new elements:

```rust
let mut v = vec![1, 2, 3, 4, 5];
v.splice(1..4, vec![10, 20]);  // Replace indices 1,2,3 with 10,20
// v is now [1, 10, 20, 5]
```

## Iterating Over Vectors

### Immutable Iteration

```rust
let v = vec![1, 2, 3];
for element in &v {
    println!("{}", element);
}
```

### Mutable Iteration

```rust
let mut v = vec![1, 2, 3];
for element in &mut v {
    *element *= 2;  // Double each element
}
```

### Consuming Iteration

```rust
let v = vec![1, 2, 3];
for element in v {
    println!("{}", element);
}
// v is no longer valid here
```

## Textio Example: Storing Messages

```rust
struct MessageQueue {
    pending: Vec<String>,
    sent: Vec<String>,
}

impl MessageQueue {
    fn new() -> Self {
        MessageQueue {
            pending: Vec::with_capacity(100),
            sent: Vec::new(),
        }
    }

    fn queue(&mut self, message: String) {
        self.pending.push(message);
    }

    fn send_next(&mut self) -> Option<String> {
        self.pending.pop()
    }

    fn mark_sent(&mut self, message: String) {
        self.sent.push(message);
    }
}
```

## Common Pitfalls

### Index Out of Bounds

```rust
let v = vec![1, 2, 3];
let x = v[10];  // Panics at runtime!
```

### Borrow Checker Issues

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);  // Error: cannot borrow as mutable while borrowed as immutable
println!("{}", first);
```

## Best Practices

1. Use `Vec::with_capacity` when you know the approximate size
2. Use `get` for safe access when index might be invalid
3. Prefer iterators over indexing for better performance
4. Use `drain` when you need to remove multiple elements efficiently
5. Call `shrink_to_fit` after bulk removals to free memory

## Summary

Vectors are versatile, growable arrays that form the backbone of many Rust programs. Understanding capacity management, safe access patterns, and the various methods available will help you write efficient and safe code for your Textio SMS API project.

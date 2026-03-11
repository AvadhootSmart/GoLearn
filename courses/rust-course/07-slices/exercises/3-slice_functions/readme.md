# Exercise 3: Slice Functions in Rust

## Overview

One of the most powerful aspects of slices is their use as function parameters. By accepting slices instead of owned types, your functions become more flexible, efficient, and idiomatic. In the Textio SMS API, you'll write many functions that process collections of messages, and understanding slice functions is essential.

## Why Use Slices as Function Parameters?

### Flexibility

A function accepting `&[T]` can be called with:
- Arrays: `[T; N]`
- Vectors: `Vec<T>`
- Other slices: `&[T]`
- Sub-slices: `&collection[start..end]`

```rust
fn process(data: &[i32]) {
    // Works with any sequence of i32
}

let arr = [1, 2, 3];
let vec = vec![4, 5, 6];

process(&arr);        // Array
process(&vec);        // Vector
process(&arr[0..2]);  // Slice of array
process(&vec[1..]);   // Slice of vector
```

### Efficiency

Passing a slice copies only 16 bytes (pointer + length), regardless of the data size:

```rust
// Efficient: copies only 16 bytes
fn efficient(data: &[i32]) { /* ... */ }

// Inefficient: copies the entire array
fn inefficient(data: [i32; 1000]) { /* ... */ }
```

### No Ownership Transfer

Slices borrow data, leaving ownership with the caller:

```rust
fn print_first(messages: &[String]) {
    if let Some(first) = messages.first() {
        println!("{}", first);
    }
    // No ownership taken, caller still owns messages
}

let msgs = vec!["Hello".to_string(), "World".to_string()];
print_first(&msgs);
// msgs is still valid here!
```

## String Slices in Functions

### `&str` vs `String` Parameters

Always prefer `&str` for string parameters:

```rust
// Preferred: Accepts String, &String, and &str
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Less preferred: Only accepts String
fn greet_owned(name: String) {
    println!("Hello, {}!", name);
}

let owned = String::from("Alice");
let borrowed = "Bob";

greet(&owned);     // Works
greet(borrowed);   // Works
greet("Charlie");  // Works

greet_owned(owned);      // Takes ownership
// greet_owned(borrowed); // Won't compile!
```

### Returning String Slices

You can return string slices, but they must borrow from input parameters:

```rust
// Returns a slice that borrows from input
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}

let text = "Hello, World!";
let word = first_word(text);
// word borrows from text, so text must remain valid
```

## Array Slices in Functions

### Basic Slice Functions

```rust
// Sum any sequence of integers
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// Find the maximum value
fn max_value(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().max()
}

// Check if slice contains a value
fn contains(haystack: &[i32], needle: i32) -> bool {
    haystack.contains(&needle)
}
```

### Working with Generic Slices

You can write functions that work with slices of any type:

```rust
// Works with slices of any type that implements Debug
fn print_all<T: std::fmt::Debug>(items: &[T]) {
    for item in items {
        println!("{:?}", item);
    }
}

// Works with slices of any comparable type
fn find_min<T: Ord>(items: &[T]) -> Option<&T> {
    items.iter().min()
}
```

### Slice Patterns

Use slice patterns for elegant matching:

```rust
fn describe_slice(slice: &[i32]) -> &'static str {
    match slice {
        [] => "Empty",
        [single] => "One element",
        [first, second] => "Two elements",
        [first, .., last] => "Multiple elements",
        _ => "Unknown"
    }
}
```

## Mutable Slice Functions

Functions can accept mutable slices to modify data:

```rust
// Double all values in place
fn double_all(values: &mut [i32]) {
    for val in values.iter_mut() {
        *val *= 2;
    }
}

// Reverse a portion of the slice
fn reverse_range(values: &mut [i32], start: usize, end: usize) {
    if end <= values.len() {
        values[start..end].reverse();
    }
}

let mut numbers = [1, 2, 3, 4, 5];
double_all(&mut numbers);
// numbers is now [2, 4, 6, 8, 10]
```

## Multiple Slice Parameters

Functions can accept multiple slices with different lifetimes:

```rust
// Find common elements between two slices
fn find_common<'a>(a: &'a [i32], b: &'a [i32]) -> Vec<&'a i32> {
    a.iter()
        .filter(|x| b.contains(x))
        .collect()
}

// Concatenate two slices into a new vector
fn concatenate(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend_from_slice(a);
    result.extend_from_slice(b);
    result
}
```

## Slice Traits and Bounds

### Common Trait Bounds

```rust
// Requires elements to be cloneable
fn clone_all<T: Clone>(items: &[T]) -> Vec<T> {
    items.to_vec()
}

// Requires elements to be displayable
fn join_with<T: std::fmt::Display>(items: &[T], separator: &str) -> String {
    items
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

// Requires elements to be equatable
fn count_occurrences<T: Eq>(items: &[T], target: &T) -> usize {
    items.iter().filter(|x| *x == target).count()
}
```

### Using `AsRef` for Maximum Flexibility

The `AsRef` trait allows even more flexibility:

```rust
// Accepts anything that can be converted to &[str]
fn process_strings<S: AsRef<str>>(strings: &[S]) {
    for s in strings {
        println!("{}", s.as_ref());
    }
}

let arr = ["a", "b", "c"];
let vec = vec!["x".to_string(), "y".to_string()];

process_strings(&arr);  // Works with &str
process_strings(&vec);  // Works with String
```

## Common Slice Function Patterns

### The "Find" Pattern

```rust
fn find_by_predicate<T, F>(slice: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    slice.iter().find(|x| predicate(x))
}
```

### The "Transform" Pattern

```rust
fn transform_slice<T, U, F>(slice: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    slice.iter().map(f).collect()
}
```

### The "Partition" Pattern

```rust
fn partition_slice<T, F>(slice: &[T], predicate: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    slice.iter().partition(|x| predicate(x))
}
```

## Performance Considerations

### Avoid Unnecessary Cloning

```rust
// Bad: Clones every element
fn bad_process(messages: &[String]) -> Vec<String> {
    messages.to_vec()
}

// Good: Works with references
fn good_process(messages: &[String]) -> Vec<&str> {
    messages.iter().map(|s| s.as_str()).collect()
}
```

### Use Iterators Over Indexing

```rust
// Slower: Bounds checking on every access
fn sum_with_index(slice: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..slice.len() {
        total += slice[i];
    }
    total
}

// Faster: No bounds checking in the loop
fn sum_with_iter(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
```

## Textio Use Cases

### Processing Message Batches

```rust
fn send_batch(messages: &[SmsMessage]) -> Vec<DeliveryResult> {
    messages
        .iter()
        .map(|msg| send_single(msg))
        .collect()
}
```

### Filtering by Criteria

```rust
fn filter_by_status(messages: &[SmsMessage], status: Status) -> Vec<&SmsMessage> {
    messages
        .iter()
        .filter(|m| m.status == status)
        .collect()
}
```

### Aggregating Data

```rust
fn calculate_statistics(messages: &[SmsMessage]) -> MessageStats {
    MessageStats {
        total: messages.len(),
        total_chars: messages.iter().map(|m| m.content.len()).sum(),
        average_length: if messages.is_empty() {
            0.0
        } else {
            messages.iter().map(|m| m.content.len()).sum::<usize>() as f64 
                / messages.len() as f64
        },
    }
}
```

## Exercise Instructions

In this exercise, you'll implement flexible functions for Textio's analytics:

1. **`count_by_status`**: Count messages by delivery status
2. **`filter_short_messages`**: Filter messages shorter than a threshold
3. **`find_longest_message`**: Find the message with the longest content
4. **`group_by_recipient`**: Group messages by recipient prefix

Run the code and verify it produces the expected output.

## Key Takeaways

- Slices make functions flexible (work with arrays, vectors, slices)
- Passing slices is efficient (only 16 bytes copied)
- Slices borrow data, preserving ownership
- Use `&str` for string parameters, not `String`
- Mutable slices (`&mut [T]`) allow in-place modification
- Generic slice functions work with any element type
- Trait bounds add flexibility while maintaining type safety
- Iterators are usually faster than indexing

## Common Pitfalls

1. **Returning slice of local variable** (compile error)
2. **Taking `&Vec<T>` instead of `&[T]`** (less flexible)
3. **Cloning when borrowing would work** (inefficient)
4. **Forgetting to handle empty slices** (logic errors)

## Further Reading

- [Rust Book: Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust API Guidelines: Flexibility](https://rust-lang.github.io/api-guidelines/flexibility.html)
- [Effective Rust: Accept Slices](https://www.lurklurk.org/effective-rust/slice.html)

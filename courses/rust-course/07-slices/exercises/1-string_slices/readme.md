# Exercise 1: String Slices in Rust

## Overview

String slices are one of Rust's most important features for safe and efficient string handling. In the Textio SMS API, you'll frequently need to extract portions of messages, validate phone numbers, and parse SMS content—all operations that rely heavily on string slices.

## What is a String Slice?

A **string slice** is a reference to a portion of a `String` (or a string literal). It's written as `&str` and represents a "view" into some UTF-8 encoded string data.

```rust
let message = String::from("Hello, Textio!");
let greeting: &str = &message[0..5]; // "Hello"
```

Unlike `String`, which owns its data and can grow or shrink, a `&str` is an immutable reference to existing string data.

## String vs &str

### String
- **Owns** its data (heap-allocated)
- **Mutable** (can grow, shrink, change)
- **Dynamically sized**
- Created with `String::from()`, `.to_string()`, or `format!()`

### &str (String Slice)
- **Borrows** data (doesn't own it)
- **Immutable** view
- **Fixed size** (cannot grow or shrink)
- Created with slicing syntax or string literals

```rust
// String - owns data
let owned: String = String::from("Hello");

// &str - borrows data
let borrowed: &str = &owned[0..3];

// String literals are &str
let literal: &str = "Hello";
```

## Slicing Syntax

Rust provides several ways to create slices using range syntax:

### 1. Full Range: `&s[..]`
Creates a slice of the entire string.

```rust
let message = "Welcome to Textio!";
let full: &str = &message[..];
// full == "Welcome to Textio!"
```

### 2. From Start: `&s[..end]`
Creates a slice from the beginning up to (but not including) `end`.

```rust
let message = "Hello, World!";
let hello: &str = &message[..5];
// hello == "Hello"
```

### 3. To End: `&s[start..]`
Creates a slice from `start` to the end of the string.

```rust
let message = "Hello, World!";
let world: &str = &message[7..];
// world == "World!"
```

### 4. Range: `&s[start..end]`
Creates a slice from `start` up to (but not including) `end`.

```rust
let message = "Hello, World!";
let middle: &str = &message[3..8];
// middle == "lo, W"
```

### 5. Exclusive End: `&s[start..=end]` (Inclusive Range)
Creates a slice from `start` up to and including `end`.

```rust
let message = "Hello";
let ell: &str = &message[1..=3];
// ell == "ell"
```

## The Fat Pointer: How Slices Work

A `&str` is a "fat pointer" containing two pieces of information:
1. **Pointer** to the start of the data
2. **Length** of the slice

```
String "Hello, World!" in memory:
Address: 0x1000
Data:    H  e  l  l  o  ,     W  o  r  l  d  !
         0  1  2  3  4  5  6  7  8  9 10 11 12

Slice &s[7..12]:
Pointer: 0x1007 (address of 'W')
Length:  5
```

This is why slices are efficient—no data is copied, only the pointer and length are stored.

## UTF-8 and Slicing: A Critical Consideration

Rust strings are UTF-8 encoded, which means characters can be 1-4 bytes. **Slicing by byte index** can cause panics if you don't respect character boundaries.

### The Problem

```rust
let hello = "Здравствуйте"; // Cyrillic characters
let slice = &hello[0..1]; // PANIC! 'З' is 2 bytes
```

This panics because `0..1` only includes 1 byte, but the first character 'З' is 2 bytes.

### Safe UTF-8 Slicing

For safe UTF-8 handling, use these methods:

```rust
// Get characters as an iterator
let hello = "Здравствуйте";
let first_char = hello.chars().next(); // Some('З')

// Get byte index of a character boundary
let hello = "hello";
let idx = hello.char_indices().nth(3).unwrap().0;
let slice = &hello[..idx]; // Safe!

// Use .get() for non-panicking slices
let hello = "Здравствуйте";
let slice = hello.get(0..2); // Some("З") - 2 bytes
let bad_slice = hello.get(0..1); // None - not at boundary
```

### When to Use Byte Slicing

Byte slicing is safe when:
1. Working with ASCII-only text
2. You know the exact byte boundaries
3. Processing binary data

## The `.as_str()` Method

The `.as_str()` method converts a `String` into a `&str`:

```rust
let message = String::from("Hello, Textio!");

// These are equivalent:
let slice1: &str = &message[..];
let slice2: &str = message.as_str();
```

Use `.as_str()` when:
- You need to pass a `String` to a function expecting `&str`
- You want explicit conversion (clearer intent)
- You're working with iterators or other contexts

## String Slices in Function Parameters

Functions should accept `&str` rather than `String` for flexibility:

```rust
// Flexible - accepts &String, &str, and string literals
fn count_characters(text: &str) -> usize {
    text.len()
}

// Can be called with:
let owned = String::from("hello");
count_characters(&owned);        // &String -> &str
count_characters("hello");        // &str literal
count_characters(&owned[..]);     // explicit slice
```

## Why Slices Don't Have Ownership

Slices are **references**, not owners:

1. **Borrowing**: A slice borrows data from its owner
2. **No Drop**: When a slice goes out of scope, nothing is deallocated
3. **Lifetime Bound**: The slice cannot outlive its source data
4. **Memory Efficient**: No allocation or copying required

```rust
fn process_message() {
    let message = String::from("Hello"); // Owner
    let slice: &str = &message[0..3];    // Borrower
    
    // slice is valid because message is still alive
    println!("{}", slice);
    
    // When this function ends:
    // 1. slice is dropped (no deallocation)
    // 2. message is dropped (memory freed)
}
```

## Range Types in Rust

The slicing syntax uses different range types:

| Syntax | Type | Example |
|--------|------|---------|
| `start..end` | `Range<usize>` | `1..5` |
| `start..` | `RangeFrom<usize>` | `1..` |
| `..end` | `RangeTo<usize>` | `..5` |
| `..` | `RangeFull` | `..` |
| `start..=end` | `RangeInclusive<usize>` | `1..=5` |

These types implement the `SliceIndex` trait, enabling slicing operations.

## Performance Benefits of Slices

1. **Zero Allocation**: No heap allocation for the slice itself
2. **Zero Copy**: Data is not copied, only referenced
3. **Cache Friendly**: Small slice metadata (16 bytes: ptr + len)
4. **Pass by Value**: Slices are Copy, so they're passed efficiently

```rust
// This is efficient - no allocation or copy
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

## Common Patterns in Textio

### Extracting Phone Numbers

```rust
fn extract_country_code(phone: &str) -> &str {
    // Assumes format: +1-555-123-4567
    &phone[1..3] // Returns "1-"
}
```

### Parsing Message Content

```rust
fn get_preview(message: &str, max_len: usize) -> &str {
    if message.len() <= max_len {
        &message[..]
    } else {
        &message[..max_len]
    }
}
```

### Validating Message Format

```rust
fn has_valid_prefix(message: &str) -> bool {
    message.starts_with("SMS:") || message.starts_with("MMS:")
}
```

## Exercise Instructions

In this exercise, you'll implement functions for Textio's message processing:

1. **`extract_area_code`**: Extract the area code from a phone number
2. **`get_message_preview`**: Get the first N characters of a message
3. **`find_keyword`**: Find the position of a keyword in the message
4. **`safe_slice`**: Safely slice a string without panicking

Run the code and verify it produces the expected output.

## Key Takeaways

- `&str` is a borrowed view into string data
- Slicing uses range syntax: `[start..end]`
- Always respect UTF-8 boundaries when slicing
- Slices are fat pointers (pointer + length)
- Use slices in function parameters for flexibility
- Slices are efficient: no allocation, no copying
- Use `.get()` for safe, non-panicking slices

## Common Pitfalls

1. **Slicing at non-character boundaries** (panics!)
2. **Using slices after their source is dropped** (compile error)
3. **Confusing byte indices with character indices**
4. **Assuming all characters are 1 byte**

## Further Reading

- [Rust Book: String Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust Reference: Slice Types](https://doc.rust-lang.org/reference/types/slice.html)
- [UTF-8 Everywhere](https://utf8everywhere.org/)

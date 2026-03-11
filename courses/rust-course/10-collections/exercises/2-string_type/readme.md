# Strings in Rust

## Introduction

Strings are fundamental to any text-processing application, and in our Textio SMS API, they're everywhere - from message content to phone numbers to API keys. Rust takes a unique approach to strings that prioritizes safety and performance, but it requires understanding two distinct types: `String` and `&str`.

## String vs &str

### The Two String Types

Rust has two main string types:

1. **`String`**: A growable, heap-allocated, UTF-8 encoded string type
2. **`&str`**: A string slice - a reference to a sequence of UTF-8 bytes

```
Memory Layout Comparison:

String (owned):
┌──────────────── Stack ────────────────┐
│  ┌────────────────────────────────┐   │
│  │ ptr ──────────────────────┐    │   │
│  │ length: 5                 │    │   │
│  │ capacity: 5               │    │   │
│  └───────────────────────────│────┘   │
└──────────────────────────────│────────┘
                               ▼
┌──────────────── Heap ─────────────────┐
│  ['H']['e']['l']['l']['o']            │
└───────────────────────────────────────┘

&str (slice):
┌──────────────── Stack ────────────────┐
│  ┌────────────────────────────────┐   │
│  │ ptr ──────────────────────┐    │   │
│  │ length: 5                 │    │   │
│  └───────────────────────────│────┘   │
└──────────────────────────────│────────┘
                               ▼
┌───────────── Somewhere ───────────────┐
│  ['H']['e']['l']['l']['o']            │
│  (could be in binary, stack, or heap) │
└───────────────────────────────────────┘
```

### When to Use Which

| Use `String` when | Use `&str` when |
|-------------------|-----------------|
| You need ownership | You only need to read |
| You need to modify | You're passing as a parameter |
| You need to grow/shrink | You're returning a view into data |
| Data must outlive current scope | Working with string literals |

## UTF-8 Encoding

Rust strings are UTF-8 encoded, meaning each character can be 1-4 bytes:

```
ASCII characters (1 byte each):
'H' 'e' 'l' 'l' 'o'
┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐
│H│ │e│ │l│ │l│ │o│
└─┘ └─┘ └─┘ └─┘ └─┘

Unicode characters (variable bytes):
'ñ' (2 bytes)  '日' (3 bytes)  '🎉' (4 bytes)
┌───┐          ┌─────┐         ┌───────┐
│C3 │          │ E6  │         │ F0 9F │
│B1 │          │ 97  │         │ 8E 89 │
└───┘          │ A5  │         └───────┘
               └─────┘

Important: String length is bytes, not characters!
```

### Byte Indexing Warning

```rust
let hello = "Hello";
println!("{}", hello.len());  // 5 bytes

let emoji = "🎉";
println!("{}", emoji.len());  // 4 bytes, not 1!

// This would panic:
// let ch = emoji[0];  // Cannot index into UTF-8!
```

## Creating Strings

### From Literals

```rust
let s = "hello".to_string();
let s = String::from("hello");
let s: String = "hello".into();
```

### Empty Strings

```rust
let s = String::new();
let s = String::with_capacity(100);
```

### From Other Types

```rust
let s = 42.to_string();
let s = true.to_string();
let s = format!("Value: {}", 42);
```

## Modifying Strings

### push and push_str

```rust
let mut s = String::from("Hello");
s.push(' ');        // Add a single character
s.push_str("World"); // Add a string slice
```

### The + Operator

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("World!");
let s3 = s1 + &s2;  // s1 is moved, s2 is borrowed
// s1 is no longer valid here!
```

Note: The left operand must be a `String`, the right must be a `&str`.

### format! Macro

```rust
let s1 = String::from("Hello");
let s2 = String::from("World");
let s = format!("{}, {}!", s1, s2);  // Both s1 and s2 remain valid
```

## Deref to &str

`String` implements `Deref<Target=str>`, which means:

```rust
let s = String::from("Hello");
let slice: &str = &s;  // Automatic coercion
```

This allows you to pass `&String` where `&str` is expected:

```rust
fn print_slice(s: &str) {
    println!("{}", s);
}

let owned = String::from("Hello");
print_slice(&owned);  // Works!
```

## String Slicing

### Safe Slicing with get

```rust
let s = "Hello, World!";
let slice = s.get(0..5);  // Some("Hello")
let invalid = s.get(0..100);  // None (safe!)
```

### Character-Based Operations

```rust
let s = "Hello";
let chars: Vec<char> = s.chars().collect();
let first_char = s.chars().next();  // Some('H')
```

## Common Operations

### Concatenation

```rust
let parts = vec!["Hello", " ", "World"];
let combined = parts.concat();  // "Hello World"
let joined = parts.join("-");   // "Hello- -World"
```

### Splitting

```rust
let s = "Hello,World,Rust";
let parts: Vec<&str> = s.split(',').collect();

let s = "Hello World";
let words: Vec<&str> = s.split_whitespace().collect();
```

### Trimming

```rust
let s = "  Hello  ";
let trimmed = s.trim();  // "Hello"
```

### Replacing

```rust
let s = "Hello, World!";
let new_s = s.replace("World", "Rust");  // "Hello, Rust!"
```

## Textio Example: Message Processing

```rust
struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

impl SmsMessage {
    fn new(to: &str, from: &str, body: &str) -> Self {
        SmsMessage {
            to: to.to_string(),
            from: from.to_string(),
            body: body.to_string(),
        }
    }
    
    fn truncate(&mut self, max_chars: usize) {
        let chars: Vec<char> = self.body.chars().take(max_chars).collect();
        self.body = chars.into_iter().collect();
    }
    
    fn preview(&self, len: usize) -> &str {
        self.body.get(0..len).unwrap_or(&self.body)
    }
}
```

## Memory Considerations

### Capacity Management

```rust
let mut s = String::with_capacity(100);
s.push_str("Hello");
println!("Length: {}, Capacity: {}", s.len(), s.capacity());

s.shrink_to_fit();  // Reduce capacity to match length
```

### Avoiding Allocations

```rust
// Bad: Multiple allocations
let mut result = String::new();
result += &s1;
result += &s2;
result += &s3;

// Better: Single allocation with format!
let result = format!("{}{}{}", s1, s2, s3);

// Best for many parts: Pre-calculate capacity
let mut result = String::with_capacity(s1.len() + s2.len() + s3.len());
result.push_str(&s1);
result.push_str(&s2);
result.push_str(&s3);
```

## Common Pitfalls

### Indexing by Character

```rust
let s = "Hello";
// s[0] would panic - can't index string!
// Use chars() instead:
let first = s.chars().next().unwrap();
```

### Non-UTF-8 Bytes

```rust
let bytes = vec![0, 159, 146, 150];
// String::from_utf8(bytes).unwrap() would panic!
// Use from_utf8_lossy instead:
let s = String::from_utf8_lossy(&bytes);
```

### Forgetting &

```rust
let s1 = String::from("Hello");
let s2 = String::from("World");
// s1 + s2;  // Error! s2 must be &str
s1 + &s2;   // Correct
```

## Best Practices

1. Use `&str` for function parameters unless you need ownership
2. Use `String::with_capacity` when building large strings
3. Prefer `format!` over `+` for multiple concatenations
4. Use `chars()` for character-level operations
5. Use `get()` for safe slicing that might fail
6. Return `String` when creating new strings, `&str` for views

## Summary

Understanding the distinction between `String` and `&str` is fundamental to Rust programming. The ownership model ensures memory safety while UTF-8 encoding provides proper Unicode support. For Textio, this knowledge helps us efficiently process SMS messages while handling international characters correctly.

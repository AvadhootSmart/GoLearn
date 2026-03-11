# Generic Functions in Rust

## Introduction

Generics are one of Rust's most powerful features, allowing you to write flexible, reusable code that works with multiple types while maintaining type safety. In this exercise, you'll learn how to create generic functions that can operate on different types without sacrificing performance.

## What Are Generics?

Generics allow you to write code that abstracts over types. Instead of writing separate functions for each type you want to support, you write one function with a type parameter that gets filled in at compile time.

### The Problem Without Generics

Consider Textio's SMS processing. Without generics, you might write:

```rust
fn get_first_i32(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers[0])
    }
}

fn get_first_string(strings: &[String]) -> Option<String> {
    if strings.is_empty() {
        None
    } else {
        Some(strings[0].clone())
    }
}

fn get_first_message(messages: &[Message]) -> Option<Message> {
    if messages.is_empty() {
        None
    } else {
        Some(messages[0].clone())
    }
}
```

This leads to code duplication and maintenance headaches.

### The Generic Solution

With generics, you write one function:

```rust
fn get_first<T: Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        None
    } else {
        Some(items[0].clone())
    }
}
```

## Type Parameter Syntax

### Basic Syntax

Type parameters are specified in angle brackets `<T>` after the function name:

```rust
fn function_name<T>(parameter: T) -> T {
    // function body
}
```

The `T` is a convention (short for "Type"), but you can use any valid identifier:
- `T` for a generic type
- `E` for errors
- `K` and `V` for keys and values in maps
- `Item` for collection elements

### Multiple Type Parameters

Functions can have multiple type parameters:

```rust
fn pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

let result = pair("Hello", 42);  // (&str, i32)
```

## The Turbofish Syntax

Sometimes the compiler can't infer the type parameter from context. In these cases, use the turbofish operator `::<Type>`:

```rust
fn create_default<T: Default>() -> T {
    T::default()
}

// Without turbofish - compiler can't infer T
let value = create_default();  // ERROR: cannot infer type

// With turbofish - explicit type specification
let value = create_default::<String>();  // OK: returns empty String
let value = create_default::<i32>();     // OK: returns 0
```

### When to Use Turbofish

1. **When type inference is ambiguous:**
```rust
fn parse<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.parse()
}

let number = parse::<i32>("42");  // Explicit type
let text = parse::<String>("hello");  // Different type
```

2. **When returning generic types:**
```rust
fn empty_collection<T>() -> Vec<T> {
    Vec::new()
}

let numbers: Vec<i32> = empty_collection();  // Type annotation
let strings = empty_collection::<String>();  // Turbofish
```

3. **In method chains:**
```rust
let result = "42"
    .parse::<i32>()
    .map(|n| n * 2)
    .unwrap();
```

## Trait Bounds

Generic functions often need to constrain what types can be used with trait bounds:

```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

### Common Trait Bounds in Textio

```rust
// For types that can be compared
fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    items.iter().max()
}

// For types that can be cloned
fn duplicate<T: Clone>(item: T) -> (T, T) {
    (item.clone(), item)
}

// Multiple trait bounds
fn process<T: Clone + std::fmt::Debug>(item: T) {
    println!("Processing: {:?}", item);
    let copy = item.clone();
}
```

### Where Clauses

For complex bounds, use `where` clauses for readability:

```rust
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Hash,
{
    format!("T: {}, U: {:?}", t, u)
}
```

## Monomorphization

Rust generics are implemented through monomorphization - the compiler generates specialized versions of generic code for each concrete type used.

```rust
fn id<T>(x: T) -> T { x }

fn main() {
    let a = id(10);        // Compiler generates id_i32
    let b = id("hello");   // Compiler generates id_str
    let c = id(3.14);      // Compiler generates id_f64
}
```

This means:
- **Zero runtime overhead** - generic code is as fast as hand-written specialized code
- **Larger binary size** - each type gets its own copy of the function
- **Compile-time type checking** - all type errors caught at compile time

## Generic Functions in Textio

Textio uses generic functions extensively:

### Response Handling

```rust
#[derive(Debug, Clone)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

fn send_request<T: DeserializeOwned>(
    endpoint: &str,
    body: &str,
) -> Result<ApiResponse<T>, reqwest::Error> {
    // Generic HTTP request handling
}
```

### Message Processing

```rust
fn process_messages<T, F>(messages: Vec<Message>, processor: F) -> Vec<T>
where
    F: Fn(Message) -> T,
{
    messages.into_iter().map(processor).collect()
}
```

### Validation

```rust
fn validate_field<T, E>(value: T, validator: fn(T) -> Result<T, E>) -> Result<T, E> {
    validator(value)
}
```

## Best Practices

1. **Use meaningful type parameter names** for complex generics:
```rust
// Less clear
fn transform<A, B, C>(a: A, b: B) -> C { ... }

// More clear
fn transform<Input, Config, Output>(input: Input, config: Config) -> Output { ... }
```

2. **Keep trait bounds minimal** - only require what you actually use:
```rust
// Too restrictive
fn process<T: Clone + Debug + Hash + Eq>(item: T) { ... }

// Better - only what's needed
fn process<T: Clone>(item: T) { ... }
```

3. **Document trait bounds** so users know what types are valid:
```rust
/// Processes a collection of items.
/// 
/// # Type Parameters
/// * `T` - Must implement `Clone` for safe copying
/// * `T` - Must implement `Display` for logging
fn process<T: Clone + Display>(items: Vec<T>) { ... }
```

4. **Use turbofish sparingly** - prefer type inference when possible:
```rust
// Prefer this when type is clear
let numbers: Vec<i32> = Vec::new();

// Use turbofish when type is ambiguous
let result = parse::<Config>(json_string);
```

## Common Patterns

### The Builder Pattern with Generics

```rust
struct RequestBuilder<T> {
    endpoint: String,
    body: T,
}

impl<T: Serialize> RequestBuilder<T> {
    fn send(self) -> Result<Response, Error> {
        // Send request with serialized body
    }
}
```

### The Newtype Pattern

```rust
struct MessageId<T>(T);

fn create_id<T: Default>() -> MessageId<T> {
    MessageId(T::default())
}
```

## Exercise Overview

In this exercise, you will:
1. Create a generic `get_first` function for Textio's message queue
2. Implement a generic `pair` function using multiple type parameters
3. Write a generic `max_value` function with trait bounds
4. Use turbofish syntax for type disambiguation
5. Implement a generic response handler for Textio's API

## Key Takeaways

- Generics eliminate code duplication while maintaining type safety
- Type parameters go in angle brackets: `<T>`
- Use turbofish `::<Type>()` when the compiler can't infer types
- Trait bounds constrain what types can be used
- Monomorphization provides zero runtime overhead
- Multiple type parameters allow complex generic relationships

## Next Steps

After mastering generic functions, you'll learn about generic structs, which allow you to create data structures that work with any type while maintaining Rust's safety guarantees.

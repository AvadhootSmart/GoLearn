# Generic Structs in Rust

## Introduction

Generic structs extend the power of generics to data structures, allowing you to define types that can hold any type of data while maintaining Rust's strict type safety. In this exercise, you'll learn how to create generic structs, implement methods for them, and work with multiple type parameters.

## Why Generic Structs?

### The Problem

Imagine Textio needs to track coordinates for various purposes - message routing, delivery zones, and analytics. Without generics:

```rust
struct IntPoint {
    x: i32,
    y: i32,
}

struct FloatPoint {
    x: f64,
    y: f64,
}

struct StringCoordinate {
    x: String,
    y: String,
}
```

This leads to duplicated code and maintenance overhead.

### The Generic Solution

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 10, y: 20 };
let float_point = Point { x: 3.14, y: 2.71 };
```

## Generic Struct Syntax

### Basic Syntax

```rust
struct StructName<T> {
    field: T,
}
```

The type parameter `<T>` is declared after the struct name and can be used for any field.

### Multiple Fields with Same Type

```rust
struct Coordinates<T> {
    latitude: T,
    longitude: T,
}

let precise = Coordinates { latitude: 37.7749, longitude: -122.4194 };
let grid = Coordinates { latitude: 10, longitude: 20 };
```

## Multiple Type Parameters

Structs can have multiple type parameters:

```rust
struct KeyValue<K, V> {
    key: K,
    value: V,
}

let phone_number = KeyValue { key: "user_id", value: 12345 };
let setting = KeyValue { key: "theme", value: "dark" };
```

### Textio Example: Generic Message Envelope

```rust
struct MessageEnvelope<T, M> {
    id: T,
    timestamp: u64,
    metadata: M,
    payload: String,
}

let sms_envelope = MessageEnvelope {
    id: "msg-123",
    timestamp: 1630000000,
    metadata: SmsMetadata { priority: 1 },
    payload: "Hello!".to_string(),
};
```

## Generic Struct Methods

### Implementation Blocks

Methods for generic structs require the type parameter in the `impl` block:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}
```

### Methods with Trait Bounds

Some methods only make sense for certain types:

```rust
impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> Point<T> {
    fn scale(&self, factor: T) -> Point<T> {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}
```

### Methods Specific to Concrete Types

You can implement methods for specific type instantiations:

```rust
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<i32> {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
```

## Associated Types vs Generic Parameters

Generic structs can use associated types in their traits:

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn map<U, F>(self, f: F) -> Container<U>
    where
        F: FnOnce(T) -> U,
    {
        Container { value: f(self.value) }
    }
}

let number = Container::new(42);
let text = number.map(|n| format!("Number: {}", n));
```

## Common Patterns in Textio

### Generic Response Wrapper

```rust
struct ApiResponse<T> {
    status: u16,
    success: bool,
    data: Option<T>,
    error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    fn ok(data: T) -> Self {
        ApiResponse {
            status: 200,
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    fn error(status: u16, error: ApiError) -> Self {
        ApiResponse {
            status,
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
```

### Generic Repository Pattern

```rust
struct Repository<T> {
    items: Vec<T>,
}

impl<T: Clone> Repository<T> {
    fn new() -> Self {
        Repository { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get_all(&self) -> Vec<T> {
        self.items.clone()
    }
    
    fn count(&self) -> usize {
        self.items.len()
    }
}
```

### Generic Configuration

```rust
struct Config<T> {
    key: String,
    value: T,
    description: String,
}

impl<T: Display> Config<T> {
    fn display(&self) {
        println!("{}: {} ({})", self.key, self.value, self.description);
    }
}
```

## Type Parameter Defaults

Rust allows default type parameters:

```rust
#[derive(Debug)]
struct Message<T = String> {
    content: T,
}

fn main() {
    let default_msg = Message { content: "Hello".to_string() }; // T = String
    let custom_msg = Message { content: 42 }; // T = i32
}
```

## Ownership and Lifetimes with Generics

Generic structs can hold references with lifetime parameters:

```rust
struct BorrowedContent<'a, T> {
    reference: &'a T,
}

struct MessageWithSender<'a, T> {
    content: T,
    sender: &'a str,
}
```

## Generic Structs vs Trait Objects

When choosing between generics and trait objects:

**Use Generic Structs when:**
- You know types at compile time
- Performance is critical
- You want static dispatch

**Use Trait Objects when:**
- Types are determined at runtime
- You need heterogeneous collections
- Binary size is a concern

```rust
// Generic - static dispatch
struct GenericHandler<T: Handler> {
    handler: T,
}

// Trait object - dynamic dispatch
struct DynamicHandler {
    handler: Box<dyn Handler>,
}
```

## Memory Layout

Generic structs are monomorphized at compile time:

```rust
struct Pair<T> {
    first: T,
    second: T,
}

// Creates three different types:
let int_pair: Pair<i32> = Pair { first: 1, second: 2 };
let float_pair: Pair<f64> = Pair { first: 1.0, second: 2.0 };
let string_pair: Pair<String> = Pair { first: "a".to_string(), second: "b".to_string() };
```

Each instantiation creates a new concrete type with its own memory layout.

## Best Practices

1. **Name type parameters meaningfully** for complex structs:
```rust
struct Map<Key, Value> { ... }
struct Message<Id, Payload, Metadata> { ... }
```

2. **Keep structs focused** - don't over-genericize:
```rust
// Too generic
struct Everything<A, B, C, D, E> { ... }

// Better
struct Message<Id, Content> { ... }
```

3. **Use trait bounds only in impl blocks** when possible:
```rust
// Bounds in struct definition - restrictive
struct Point<T: Add> { x: T, y: T }

// Bounds in impl - flexible
struct Point<T> { x: T, y: T }
impl<T: Add> Point<T> { ... }
```

4. **Document what type parameters represent**:
```rust
/// A message envelope for Textio's routing system.
/// 
/// # Type Parameters
/// - `Id`: The unique identifier type (e.g., UUID, i64)
/// - `Content`: The message payload type
struct Envelope<Id, Content> { ... }
```

## Exercise Overview

In this exercise, you will:
1. Create a generic `Point<T>` struct for Textio's mapping features
2. Implement a generic `Pair<T, U>` struct with multiple type parameters
3. Build a generic `Container<T>` with methods
4. Create a generic `ApiResponse<T>` for Textio's API
5. Implement a generic `Repository<T>` for data storage

## Key Takeaways

- Generic structs use `<T>` after the struct name
- Multiple type parameters are declared as `<T, U, V>`
- Methods require `<T>` in the impl block
- Trait bounds can be added to specific methods
- Monomorphization creates specialized code at compile time
- Use meaningful names for type parameters in complex structs

## Next Steps

Now that you understand generic structs, you'll learn about generic enums, which combine the power of sum types with generics to create flexible, type-safe abstractions like `Option<T>` and `Result<T, E>`.

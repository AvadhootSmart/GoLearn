# Closures

## Introduction

Closures are anonymous functions that can capture their environment. They're essential for writing concise, expressive Rust code, especially when working with iterators. Closures allow you to define behavior inline and pass it around as values.

## Closure Syntax

Closures use vertical pipes to enclose parameters:

```rust
// Basic closure
let add_one = |x| x + 1;
let result = add_one(5);  // 6

// Multiple parameters
let add = |x, y| x + y;
let sum = add(2, 3);  // 5

// Explicit return type
let square = |x: i32| -> i32 { x * x };

// Multi-line closure with block
let describe = |name: &str, age: i32| {
    let decade = age / 10;
    format!("{} is in their {}0s", name, decade)
};
```

## Type Inference

Unlike functions, closures don't require explicit type annotations. The compiler infers types from usage:

```rust
// Compiler infers x is i32 from the addition
let add = |x| x + 1;
let n: i32 = add(5);

// Once inferred, the type is fixed
// This would fail: add(5.0)  // expected i32, found f64
```

However, you can add explicit types when needed:

```rust
let multiply = |x: i32, y: i32| -> i32 { x * y };
```

## Capturing the Environment

Closures can capture variables from their surrounding scope:

```rust
let multiplier = 5;
let multiply = |x| x * multiplier;  // Captures multiplier

let result = multiply(3);  // 15
```

The closure "closes over" the `multiplier` variable, hence the name "closure."

## Closure vs Function

```rust
// Function - cannot capture environment
fn add_one(x: i32) -> i32 {
    x + 1
}

// Closure - can capture environment
let n = 1;
let add_n = |x| x + n;
```

## Fn Traits

Closures implement one or more of these traits based on how they capture variables:

### `Fn` - Immutable Borrows

The closure captures variables by immutable reference:

```rust
let greeting = String::from("Hello");
let say_hello = || println!("{}", greeting);  // Borrows greeting immutably

say_hello();
say_hello();  // Can be called multiple times
println!("{}", greeting);  // greeting still accessible
```

### `FnMut` - Mutable Borrows

The closure captures variables by mutable reference and may modify them:

```rust
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};

println!("{}", increment());  // 1
println!("{}", increment());  // 2
// println!("{}", count);     // Error: count is borrowed
println!("{}", increment());  // 3
```

### `FnOnce` - Takes Ownership

The closure takes ownership of captured variables and can only be called once:

```rust
let data = vec![1, 2, 3];
let consume = || {
    let _ = data;  // Takes ownership
    println!("Data consumed");
};

consume();  // Works
// consume();  // Error: closure already called
```

## Trait Hierarchy

Every closure that implements `Fn` also implements `FnMut` and `FnOnce`. The relationship:

```
Fn ⊆ FnMut ⊆ FnOnce
```

This means:
- A `Fn` closure can be used anywhere `FnMut` or `FnOnce` is expected
- A `FnMut` closure can be used anywhere `FnOnce` is expected
- A `FnOnce` closure can only be used where `FnOnce` is expected

## Closures as Function Parameters

You can accept closures as function parameters using generics:

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

let double = |x| x * 2;
let result = apply(double, 5);  // 10
```

### Choosing the Right Trait Bound

```rust
// Use Fn when you need to call the closure multiple times
fn call_twice<F>(f: F)
where
    F: Fn(),
{
    f();
    f();
}

// Use FnMut when the closure needs to modify captured variables
fn modify_and_call<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

// Use FnOnce when the closure consumes its captures
fn consume_closure<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
```

## The `impl Fn` Syntax

For simpler function signatures, you can use `impl Trait`:

```rust
fn process_messages(messages: Vec<String>, transform: impl Fn(&str) -> String) -> Vec<String> {
    messages.iter().map(|m| transform(m)).collect()
}
```

## Returning Closures

Closures have anonymous types, so you must use `impl Trait` or `Box<dyn Trait>`:

```rust
// Using impl Trait (static dispatch)
fn get_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// Using Box (dynamic dispatch)
fn get_processor(mode: &str) -> Box<dyn Fn(i32) -> i32> {
    match mode {
        "double" => Box::new(|x| x * 2),
        "triple" => Box::new(|x| x * 3),
        _ => Box::new(|x| x),
    }
}
```

## Closures with Iterators

Closures shine when used with iterator methods:

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Filter with closure
let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();

// Map with closure
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// Sort with custom closure
let mut sorted = numbers.clone();
sorted.sort_by(|a, b| b.cmp(a));  // Descending order
```

## Textio Example: Message Processing

```rust
struct Message {
    content: String,
    recipient: String,
    priority: u8,
}

fn process_messages<F>(messages: Vec<Message>, predicate: F) -> Vec<Message>
where
    F: Fn(&Message) -> bool,
{
    messages.into_iter().filter(predicate).collect()
}

let messages = vec![/* ... */];

// Using closure to filter high-priority messages
let urgent = process_messages(messages, |m| m.priority >= 8);

// Using closure with captured variable
let min_priority = 5;
let filtered = process_messages(all_messages, move |m| m.priority >= min_priority);
```

## Common Patterns

### Builder Pattern with Closures

```rust
struct QueryBuilder {
    filters: Vec<Box<dyn Fn(&Record) -> bool>>,
}

impl QueryBuilder {
    fn add_filter<F: 'static + Fn(&Record) -> bool>(mut self, f: F) -> Self {
        self.filters.push(Box::new(f));
        self
    }
}
```

### Event Handlers

```rust
struct Button {
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    fn set_on_click<F: 'static + FnMut()>(&mut self, handler: F) {
        self.on_click = Some(Box::new(handler));
    }
}
```

### Configuration Callbacks

```rust
fn configure_server<F>(port: u16, startup_hook: F)
where
    F: FnOnce(),
{
    println!("Starting server on port {}", port);
    startup_hook();
}
```

## Exercise Overview

In this exercise, you will:
1. Define and use closures with various syntaxes
2. Understand type inference in closures
3. Use closures with iterator methods
4. Pass closures as function parameters
5. Implement Textio message filtering with closures

## Key Takeaways

- Closures are anonymous functions that can capture their environment
- Syntax: `|params| expression` or `|params| { block }`
- Types are inferred but can be explicitly specified
- Closures implement `Fn`, `FnMut`, or `FnOnce` based on capture behavior
- `Fn` ⊆ `FnMut` ⊆ `FnOnce` - the trait hierarchy
- Use `impl Fn(...)` for function parameters accepting closures
- Use `Box<dyn Fn(...)>` when you need to store closures

## Further Reading

- [Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Fn Trait Documentation](https://doc.rust-lang.org/std/ops/trait.Fn.html)

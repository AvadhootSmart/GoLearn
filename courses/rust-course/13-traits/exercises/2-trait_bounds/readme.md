# Trait Bounds in Rust

## Introduction

Trait bounds allow you to constrain generic types to those that implement specific traits. This is how Rust achieves compile-time polymorphism while maintaining type safety. In Textio, trait bounds ensure that our generic functions only work with types that have the required capabilities.

## Basic Trait Bounds

The simplest form of a trait bound appears in generic function definitions:

```rust
fn send_message<T: Message>(msg: T) {
    println!("Sending to: {}", msg.recipient());
}
```

This function accepts any type `T` that implements the `Message` trait. The compiler ensures that `msg.recipient()` is valid because the trait bound guarantees the method exists.

## Multiple Trait Bounds

When a generic type needs to satisfy multiple traits:

```rust
fn process<T: Message + Clone>(msg: T) {
    let copy = msg.clone();
    println!("Original: {}", copy.content());
}
```

The `+` syntax combines multiple trait bounds.

## The `where` Clause

For complex bounds, the `where` clause provides better readability:

```rust
fn send_and_track<M, S>(message: M, status: S)
where
    M: Message + Clone,
    S: StatusReport + Updatable,
{
    // Implementation
}
```

### When to Use `where` Clauses

1. **Multiple type parameters** with complex bounds
2. **Long trait lists** that would make function signatures hard to read
3. **Nested generics** like `T: Container<Item: Clone>`

```rust
// Without where clause - harder to read
fn complex_function<T: Message + Clone + Send + Sync, U: StatusReport + Debug>(a: T, b: U) {}

// With where clause - clearer
fn complex_function<T, U>(a: T, b: U)
where
    T: Message + Clone + Send + Sync,
    U: StatusReport + Debug,
{
}
```

## Trait Bounds on Structs

Generic structs can also have trait bounds:

```rust
struct MessageSender<T: Message> {
    message: T,
    sent: bool,
}

impl<T: Message> MessageSender<T> {
    fn new(message: T) -> Self {
        MessageSender {
            message,
            sent: false,
        }
    }
    
    fn send(&mut self) {
        println!("Sending to: {}", self.message.recipient());
        self.sent = true;
    }
}
```

## Trait Bounds on Implementations

You can implement traits conditionally based on bounds:

```rust
impl<T: Message + Clone> Cloneable for MessageSender<T> {
    fn duplicate(&self) -> Self {
        MessageSender {
            message: self.message.clone(),
            sent: self.sent,
        }
    }
}
```

## Generic Functions vs Trait Objects

Understanding when to use generics vs trait objects:

| Generics (Static Dispatch) | Trait Objects (Dynamic Dispatch) |
|---------------------------|----------------------------------|
| Compile-time resolution | Runtime resolution |
| Faster (monomorphization) | Slight overhead |
| Binary size increases | Single version |
| Must be known at compile time | Can be determined at runtime |

## Monomorphization

The compiler generates specialized code for each concrete type:

```rust
fn send<T: Message>(msg: T) { }

// After monomorphization, roughly:
fn send_sms(msg: SmsMessage) { }
fn send_mms(msg: MmsMessage) { }
```

This is why generics are fast - no runtime dispatch overhead.

## Practical Examples in Textio

### Message Validation with Bounds

```rust
fn validate_and_send<T>(msg: T) -> Result<(), String>
where
    T: Message + Validatable,
{
    msg.validate()?;
    println!("Sending: {}", msg.content());
    Ok(())
}
```

### Batch Processing

```rust
fn batch_send<T>(messages: Vec<T>)
where
    T: Message,
{
    for msg in messages {
        println!("To: {} - {}", msg.recipient(), msg.content());
    }
}
```

### Multiple Type Parameters

```rust
fn log_message<M, L>(message: &M, logger: &mut L)
where
    M: Message,
    L: Write,
{
    writeln!(logger, "From: {}", message.sender()).unwrap();
    writeln!(logger, "To: {}", message.recipient()).unwrap();
    writeln!(logger, "Content: {}", message.content()).unwrap();
}
```

## Bounds in Type Aliases

Type aliases can include trait bounds:

```rust
type MessageResult<T> = Result<T, MessageError>;
type SendableMessage = dyn Message + Send + Sync;
```

## Advanced Patterns

### Bounds with Associated Types

```rust
fn get_first_item<C>(container: &C) -> Option<&C::Item>
where
    C: Container,
    C::Item: Clone,
{
    container.get()
}
```

### Conditional Implementation

```rust
impl<T: Message + Default> MessageBuilder for T {
    fn build_default() -> Self {
        T::default()
    }
}
```

## Common Pitfalls

### Overly Restrictive Bounds

```rust
// Too restrictive
fn process<T: Message + Clone + Debug + Default + Send + Sync>(msg: T) {}

// Better - only what you need
fn process<T: Message>(msg: T) {}
```

### Missing Bounds

```rust
// Error: cannot clone T without Clone bound
fn duplicate<T>(item: T) -> (T, T) {
    (item.clone(), item)  // Won't compile!
}

// Fixed
fn duplicate<T: Clone>(item: T) -> (T, T) {
    (item.clone(), item)
}
```

## Exercise Overview

In this exercise, you will:

1. Write generic functions with trait bounds
2. Use `where` clauses for complex bounds
3. Combine multiple trait bounds with `+`
4. Create generic structs with trait bounds
5. Implement conditional trait implementations

## Key Takeaways

- Trait bounds constrain generic types to those implementing specific traits
- Use `where` clauses for better readability with complex bounds
- Multiple bounds are combined with `+`
- Generics use static dispatch (monomorphization)
- Only include bounds that are actually needed

## Next Steps

Now that you understand trait bounds, you'll learn about default implementations that reduce boilerplate when implementing traits.

# Trait Objects and Dynamic Dispatch

## Introduction

Trait objects enable runtime polymorphism through dynamic dispatch. Unlike generics (static dispatch), trait objects allow you to work with different types implementing the same trait through a single interface at runtime.

In Textio, trait objects are useful when you need to store or process different message types together in collections or when the concrete type isn't known until runtime.

## Static vs Dynamic Dispatch

### Static Dispatch (Generics)

```rust
fn send<T: Message>(msg: T) {
    // Compiler generates separate code for each T
}

send(sms);    // Calls send::<SmsMessage>
send(email);  // Calls send::<EmailMessage>
```

### Dynamic Dispatch (Trait Objects)

```rust
fn send(msg: &dyn Message) {
    // Single function, looks up method at runtime
}

send(&sms);   // Both use the same code
send(&email); // Method lookup at runtime
```

## Creating Trait Objects

### Using `dyn Trait`

```rust
let message: &dyn Message = &sms;
println!("{}", message.content());
```

### Using `Box<dyn Trait>`

For owned trait objects:

```rust
let message: Box<dyn Message> = Box::new(sms);
println!("{}", message.content());
```

## When to Use Trait Objects

1. **Heterogeneous collections** - storing different types together
2. **Runtime type decisions** - type determined at runtime
3. **Reducing code size** - no monomorphization bloat
4. **Plugin architectures** - dynamically loaded types

## Heterogeneous Collections

One of the most common uses:

```rust
let messages: Vec<Box<dyn Message>> = vec![
    Box::new(SmsMessage { /* ... */ }),
    Box::new(EmailMessage { /* ... */ }),
    Box::new(PushMessage { /* ... */ }),
];

for msg in &messages {
    println!("To: {}", msg.recipient());
}
```

## Object Safety

Not all traits can be made into trait objects. A trait is **object safe** if:

1. It doesn't return `Self`
2. It doesn't use `Self` in generic parameters
3. It has no associated constants or types (without bounds)
4. All methods are object safe

### Object Safe Examples

```rust
trait Message {
    fn content(&self) -> &str;  // OK
    fn send(&self);              // OK
}

let msg: &dyn Message = &sms;  // Works!
```

### Not Object Safe

```rust
trait Cloneable {
    fn clone_me(&self) -> Self;  // Returns Self - NOT OK
}

trait Factory {
    fn create() -> Self;         // No &self - NOT OK
}

trait Compare {
    fn equals(&self, other: &Self);  // Uses Self in param - NOT OK
}
```

### Making Traits Object Safe

```rust
// Instead of returning Self, return Box<dyn Trait>
trait CloneableObject {
    fn clone_boxed(&self) -> Box<dyn CloneableObject>;
}

impl CloneableObject for SmsMessage {
    fn clone_boxed(&self) -> Box<dyn CloneableObject> {
        Box::new(self.clone())
    }
}
```

## The `dyn` Keyword

`dyn` explicitly indicates dynamic dispatch:

```rust
// These are equivalent in older Rust:
fn process(msg: &dyn Message)  // Modern, explicit
fn process(msg: &Message)      // Legacy, implicit
```

Always use `dyn` for clarity in modern Rust.

## Memory Layout

Trait objects use a "fat pointer" containing:

1. **Data pointer** - points to the actual data
2. **vtable pointer** - points to method dispatch table

```
+----------------+----------------+
|  Data Pointer  | VTable Pointer |
+----------------+----------------+
       |                |
       v                v
   [actual data]    [method table]
```

## Performance Considerations

| Aspect | Static Dispatch | Dynamic Dispatch |
|--------|----------------|------------------|
| Speed | Faster (direct call) | Slight overhead (vtable lookup) |
| Binary size | Larger (monomorphization) | Smaller (single version) |
| Compile time | Longer | Shorter |
| Flexibility | Compile-time only | Runtime flexibility |

## Practical Textio Example

### Multi-Notifier System

```rust
trait Notifier {
    fn notify(&self, message: &str);
    fn name(&self) -> &str;
}

struct SmsNotifier { /* ... */ }
struct EmailNotifier { /* ... */ }
struct PushNotifier { /* ... */ }

impl Notifier for SmsNotifier { /* ... */ }
impl Notifier for EmailNotifier { /* ... */ }
impl Notifier for PushNotifier { /* ... */ }

fn broadcast(notifiers: &[Box<dyn Notifier>], msg: &str) {
    for notifier in notifiers {
        println!("Sending via {}...", notifier.name());
        notifier.notify(msg);
    }
}
```

## Where Clauses with Trait Objects

```rust
fn process(msg: &(dyn Message + Send)) {
    // Message trait object that also implements Send
}

fn store(msg: Box<dyn Message + Sync + Send>) {
    // Owned trait object with multiple auto traits
}
```

## Trait Objects with Associated Types

```rust
trait Processor {
    type Output;
    fn process(&self) -> Self::Output;
}

// Can't use dyn Processor directly - need to specify type
fn use_processor(p: &dyn Processor<Output = String>) {
    let result = p.process();
}
```

## Downcasting

Sometimes you need to recover the concrete type:

```rust
use std::any::Any;

trait Message: Any {
    fn content(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

impl Message for SmsMessage {
    fn content(&self) -> &str { &self.body }
    fn as_any(&self) -> &dyn Any { self }
}

fn process(msg: &dyn Message) {
    if let Some(sms) = msg.as_any().downcast_ref::<SmsMessage>() {
        // Now have access to SmsMessage-specific methods
    }
}
```

## Exercise Overview

In this exercise, you will:

1. Create trait objects with `&dyn Trait`
2. Use `Box<dyn Trait>` for owned trait objects
3. Build heterogeneous collections
4. Understand object safety constraints
5. Implement a multi-channel notification system

## Key Takeaways

- Trait objects enable runtime polymorphism
- Use `dyn Trait` for references, `Box<dyn Trait>` for ownership
- Object safety rules determine if a trait can be used dynamically
- Dynamic dispatch has slight runtime overhead but enables flexibility
- Fat pointers store data and vtable references

## Next Steps

After mastering trait objects, you'll learn about Rust's most common standard library traits and how to implement them for your types.

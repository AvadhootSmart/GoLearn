# Multiple Type Parameters, Constraints, and PhantomData

## Introduction

Advanced generic programming in Rust involves working with multiple type parameters, complex trait bounds, and sometimes needing to track types at compile time without storing their values. This exercise covers these advanced patterns, including the powerful `PhantomData` marker type used extensively in Rust's standard library.

## Multiple Type Parameters

### Basic Multi-Parameter Generics

When a type needs to work with multiple independent types, use multiple type parameters:

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

struct Triple<T, U, V> {
    first: T,
    second: U,
    third: V,
}
```

### Textio Example: Generic Repository

```rust
struct Repository<Id, Entity> {
    items: HashMap<Id, Entity>,
    id_generator: Box<dyn Fn() -> Id>,
}

impl<Id, Entity> Repository<Id, Entity>
where
    Id: Eq + Hash + Clone,
    Entity: Clone,
{
    fn new(generator: Box<dyn Fn() -> Id>) -> Self {
        Repository {
            items: HashMap::new(),
            id_generator: generator,
        }
    }
    
    fn add(&mut self, entity: Entity) -> Id {
        let id = (self.id_generator)();
        self.items.insert(id.clone(), entity);
        id
    }
}
```

## Trait Bounds and Constraints

### Where Clauses

Complex trait bounds are more readable with `where` clauses:

```rust
fn process<T, U, V>(input: T, config: U) -> V
where
    T: IntoIterator,
    T::Item: Clone + Debug,
    U: Configuration + Send + Sync,
    V: FromIterator<T::Item>,
{
    // Implementation
}
```

### Multiple Bounds

Combine multiple traits with `+`:

```rust
fn send_message<T>(message: T) -> Result<Receipt, Error>
where
    T: Clone + Serialize + Send + 'static,
{
    // message can be cloned, serialized, sent across threads
}
```

### Nested Bounds

Some constraints involve associated types:

```rust
fn aggregate<T, I>(items: I) -> T::Summary
where
    T: Aggregate,
    I: IntoIterator<Item = T>,
{
    // Work with items that implement Aggregate
}
```

## Default Type Parameters

Type parameters can have defaults:

```rust
#[derive(Debug)]
struct Message<T = String, Id = u64> {
    id: Id,
    content: T,
}

let default_msg: Message = Message { id: 1, content: "Hello".to_string() };
let custom_msg: Message<Vec<u8>, String> = Message { 
    id: "msg-001".to_string(), 
    content: vec![1, 2, 3] 
};
```

## PhantomData: Type Tracking Without Storage

### The Problem

Sometimes you need a type parameter for type safety but don't actually store a value of that type:

```rust
// This won't compile - unused type parameter
struct MessageHandler<T> {
    config: Config,
    // T is not used - compiler error!
}
```

### The Solution: PhantomData

`PhantomData<T>` is a zero-sized type that tells the compiler "act as if this struct owns a T":

```rust
use std::marker::PhantomData;

struct MessageHandler<T> {
    config: Config,
    _phantom: PhantomData<T>,  // Zero-sized, no runtime cost
}
```

### Why Use PhantomData?

1. **Type Safety for Variance**
```rust
struct Channel<T> {
    name: String,
    _marker: PhantomData<T>,
}

let sms_channel: Channel<SmsMessage> = Channel { 
    name: "sms".to_string(), 
    _marker: PhantomData 
};
let email_channel: Channel<EmailMessage> = Channel { 
    name: "email".to_string(), 
    _marker: PhantomData 
};
// These are different types at compile time!
```

2. **Ownership Semantics**
```rust
// Act as if we own a T
struct Owner<T> {
    _marker: PhantomData<T>,
}

// Act as if we have a reference to T
struct RefUser<'a, T> {
    _marker: PhantomData<&'a T>,
}
```

3. **Send/Sync Traits**
```rust
// Make type !Send if T is not Send
struct NotSendIfTNotSend<T> {
    _marker: PhantomData<T>,
}
```

## Advanced Patterns in Textio

### Type-State Pattern

Use generics to encode state in the type system:

```rust
struct Draft;
struct Validated;
struct Sent;

struct Message<State> {
    content: String,
    recipient: String,
    _state: PhantomData<State>,
}

impl Message<Draft> {
    fn new(content: String, recipient: String) -> Self {
        Message {
            content,
            recipient,
            _state: PhantomData,
        }
    }
    
    fn validate(self) -> Result<Message<Validated>, String> {
        if self.recipient.starts_with('+') {
            Ok(Message {
                content: self.content,
                recipient: self.recipient,
                _state: PhantomData,
            })
        } else {
            Err("Invalid recipient".to_string())
        }
    }
}

impl Message<Validated> {
    fn send(self) -> Message<Sent> {
        println!("Sending to {}", self.recipient);
        Message {
            content: self.content,
            recipient: self.recipient,
            _state: PhantomData,
        }
    }
}

// Can only send validated messages!
let draft = Message::<Draft>::new("Hello".to_string(), "+1234567890".to_string());
let validated = draft.validate().unwrap();
let sent = validated.send();
```

### Generic Service Builder

```rust
struct ServiceBuilder<Config, Transport, Serializer> {
    config: Option<Config>,
    transport: Option<Transport>,
    serializer: Option<Serializer>,
}

impl<Config, Transport, Serializer> ServiceBuilder<Config, Transport, Serializer> {
    fn new() -> Self {
        ServiceBuilder {
            config: None,
            transport: None,
            serializer: None,
        }
    }
}

impl<Config, Transport, Serializer> ServiceBuilder<Config, Transport, Serializer> {
    fn with_config<C>(self, config: C) -> ServiceBuilder<C, Transport, Serializer> {
        ServiceBuilder {
            config: Some(config),
            transport: self.transport,
            serializer: self.serializer,
        }
    }
}
```

### Generic Event System

```rust
struct Event<Payload, Metadata> {
    id: u64,
    timestamp: u64,
    payload: Payload,
    metadata: Metadata,
}

trait EventHandler<Payload, Metadata> {
    fn handle(&self, event: &Event<Payload, Metadata>) -> Result<(), Error>;
}

struct EventBus<Payload, Metadata> {
    handlers: Vec<Box<dyn EventHandler<Payload, Metadata>>>,
}
```

## Higher-Ranked Trait Bounds (HRTBs)

For complex generic constraints involving lifetimes:

```rust
fn process<F>(f: F) 
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    // f works with any lifetime
}
```

## Generic Constraints with Associated Types

```rust
trait Database {
    type Error;
    type Connection;
    
    fn connect(&self) -> Result<Self::Connection, Self::Error>;
}

fn use_db<D: Database>(db: D) -> Result<(), D::Error> {
    let conn = db.connect()?;
    Ok(())
}
```

## Best Practices

1. **Use descriptive type parameter names** for complex generics:
```rust
// Instead of
struct Handler<A, B, C> { ... }

// Use
struct Handler<Input, Output, Config> { ... }
```

2. **Prefer where clauses** for bounds with more than 2 traits:
```rust
// Hard to read
fn foo<T: Clone + Debug + Send + Sync>(t: T) { }

// Better
fn foo<T>(t: T) 
where
    T: Clone + Debug + Send + Sync,
{ }
```

3. **Use PhantomData intentionally** - document why it's needed:
```rust
/// Uses PhantomData to enforce type-safe channel routing
struct Channel<T> {
    name: String,
    /// Marker to ensure type safety without runtime cost
    _type_marker: PhantomData<T>,
}
```

4. **Consider type inference** - provide type annotations where helpful:
```rust
// Let compiler infer
let result = process(data);

// Be explicit when needed
let result: Result<Success, Error> = process::<MyType>(data);
```

## Exercise Overview

In this exercise, you will:
1. Create a generic `KeyValue<K, V>` store with multiple type parameters
2. Build a type-state `Message<State>` using PhantomData
3. Implement a generic `Channel<Id, Message>` with constraints
4. Create a generic `Processor<Input, Output, Config>` with complex bounds
5. Build a type-safe builder pattern

## Key Takeaways

- Multiple type parameters allow independent type variations
- Where clauses improve readability for complex bounds
- PhantomData tracks types without storing values
- Type-state pattern encodes state in the type system
- Associated types can simplify generic interfaces
- HRTBs handle complex lifetime scenarios

## Conclusion

Advanced generics unlock powerful patterns for building flexible, type-safe abstractions. The combination of multiple type parameters, trait bounds, and PhantomData enables designs that would be impossible or unsafe in other languages. These patterns form the foundation of many Rust libraries and are essential for building robust systems like Textio's SMS API.

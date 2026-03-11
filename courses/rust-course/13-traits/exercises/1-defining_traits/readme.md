# Defining Traits in Rust

## Introduction

Traits are Rust's approach to shared behavior. They define functionality that multiple types can implement, similar to interfaces in other languages like Java or TypeScript. Understanding traits is essential for writing idiomatic Rust code and building extensible systems.

In the Textio SMS API, traits allow us to define common behaviors that different message types can share, making our code more flexible and reusable.

## What is a Trait?

A trait defines a set of methods that types must implement. It describes what a type can do, not what a type is. This distinction is crucial for understanding Rust's approach to polymorphism.

```rust
trait Describe {
    fn describe(&self) -> String;
}
```

This trait says: "Any type that implements `Describe` must provide a `describe` method that takes `&self` and returns a `String`."

## Defining a Trait

The basic syntax for defining a trait:

```rust
trait TraitName {
    fn method_name(&self) -> ReturnType;
    fn another_method(&self, param: Type) -> ReturnType;
}
```

### Example: Message Trait for Textio

```rust
trait Message {
    fn content(&self) -> &str;
    fn recipient(&self) -> &str;
    fn sender(&self) -> &str;
}
```

This trait defines what it means to be a "Message" in our SMS system. Any type that implements `Message` must provide these three methods.

## Implementing a Trait

To implement a trait for a type, use the `impl TraitName for TypeName` syntax:

```rust
struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

impl Message for SmsMessage {
    fn content(&self) -> &str {
        &self.body
    }
    
    fn recipient(&self) -> &str {
        &self.to
    }
    
    fn sender(&self) -> &str {
        &self.from
    }
}
```

### Implementing for Multiple Types

The same trait can be implemented for different types:

```rust
struct EmailMessage {
    email_to: String,
    email_from: String,
    subject: String,
    body: String,
}

impl Message for EmailMessage {
    fn content(&self) -> &str {
        &self.body
    }
    
    fn recipient(&self) -> &str {
        &self.email_to
    }
    
    fn sender(&self) -> &str {
        &self.email_from
    }
}
```

## Understanding `&self`

In trait methods, `&self` refers to the instance the method is called on:

```rust
trait Message {
    fn length(&self) -> usize {
        self.content().len()
    }
}
```

- `&self` - borrows the instance immutably
- `&mut self` - borrows the instance mutably
- `self` - takes ownership of the instance

### Borrowing Rules in Traits

```rust
trait Mutatable {
    fn update(&mut self, new_value: String);
    fn consume(self) -> String;
}

impl Mutatable for SmsMessage {
    fn update(&mut self, new_value: String) {
        self.body = new_value;
    }
    
    fn consume(self) -> String {
        self.body
    }
}
```

## Using Trait Methods

Once implemented, call methods on instances:

```rust
fn main() {
    let sms = SmsMessage {
        to: String::from("+1234567890"),
        from: String::from("+0987654321"),
        body: String::from("Hello from Textio!"),
    };
    
    println!("To: {}", sms.recipient());
    println!("From: {}", sms.sender());
    println!("Message: {}", sms.content());
}
```

## Associated Types

Traits can have associated types that implementing types must specify:

```rust
trait Container {
    type Item;
    
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
}

struct MessageQueue {
    messages: Vec<SmsMessage>,
}

impl Container for MessageQueue {
    type Item = SmsMessage;
    
    fn get(&self) -> Option<&Self::Item> {
        self.messages.first()
    }
    
    fn insert(&mut self, item: Self::Item) {
        self.messages.push(item);
    }
}
```

## Associated Constants

Traits can define constants that implementations must provide:

```rust
trait MessageLimits {
    const MAX_LENGTH: usize;
    const MAX_RECIPIENTS: usize;
    
    fn is_within_limits(&self) -> bool;
}

impl MessageLimits for SmsMessage {
    const MAX_LENGTH: usize = 160;
    const MAX_RECIPIENTS: usize = 100;
    
    fn is_within_limits(&self) -> bool {
        self.body.len() <= Self::MAX_LENGTH
    }
}
```

## Trait Visibility

Traits follow Rust's visibility rules:

```rust
pub trait PublicTrait {
    fn public_method(&self);
}

trait PrivateTrait {
    fn private_method(&self);
}
```

By default, traits are private to the module. Use `pub trait` to make them public.

## The Orphan Rule

You can implement a trait for a type only if either the trait or the type is defined in your crate. This prevents external crates from implementing external traits for external types.

```rust
// This works: our trait, external type
impl MyTrait for Vec<String> { }

// This works: external trait, our type
impl Display for MyStruct { }

// This fails: external trait, external type
// impl Display for Vec<String> { }
```

## Multiple Trait Implementations

A type can implement multiple traits:

```rust
trait Sendable {
    fn send(&self) -> Result<(), String>;
}

trait Trackable {
    fn tracking_id(&self) -> String;
}

struct PriorityMessage {
    // fields...
}

impl Sendable for PriorityMessage { }
impl Trackable for PriorityMessage { }
impl Message for PriorityMessage { }
```

## Common Patterns in Textio

### Message Validation

```rust
trait Validatable {
    fn validate(&self) -> Result<(), ValidationError>;
}

impl Validatable for SmsMessage {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.to.is_empty() {
            return Err(ValidationError::EmptyRecipient);
        }
        if self.body.len() > 160 {
            return Err(ValidationError::TooLong);
        }
        Ok(())
    }
}
```

### Message Formatting

```rust
trait Formattable {
    fn format(&self) -> String;
    fn format_with_header(&self, header: &str) -> String;
}
```

## Exercise Overview

In this exercise, you will:

1. Define a `Message` trait with methods for accessing message properties
2. Implement the trait for `SmsMessage` and `MmsMessage` types
3. Create a `StatusReport` trait for tracking message delivery
4. Use `&self` and `&mut self` appropriately
5. Work with associated types in traits

## Key Takeaways

- Traits define shared behavior across types
- Use `impl Trait for Type` to implement traits
- `&self` provides immutable access to the instance
- Types can implement multiple traits
- Associated types allow flexible trait definitions
- The orphan rule ensures coherent implementations

## Next Steps

After mastering trait definition and implementation, you'll learn about trait bounds and how to use traits with generic types to write more flexible and reusable code.

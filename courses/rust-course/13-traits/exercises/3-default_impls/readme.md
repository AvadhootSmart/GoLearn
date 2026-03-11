# Default Implementations in Traits

## Introduction

Default implementations allow traits to provide method bodies that implementing types can use or override. This reduces boilerplate and provides sensible default behavior while still allowing customization when needed.

In Textio, default implementations help standardize common operations like message formatting and validation while allowing specific message types to customize behavior.

## Basic Default Implementation

Traits can provide default method implementations:

```rust
trait Message {
    fn content(&self) -> &str;
    
    // Default implementation
    fn is_empty(&self) -> bool {
        self.content().is_empty()
    }
}
```

Types implementing `Message` get `is_empty` for free:

```rust
struct SmsMessage {
    body: String,
}

impl Message for SmsMessage {
    fn content(&self) -> &str {
        &self.body
    }
    // is_empty is automatically available!
}
```

## Overriding Default Implementations

Implementing types can override defaults:

```rust
struct MmsMessage {
    body: String,
    media_url: String,
}

impl Message for MmsMessage {
    fn content(&self) -> &str {
        &self.body
    }
    
    // Override: MMS is empty only if both body and media are empty
    fn is_empty(&self) -> bool {
        self.body.is_empty() && self.media_url.is_empty()
    }
}
```

## When to Use Default Implementations

1. **Common behavior** shared by most implementations
2. **Convenience methods** that combine other trait methods
3. **Sensible defaults** that work for most cases
4. **Reducing boilerplate** in implementations

## Default Methods Calling Required Methods

Default implementations often call required methods:

```rust
trait Serializable {
    // Required method
    fn to_json(&self) -> String;
    
    // Default implementation using to_json
    fn to_json_pretty(&self) -> String {
        let json = self.to_json();
        // Simple pretty printing logic
        format!("{{\n  {}\n}}", json)
    }
}
```

## Multiple Default Methods

Traits can have multiple default methods:

```rust
trait MessageHandler {
    fn message(&self) -> &str;
    
    fn log_message(&self) {
        println!("[LOG] {}", self.message());
    }
    
    fn log_with_prefix(&self, prefix: &str) {
        println!("[{}] {}", prefix, self.message());
    }
    
    fn message_length(&self) -> usize {
        self.message().len()
    }
}
```

## Default Implementations with Associated Types

```rust
trait Container {
    type Item;
    
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
    
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }
    
    fn last(&self) -> Option<&Self::Item> {
        if self.len() == 0 {
            None
        } else {
            self.get(self.len() - 1)
        }
    }
}
```

## Practical Textio Examples

### Message Formatting with Defaults

```rust
trait Formattable {
    fn raw_content(&self) -> &str;
    fn sender(&self) -> &str;
    
    fn format_simple(&self) -> String {
        format!("From: {} - {}", self.sender(), self.raw_content())
    }
    
    fn format_detailed(&self) -> String {
        format!(
            "Sender: {}\nMessage: {}\nLength: {}",
            self.sender(),
            self.raw_content(),
            self.raw_content().len()
        )
    }
}
```

### Validation with Defaults

```rust
trait Validatable {
    fn content(&self) -> &str;
    fn max_length(&self) -> usize;
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.content().is_empty() {
            return Err(ValidationError::Empty);
        }
        if self.content().len() > self.max_length() {
            return Err(ValidationError::TooLong);
        }
        Ok(())
    }
}
```

## Default Implementation Patterns

### Template Method Pattern

```rust
trait MessageProcessor {
    fn process(&self, content: &str) -> String;
    
    fn handle(&self, msg: &str) -> String {
        let preprocessed = self.preprocess(msg);
        let result = self.process(&preprocessed);
        self.postprocess(&result)
    }
    
    fn preprocess(&self, msg: &str) -> String {
        msg.trim().to_string()
    }
    
    fn postprocess(&self, result: &str) -> String {
        result.to_string()
    }
}
```

### Builder Pattern with Defaults

```rust
trait MessageBuilder {
    fn new() -> Self;
    fn set_content(&mut self, content: &str);
    fn set_recipient(&mut self, recipient: &str);
    fn build(self) -> String;
    
    fn with_content(mut self, content: &str) -> Self
    where
        Self: Sized,
    {
        self.set_content(content);
        self
    }
}
```

## Limitations of Default Implementations

1. **Cannot access struct fields** - only trait methods
2. **Cannot call methods from other traits** without bounds
3. **Cannot be abstract** - must provide implementation

```rust
trait Example {
    fn data(&self) -> &str;
    
    // This works - uses trait method
    fn length(&self) -> usize {
        self.data().len()
    }
    
    // This doesn't work - can't access fields
    // fn first_char(&self) -> char {
    //     self.data[0]  // Error!
    // }
}
```

## Supertraits with Default Methods

```rust
trait AdvancedMessage: Message {
    fn priority(&self) -> u8 {
        1  // Default priority
    }
    
    fn is_high_priority(&self) -> bool {
        self.priority() > 5
    }
}
```

## Exercise Overview

In this exercise, you will:

1. Create traits with default implementations
2. Override default methods in specific implementations
3. Build a serialization system with sensible defaults
4. Use default methods that call required methods
5. Implement the template method pattern

## Key Takeaways

- Default implementations reduce boilerplate
- Implementations can override defaults when needed
- Default methods can call required trait methods
- Use defaults for common behavior and convenience methods
- Override for type-specific customization

## Next Steps

After mastering default implementations, you'll learn about trait objects and dynamic dispatch for runtime polymorphism.

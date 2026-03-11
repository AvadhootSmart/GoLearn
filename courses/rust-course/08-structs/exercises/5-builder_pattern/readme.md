# Builder Pattern

The builder pattern is a design pattern for constructing complex objects step-by-step. It's especially useful when a struct has many optional fields or when construction requires validation.

## The Problem

Consider a Message struct with many optional fields:

```rust
struct Message {
    to: String,
    from: String,
    body: String,
    priority: Option<u8>,
    delay: Option<u32>,
    callback_url: Option<String>,
    metadata: Option<String>,
    scheduled_at: Option<u64>,
}

// Creating with all defaults - which are optional?
let msg = Message {
    to: String::from("+15550001"),
    from: String::from("+15550002"),
    body: String::from("Hello"),
    priority: None,
    delay: None,
    callback_url: None,
    metadata: None,
    scheduled_at: None,
};
```

This is verbose and error-prone. The builder pattern solves this.

## Basic Builder Pattern

```rust
struct Message {
    to: String,
    from: String,
    body: String,
}

struct MessageBuilder {
    to: Option<String>,
    from: Option<String>,
    body: Option<String>,
}

impl MessageBuilder {
    fn new() -> Self {
        MessageBuilder {
            to: None,
            from: None,
            body: None,
        }
    }
    
    fn to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }
    
    fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
    
    fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    
    fn build(self) -> Result<Message, String> {
        let to = self.to.ok_or("to is required")?;
        let from = self.from.ok_or("from is required")?;
        let body = self.body.ok_or("body is required")?;
        
        Ok(Message { to, from, body })
    }
}

// Usage:
let msg = MessageBuilder::new()
    .to(String::from("+15550001"))
    .from(String::from("+15550002"))
    .body(String::from("Hello"))
    .build()
    .unwrap();
```

## Key Components

1. **Builder Struct** - Holds optional versions of all fields
2. **Constructor** - `new()` creates an empty builder
3. **Setter Methods** - Each returns `Self` for chaining
4. **Build Method** - Validates and creates the final struct

## Method Chaining

Setters return `Self` to enable chaining:

```rust
fn to(mut self, to: String) -> Self {
    self.to = Some(to);
    self  // Return self for chaining
}

// Without chaining:
let mut builder = MessageBuilder::new();
builder = builder.to(String::from("+15550001"));
builder = builder.from(String::from("+15550002"));
let msg = builder.build();

// With chaining:
let msg = MessageBuilder::new()
    .to(String::from("+15550001"))
    .from(String::from("+15550002"))
    .body(String::from("Hello"))
    .build();
```

## Validation in Build

The `build()` method can validate all fields:

```rust
fn build(self) -> Result<Message, String> {
    let to = self.to.ok_or("Missing 'to' field")?;
    let from = self.from.ok_or("Missing 'from' field")?;
    let body = self.body.ok_or("Missing 'body' field")?;
    
    if !to.starts_with('+') {
        return Err("'to' must start with '+'".to_string());
    }
    
    if body.len() > 160 {
        return Err("Body exceeds 160 characters".to_string());
    }
    
    Ok(Message { to, from, body })
}
```

## Default Values

Set defaults for optional fields:

```rust
fn build(self) -> Message {
    Message {
        to: self.to.unwrap_or_default(),
        from: self.from.unwrap_or(String::from("Textio")),
        body: self.body.unwrap_or_default(),
        priority: self.priority.unwrap_or(1),
    }
}
```

## Required vs Optional Fields

Distinguish between required and optional:

```rust
struct MessageBuilder {
    // Required - must be set
    to: Option<String>,
    body: Option<String>,
    // Optional - have defaults
    from: Option<String>,
    priority: Option<u8>,
}

impl MessageBuilder {
    // Start with required fields
    fn new(to: String, body: String) -> Self {
        MessageBuilder {
            to: Some(to),
            body: Some(body),
            from: None,
            priority: None,
        }
    }
    
    // Optional setters
    fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
    
    fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }
}

// Required fields in constructor, optional via chaining
let msg = MessageBuilder::new(
    String::from("+15550001"),
    String::from("Hello"),
)
.from(String::from("+15550002"))
.priority(3)
.build();
```

## Type-State Pattern (Advanced)

Enforce required fields at compile time:

```rust
struct Builder<T> {
    to: Option<String>,
    from: Option<String>,
    body: Option<String>,
    _state: PhantomData<T>,
}

struct NoTo;
struct HasTo;

impl Builder<NoTo> {
    fn new() -> Self {
        Builder {
            to: None,
            from: None,
            body: None,
            _state: PhantomData,
        }
    }
    
    fn to(self, to: String) -> Builder<HasTo> {
        Builder {
            to: Some(to),
            from: self.from,
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl Builder<HasTo> {
    fn build(self) -> Message {
        Message {
            to: self.to.unwrap(),
            from: self.from.unwrap_or_default(),
            body: self.body.unwrap_or_default(),
        }
    }
}
```

This prevents calling `build()` without setting `to`.

## When to Use Builder Pattern

Use it when:
- Many optional fields
- Complex validation needed
- Readable construction is important
- Construction is multi-step

Don't use it when:
- Struct has few fields (2-3)
- All fields are required
- Simple construction is fine

## Alternative: `..Default::default()`

For simpler cases, use `Default`:

```rust
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
}

let config = Config {
    host: String::from("localhost"),
    ..Default::default()
};
```

## Exercise: Textio Message Builder

In this exercise, you'll implement a builder for Textio's Message struct with optional fields and validation.

### Tasks

1. Create a MessageBuilder struct
2. Implement setter methods with chaining
3. Implement build() with validation
4. Handle required vs optional fields
5. Set default values for optional fields

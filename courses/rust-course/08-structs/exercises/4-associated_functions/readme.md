# Associated Functions

Associated functions are functions defined within an `impl` block that don't take `self` as a parameter. They're called on the type itself, not on an instance.

## Methods vs Associated Functions

```rust
struct Message {
    to: String,
    body: String,
}

impl Message {
    // Method - takes &self, called on instance
    fn length(&self) -> usize {
        self.body.len()
    }
    
    // Associated function - no self, called on type
    fn new(to: String, body: String) -> Message {
        Message { to, body }
    }
}

// Different call syntax:
let msg = Message::new(String::from("+15550001"), String::from("Hello"));  // Type::
let len = msg.length();  // instance.method()
```

## The `new` Constructor

The most common associated function is `new()`:

```rust
impl Message {
    fn new(to: String, from: String, body: String) -> Message {
        Message {
            to,
            from,
            body,
            delivered: false,
            attempts: 0,
        }
    }
}

let msg = Message::new(
    String::from("+15550001"),
    String::from("+15550002"),
    String::from("Hello"),
);
```

Note: `new` isn't a keyword - it's just a convention.

## The `Self` Keyword

`Self` (capital S) refers to the type implementing the methods:

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {  // Self = Point
        Self { x, y }                  // Self = Point
    }
    
    fn origin() -> Self {              // Self = Point
        Self { x: 0.0, y: 0.0 }
    }
    
    fn distance(&self, other: &Self) -> f64 {  // Self = Point
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

Using `Self` makes refactoring easier - if you rename the struct, you only change one place.

## Multiple Constructors

You can have multiple associated functions for different construction scenarios:

```rust
impl Message {
    fn new(to: String, from: String, body: String) -> Self {
        Message {
            to,
            from,
            body,
            delivered: false,
            attempts: 0,
        }
    }
    
    fn empty() -> Self {
        Message {
            to: String::new(),
            from: String::new(),
            body: String::new(),
            delivered: false,
            attempts: 0,
        }
    }
    
    fn notification(to: String, body: String) -> Self {
        Self::new(to, String::from("Textio"), body)
    }
    
    fn from_json(json: &str) -> Option<Self> {
        // Parse JSON and return Message
        // Returns None if parsing fails
        None
    }
}
```

## Factory Pattern

Associated functions can create different variants:

```rust
struct Config {
    api_endpoint: String,
    timeout: u32,
    retries: u32,
}

impl Config {
    fn default() -> Self {
        Config {
            api_endpoint: String::from("https://api.textio.com"),
            timeout: 30,
            retries: 3,
        }
    }
    
    fn development() -> Self {
        Config {
            api_endpoint: String::from("http://localhost:8080"),
            timeout: 60,
            retries: 1,
        }
    }
    
    fn production() -> Self {
        Config {
            api_endpoint: String::from("https://api.textio.com"),
            timeout: 10,
            retries: 5,
        }
    }
}
```

## Common Associated Functions

### Default Values

```rust
impl User {
    fn guest() -> Self {
        User {
            username: String::from("guest"),
            email: String::new(),
            active: true,
        }
    }
}
```

### Parsing/Converting

```rust
impl PhoneNumber {
    fn parse(s: &str) -> Result<Self, String> {
        if s.starts_with('+') && s.len() >= 10 {
            Ok(PhoneNumber(s.to_string()))
        } else {
            Err(format!("Invalid phone number: {}", s))
        }
    }
}
```

### From Other Types

```rust
impl Message {
    fn from_notification(notification: Notification) -> Self {
        Message {
            to: notification.recipient,
            from: String::from("Textio"),
            body: notification.text,
            delivered: false,
            attempts: 0,
        }
    }
}
```

## Naming Conventions

| Purpose | Convention | Example |
|---------|------------|---------|
| General constructor | `new` | `Message::new()` |
| Default instance | `default` | `Config::default()` |
| Parse from string | `from_str` or `parse` | `PhoneNumber::parse()` |
| Convert from type | `from_` prefix | `Message::from_bytes()` |
| Create empty | `empty` or `new` | `String::new()` |
| Specialized | descriptive name | `Config::development()` |

## Associated Constants

You can also define constants in `impl` blocks:

```rust
struct Limits {
    max_length: usize,
}

impl Limits {
    const MAX_MESSAGE_LENGTH: usize = 160;
    const MAX_RETRIES: u8 = 3;
    const DEFAULT_TIMEOUT: u32 = 30;
    
    fn default() -> Self {
        Limits {
            max_length: Self::MAX_MESSAGE_LENGTH,
        }
    }
}

println!("Max length: {}", Limits::MAX_MESSAGE_LENGTH);
```

## Combining Methods and Associated Functions

Most types have both:

```rust
impl Message {
    // Associated functions - called on type
    fn new(to: String, from: String, body: String) -> Self {
        Self { to, from, body, delivered: false, attempts: 0 }
    }
    
    fn empty() -> Self {
        Self::new(String::new(), String::new(), String::new())
    }
    
    // Methods - called on instance
    fn send(&mut self) -> bool {
        self.attempts += 1;
        self.delivered = true;
        true
    }
    
    fn is_sent(&self) -> bool {
        self.delivered
    }
}
```

## When to Use Each

**Associated Functions:**
- Creating new instances
- Parsing/converting from other types
- Factory methods
- Constants

**Methods:**
- Operating on instance data
- Querying state
- Modifying state
- Converting instance to another type

## Exercise: Textio Constructors

In this exercise, you'll implement associated functions for Textio's Message struct. You'll create multiple constructors, use the `Self` keyword, and implement factory methods.

### Tasks

1. Implement a `new()` constructor
2. Use `Self` throughout the impl block
3. Create multiple constructor variants
4. Implement parsing/conversion associated functions
5. Add associated constants

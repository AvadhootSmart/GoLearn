# Methods

Methods are functions defined within the context of a struct (or enum/trait). They have access to the struct's data through the `self` parameter.

## Defining Methods

Methods are defined in `impl` blocks:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 10, height: 20 };
println!("Area: {}", rect.area());
```

Key elements:
- `impl` keyword starts the implementation block
- `&self` is shorthand for `self: &Self`
- Methods are called with dot notation: `instance.method()`

## The self Parameter

There are three variants of `self`:

### 1. `&self` - Immutable Borrow

Read data without taking ownership:

```rust
impl Message {
    fn is_long(&self) -> bool {
        self.body.len() > 160
    }
    
    fn recipient(&self) -> &str {
        &self.to
    }
}
```

Use `&self` when you only need to read data.

### 2. `&mut self` - Mutable Borrow

Modify data without taking ownership:

```rust
impl Message {
    fn mark_delivered(&mut self) {
        self.delivered = true;
    }
    
    fn append_signature(&mut self, signature: &str) {
        self.body.push_str(signature);
    }
}
```

Use `&mut self` when you need to modify the struct's data.

### 3. `self` - Takes Ownership

Consume the struct:

```rust
impl Message {
    fn into_body(self) -> String {
        self.body  // Transfer ownership of body
    }
}
```

Use `self` when the method transforms the struct into something else and the original shouldn't be used afterward.

## Method vs Function

Methods differ from regular functions:

```rust
// Regular function - data passed as parameter
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
area(&rect);

// Method - called on instance
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
rect.area();
```

Benefits of methods:
- Better organization (functionality grouped with data)
- Clearer call site (`.area()` vs `area(&rect)`)
- Can use `self` shorthand
- Automatic referencing (Rust adds `&` or `&mut` as needed)

## Automatic Referencing

Rust automatically adds `&`, `&mut`, or `*` to match the method signature:

```rust
impl Rectangle {
    fn area(&self) -> u32 { /* ... */ }
    fn scale(&mut self, factor: u32) { /* ... */ }
}

let rect = Rectangle { width: 10, height: 20 };
let mut rect2 = Rectangle { width: 5, height: 5 };

// These are equivalent:
rect.area();
(&rect).area();  // Rust adds & automatically

rect2.scale(2);
(&mut rect2).scale(2);  // Rust adds &mut automatically
```

## Multiple impl Blocks

You can have multiple `impl` blocks for the same struct:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

This is useful when:
- Implementing multiple traits
- Organizing code in large files
- Generated code (macros) adds methods

## Methods with Parameters

Methods can take additional parameters:

```rust
impl Message {
    fn send_to(&self, phone: &str) -> bool {
        // Simulate sending
        println!("Sending to: {}", phone);
        true
    }
    
    fn with_priority(&self, level: u8) -> PriorityMessage {
        PriorityMessage {
            message: self.clone(),
            priority: level,
        }
    }
}
```

## Getter Methods

Create read-only access to private fields:

```rust
pub struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
}
```

This is the "getter" pattern - provides controlled read access.

## Method Chaining

Methods that return `Self` enable chaining:

```rust
impl MessageBuilder {
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
}

let msg = MessageBuilder::new()
    .to(String::from("+15550001"))
    .from(String::from("+15550002"))
    .body(String::from("Hello"))
    .build();
```

## Common Patterns in Textio

### Validation Methods

```rust
impl PhoneNumber {
    fn is_valid(&self) -> bool {
        self.0.starts_with('+') && self.0.len() >= 10
    }
}
```

### State Modification

```rust
impl Message {
    fn send(&mut self) -> Result<(), String> {
        if self.sent {
            return Err("Already sent".to_string());
        }
        self.sent = true;
        Ok(())
    }
}
```

### Transformation Methods

```rust
impl Message {
    fn to_json(&self) -> String {
        format!(
            r#"{{"to":"{}","from":"{}","body":"{}"}}"#,
            self.to, self.from, self.body
        )
    }
}
```

## Best Practices

1. **Use `&self` by default** - Only use `&mut self` or `self` when needed
2. **Name getters after the field** - `fn username(&self)` not `fn get_username(&self)`
3. **Name boolean getters with `is_`, `has_`, etc.** - `fn is_sent(&self)`
4. **Keep methods focused** - Each method should do one thing
5. **Consider ergonomics** - Think about how the method will be called

## Exercise: Textio Message Methods

In this exercise, you'll implement methods for Textio's Message struct. You'll use `&self`, `&mut self`, and create methods for validation, modification, and querying.

### Tasks

1. Create `impl` blocks for the Message struct
2. Implement `&self` methods for querying data
3. Implement `&mut self` methods for modifying data
4. Create methods with additional parameters
5. Use multiple `impl` blocks

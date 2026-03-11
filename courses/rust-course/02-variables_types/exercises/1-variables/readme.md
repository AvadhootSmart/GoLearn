# Exercise 1: Variable Declaration

## Why This Concept Exists

In Rust, variables are the fundamental building blocks for storing and manipulating data. Unlike some languages where variables are just named memory locations, Rust's variable system is designed with three core principles in mind:

1. **Safety by Default**: Variables are immutable by default, preventing accidental modifications that lead to bugs
2. **Explicit Intent**: The programmer must explicitly declare when a variable needs to change
3. **Type Safety**: Every variable has a known type at compile time, catching errors before your code runs

For the Textio SMS API project, variables will store everything from message content to phone numbers, delivery statuses to user preferences. Understanding how to declare variables properly is essential for building reliable software.

---

## Basic Variable Declaration

Variables in Rust are declared using the `let` keyword:

```rust
fn main() {
    // Basic variable declaration
    let message = "Hello, Textio!";
    
    // This creates a variable named 'message'
    // Rust infers the type as &str (string slice)
    println!("{}", message);
}
```

### With Explicit Type Annotations

You can explicitly specify the type of a variable using a colon followed by the type:

```rust
fn main() {
    // Explicit type annotation
    let message_count: i32 = 42;
    
    // The type annotation tells Rust exactly what type to use
    // This is useful when the type might be ambiguous
    println!("Messages sent: {}", message_count);
}
```

### Type Inference vs Explicit Types

Rust has powerful type inference, but sometimes explicit annotations are needed:

```rust
fn main() {
    // Type inference - Rust figures out the type
    let count = 100;           // inferred as i32
    let price = 9.99;          // inferred as f64
    let is_active = true;      // inferred as bool
    let initial = 'A';         // inferred as char
    
    // Explicit types - when you need control
    let small_number: u8 = 255;        // unsigned 8-bit
    let precise_price: f32 = 9.99;     // 32-bit float
    let big_count: i64 = 1000000000;   // 64-bit integer
    
    // When inference needs help
    let numbers: Vec<i32> = Vec::new();  // empty vector needs type
}
```

---

## Variable Naming Rules

Rust follows specific naming conventions:

```rust
fn main() {
    // Valid variable names
    let message = "Hello";
    let message_count = 10;
    let isDelivered = false;      // snake_case is preferred
    let _private = "internal";    // underscore prefix for unused
    let _ = "ignored";            // underscore alone for truly unused
    
    // Invalid names (will cause compile errors):
    // let 2fast = true;          // can't start with number
    // let my-var = 10;           // hyphens not allowed
    // let fn = "test";           // reserved keywords
    
    // Preferred Rust naming: snake_case
    let message_content = "Hello, Textio!";
    let phone_number = "+1234567890";
    let is_delivered = false;
}
```

---

## Multiple Variable Declaration

You can declare multiple variables in various ways:

```rust
fn main() {
    // Multiple declarations
    let x = 1;
    let y = 2;
    let z = 3;
    
    // Tuple destructuring (covered in later modules)
    let (width, height) = (100, 200);
    
    // Chaining declarations
    let a = 1;
    let b = a + 1;
    let c = b + 1;
    
    println!("a={}, b={}, c={}", a, b, c);
}
```

---

## Mental Model: How Variables Work

Think of a variable as a labeled box in a warehouse:

```
┌─────────────────────────────────────────────────────────┐
│  MEMORY (The Warehouse)                                │
│                                                         │
│   ┌──────────────┐                                      │
│   │    "Hello"   │  ← Box labeled "message"             │
│   └──────────────┘    Contains string data              │
│                                                         │
│   ┌──────────────┐                                      │
│   │     42       │  ← Box labeled "count"              │
│   └──────────────┘    Contains integer i32              │
│                                                         │
│   ┌──────────────┐                                      │
│   │    true       │  ← Box labeled "is_sent"           │
│   └──────────────┘    Contains boolean                  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Key Points:**
- The variable name is the label on the box
- The value is what's inside the box
- The type determines the size and shape of the box
- Once placed, the contents cannot change (unless you use `mut`)

---

## Common Pitfalls

### Pitfall 1: Forgetting `let` Keyword

```rust
fn main() {
    // WRONG - This will not compile
    // message = "Hello";
    
    // CORRECT - Always use 'let' for declaration
    let message = "Hello";
}
```

**Error Message:**
```
error[E0425]: cannot find value `message` in this scope
```

### Pitfall 2: Using Reserved Keywords as Names

```rust
fn main() {
    // WRONG - These are reserved keywords
    // let fn = "test";      // 'fn' is a keyword
    // let let = 10;         // 'let' is a keyword
    // let match = true;     // 'match' is a keyword
    
    // CORRECT - Use descriptive, non-keyword names
    let function_name = "test";
    let count = 10;
    let is_matched = true;
}
```

### Pitfall 3: Type Mismatch in Annotations

```rust
fn main() {
    // WRONG - Type doesn't match value
    // let number: i32 = "hello";  // i32 expected, &str provided
    
    // CORRECT - Type matches value
    let number: i32 = 42;
    let text: &str = "hello";
}
```

**Error Message:**
```
error[E0308]: mismatched types
expected `i32`, found `&str`
```

### Pitfall 4: Unused Variables

```rust
fn main() {
    // WARNING - Variable declared but never used
    let unused_variable = "I'm not used";
    
    // CORRECT - Either use it or prefix with underscore
    let _unused_variable = "Intentionally unused";
    
    // Or actually use it
    let used_variable = "I'm used";
    println!("{}", used_variable);
}
```

**Warning Message:**
```
warning: unused variable: `unused_variable`
```

---

## Under the Hood

### Memory Allocation

When you declare a variable, Rust allocates memory based on the type:

```rust
fn main() {
    // Stack-allocated (fixed size, fast)
    let number: i32 = 42;        // 4 bytes on stack
    let flag: bool = true;        // 1 byte on stack
    let letter: char = 'A';       // 4 bytes on stack (Unicode)
    
    // The compiler knows exact sizes at compile time
    // This enables optimizations and memory safety
}
```

### Compile-Time Type Checking

Rust performs all type checking at compile time:

```rust
fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    
    // This is checked at compile time
    // The compiler ensures both operands are i32
    let sum = x + y;  // sum is inferred as i32
    
    // This would fail at compile time:
    // let text: &str = "hello";
    // let result = x + text;  // error: cannot add i32 and &str
}
```

### Variable Scope

Variables exist only within their scope:

```rust
fn main() {
    // 'message' doesn't exist yet
    
    {  // New scope begins
        let message = "Hello";
        println!("{}", message);  // 'message' exists here
    }  // 'message' goes out of scope, memory freed
    
    // 'message' no longer exists
    // println!("{}", message);  // Error: not found in scope
}
```

---

## Real-World Textio Examples

### Storing Message Data

```rust
fn main() {
    // SMS message components
    let sender = "+1234567890";
    let recipient = "+0987654321";
    let message_body = "Your verification code is 123456";
    let message_length = 32;
    let is_delivered = false;
    
    println!("From: {}", sender);
    println!("To: {}", recipient);
    println!("Message: {}", message_body);
    println!("Length: {} characters", message_length);
}
```

### Configuration Values

```rust
fn main() {
    // Textio API configuration
    let api_version: &str = "v2";
    let max_message_length: i32 = 160;
    let rate_limit: i32 = 1000;
    let enable_logging: bool = true;
    
    println!("Textio API {}", api_version);
    println!("Max message length: {}", max_message_length);
    println!("Rate limit: {} requests/hour", rate_limit);
}
```

### User Account Information

```rust
fn main() {
    // User account details
    let user_id: i64 = 12345678901;
    let username: &str = "textio_user";
    let account_balance: f64 = 150.50;
    let is_premium: bool = true;
    
    println!("User: {} (ID: {})", username, user_id);
    println!("Balance: ${}", account_balance);
    println!("Premium: {}", is_premium);
}
```

---

## Debug Printing

Rust provides powerful debugging tools:

```rust
fn main() {
    let message = "Hello, Textio!";
    let count = 42;
    
    // Debug formatting with {:?}
    println!("Debug: {:?}", message);
    println!("Debug: {:?}", count);
    
    // dbg! macro - prints to stderr with file/line info
    dbg!(message);
    dbg!(count);
    
    // dbg! returns the value, so you can use it inline
    let result = dbg!(count * 2);
    println!("Result: {}", result);
}
```

---

## Exercise Task

Create a program for Textio that demonstrates variable declaration:

1. Create variables for an SMS message:
   - `sender_phone` - a phone number as a string
   - `recipient_phone` - a phone number as a string  
   - `message_content` - the SMS text
   - `character_count` - length of message (explicit i32)
   - `is_delivered` - delivery status (boolean)

2. Create variables for API configuration:
   - `api_endpoint` - the API URL
   - `max_length` - maximum message length (explicit u16)
   - `timeout_seconds` - request timeout (explicit u32)

3. Print all variables using both regular `println!` and `dbg!` macro

4. Demonstrate type inference vs explicit types with comments

**Starter Code**: See `code.rs`
**Solution**: See `complete.rs`
**Expected Output**: See `expected.txt`

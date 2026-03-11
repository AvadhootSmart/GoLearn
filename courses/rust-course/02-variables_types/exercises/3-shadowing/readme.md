# Exercise 3: Variable Shadowing

## Why This Concept Exists

Variable shadowing is a unique Rust feature that allows you to declare a new variable with the same name as a previous one. The new variable "shadows" the old one. This exists for several important reasons:

1. **Type Transformations**: Transform values while keeping meaningful names (e.g., parse string to number)
2. **Immutability After Processing**: Process data mutably, then "lock" it as immutable
3. **Name Reuse**: Use the same name for conceptually the same data in different forms
4. **Scope-Based Cleanup**: Temporarily modify values within nested scopes

For Textio, shadowing is useful when transforming message data, parsing user input, or processing API responses.

---

## What is Shadowing?

Shadowing occurs when you declare a new variable with the same name as an existing one:

```rust
fn main() {
    let x = 5;
    println!("First x: {}", x);
    
    // This SHADOWS the first x
    let x = x + 1;
    println!("Second x: {}", x);
    
    // This SHADOWS the second x
    let x = x * 2;
    println!("Third x: {}", x);
}
```

**Output:**
```
First x: 5
Second x: 6
Third x: 12
```

### Key Difference from Mutation

```rust
fn main() {
    // SHADOWING - Creates new variable, can change type
    let value = "42";       // &str
    let value = value.parse::<i32>().unwrap();  // i32
    let value = value * 2;  // Still i32
    println!("Value: {}", value);
    
    // MUTATION - Modifies existing variable, same type only
    let mut count = 5;      // i32
    count = count + 1;      // Still i32
    // count = "hello";     // ERROR: Can't change type!
}
```

---

## Type Transformation with Shadowing

One of the most powerful uses of shadowing is transforming types:

```rust
fn main() {
    // Start with string input
    let user_input = "42";
    println!("Input type: &str, value: {}", user_input);
    
    // Shadow with parsed integer
    let user_input: i32 = user_input.parse().unwrap();
    println!("Parsed type: i32, value: {}", user_input);
    
    // Shadow with different computation
    let user_input = user_input * 10;
    println!("Multiplied: {}", user_input);
}
```

### Textio Example: Parsing Phone Numbers

```rust
fn main() {
    // Raw input from API
    let phone = "+1-555-123-4567";
    println!("Raw phone: {}", phone);
    
    // Shadow with cleaned version (remove dashes)
    let phone = phone.replace("-", "");
    println!("Cleaned: {}", phone);
    
    // Shadow with formatted version
    let phone = format!("+{}", phone.trim_start_matches('+'));
    println!("Formatted: {}", phone);
}
```

---

## Shadowing vs Mutation

Understanding when to use each is crucial:

### Use Shadowing When:

```rust
fn main() {
    // 1. Type needs to change
    let data = "100";           // &str
    let data: i32 = data.parse().unwrap();  // i32
    
    // 2. Value should be immutable after transformation
    let mut temp = String::new();
    temp.push_str("Hello");
    temp.push_str(", World!");
    let message = temp;  // Now immutable
    
    // 3. Conceptually same data, different representation
    let bytes = "hello";
    let bytes = bytes.as_bytes();  // &[u8]
}
```

### Use Mutation When:

```rust
fn main() {
    // 1. Accumulating or modifying same type
    let mut count = 0;
    count = count + 1;
    count = count + 1;
    
    // 2. State that genuinely changes
    let mut status = "pending";
    status = "processing";
    status = "complete";
    
    // 3. Performance-critical in-place modification
    let mut buffer = String::with_capacity(100);
    buffer.push_str("data");
}
```

---

## Mental Model: New Boxes, Same Label

Think of shadowing as creating a new box with the same label:

```
First Declaration:
┌──────────────┐
│    "42"      │  ← Box 1, labeled "value"
└──────────────┘

Shadowing (let value = ...):
┌──────────────┐
│    "42"      │  ← Box 1, now hidden
└──────────────┘
┌──────────────┐
│      42      │  ← Box 2, labeled "value" (shadows Box 1)
└──────────────┘

The old box still exists but is no longer accessible.
```

**Key Insight**: Shadowing creates a NEW variable. The old one becomes inaccessible but isn't modified.

---

## Scope-Based Shadowing

Shadowing can be temporary using scopes:

```rust
fn main() {
    let message = "Hello";
    println!("Outer: {}", message);
    
    {
        // This shadows the outer 'message'
        let message = "Goodbye";
        println!("Inner: {}", message);
    }
    
    // Back to original
    println!("Outer again: {}", message);
}
```

**Output:**
```
Outer: Hello
Inner: Goodbye
Outer again: Hello
```

### Textio Example: Temporary Processing

```rust
fn main() {
    let status = "idle";
    println!("Status: {}", status);
    
    {
        // Temporarily shadow for processing
        let status = "processing";
        println!("Processing: {}", status);
        // ... do work ...
    }
    
    // Original status restored
    println!("Status: {}", status);
}
```

---

## Common Pitfalls

### Pitfall 1: Accidental Shadowing

```rust
fn main() {
    let count = 10;
    
    // Oops! Accidentally shadowing instead of modifying
    let count = count + 1;  // Creates new variable, old one hidden
    
    // This might not be what you intended
    println!("Count: {}", count);  // 11
    
    // If you meant to modify, use mut instead
    let mut total = 10;
    total = total + 1;  // Modifies existing variable
}
```

### Pitfall 2: Shadowing with Wrong Type Unintentionally

```rust
fn main() {
    let id = 12345;
    
    // Be careful with type changes
    let id = "12345";  // Now a string!
    
    // This might cause issues later
    // let doubled = id * 2;  // Error: can't multiply string
}
```

### Pitfall 3: Confusing Shadowing with Assignment

```rust
fn main() {
    let x = 5;
    
    // This is SHADOWING (new variable)
    let x = 10;
    
    // This would be ASSIGNMENT (needs mut)
    // x = 15;  // Error without mut
    
    // The difference:
    // - Shadowing: let x = ... (new variable)
    // - Assignment: x = ... (modify existing, needs mut)
}
```

### Pitfall 4: Shadowing in Wrong Scope

```rust
fn main() {
    let value = 100;
    
    {
        let value = 200;  // Shadows in inner scope
        println!("Inner: {}", value);  // 200
    }
    
    println!("Outer: {}", value);  // 100 (original restored)
    
    // If you wanted permanent change, use mut
    let mut value = 100;
    {
        value = 200;  // Modifies the same variable
    }
    println!("Modified: {}", value);  // 200
}
```

---

## Under the Hood

### Memory Allocation

```rust
fn main() {
    let x = 5;
    // Memory: [5] at address 0x1000
    
    let x = 10;
    // Memory: [5] at 0x1000 (still there, but inaccessible)
    //         [10] at 0x1004 (new allocation)
    
    // The old value may be freed when it goes out of scope
    // depending on the type and ownership rules
}
```

### Compiler Behavior

The compiler treats shadowed variables as completely separate:

```rust
fn main() {
    let x: i32 = 5;
    // Compiler: "x is i32 at location A"
    
    let x: &str = "hello";
    // Compiler: "x is &str at location B"
    // The old x is no longer accessible
    
    // These are two different variables to the compiler
}
```

### Debug Info Preservation

```rust
fn main() {
    let value = 100;
    dbg!(value);
    
    let value = "transformed";
    dbg!(value);
    
    // Debug output shows both shadowing events
}
```

---

## Real-World Textio Examples

### Parsing API Response

```rust
fn main() {
    // Raw JSON response
    let response = r#"{"status": "ok", "message_id": 12345}"#;
    println!("Raw: {}", response);
    
    // Shadow with parsed version (simplified)
    let status = "ok";
    let message_id = 12345;
    
    // Shadow with processed values
    let status = status.to_uppercase();
    let message_id = message_id * 1000;  // Convert to internal format
    
    println!("Status: {}", status);
    println!("Internal ID: {}", message_id);
}
```

### Message Processing Pipeline

```rust
fn main() {
    // Input message
    let message = "  Hello, Textio!  ";
    println!("Raw: '{}'", message);
    
    // Shadow with trimmed version
    let message = message.trim();
    println!("Trimmed: '{}'", message);
    
    // Shadow with lowercase
    let message = message.to_lowercase();
    println!("Lowercase: '{}'", message);
    
    // Shadow with character count
    let message = message.len();
    println!("Length: {}", message);
}
```

### User Input Validation

```rust
fn main() {
    // User input
    let phone = "555-123-4567";
    println!("Input: {}", phone);
    
    // Shadow with cleaned version
    let phone = phone.replace("-", "");
    println!("Cleaned: {}", phone);
    
    // Shadow with validation result
    let phone = if phone.len() == 10 {
        "valid"
    } else {
        "invalid"
    };
    println!("Status: {}", phone);
}
```

### Configuration Loading

```rust
fn main() {
    // Default config
    let max_retries = "3";
    let timeout = "30";
    
    // Shadow with parsed values
    let max_retries: i32 = max_retries.parse().unwrap();
    let timeout: i32 = timeout.parse().unwrap();
    
    // Shadow with environment override (simulated)
    let max_retries = 5;  // Override from env
    let timeout = 60;      // Override from env
    
    println!("Max retries: {}", max_retries);
    println!("Timeout: {}s", timeout);
}
```

---

## Exercise Task

Create a program that demonstrates shadowing in a Textio message processing pipeline:

1. Start with a raw message string
2. Shadow it with a trimmed version
3. Shadow it with an uppercase version
4. Shadow it with the character count (type changes to i32)
5. Demonstrate scope-based shadowing with a temporary status
6. Show the difference between shadowing and mutation

**Starter Code**: See `code.rs`
**Solution**: See `complete.rs`
**Expected Output**: See `expected.txt`

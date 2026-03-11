# Return Values in Rust

## Why This Concept Exists

Return values are how functions communicate results back to callers. They complete the input-output cycle: parameters bring data in, return values send data out. Without return values, functions could only perform side effects (like printing) and couldn't pass computed results to other code.

In Textio, return values let you:
- Get validation results (valid/invalid, error messages)
- Retrieve calculated costs and statistics
- Chain operations: one function's output becomes another's input
- Handle errors gracefully with `Result<T, E>`

Rust's approach to return values—particularly implicit returns through expressions—is unique and powerful.

---

## Basic Return Syntax

Use `->` followed by the type to declare a return type:

```rust
fn get_max_length() -> usize {
    160
}

fn calculate_tax(amount: f64) -> f64 {
    amount * 0.1
}
```

The last expression in the function body (without a semicolon) is the return value.

---

## Explicit vs Implicit Return

### Implicit Return (Idiomatic)

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon, no `return` keyword
}
```

### Explicit Return (Also valid)

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;  // Works, but less common for final return
}
```

The `return` keyword is typically reserved for early returns (exiting before the end).

---

## The Unit Type `()`

Functions without a specified return type return the unit type `()`:

```rust
fn log_message(msg: &str) {
    println!("[LOG] {}", msg);
    // Implicitly returns ()
}

fn log_message_explicit(msg: &str) -> () {
    println!("[LOG] {}", msg);
    // Explicitly returns ()
}
```

`()` is a type with exactly one value, also written `()`. It represents "no meaningful value" but is still a valid type.

---

## Multiple Return Paths

Functions can have multiple paths that return different values:

```rust
fn validate_length(text: &str, max: usize) -> bool {
    if text.len() <= max {
        true   // Early return via if expression
    } else {
        false
    }
}

fn get_status(code: u8) -> &'static str {
    match code {
        0 => "pending",
        1 => "sent",
        2 => "delivered",
        _ => "unknown",
    }
}
```

All return paths must return the same type:

```rust
fn broken(flag: bool) -> i32 {
    if flag {
        42
    } else {
        "error"  // ERROR: expected i32, found &str
    }
}
```

---

## Early Returns with `return`

Use `return` to exit a function before the end:

```rust
fn validate_phone(phone: &str) -> Result<String, String> {
    if phone.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }
    
    if phone.len() < 10 {
        return Err("Phone number too short".to_string());
    }
    
    if !phone.starts_with('+') {
        return Err("Phone number must start with +".to_string());
    }
    
    Ok(phone.to_string())
}
```

This pattern (guard clauses) is common in Rust for validation:

```rust
fn process_message(msg: &str) -> Result<String, String> {
    // Guard clauses first
    if msg.is_empty() {
        return Err("Message cannot be empty".to_string());
    }
    if msg.len() > 160 {
        return Err("Message too long".to_string());
    }
    
    // Main logic follows
    Ok(msg.trim().to_string())
}
```

---

## Returning Complex Types

### Tuples

```rust
fn parse_dimensions(input: &str) -> (usize, usize) {
    let parts: Vec<&str> = input.split('x').collect();
    let width = parts.get(0).unwrap().parse().unwrap_or(0);
    let height = parts.get(1).unwrap().parse().unwrap_or(0);
    (width, height)
}

fn main() {
    let (w, h) = parse_dimensions("160x600");
    println!("{} x {}", w, h);
}
```

### Option<T>

```rust
fn find_message(messages: &[&str], search: &str) -> Option<usize> {
    for (i, msg) in messages.iter().enumerate() {
        if msg.contains(search) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let msgs = ["Hello", "World", "Test"];
    match find_message(&msgs, "World") {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}
```

### Result<T, E>

```rust
fn parse_phone(input: &str) -> Result<String, String> {
    let cleaned: String = input.chars().filter(|c| c.is_numeric()).collect();
    
    if cleaned.len() < 10 {
        Err("Phone number too short".to_string())
    } else {
        Ok(format!("+{}", cleaned))
    }
}
```

---

## Returning References

When returning references, they must be tied to input references (lifetimes):

```rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn main() {
    let sentence = "Hello World";
    let first = first_word(sentence);
    println!("{}", first);  // "Hello"
}
```

You cannot return a reference to data created inside the function:

```rust
fn broken() -> &str {
    let s = String::from("Hello");
    &s  // ERROR: returns a reference to data owned by the function
}
```

Instead, return owned data:

```rust
fn works() -> String {
    let s = String::from("Hello");
    s  // Transfer ownership to caller
}
```

---

## Never Type `!`

Some functions never return:

```rust
fn panic_with_message(msg: &str) -> ! {
    panic!("{}", msg);
}

fn infinite_loop() -> ! {
    loop {
        // Runs forever
    }
}
```

The `!` type (never type) indicates the function will never complete normally. This is useful for type checking:

```rust
fn get_value(maybe: Option<i32>) -> i32 {
    match maybe {
        Some(v) => v,
        None => panic_with_message("No value"),  // Returns !, coerces to i32
    }
}
```

---

## Mental Model: Return as "Answer"

Think of a function call as a question and the return value as the answer:

```rust
fn is_valid_phone(phone: &str) -> bool { /* ... */ }

// Question: Is "+15551234567" a valid phone?
let answer = is_valid_phone("+15551234567");  // true
```

The return type tells you what kind of answer to expect:
- `bool` → Yes/No answer
- `Option<T>` → Maybe an answer, maybe nothing
- `Result<T, E>` → Either an answer or an explanation of failure
- `()` → No meaningful answer (just did something)

---

## Mental Model: The Expression Pipeline

Return values flow through expressions:

```rust
fn process(text: &str) -> String {
    text.trim()              // Returns &str
        .to_uppercase()      // Returns String
        .replace(" ", "_")   // Returns String
}                            // Final String is returned
```

Each step produces a value that becomes input to the next. The final value "falls out" of the function.

---

## Common Pitfalls

### Pitfall 1: Semicolon Changes Return Type

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;  // ERROR: expected i32, found ()
}

fn add_fixed(a: i32, b: i32) -> i32 {
    a + b   // No semicolon
}
```

### Pitfall 2: Missing Return Path

```rust
fn broken(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    // ERROR: not all control paths return a value
}

fn fixed(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    0  // Default return
}
```

### Pitfall 3: Returning Local Reference

```rust
fn broken() -> &str {
    let s = String::from("Hello");
    &s  // ERROR: cannot return reference to local variable
}

fn fixed() -> String {
    let s = String::from("Hello");
    s   // Move ownership instead
}
```

### Pitfall 4: Type Mismatch Across Paths

```rust
fn broken(flag: bool) -> String {
    if flag {
        String::from("yes")
    } else {
        "no"  // ERROR: expected String, found &str
    }
}

fn fixed(flag: bool) -> String {
    if flag {
        String::from("yes")
    } else {
        String::from("no")  // Both return String
    }
}
```

---

## Under the Hood: Return Value Optimization

Rust optimizes return values to avoid unnecessary copies:

### Return Value Optimization (RVO)

```rust
fn create_string() -> String {
    String::from("Hello")  // Constructed directly in caller's memory
}

fn main() {
    let s = create_string();  // No copy happens!
}
```

The compiler arranges for the return value to be constructed directly in the caller's stack frame, avoiding a copy.

### Named Return Value Optimization

```rust
fn create_large_struct() -> LargeStruct {
    let result = LargeStruct {
        // ... lots of fields
    };
    result  // Moved, not copied (optimization may eliminate the move)
}
```

---

## Under the Hood: Calling Convention for Returns

For small values, returns go through registers:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// Result placed in RAX register (x86-64)
```

For larger values, the caller allocates space and passes a pointer:

```rust
fn create_string() -> String {
    String::from("Hello")
}
// Caller reserves space, passes pointer to function
// Function constructs String in that space
```

---

## Real-World Use Cases in Textio

### 1. Validation with Result
```rust
fn validate_sms(recipient: &str, body: &str) -> Result<(), String> {
    if recipient.is_empty() {
        return Err("Recipient required".to_string());
    }
    if body.is_empty() {
        return Err("Message body required".to_string());
    }
    if body.len() > 1600 {
        return Err("Message exceeds 1600 characters".to_string());
    }
    Ok(())
}
```

### 2. Optional Lookup
```rust
fn find_user_by_phone(phone: &str, users: &[User]) -> Option<&User> {
    users.iter().find(|u| u.phone == phone)
}
```

### 3. Multiple Return Values
```rust
fn calculate_message_parts(body: &str) -> (usize, usize, bool) {
    let char_count = body.len();
    let segments = (char_count + 152) / 153;
    let is_unicode = body.chars().any(|c| c.len_utf8() > 1);
    (char_count, segments, is_unicode)
}
```

### 4. Early Return Pattern
```rust
fn send_sms(recipient: &str, body: &str) -> Result<MessageId, SmsError> {
    // Validation with early returns
    if recipient.is_empty() {
        return Err(SmsError::EmptyRecipient);
    }
    if body.is_empty() {
        return Err(SmsError::EmptyBody);
    }
    
    // Rate limiting check
    if !check_rate_limit(recipient)? {
        return Err(SmsError::RateLimited);
    }
    
    // Actual sending logic
    let message_id = submit_to_carrier(recipient, body)?;
    Ok(message_id)
}
```

### 5. Builder Pattern Return
```rust
struct SmsBuilder {
    recipient: Option<String>,
    body: Option<String>,
}

impl SmsBuilder {
    fn new() -> Self {
        Self { recipient: None, body: None }
    }
    
    fn recipient(mut self, r: &str) -> Self {
        self.recipient = Some(r.to_string());
        self  // Return self for chaining
    }
    
    fn body(mut self, b: &str) -> Self {
        self.body = Some(b.to_string());
        self
    }
    
    fn build(self) -> Result<Sms, String> {
        Ok(Sms {
            recipient: self.recipient.ok_or("recipient required")?,
            body: self.body.ok_or("body required")?,
        })
    }
}
```

---

## Exercise Task

Create functions demonstrating various return patterns:

1. `validate_sender_id(id: &str) -> Result<&'static str, &'static str>`
   - Must be 1-11 alphanumeric characters
   - Return Ok("valid") or Err with reason

2. `calculate_delivery_stats(messages: &[bool]) -> (u32, u32, f64)`
   - Input: slice of booleans (true = delivered, false = failed)
   - Return: (delivered_count, failed_count, success_rate_percentage)
   - Handle empty slice gracefully (0, 0, 0.0)

3. `find_undelivered(indices: &[usize], delivered: &[bool]) -> Vec<usize>`
   - Return indices from `indices` where corresponding `delivered` is false
   - Use early return if inputs have different lengths (return empty vec)

4. `format_cost_table(rates: &[(f64, f64)]) -> String`
   - Input: slice of (quantity, rate) tuples
   - Return formatted table with headers and rows
   - Include total at bottom
   - Use early return for empty input

Main should test all functions with values matching expected output.

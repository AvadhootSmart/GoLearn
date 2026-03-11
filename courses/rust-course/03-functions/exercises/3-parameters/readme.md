# Function Parameters in Rust

## Why This Concept Exists

Parameters are how functions receive input data. They define the "contract" between caller and function—what data the function needs and in what form. Well-designed parameters make functions:
- **Reusable**: Work with different inputs
- **Type-safe**: Compiler catches mismatches
- **Self-documenting**: Types explain expected data
- **Composable**: Functions can pass data to other functions

In Textio, parameters let you write a single `send_sms` function that works with any phone number and message, rather than hardcoding values.

---

## Basic Parameter Syntax

Parameters are declared in parentheses after the function name:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Textio User");  // Pass "Textio User" as the argument
}
```

Each parameter has:
1. A **name** (used inside the function)
2. A **type** (explicitly declared with `:`)

---

## Multiple Parameters

Multiple parameters are separated by commas:

```rust
fn send_sms(recipient: &str, message: &str, priority: u8) {
    println!("Sending to {}: {} (priority {})", recipient, message, priority);
}

fn calculate_cost(messages: u32, rate: f64, tax_rate: f64) -> f64 {
    let subtotal = messages as f64 * rate;
    subtotal * (1.0 + tax_rate)
}
```

Style convention: Put each parameter on its own line for long signatures:

```rust
fn send_bulk_sms(
    recipients: &[&str],
    message: &str,
    sender_id: &str,
    priority: u8,
    delivery_window_start: u32,
    delivery_window_end: u32,
) -> u32 {
    // Implementation
    recipients.len() as u32
}
```

---

## Type Annotations

Rust requires explicit type annotations for parameters—no type inference here:

```rust
// Correct - explicit types
fn process(amount: f64, count: usize) -> f64 {
    amount * count as f64
}

// ERROR - missing type annotations
fn process(amount, count) {  // Expected type for function parameter
    amount * count
}
```

This explicitness serves a purpose:
- Functions become their own documentation
- Public APIs are clear about expectations
- Compiler catches more errors at compile time

---

## Parameter Patterns

### Mutable Parameters

Parameters are immutable by default. Use `mut` to modify them:

```rust
fn append_country_code(phone: &mut String) {
    if !phone.starts_with('+') {
        phone.insert(0, '+');
    }
}

fn main() {
    let mut number = String::from("15551234567");
    append_country_code(&mut number);
    println!("{}", number);  // +15551234567
}
```

### Slice Parameters

For strings and arrays, take slices for flexibility:

```rust
// Takes any string-like reference
fn count_chars(text: &str) -> usize {
    text.len()
}

// Takes any array-like reference
fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    let s = String::from("Hello");
    count_chars(&s);      // Works with String reference
    count_chars("Hi");    // Works with &str literal
    
    let arr = [1, 2, 3, 4, 5];
    sum_numbers(&arr);    // Works with array reference
    sum_numbers(&arr[1..4]);  // Works with slice
}
```

### Ownership Patterns

Choose parameter types based on ownership needs:

```rust
// Borrows the string (doesn't take ownership)
fn print_length(s: &str) {
    println!("Length: {}", s.len());
}

// Takes ownership (caller loses access)
fn consume_string(s: String) -> usize {
    s.len()
}

// Can modify (unique borrow)
fn to_uppercase_in_place(s: &mut String) {
    *s = s.to_uppercase();
}
```

---

## Default Values (Pattern)

Rust doesn't have default parameter values, but you can simulate them:

```rust
fn send_sms_simple(recipient: &str, message: &str) {
    send_sms_full(recipient, message, 1, false);
}

fn send_sms_full(
    recipient: &str,
    message: &str,
    priority: u8,
    require_delivery_receipt: bool,
) {
    println!("Sending to {}: {} [priority: {}, receipt: {}]",
             recipient, message, priority, require_delivery_receipt);
}

fn main() {
    send_sms_simple("+15551234567", "Hello!");  // Uses defaults
    send_sms_full("+15551234567", "Hello!", 5, true);  // Full control
}
```

Or use the builder pattern for complex cases:

```rust
struct SmsBuilder<'a> {
    recipient: &'a str,
    message: &'a str,
    priority: u8,
}

impl<'a> SmsBuilder<'a> {
    fn new(recipient: &'a str, message: &'a str) -> Self {
        Self { recipient, message, priority: 1 }
    }
    
    fn priority(mut self, p: u8) -> Self {
        self.priority = p;
        self
    }
    
    fn send(&self) {
        println!("Sending to {}: {} [priority: {}]",
                 self.recipient, self.message, self.priority);
    }
}
```

---

## Variable Number of Arguments

Rust doesn't have variadic functions, but you can use slices:

```rust
fn sum_all(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn concat_strings(parts: &[&str]) -> String {
    parts.concat()
}

fn main() {
    sum_all(&[1, 2, 3]);           // 6
    sum_all(&[1, 2, 3, 4, 5]);     // 15
    
    concat_strings(&["Hello", " ", "World"]);  // "Hello World"
}
```

---

## Mental Model: Parameters as Named Slots

Think of parameters as labeled slots that must be filled:

```
fn send_sms(recipient: &str, message: &str, priority: u8)
            └─────┬─────┘  └────┬────┘  └───┬───┘
                  │             │           │
            Slot 1: must    Slot 2: must   Slot 3: must
            be &str         be &str        be u8
```

When you call the function:
```rust
send_sms("+15551234567", "Hello", 1);
         └─────┬─────┘   └─┬──┘  └┬┘
               │           │      │
         fills slot 1  fills    fills
                      slot 2   slot 3
```

The compiler ensures each slot gets the right type of data.

---

## Mental Model: Pass by Value vs Reference

```rust
// Pass by value (Copy types)
fn double(n: i32) -> i32 {
    n * 2  // n is a copy, original unchanged
}

// Pass by reference
fn first_char(s: &str) -> char {
    s.chars().next().unwrap_or(' ')
}

// Pass by mutable reference
fn capitalize(s: &mut String) {
    if let Some(first) = s.chars().next() {
        s.replace_range(0..1, &first.to_uppercase().to_string());
    }
}
```

---

## Common Pitfalls

### Pitfall 1: Missing Type Annotation

```rust
fn process(value) { }  // ERROR: expected type for function parameter

fn process(value: i32) { }  // Correct
```

### Pitfall 2: Wrong Type Passed

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet(42);  // ERROR: expected &str, found integer
}
```

### Pitfall 3: Ownership Confusion

```rust
fn take_string(s: String) {
    println!("{}", s);
}  // s is dropped here

fn main() {
    let my_string = String::from("Hello");
    take_string(my_string);
    println!("{}", my_string);  // ERROR: value borrowed after move
}
```

Fix: Borrow instead
```rust
fn borrow_string(s: &str) {
    println!("{}", s);
}

fn main() {
    let my_string = String::from("Hello");
    borrow_string(&my_string);
    println!("{}", my_string);  // OK!
}
```

### Pitfall 4: Mutable Parameter Not Marked

```rust
fn add_one(n: i32) {
    n = n + 1;  // ERROR: cannot assign twice to immutable variable
}

fn add_one_correct(n: &mut i32) {
    *n = *n + 1;  // OK - dereference and modify
}
```

---

## Under the Hood: Parameter Passing

How parameters are passed depends on their type:

### Copy Types (stack-allocated, small)
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// a and b are copied into the function's stack frame
```

### References (pointers)
```rust
fn len(s: &str) -> usize {
    s.len()
}
// Only a pointer (fat pointer: address + length) is passed
// The actual data stays where it is
```

### Ownership Transfer (move)
```rust
fn consume(v: Vec<i32>) {
    // v is moved here, original location is invalidated
}
// Ownership transfer is compile-time concept
// At runtime, only a pointer might be copied
```

---

## Under the Hood: ABI and Calling Convention

Rust uses the platform's C calling convention by default:

```rust
fn example(a: i32, b: i64, c: f64, d: i32) {
    // On x86-64:
    // a -> rdi register
    // b -> rsi register  
    // c -> xmm0 register (floating point)
    // d -> stack (ran out of registers)
}
```

For small functions, the compiler may inline them, eliminating parameter passing entirely:

```rust
#[inline]
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let y = square(5);  // May compile to just: mov y, 25
}
```

---

## Real-World Use Cases in Textio

### 1. Message Validation
```rust
fn validate_sms(recipient: &str, body: &str, max_length: usize) -> Result<(), String> {
    if recipient.is_empty() {
        return Err("Recipient is required".to_string());
    }
    if body.len() > max_length {
        return Err(format!("Message exceeds {} characters", max_length));
    }
    Ok(())
}
```

### 2. Cost Calculator
```rust
fn calculate_sms_cost(
    segments: u32,
    rate_per_segment: f64,
    international: bool,
    premium_sender: bool,
) -> f64 {
    let base = segments as f64 * rate_per_segment;
    let international_multiplier = if international { 1.5 } else { 1.0 };
    let premium_fee = if premium_sender { 0.01 } else { 0.0 };
    base * international_multiplier + premium_fee
}
```

### 3. Phone Number Parser
```rust
fn parse_phone_number(raw: &str, default_country: &str) -> String {
    let digits: String = raw.chars().filter(|c| c.is_numeric()).collect();
    if digits.starts_with('+') || digits.len() > 10 {
        digits
    } else {
        format!("{}{}", default_country, digits)
    }
}
```

### 4. Batch Processing
```rust
fn process_batch(messages: &[(&str, &str)], rate: f64) -> (u32, f64) {
    let count = messages.len() as u32;
    let cost = count as f64 * rate;
    (count, cost)
}
```

---

## Exercise Task

Create functions with various parameter patterns:

1. `format_phone_number(country_code: &str, number: &str) -> String`
   - Combines country code and number into format "+{code}{number}"
   - Ensure exactly one '+' at the start

2. `calculate_message_cost(segments: u32, rate: f64, is_international: bool, is_priority: bool) -> f64`
   - Base cost = segments * rate
   - International: multiply by 1.5
   - Priority: add $0.02 per segment
   - Return final cost

3. `truncate_message(message: &str, max_length: usize) -> &str`
   - Return a slice of message that fits within max_length
   - If message fits, return the whole thing
   - Use slice syntax

4. `validate_sms_request(recipient: &str, sender: &str, body: &str, max_segments: u8) -> Result<u8, String>`
   - Calculate segments needed (body length / 153, rounded up)
   - Validate: recipient not empty, sender not empty, segments <= max_segments
   - Return Ok(segments_needed) or Err with descriptive message

Main should demonstrate all functions with test cases matching expected output.

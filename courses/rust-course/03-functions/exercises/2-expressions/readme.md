# Expressions in Rust

## Why This Concept Exists

Rust is primarily an **expression-based language**, unlike C or Java which are statement-based. This distinction is fundamental to understanding idiomatic Rust and is one of the features that makes Rust code concise and elegant.

In a statement-based language, you write a series of commands. In an expression-based language, you compose values. This matters because:
- **Fewer intermediate variables**: Expressions can be chained directly
- **Implicit returns**: The last expression in a block is its value
- **Powerful control flow**: `if` is an expression, not just a statement
- **Functional patterns**: Map, filter, and chains work naturally

For Textio, this means message processing pipelines, cost calculations, and validation logic can be written as elegant expression chains rather than verbose statement sequences.

---

## Statements vs Expressions

### Statements
A **statement** performs an action but doesn't produce a value. It ends with a semicolon:

```rust
let x = 5;              // Statement - declares a variable
println!("Hello");      // Statement - prints to stdout
x = 10;                 // Statement - assigns a value
```

### Expressions
An **expression** evaluates to a value:

```rust
5                       // Expression - evaluates to 5
5 + 10                  // Expression - evaluates to 15
x                       // Expression - evaluates to x's value
x * 2                   // Expression - evaluates to x times 2
```

### The Key Insight

Any expression followed by a semicolon becomes a statement:

```rust
fn example() {
    5           // Expression that evaluates to 5
    ;           // Now it's a statement, returns ()
    
    5;          // Same thing on one line - statement returning ()
    
    let x = 5;  // Statement (the whole line)
                // But 5 by itself was an expression
}
```

---

## Implicit Returns

In Rust, the last expression in a function body (without a semicolon) is the return value:

```rust
fn get_max_sms_length() -> usize {
    160  // Expression - this IS the return value
}

fn get_default_country_code() -> &'static str {
    "+1"  // Expression - automatically returned
}
```

This is equivalent to:

```rust
fn get_max_sms_length() -> usize {
    return 160;  // Explicit return - works but less idiomatic
}
```

The semicolon makes all the difference:

```rust
fn works() -> i32 {
    42  // No semicolon - this is an expression, returns 42
}

fn broken() -> i32 {
    42; // With semicolon - this is a statement returning ()
    // ERROR: expected i32, found ()
}
```

---

## Block Expressions

A block `{ }` is an expression that can contain statements and ends with an optional final expression:

```rust
fn main() {
    let result = {  // This block is an expression
        let a = 10;
        let b = 20;
        a + b       // Final expression - no semicolon!
    };              // Semicolon here ends the statement
    
    println!("Result: {}", result);  // Prints: Result: 30
}
```

The block evaluates to its final expression. This is incredibly useful for scoped computations:

```rust
fn calculate_delivery_fee(distance_km: f64, urgent: bool) -> f64 {
    let base_fee = 2.0;
    
    let total = {
        let distance_fee = distance_km * 0.5;
        let urgency_multiplier = if urgent { 2.0 } else { 1.0 };
        (base_fee + distance_fee) * urgency_multiplier
    };
    
    total
}
```

---

## Expression Chaining

Because expressions produce values, you can chain them:

```rust
fn process_message(msg: &str) -> String {
    msg.trim()           // Returns &str
      .to_uppercase()    // Returns String
      .replace(" ", "_") // Returns String
}

fn validate_and_format(phone: &str) -> Option<String> {
    let cleaned: String = phone.chars().filter(|c| c.is_numeric()).collect();
    
    if cleaned.len() >= 10 {
        Some(format!("+{}", cleaned))
    } else {
        None
    }
}
```

---

## If as an Expression

Unlike many languages, `if` in Rust is an expression that produces a value:

```rust
fn get_rate(priority: &str) -> f64 {
    if priority == "high" {
        0.10  // No semicolon - this is the value
    } else if priority == "medium" {
        0.05
    } else {
        0.02
    }
}

fn format_status(delivered: bool) -> String {
    let status = if delivered { "DELIVERED" } else { "PENDING" };
    format!("Status: {}", status)
}
```

Both branches must return the same type:

```rust
fn broken(delim: bool) -> i32 {
    if delim {
        42
    } else {
        "hello"  // ERROR: expected i32, found &str
    }
}
```

---

## Match as an Expression

`match` is also an expression:

```rust
fn http_status_message(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status",
    }
}

fn categorize_message_length(len: usize) -> String {
    match len {
        0..=70 => "Single SMS".to_string(),
        71..=134 => "Double SMS".to_string(),
        _ => format!("Multi-part SMS ({} parts)", (len + 152) / 153),
    }
}
```

---

## Mental Model: The Semicolon Toggle

Think of the semicolon as a toggle switch:

```
EXPRESSION          → produces a value
EXPRESSION;         → produces () (unit), discards the value

{                   → block starts
    statement 1;    → does something, returns ()
    statement 2;    → does something, returns ()
    expression      → this becomes the block's value
}                   → block ends with that value
```

```rust
fn with_semicolon() {
    let x = 5;   // Statement
    x + 1;       // Statement (discards result)
    println!("Done");  // Statement
}   // Returns ()

fn without_semicolon() -> i32 {
    let x = 5;   // Statement
    let y = x + 1;  // Statement
    y + 10       // Expression - becomes the return value!
}   // Returns 16
```

---

## Mental Model: Everything Has a Value

In Rust, almost everything can be thought of as having a value:

```rust
let a = 5;           // `a` is 5, but the whole line is a statement
let b = { 10 };      // `b` is 10, the block evaluated to 10
let c = if true { 1 } else { 2 };  // `c` is 1, if evaluated to 1
let d = loop { break 42; };  // `d` is 42, loop broke with value 42
let e = match 5 { _ => 10 };  // `e` is 10, match evaluated to 10
```

Even blocks and scopes have values:

```rust
fn expensive_calculation() -> i32 {
    // Imagine this is a complex calculation
    42
}

fn main() {
    let result = {
        let temp = expensive_calculation();
        temp * 2
    };
    // `temp` is out of scope here
    println!("{}", result);
}
```

---

## Common Pitfalls

### Pitfall 1: Semicolon Changes Return Type

```rust
fn add_one(x: i32) -> i32 {
    x + 1;  // ERROR: expected i32, found ()
}

fn add_one_fixed(x: i32) -> i32 {
    x + 1   // No semicolon - returns i32
}
```

### Pitfall 2: Type Mismatch in If Branches

```rust
fn get_value(flag: bool) -> i32 {
    if flag {
        42
    } else {
        "forty-two"  // ERROR: if and else have incompatible types
    }
}
```

### Pitfall 3: Forgetting Block Can Be Expression

```rust
// Unnecessarily verbose
fn calculate() -> i32 {
    let result = {
        let a = 10;
        let b = 20;
        a + b
    };
    return result;
}

// More idiomatic
fn calculate() -> i32 {
    let a = 10;
    let b = 20;
    a + b
}
```

### Pitfall 4: Unit Type Confusion

```rust
fn main() {
    let result = println!("Hello");  // result is ()
    
    // This won't work as expected:
    let x = if true { println!("yes"); } else { println!("no"); };
    // x is (), both branches must return (), which they do
}
```

---

## Under the Hood: How Expressions Are Evaluated

When Rust compiles expressions, it:

1. **Type checks**: Ensures the expression produces the expected type
2. **Evaluates sub-expressions**: Inner expressions are evaluated first
3. **Propagates values**: The result flows to where it's used

For blocks, the compiler tracks:
- What variables are in scope
- What the final expression's type is
- Whether control flow can reach the end (all paths must return)

```rust
let x = {
    let a = 1;
    let b = 2;
    a + b
};
```

Assembly-level view:
```
; Evaluate a + b
mov eax, 1        ; a = 1
add eax, 2        ; eax = a + b = 3
; eax now holds the block's value
mov [x], eax      ; Store in x
```

---

## Under the Hood: No Extra Allocations

Block expressions don't create extra allocations or indirection:

```rust
fn example() -> i32 {
    { 5 }  // This is just 5, no hidden overhead
}
```

The compiler optimizes away the block entirely. The expression `{ 5 }` compiles to the same code as just `5`.

---

## Real-World Use Cases in Textio

### 1. Message Validation
```rust
fn validate_message(msg: &str) -> Result<String, String> {
    let trimmed = msg.trim();
    
    if trimmed.is_empty() {
        Err("Message cannot be empty".to_string())
    } else if trimmed.len() > 160 {
        Err(format!("Message too long: {} chars (max 160)", trimmed.len()))
    } else {
        Ok(trimmed.to_string())
    }
}
```

### 2. Cost Calculation with Block Scope
```rust
fn calculate_total(messages: u32, base_rate: f64) -> f64 {
    let subtotal = {
        let rate = if messages > 10000 { base_rate * 0.8 } else { base_rate };
        messages as f64 * rate
    };
    
    let tax = subtotal * 0.1;
    subtotal + tax
}
```

### 3. Phone Number Formatting
```rust
fn format_phone(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    
    match digits.len() {
        10 => format!("+1{}", digits),
        11 if digits.starts_with('1') => format!("+{}", digits),
        _ => format!("+{}", digits),
    }
}
```

### 4. Status Parsing
```rust
fn parse_status(code: u8) -> &'static str {
    match code {
        0 => "QUEUED",
        1 => "SENT",
        2 => "DELIVERED",
        3 => "FAILED",
        4 => "BOUNCED",
        _ => "UNKNOWN",
    }
}
```

---

## Exercise Task

Create functions that demonstrate expression-based programming:

1. `classify_message_length(len: usize) -> &'static str`
   - Returns "short" for 0-70 chars
   - Returns "medium" for 71-134 chars
   - Returns "long" for 135+ chars
   - Use a match expression

2. `calculate_segment_cost(segments: u32, rate: f64) -> f64`
   - Use a block expression to calculate the base cost
   - Apply a 10% discount for 10+ segments
   - Return the final cost (no explicit `return` keyword)

3. `format_delivery_report(id: u64, success: bool, attempts: u8) -> String`
   - Use if expressions to determine status text
   - Use a block to build the report string
   - Return formatted string: "Message #X: STATUS (Y attempts)"

4. `get_rate_tier(monthly_volume: u32) -> &'static str`
   - "tier1" for 0-1000 messages
   - "tier2" for 1001-10000 messages
   - "tier3" for 10001-100000 messages
   - "enterprise" for 100001+ messages

Main should call all functions with test values matching the expected output.

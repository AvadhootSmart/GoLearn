# Basic Functions in Rust

## Why This Concept Exists

Functions are the fundamental building blocks of any Rust program. They allow you to encapsulate logic into reusable, named units of code. In a real-world SMS API like Textio, functions let you organize complex operations—validating phone numbers, formatting messages, calculating costs—into manageable pieces.

Without functions, every piece of code would need to be duplicated wherever it's needed. Functions give us:
- **Reusability**: Write once, use many times
- **Abstraction**: Hide implementation details behind a clear interface
- **Testability**: Test isolated units of logic
- **Readability**: Give meaningful names to operations
- **Maintainability**: Change behavior in one place

Rust's function syntax is intentionally minimal and explicit, designed to make code easy to read and hard to misinterpret.

---

## Basic Function Syntax

Every Rust function starts with the `fn` keyword, followed by a name, parameters in parentheses, and a body in curly braces:

```rust
fn greet() {
    println!("Hello from Textio!");
}
```

This is the simplest possible function:
- `fn` - the keyword that declares a function
- `greet` - the function name (snake_case by convention)
- `()` - empty parameter list
- `{ }` - the function body

### Calling a Function

To execute a function, you call it by name with parentheses:

```rust
fn main() {
    greet();  // Calls the greet function
    greet();  // Can call it multiple times
}

fn greet() {
    println!("Welcome to Textio SMS API!");
}
```

Output:
```
Welcome to Textio SMS API!
Welcome to Textio SMS API!
```

---

## Function Naming Conventions

Rust enforces `snake_case` for function names:

```rust
// Good - snake_case
fn send_sms_message() { }
fn calculate_delivery_cost() { }
fn validate_phone_number() { }

// Bad - would trigger compiler warning
fn sendSMSMessage() { }  // compiler will warn
fn CalculateDeliveryCost() { }  // compiler will warn
```

The compiler will accept other naming styles but will warn you to use snake_case.

---

## Function Definition vs Declaration

Unlike C or C++, Rust doesn't separate function declaration from definition. When you write a function, you're both declaring it (telling the compiler it exists) and defining it (providing the implementation):

```rust
// This is both declaration AND definition
fn format_message(content: &str) -> String {
    format!("[Textio] {}", content)
}

// You can call functions defined AFTER main
fn main() {
    let msg = format_message("Hello");  // Works even though format_message
                                        // is defined later
}

fn format_message(content: &str) -> String {
    format!("[Textio] {}", content)
}
```

Rust doesn't care about the order of function definitions within a file—you can call functions defined after the point of call.

---

## The Function Body

The body of a function is a block that can contain:
- Statements (instructions that perform an action)
- Expressions (code that produces a value)
- Other function calls

```rust
fn process_message() {
    // Statement: let declaration
    let max_length = 160;
    
    // Statement: expression with semicolon
    println!("Max SMS length: {}", max_length);
    
    // Expression used as statement (with semicolon)
    let result = 10 + 20;  // 10 + 20 is expression, whole line is statement
    
    // Function call as statement
    validate_length(50);
}

fn validate_length(len: usize) {
    println!("Length {} is valid", len);
}
```

---

## Functions Without Return Values

Functions that don't return a value implicitly return the unit type `()`:

```rust
fn log_message(msg: &str) {
    println!("[LOG] {}", msg);
    // Implicitly returns ()
}

// This is equivalent to explicitly writing:
fn log_message_explicit(msg: &str) -> () {
    println!("[LOG] {}", msg);
}
```

The unit type `()` is Rust's way of representing "no meaningful value." It's a type with exactly one value, also written `()`.

---

## Nested Function Calls

Functions can call other functions, creating a call stack:

```rust
fn main() {
    send_notification("+1234567890", "Your code is 1234");
}

fn send_notification(phone: &str, message: &str) {
    log_attempt(phone);
    let formatted = format_for_sms(message);
    deliver(phone, &formatted);
    log_success(phone);
}

fn log_attempt(phone: &str) {
    println!("Attempting delivery to {}", phone);
}

fn format_for_sms(message: &str) -> String {
    format!("Textio: {}", message)
}

fn deliver(phone: &str, message: &str) {
    println!("Delivered to {}: {}", phone, message);
}

fn log_success(phone: &str) {
    println!("Successfully delivered to {}", phone);
}
```

Output:
```
Attempting delivery to +1234567890
Delivered to +1234567890: Textio: Your code is 1234
Successfully delivered to +1234567890
```

---

## Mental Model: Functions as Recipes

Think of a function like a recipe card:

1. **Name**: The recipe title (`fn send_sms`)
2. **Ingredients**: The parameters you need (`phone: &str, message: &str`)
3. **Instructions**: The body with steps
4. **Result**: What you get back (return value)

When you "call" a recipe, you:
- Gather your ingredients (arguments)
- Follow the steps (execute the body)
- Get your result (return value)

Unlike a recipe, though, functions start fresh each time—they don't remember previous calls.

---

## Mental Model: Functions as Black Boxes

From the caller's perspective, a function is a black box:
- You put inputs in
- Something happens inside
- You get output out

```rust
let cost = calculate_cost(100, 0.05);  // Input: 100 messages, $0.05 each
                                        // Output: $5.00
```

You don't need to know HOW `calculate_cost` works to use it. This abstraction is powerful for managing complexity.

---

## Common Pitfalls

### Pitfall 1: Forgetting Parentheses

```rust
fn main() {
    greet;  // ERROR: expected function, found `greet`
    greet();  // Correct
}

fn greet() {
    println!("Hello!");
}
```

Without parentheses, you're referring to the function itself (a function pointer), not calling it.

### Pitfall 2: Wrong Naming Convention

```rust
fn SendMessage() { }  // Compiler warning: function `SendMessage` should have a snake case name
```

Rust will compile this but warns you to follow conventions.

### Pitfall 3: Calling Before Definition (Actually Works!)

```rust
fn main() {
    helper();  // This WORKS in Rust!
}

fn helper() {
    println!("I'm helping!");
}
```

Unlike some languages, Rust doesn't require functions to be defined before they're called within the same crate.

### Pitfall 4: Expecting Return Values from Void Functions

```rust
fn main() {
    let result = log_message("Hello");  // result is ()
    println!("{:?}", result);  // Prints: ()
}

fn log_message(msg: &str) {
    println!("{}", msg);
}
```

Functions without explicit return types return `()`, which might not be what you expect.

---

## Under the Hood: Stack Frames

When you call a function, Rust creates a **stack frame** for that function:

```rust
fn main() {
    let a = 10;        // main's stack frame gets 'a'
    helper(20);        // new stack frame for 'helper' with 'x'
    let b = 30;        // back to main's frame
}

fn helper(x: i32) {
    let y = x + 5;     // helper's frame gets 'y'
}
```

Memory layout during execution:
```
|------------------|
| main's frame     |
| a = 10           |
|------------------|
| helper's frame   |  <- Created when helper is called
| x = 20           |
| y = 25           |
|------------------|
```

When `helper` returns, its frame is popped off the stack. This is why local variables don't persist between calls.

---

## Under the Hood: Calling Convention

When you call a function, the CPU:

1. **Pushes arguments** to the stack or registers
2. **Pushes the return address** (where to continue after the function)
3. **Jumps** to the function's code
4. **Executes** the function body
5. **Places the return value** in a designated location (typically a register)
6. **Pops** the return address and jumps back

This happens in nanoseconds, making function calls very fast.

---

## Real-World Use Cases in Textio

### 1. SMS Validation
```rust
fn validate_phone_number(phone: &str) -> bool {
    phone.len() >= 10 && phone.starts_with('+')
}
```

### 2. Message Formatting
```rust
fn format_sms(sender: &str, message: &str) -> String {
    format!("From: {}\n{}", sender, message)
}
```

### 3. Cost Calculation
```rust
fn calculate_sms_cost(message_count: u32, rate: f64) -> f64 {
    message_count as f64 * rate
}
```

### 4. Status Logging
```rust
fn log_delivery_status(message_id: u64, status: &str) {
    println!("[{}] Message {}: {}", 
             chrono_now(), message_id, status);
}

fn chrono_now() -> String {
    // Would return timestamp in real app
    String::from("2024-01-15T10:30:00Z")
}
```

---

## Exercise Task

Create a simple SMS processing system with four functions:

1. `welcome()` - Prints a welcome message for Textio users
2. `validate_length(message: &str, max_chars: usize)` - Prints whether a message is within the character limit
3. `format_message(sender: &str, recipient: &str, body: &str)` - Returns a formatted SMS string
4. `calculate_cost(message_count: u32, rate_per_sms: f64)` - Returns the total cost

Your `main` function should:
- Call `welcome()`
- Validate a 50-character message with a 160-char limit
- Format and print a message from "+15551234567" to "+15559876543"
- Calculate and print the cost for 1000 messages at $0.0075 each

The output should match `expected.txt` exactly.

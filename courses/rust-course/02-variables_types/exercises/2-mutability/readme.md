# Exercise 2: Mutability

## Why This Concept Exists

Rust's approach to mutability is one of its most distinctive features. Variables are **immutable by default**, meaning once you assign a value, it cannot change. This design choice exists for several critical reasons:

1. **Preventing Bugs**: Many bugs come from accidentally changing data you didn't mean to modify
2. **Thread Safety**: Immutable data is inherently safe to share across threads
3. **Compiler Optimization**: The compiler can make better optimizations when it knows data won't change
4. **Explicit Intent**: When you need mutation, you must declare it with `mut`, making your intentions clear

For Textio, this means message statuses, delivery counts, and user balances can only change when you explicitly allow it, preventing accidental data corruption.

---

## Immutability by Default

In Rust, variables are immutable unless you explicitly make them mutable:

```rust
fn main() {
    // This variable is IMMUTABLE by default
    let message = "Hello";
    
    // This would cause a compile error:
    // message = "Goodbye";  // error: cannot assign twice to immutable variable
    
    println!("{}", message);
}
```

**Compile Error:**
```
error[E0384]: cannot assign twice to immutable variable `message`
```

### Why This Matters

Consider a Textio message delivery system:

```rust
fn main() {
    // Without mut, this status can never accidentally change
    let delivery_status = "pending";
    
    // Some code that might accidentally try to change it...
    // delivery_status = "delivered";  // Compile error! Caught at compile time!
    
    println!("Status: {}", delivery_status);
}
```

---

## The `mut` Keyword

To make a variable mutable, add `mut` before the variable name:

```rust
fn main() {
    // Mutable variable - can be changed
    let mut counter = 0;
    
    println!("Initial: {}", counter);
    
    // This is now allowed
    counter = 1;
    println!("Updated: {}", counter);
    
    counter = counter + 1;
    println!("Incremented: {}", counter);
}
```

### Textio Example: Message Counter

```rust
fn main() {
    // We need to track messages sent, so this must be mutable
    let mut messages_sent = 0;
    
    // Send first message
    messages_sent = messages_sent + 1;
    println!("Messages sent: {}", messages_sent);
    
    // Send second message
    messages_sent = messages_sent + 1;
    println!("Messages sent: {}", messages_sent);
    
    // Send third message
    messages_sent = messages_sent + 1;
    println!("Messages sent: {}", messages_sent);
}
```

---

## When to Use `mut`

### Use `mut` When:

```rust
fn main() {
    // 1. Accumulating values
    let mut total_cost = 0.0;
    total_cost = total_cost + 10.50;
    total_cost = total_cost + 5.25;
    
    // 2. State that must change
    let mut is_connected = false;
    is_connected = true;  // Connection established
    
    // 3. Loop counters
    let mut index = 0;
    index = index + 1;
    
    // 4. Building strings
    let mut log_message = String::new();
    log_message.push_str("Error: ");
    log_message.push_str("Connection timeout");
}
```

### Don't Use `mut` When:

```rust
fn main() {
    // 1. Constants that should never change
    let api_key = "sk-textio-12345";  // Should never change
    let max_retries = 3;              // Configuration value
    
    // 2. Input parameters (function arguments)
    // These should typically remain as-is
    
    // 3. Computed values that don't need to change
    let message_length = 160;
    let is_valid = message_length <= 160;
    
    // 4. Read-only data
    let user_name = "alice";
    let account_type = "premium";
}
```

---

## Mental Model: The Locked Box

Think of variables as boxes with different security levels:

```
IMMUTABLE (default) - Locked Box
┌─────────────────────┐
│ 🔒 "pending"       │  ← Cannot be opened or changed
└─────────────────────┘
Label: delivery_status

MUTABLE - Unlocked Box
┌─────────────────────┐
│ 🔓 0               │  ← Can be opened and contents replaced
└─────────────────────┘
Label: mut counter
```

**Key Insight**: The `mut` keyword is like unlocking the box. Without it, the box is sealed shut.

---

## Common Pitfalls

### Pitfall 1: Forgetting `mut` When You Need It

```rust
fn main() {
    // WRONG - Variable needs to change but isn't mutable
    let count = 0;
    // count = count + 1;  // Error: cannot assign twice
    
    // CORRECT - Add mut when you need to change the value
    let mut count = 0;
    count = count + 1;  // This works!
}
```

**Error Message:**
```
error[E0384]: cannot assign twice to immutable variable `count`
```

### Pitfall 2: Using `mut` Unnecessarily

```rust
fn main() {
    // NOT IDEAL - Variable never changes, doesn't need mut
    let mut api_version = "v2";
    println!("API version: {}", api_version);
    // api_version is never modified
    
    // BETTER - Remove mut if not needed
    let api_version = "v2";
    println!("API version: {}", api_version);
}
```

**Compiler Warning:**
```
warning: variable does not need to be mutable
```

### Pitfall 3: Confusing Variable Declaration with Assignment

```rust
fn main() {
    // DECLARATION - Creates new variable
    let x = 5;
    
    // ASSIGNMENT - Changes existing variable (needs mut)
    // x = 10;  // Error without mut
    
    // CORRECT - Use mut for reassignment
    let mut y = 5;
    y = 10;  // This is assignment, not declaration
    
    // This creates a NEW variable (shadowing, covered next)
    let x = 10;  // This is declaration, not assignment
}
```

### Pitfall 4: Thinking `mut` Changes the Type

```rust
fn main() {
    let mut value = 42;  // Type is i32
    
    // WRONG - Can't assign different type even with mut
    // value = "hello";  // Error: expected i32, found &str
    
    // CORRECT - mut allows same-type reassignment only
    value = 100;  // Same type (i32), this works
}
```

**Error Message:**
```
error[E0308]: mismatched types
expected `i32`, found `&str`
```

---

## Under the Hood

### Memory Behavior

```rust
fn main() {
    // Immutable variable
    let fixed = 42;
    // Memory: [42] at address 0x1000
    // The value at 0x1000 cannot be modified
    
    // Mutable variable
    let mut changeable = 42;
    // Memory: [42] at address 0x2000
    
    changeable = 100;
    // Memory: [100] at address 0x2000
    // The value at 0x2000 is overwritten
}
```

### Compile-Time Guarantees

The Rust compiler uses mutability to provide guarantees:

```rust
fn main() {
    let x = 5;
    
    // The compiler knows x will NEVER change
    // It can optimize based on this knowledge
    
    let mut y = 5;
    y = 10;
    
    // The compiler knows y MIGHT change
    // It must account for this possibility
}
```

### Borrow Checker Interaction

Mutability affects how you can borrow values:

```rust
fn main() {
    let immutable_data = vec![1, 2, 3];
    let ref1 = &immutable_data;
    let ref2 = &immutable_data;
    // Multiple immutable references: OK!
    
    let mut mutable_data = vec![1, 2, 3];
    let ref_mut = &mut mutable_data;
    // Only ONE mutable reference at a time
    // let ref_mut2 = &mut mutable_data;  // Error!
}
```

---

## Real-World Textio Examples

### Tracking Message Delivery

```rust
fn main() {
    // Delivery status must change as message progresses
    let mut status = "queued";
    println!("Status: {}", status);
    
    status = "sending";
    println!("Status: {}", status);
    
    status = "delivered";
    println!("Status: {}", status);
}
```

### User Account Balance

```rust
fn main() {
    // Account balance changes with each transaction
    let mut balance: f64 = 100.00;
    println!("Starting balance: ${}", balance);
    
    // User sends a message (costs $0.05)
    balance = balance - 0.05;
    println!("After message: ${}", balance);
    
    // User adds credit
    balance = balance + 50.00;
    println!("After top-up: ${}", balance);
}
```

### Rate Limiting Counter

```rust
fn main() {
    // Track API requests within time window
    let mut requests_this_hour = 0;
    let max_requests = 1000;
    
    // Request 1
    requests_this_hour = requests_this_hour + 1;
    println!("Requests: {}/{}", requests_this_hour, max_requests);
    
    // Request 2
    requests_this_hour = requests_this_hour + 1;
    println!("Requests: {}/{}", requests_this_hour, max_requests);
    
    // Check if limit reached
    let limit_reached = requests_this_hour >= max_requests;
    println!("Limit reached: {}", limit_reached);
}
```

### Message Queue Processing

```rust
fn main() {
    // Process messages in queue
    let mut queue_size = 5;
    println!("Messages in queue: {}", queue_size);
    
    // Process one message
    queue_size = queue_size - 1;
    println!("After processing: {}", queue_size);
    
    // New message arrives
    queue_size = queue_size + 1;
    println!("After arrival: {}", queue_size);
    
    // Process remaining
    queue_size = 0;
    println!("Queue cleared: {}", queue_size);
}
```

---

## Exercise Task

Create a program that simulates Textio's message tracking system:

1. Create an immutable variable for the API version (never changes)
2. Create mutable variables for:
   - `messages_sent` - starts at 0, increments with each message
   - `delivery_successes` - starts at 0, increments on successful delivery
   - `account_balance` - starts at 100.0, decreases by 0.05 per message
   - `current_status` - starts as "idle", changes through states

3. Simulate sending 3 messages:
   - Increment `messages_sent`
   - Increment `delivery_successes`
   - Deduct cost from `account_balance`
   - Update `current_status` through: "sending" → "sent" → "idle"

4. Print state after each message

**Starter Code**: See `code.rs`
**Solution**: See `complete.rs`
**Expected Output**: See `expected.txt`

# Exercise 2: Mutable References (&mut T)

## Learning Objectives

By the end of this exercise, you will understand:
- What mutable references are and when to use them
- How to create and use `&mut T`
- The restriction on mutable references (only one at a time)
- The relationship between mutable and immutable references
- How to modify data through references
- Real-world patterns for mutable borrowing in Textio

---

## Introduction to Mutable References

While immutable references let you **read** data without taking ownership, mutable references let you **modify** data without taking ownership. This is powerful but comes with important restrictions.

### The `&mut T` Syntax

```rust
fn main() {
    let mut message = String::from("Hello");
    
    // Create a mutable reference
    let message_ref = &mut message;
    
    // Modify through the reference
    message_ref.push_str(", World!");
    
    println!("{}", message_ref);  // "Hello, World!"
}
```

### Key Requirements

1. The original variable must be declared with `mut`
2. You must use `&mut` to create the reference
3. The reference type is `&mut T`

```rust
fn main() {
    let message = String::from("Hello");  // NOT mut!
    
    // Error: cannot borrow as mutable
    // let ref = &mut message;
}
```

---

## The One Mutable Reference Rule

Rust enforces a crucial rule: **You can have only ONE mutable reference to a particular piece of data at a time.**

### Why This Rule Exists

This rule prevents **data races** at compile time. A data race occurs when:
1. Two or more pointers access the same data at the same time
2. At least one of them is writing
3. There's no synchronization mechanism

```rust
fn main() {
    let mut message = String::from("Hello");
    
    let ref1 = &mut message;
    
    // Error: cannot borrow `message` as mutable more than once
    // let ref2 = &mut message;
    
    println!("{}", ref1);
}
```

### Comparison with Immutable References

```rust
fn main() {
    let message = String::from("Hello");
    
    // Multiple immutable references: ALLOWED
    let ref1 = &message;
    let ref2 = &message;
    let ref3 = &message;
    // All can read simultaneously
    
    let mut data = String::from("Hello");
    
    // Multiple mutable references: NOT ALLOWED
    let mref1 = &mut data;
    // let mref2 = &mut data;  // Error!
}
```

---

## The Complete Borrowing Rules

Let's put it all together:

### Rule 1: One Mutable OR Many Immutable (Not Both)

```rust
fn main() {
    let mut s = String::from("Hello");
    
    // OK: Multiple immutable references
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    
    // OK: One mutable reference (after immutable refs are done)
    let mr = &mut s;
    mr.push_str(", World!");
    println!("{}", mr);
}
```

### Rule 2: Cannot Mix Mutable and Immutable References

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;  // Immutable borrow
    
    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let mr = &mut s;
    
    println!("{}", r1);
}
```

### Why Mixing Is Dangerous

```rust
// Hypothetical scenario (not allowed in Rust)
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;      // Immutable reference
    let mr = &mut s;  // Mutable reference (ERROR if allowed!)
    
    mr.push_str(" World!");  // Modifies the string
    
    // r1 might now point to invalid data!
    // If push_str reallocated the string's buffer,
    // r1 would be a dangling reference
    println!("{}", r1);
}
```

---

## Modifying Through References

### String Modification

```rust
fn main() {
    let mut message = String::from("Hello");
    
    append_exclamation(&mut message);
    
    println!("{}", message);  // "Hello!"
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}
```

### Struct Modification

```rust
struct SmsMessage {
    to: String,
    body: String,
    sent: bool,
}

fn main() {
    let mut msg = SmsMessage {
        to: String::from("+15551234567"),
        body: String::from("Hello"),
        sent: false,
    };
    
    mark_as_sent(&mut msg);
    
    println!("Sent status: {}", msg.sent);  // true
}

fn mark_as_sent(msg: &mut SmsMessage) {
    msg.sent = true;
}
```

### Collection Modification

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    double_all(&mut numbers);
    
    println!("{:?}", numbers);  // [2, 4, 6, 8, 10]
}

fn double_all(nums: &mut Vec<i32>) {
    for num in nums.iter_mut() {
        *num *= 2;
    }
}
```

---

## Iterating Mutably

### Using `iter_mut()`

```rust
fn main() {
    let mut messages = vec![
        String::from("hello"),
        String::from("world"),
    ];
    
    for msg in messages.iter_mut() {
        // msg is &mut String
        msg.push('!');
    }
    
    println!("{:?}", messages);  // ["hello!", "world!"]
}
```

### The `*` Dereference Operator

When you have a mutable reference and want to assign to it, use `*`:

```rust
fn main() {
    let mut x = 5;
    let x_ref = &mut x;
    
    // Wrong: this would reassign the reference itself
    // x_ref = 10;
    
    // Right: dereference to modify the value
    *x_ref = 10;
    
    println!("{}", x);  // 10
}
```

---

## Scope-Based Reference Management

References end when they're no longer used, not necessarily at the end of their scope (thanks to NLL).

### Example: References Can End Early

```rust
fn main() {
    let mut s = String::from("Hello");
    
    {
        let r1 = &mut s;
        r1.push_str(", World!");
    }  // r1 goes out of scope here
    
    // Now we can create a new mutable reference
    let r2 = &mut s;
    r2.push('!');
    
    println!("{}", s);  // "Hello, World!!"
}
```

### With NLL: Usage-Based Lifetimes

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;
    println!("{}", r1);  // Last use of r1
    
    // r1 is "dead" now, so this is allowed:
    let r2 = &mut s;
    r2.push_str(", World!");
    
    println!("{}", s);
}
```

---

## Mutable References in Functions

### Taking Mutable References

```rust
fn main() {
    let mut message = String::from("Hello");
    
    add_greeting(&mut message);
    add_recipient(&mut message, "Alice");
    
    println!("{}", message);
}

fn add_greeting(msg: &mut String) {
    msg.insert_str(0, "Dear user, ");
}

fn add_recipient(msg: &mut String, name: &str) {
    msg.push_str(&format!("\n\nSincerely, {}", name));
}
```

### Returning Mutable References

```rust
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    let first = get_first_mut(&mut data);
    *first = 100;
    
    println!("{:?}", data);  // [100, 2, 3, 4, 5]
}

fn get_first_mut(v: &mut Vec<i32>) -> &mut i32 {
    &mut v[0]
}
```

---

## Common Patterns in Textio

### Pattern 1: Modifying Message Status

```rust
struct Message {
    id: u64,
    body: String,
    status: MessageStatus,
}

enum MessageStatus {
    Pending,
    Sent,
    Failed,
}

fn main() {
    let mut msg = Message {
        id: 1,
        body: String::from("Hello"),
        status: MessageStatus::Pending,
    };
    
    send_message(&mut msg);
    
    // msg.status is now MessageStatus::Sent
}

fn send_message(msg: &mut Message) {
    // Simulate sending...
    msg.status = MessageStatus::Sent;
}
```

### Pattern 2: Building Messages

```rust
struct MessageBuilder {
    to: String,
    body: String,
}

fn main() {
    let mut builder = MessageBuilder {
        to: String::new(),
        body: String::new(),
    };
    
    set_recipient(&mut builder, "+15551234567");
    set_body(&mut builder, "Your code is 123456");
    
    println!("To: {}", builder.to);
    println!("Body: {}", builder.body);
}

fn set_recipient(builder: &mut MessageBuilder, phone: &str) {
    builder.to = phone.to_string();
}

fn set_body(builder: &mut MessageBuilder, text: &str) {
    builder.body = text.to_string();
}
```

### Pattern 3: Batch Processing

```rust
fn main() {
    let mut messages = vec![
        String::from("hello"),
        String::from("world"),
        String::from("textio"),
    ];
    
    // Process all messages
    process_messages(&mut messages);
    
    println!("{:?}", messages);
}

fn process_messages(messages: &mut Vec<String>) {
    for msg in messages.iter_mut() {
        // Capitalize first letter
        if let Some(first) = msg.chars().next() {
            let capitalized = first.to_uppercase().collect::<String>();
            msg.replace_range(..1, &capitalized);
        }
    }
}
```

---

## Common Errors and Fixes

### Error 1: Borrowing as Mutable Twice

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &mut s;
    let r2 = &mut s;  // Error!
    
    println!("{} {}", r1, r2);
}
```

**Fix: Use Scopes**

```rust
fn main() {
    let mut s = String::from("Hello");
    
    {
        let r1 = &mut s;
        r1.push_str(" there");
    }
    
    let r2 = &mut s;
    r2.push('!');
    
    println!("{}", s);
}
```

**Fix: Use Sequential Borrows**

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &mut s;
    r1.push_str(" there");
    
    // r1 is no longer used after this point
    
    let r2 = &mut s;  // Now OK with NLL
    r2.push('!');
    
    println!("{}", s);
}
```

### Error 2: Immutable Borrow While Mutable Exists

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;
    let r2 = &mut s;  // Error!
    
    println!("{}", r1);
}
```

**Fix: Order Your Borrows**

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;
    println!("{}", r1);
    // r1 is no longer used
    
    let r2 = &mut s;  // Now OK
    r2.push('!');
    
    println!("{}", s);
}
```

### Error 3: Forgetting `mut` on Variable

```rust
fn main() {
    let s = String::from("Hello");  // Missing mut!
    
    let r = &mut s;  // Error: cannot borrow as mutable
}
```

**Fix: Add `mut`**

```rust
fn main() {
    let mut s = String::from("Hello");  // Now mutable
    
    let r = &mut s;  // OK
    r.push('!');
    
    println!("{}", s);
}
```

---

## When to Use Mutable References

### Use `&mut T` When:

1. You need to modify data you don't own
2. You want to avoid expensive cloning
3. You're implementing builder patterns
4. You need to update state in place

### Don't Use `&mut T` When:

1. You only need to read data (use `&T`)
2. You need multiple simultaneous accessors
3. You want to transfer ownership (use the value directly)

---

## Summary

### Key Concepts

1. **`&mut T`** creates a mutable reference that allows modification
2. **Only ONE** mutable reference can exist at a time
3. **Cannot mix** mutable and immutable references simultaneously
4. **NLL** allows new references after old ones are last used
5. **`*`** operator dereferences to access/modify the value

### The Golden Rule

> At any given time, you can have **either** one mutable reference **or** any number of immutable references.

This rule prevents data races at compile time, making concurrent programming safer.

### Rules Summary

| Reference Type | How Many? | Can Modify? |
|----------------|-----------|-------------|
| `&T` (immutable) | Multiple | No |
| `&mut T` (mutable) | One | Yes |

---

## Next Steps

In the next exercise, you'll dive deeper into the **borrowing rules** and see more complex scenarios where these rules interact.

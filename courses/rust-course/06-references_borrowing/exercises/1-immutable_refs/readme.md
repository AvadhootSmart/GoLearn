# Exercise 1: Immutable References (&T)

## Learning Objectives

By the end of this exercise, you will understand:
- What immutable references are and why they're useful
- The difference between ownership and borrowing
- How to create and use immutable references
- Deref coercion and automatic conversions
- When to use `&T` vs taking ownership
- The borrow checker's role with immutable references

---

## Introduction to References

In the previous module, you learned about ownership and how values move between variables. While ownership is powerful, constantly moving values can be cumbersome. This is where **references** come in.

A **reference** is like a pointer - it's an address that leads to data owned by another variable. Unlike pointers in languages like C or C++, Rust references are **always valid** and **trackably scoped**.

### The Problem References Solve

```rust
fn main() {
    let message = String::from("Hello, Textio!");
    
    // Without references, we'd need to pass ownership
    let length = calculate_length_take_ownership(message);
    
    // Error! message has been moved
    // println!("{}", message);
}

fn calculate_length_take_ownership(s: String) -> usize {
    s.len()
}
```

This is problematic - we just wanted to calculate the length, not consume the string!

### The Solution: Borrowing

```rust
fn main() {
    let message = String::from("Hello, Textio!");
    
    // With references, we BORROW the value
    let length = calculate_length(&message);
    
    // message is still valid!
    println!("Message: {}", message);
    println!("Length: {}", length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but since it doesn't have ownership,
  // nothing happens to the String it points to
```

---

## Understanding `&T` Syntax

The `&` symbol has two meanings in Rust:

1. **Creating a reference**: `&value` creates a reference to `value`
2. **Reference type**: `&T` is the type of a reference to a value of type `T`

```rust
fn main() {
    let x = 5;
    
    // Creating a reference
    let ref_to_x = &x;      // &x creates a reference to x
    
    // Type annotation (usually inferred)
    let ref_to_x: &i32 = &x;
    
    println!("x = {}", x);           // 5
    println!("ref_to_x = {}", ref_to_x); // 5 (automatic dereference in println!)
}
```

---

## Reference vs Pointer Mental Model

While references are similar to pointers, thinking about them correctly helps understand Rust's guarantees:

### Traditional Pointer
```
Address: 0x1234
Value at address: "Hello"
No guarantees about:
- Is the memory still valid?
- Who owns this memory?
- Can I modify it?
- When will it be freed?
```

### Rust Reference
```
Reference to a String owned by variable 'message'
Address: 0x1234
Value at address: "Hello"

Guarantees:
- Memory is definitely valid
- 'message' owns the String
- Cannot modify through this reference (immutable)
- Will be freed when 'message' goes out of scope
- Reference cannot outlive 'message'
```

---

## Borrowing: The Core Concept

**Borrowing** means creating a reference to a value without taking ownership. Just like borrowing a book from a library:
- You can read it (access the value)
- You don't own it (original owner still has it)
- You must return it (reference has limited scope)
- You can't modify it (immutable borrowing)

```rust
fn main() {
    let message = String::from("Welcome to Textio!");
    
    // We BORROW message by creating a reference
    let borrowed = &message;
    
    // Both are accessible
    println!("Original: {}", message);
    println!("Borrowed: {}", borrowed);
    
    // message still owns the String
    // borrowed is just a temporary view
}
```

---

## Multiple Immutable References

You can have as many immutable references as you want simultaneously:

```rust
fn main() {
    let message = String::from("Hello");
    
    let ref1 = &message;
    let ref2 = &message;
    let ref3 = &message;
    
    // All can coexist peacefully
    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
    println!("ref3: {}", ref3);
    
    // Like multiple people reading the same book
}
```

This is safe because nobody can modify the data through these references!

---

## Deref Coercion

Rust has a powerful feature called **deref coercion** that automatically converts references when needed.

### The `Deref` Trait

When you have a reference, Rust can automatically "dereference" it to access the underlying value:

```rust
fn main() {
    let s = String::from("hello");
    let s_ref: &String = &s;
    
    // Automatic dereference for method calls
    // s_ref.len() works even though len() is defined on String, not &String
    println!("Length: {}", s_ref.len());
    
    // This is equivalent to:
    println!("Length: {}", (*s_ref).len());
}
```

### `&String` to `&str` Conversion

This is the most common deref coercion you'll encounter:

```rust
fn print_slice(data: &str) {
    println!("Slice: {}", data);
}

fn main() {
    let s = String::from("Hello, Textio!");
    
    // &String automatically coerces to &str
    print_slice(&s);
    
    // This works because String implements Deref<Target=str>
    // So &String can be used anywhere &str is expected
}
```

### Why This Matters for Textio

```rust
fn validate_phone_number(phone: &str) -> bool {
    phone.len() == 10 && phone.chars().all(|c| c.is_numeric())
}

fn main() {
    let phone = String::from("5551234567");
    
    // Can pass &String where &str is expected
    if validate_phone_number(&phone) {
        println!("Valid phone number!");
    }
    
    // Also works with string literals (which are already &str)
    if validate_phone_number("5559876543") {
        println!("Also valid!");
    }
}
```

---

## References in Function Signatures

### Taking References as Parameters

```rust
// This function borrows a String
fn analyze_message(msg: &String) {
    println!("Message: {}", msg);
    println!("Length: {}", msg.len());
    println!("First char: {}", msg.chars().next().unwrap());
}

// Better: accept &str for flexibility
fn analyze_message_flexible(msg: &str) {
    println!("Message: {}", msg);
    println!("Length: {}", msg.len());
}

fn main() {
    let message = String::from("Hello, Textio user!");
    
    analyze_message(&message);
    analyze_message_flexible(&message);  // Works with &String
    analyze_message_flexible("Direct literal");  // Works with &str
}
```

### Returning References

You can return references, but they must reference something that outlives the function:

```rust
fn main() {
    let message = String::from("Hello, Textio!");
    
    // Get a reference to part of message
    let first_word = get_first_word(&message);
    
    println!("First word: {}", first_word);
}

// Returns a reference to data that someone else owns
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

---

## The Borrow Checker

The **borrow checker** is Rust's compile-time tool that ensures references are used safely.

### What It Checks

1. **References don't outlive their referent**: A reference must never point to freed memory
2. **No mutation through immutable references**: If you have `&T`, you can't modify
3. **References are always valid**: No null references in safe Rust

### Example: Caught by the Borrow Checker

```rust
fn main() {
    let reference;
    
    {
        let value = String::from("Hello");
        reference = &value;  // Error!
    } // value is dropped here
    
    // reference would be dangling here!
    println!("{}", reference);
}
```

Error:
```
error[E0597]: `value` does not live long enough
  --> src/main.rs:5:21
   |
5  |         reference = &value;
   |                     ^^^^^^^ borrowed value does not live long enough
6  |     }
   |     - `value` dropped here while still borrowed
7  |     
8  |     println!("{}", reference);
   |                    --------- borrow later used here
```

---

## Non-Lexical Lifetimes (NLL)

Before Rust 2018, references lived until the end of their scope (lexical lifetimes). Now, Rust uses **Non-Lexical Lifetimes (NLL)**, which are smarter about when references are actually used.

### Example: NLL in Action

```rust
fn main() {
    let mut message = String::from("Hello");
    
    let first_reference = &message;
    println!("{}", first_reference);
    // first_reference is no longer used after this point
    
    // This is allowed! first_reference's "lifetime" ended
    // when it was last used, not at the end of the scope
    message.push_str(", World!");
    
    println!("{}", message);
}
```

In older Rust (before NLL), this would error because `first_reference` would live until the end of the scope. With NLL, Rust sees that `first_reference` isn't used after the first `println!`, so modifying `message` is allowed.

### Without NLL (Conceptual)

```rust
fn main() {
    let mut message = String::from("Hello");
    
    let first_reference = &message;
    println!("{}", first_reference);
    
    // Old Rust would error here:
    // "cannot borrow message as mutable because it is also 
    //  borrowed as immutable"
    message.push_str(", World!");
}
```

---

## Textio Example: Processing Messages

Let's see how immutable references help in a real Textio scenario:

```rust
struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

fn main() {
    let message = SmsMessage {
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: String::from("Your verification code is 123456"),
    };
    
    // We can inspect the message without taking ownership
    validate_recipient(&message.to);
    validate_sender(&message.from);
    analyze_body(&message.body);
    
    // message is still valid - we can use it further
    send_message(&message);
}

fn validate_recipient(phone: &str) {
    if phone.starts_with('+') && phone.len() >= 10 {
        println!("Valid recipient: {}", phone);
    }
}

fn validate_sender(phone: &str) {
    println!("Sender: {}", phone);
}

fn analyze_body(body: &str) {
    println!("Body length: {} characters", body.len());
    println!("Body preview: {}...", &body[..20.min(body.len())]);
}

fn send_message(msg: &SmsMessage) {
    println!("Sending SMS from {} to {}...", msg.from, msg.to);
    println!("Message: {}", msg.body);
}
```

---

## Common Patterns

### Pattern 1: Inspect Without Consuming

```rust
fn main() {
    let messages = vec![
        String::from("Hello"),
        String::from("World"),
    ];
    
    for msg in &messages {  // Borrow each element
        println!("{}", msg);
    }
    
    // messages is still valid!
    println!("Total messages: {}", messages.len());
}
```

### Pattern 2: Multiple Analysis Functions

```rust
fn main() {
    let data = String::from("Important SMS content");
    
    let length = get_length(&data);
    let word_count = count_words(&data);
    let first_char = get_first_char(&data);
    
    println!("Length: {}, Words: {}, First: {}", length, word_count, first_char);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn count_words(s: &String) -> usize {
    s.split_whitespace().count()
}

fn get_first_char(s: &String) -> char {
    s.chars().next().unwrap_or(' ')
}
```

---

## Common Borrow Checker Errors

### Error 1: Using After Move

```rust
fn main() {
    let s = String::from("Hello");
    let s2 = s;  // s is moved
    
    // println!("{}", s);  // Error: value borrowed after move
    println!("{}", s2);  // OK
}
```

### Fix: Use Reference

```rust
fn main() {
    let s = String::from("Hello");
    let s2 = &s;  // Borrow instead of move
    
    println!("{}", s);   // OK
    println!("{}", s2);  // OK
}
```

### Error 2: Returning Reference to Local

```rust
fn get_string() -> &String {
    let s = String::from("Hello");
    &s  // Error: returns reference to local variable
}
```

### Fix: Return Owned Value

```rust
fn get_string() -> String {
    let s = String::from("Hello");
    s  // Return ownership
}
```

---

## Summary

### Key Concepts

1. **References (`&T`)** let you borrow values without taking ownership
2. **Borrowing** is like lending a book - you can read it but don't own it
3. **Multiple immutable references** are allowed simultaneously
4. **Deref coercion** automatically converts `&String` to `&str`
5. **The borrow checker** ensures safety at compile time
6. **NLL** allows smarter reference lifetimes based on usage

### When to Use Immutable References

- When you need to read data but not modify it
- When you want to use a value in multiple places
- When passing data to functions that shouldn't take ownership
- When iterating over collections without consuming them

### Rules to Remember

1. You can have **any number** of immutable references simultaneously
2. The owner must remain valid while references exist
3. You cannot modify through an immutable reference
4. References are always valid (no null references)

---

## Next Steps

In the next exercise, you'll learn about **mutable references (`&mut T`)** and how to modify borrowed data safely.

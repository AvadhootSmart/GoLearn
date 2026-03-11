# Exercise 1: Lifetime Basics

## Why Lifetimes Exist

Lifetimes are one of Rust's most distinctive features. They exist for one fundamental reason: **to prevent dangling references**. The borrow checker needs to know how long references are valid to ensure memory safety without garbage collection.

### The Core Problem

Consider what happens when we try to return a reference to a local variable:

```rust
fn get_message() -> &str {
    let message = String::from("Hello");
    &message  // Error: returns a reference to data owned by the function
}
```

This fails because `message` is dropped when `get_message()` ends. The reference we're returning would point to deallocated memory - a dangling reference. In languages like C, this would be undefined behavior. Rust prevents it at compile time.

### How Rust Tracks Reference Validity

Every reference in Rust has a **lifetime** - the scope for which that reference is valid. Consider:

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

The reference `r` has lifetime `'a`, but it points to `x` which has lifetime `'b`. Since `'b` is shorter than `'a`, Rust rejects this code. We'd be using `r` after `x` is gone.

### What Lifetime Annotations Actually Do

Here's the key insight: **lifetime annotations don't change how long references live**. They merely describe the relationships between lifetimes so the compiler can verify safety.

When you write:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}
```

You're telling the compiler: "The returned reference will be valid for the same lifetime as the input reference." The function doesn't extend or shorten any lifetimes - it just documents the relationship.

### Lifetime Annotation Syntax

Lifetime parameters are declared in angle brackets, similar to generic type parameters:

```rust
// Single lifetime parameter
fn function<'a>(x: &'a str) -> &'a str { x }

// Multiple lifetime parameters
fn function<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }

// Combined with generic types
fn function<'a, T>(x: &'a T) -> &'a T { x }
```

Lifetime names conventionally start with a single quote and use short, lowercase names:
- `'a`, `'b`, `'c` for simple cases
- More descriptive names like `'input`, `'output` for clarity in complex code

### Lifetimes in Structs

When a struct holds references, it must declare lifetime parameters:

```rust
struct Message<'a> {
    content: &'a str,
    sender: &'a str,
}

impl<'a> Message<'a> {
    fn new(content: &'a str, sender: &'a str) -> Self {
        Message { content, sender }
    }
    
    fn get_content(&self) -> &'a str {
        self.content
    }
}
```

The struct is annotated with `'a` to indicate that its references must live at least as long as the struct instance.

### Lifetimes in Methods

In method signatures, you often see:

```rust
impl<'a> MyStruct<'a> {
    fn method(&self) -> &'a str {
        // ...
    }
    
    fn other_method<'b>(&self, other: &'b str) -> &'b str 
    where 'a: 'b 
    {
        // ...
    }
}
```

The `&self` reference has an implicit lifetime, but when returning data held by the struct, you use `'a`.

### Textio Example: Message Parsing

In our Textio SMS API, we might parse messages and return slices:

```rust
struct ParsedMessage<'a> {
    phone: &'a str,      // Phone number slice
    body: &'a str,       // Message body slice
    timestamp: &'a str,  // Timestamp slice
}

fn parse_message<'a>(raw: &'a str) -> Option<ParsedMessage<'a>> {
    let parts: Vec<&str> = raw.splitn(3, '|').collect();
    if parts.len() == 3 {
        Some(ParsedMessage {
            phone: parts[0],
            body: parts[1],
            timestamp: parts[2],
        })
    } else {
        None
    }
}
```

The `'a` lifetime connects the input `raw` to all the output references. This tells the compiler: "The returned `ParsedMessage` contains slices from the input string, so the input must live at least as long as the parsed result."

### The Borrow Checker's Reasoning

When the borrow checker analyzes code, it asks:

1. What are the lifetimes of all references?
2. Do borrowed values live long enough?
3. Are there any dangling references?

Lifetime annotations help answer these questions by making relationships explicit.

### Common Patterns

**Pass-through Pattern**: Return a reference derived from input:
```rust
fn select<'a>(condition: bool, a: &'a str, b: &'a str) -> &'a str {
    if condition { a } else { b }
}
```

**Struct Containment Pattern**: Struct holds references to external data:
```rust
struct Cache<'a> {
    entries: Vec<&'a str>,
}
```

**Lifetime Bounds on Generics**: When a generic type might contain references:
```rust
fn store<T: 'static>(value: T) {
    // T cannot contain any non-'static references
}
```

### What Happens Without Lifetimes

If we try to define a struct with references but no lifetime annotation:

```rust
struct Message {
    content: &str,  // Error: missing lifetime specifier
}
```

The compiler needs to know how long `content` is valid relative to the struct. Without this information, it can't guarantee safety.

### Lifetime Coercion and Subtyping

Rust supports lifetime subtyping - a longer lifetime can be coerced to a shorter one:

```rust
fn print_str<'a>(s: &'a str) {
    println!("{}", s);
}

fn main() {
    let long_lived = String::from("Hello");
    let r = &long_lived;  // Has 'long lifetime
    
    {
        let short_lived = String::from("World");
        let s = &short_lived;  // Has 'short lifetime
        
        // 'long can be coerced to 'short
        print_str(r);  // Works fine
        print_str(s);  // Works fine
    }
    
    // But 'short cannot be used as 'long
    // print_str(s);  // Error: s doesn't live long enough
}
```

### Understanding the "Lifetime" Terminology

The term "lifetime" can be misleading. It's not about:
- How long a value lives in memory (that's scope)
- When memory is allocated or deallocated
- Runtime behavior

Instead, it's about:
- Compile-time verification of reference validity
- Relationships between reference scopes
- Static guarantees the compiler enforces

### Key Takeaways

1. **Lifetimes prevent dangling references** - They're the mechanism Rust uses to ensure references always point to valid data.

2. **Annotations describe relationships** - They don't change behavior; they document constraints.

3. **The borrow checker uses them** - It verifies that all lifetime constraints are satisfied.

4. **Structs need them for references** - Any struct holding references must declare lifetime parameters.

5. **They're often inferred** - Many common patterns don't require explicit annotations due to elision rules (covered in the next exercise).

### Debugging Lifetime Errors

When you see lifetime errors:

1. Read the error message carefully - it often tells you exactly what's wrong
2. Draw a diagram of scopes and lifetimes
3. Check if any references might outlive their referents
4. Consider if lifetime annotations would clarify relationships
5. Remember: the compiler is preventing a real bug

### Exercise Preview

In this exercise, you'll:
- Add lifetime annotations to functions and structs
- Implement a `PhoneNumber` struct that holds string slices
- Create parsing functions that return references
- Build a simple Textio message validator

# Exercise 2: Lifetime Elision

## What is Lifetime Elision?

Lifetime annotations can be verbose. Rust recognizes certain patterns where lifetimes are so predictable that they can be inferred. This inference system is called **lifetime elision**. The compiler applies a set of rules to determine lifetimes without explicit annotations.

### The Three Elision Rules

Rust uses three rules to infer lifetimes. These rules are applied in order:

#### Rule 1: Input Lifetimes

**Each reference parameter gets its own lifetime parameter.**

```rust
// Without elision
fn foo<'a>(x: &'a i32) { }
fn bar<'a, 'b>(x: &'a i32, y: &'b i32) { }

// With elision (compiler infers the above)
fn foo(x: &i32) { }
fn bar(x: &i32, y: &i32) { }
```

The compiler automatically assigns different lifetime parameters to each input reference.

#### Rule 2: Output Lifetimes from `&self` or `&mut self`

**If there's exactly one input lifetime (from `&self` or `&mut self`), that lifetime is assigned to all output references.**

```rust
// Without elision
impl<'a> MyStruct<'a> {
    fn get_data(&self) -> &'a str {
        self.data
    }
}

// With elision (compiler infers the above)
impl MyStruct<'_> {
    fn get_data(&self) -> &str {
        self.data
    }
}
```

This makes method signatures much cleaner.

#### Rule 3: Multiple Input Lifetimes

**If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output references.**

```rust
// Without elision
impl<'a> MyStruct<'a> {
    fn combine<'b>(&self, other: &'b str) -> &str {
        // Returns something from self, not other
        self.data
    }
}

// With elision
impl MyStruct<'_> {
    fn combine(&self, other: &str) -> &str {
        self.data
    }
}
```

### When Elision Works vs. When It Doesn't

#### Cases Where Elision Works

**Single input reference:**
```rust
fn first_word(s: &str) -> &str {
    // Elision: output gets same lifetime as input
}
```

**Methods with `&self`:**
```rust
impl Message {
    fn body(&self) -> &str {
        // Elision: output gets self's lifetime
        &self.body
    }
}
```

**Multiple inputs where output clearly comes from one:**
```rust
impl Parser {
    fn parse(&mut self, input: &str) -> &str {
        // Output comes from internal state, not input
        // Elision applies due to &mut self
    }
}
```

#### Cases Where Elision Fails

**Multiple input references, ambiguous output:**
```rust
fn longest(x: &str, y: &str) -> &str {
    // Error: cannot infer lifetime
    // Does output come from x or y?
    if x.len() > y.len() { x } else { y }
}
```

**Must be explicit:**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Understanding Input vs. Output Lifetimes

**Input lifetimes** are those on function parameters:
```rust
fn example<'a, 'b>(x: &'a str, y: &'b str)
//                   ^^^^ input    ^^^^ input
```

**Output lifetimes** are those on return types:
```rust
fn example<'a, 'b>(x: &'a str) -> &'a str
//                                  ^^^^ output
```

Elision rules only infer output lifetimes from input lifetimes, never the reverse.

### The `_` Placeholder

In some contexts, you can use `'_` to explicitly indicate "elide this lifetime":

```rust
struct Message<'a> {
    body: &'a str,
}

fn main() {
    let msg: Message<'_> = get_message();
    //           ^^^ "use the elided lifetime"
}
```

This is useful when you want to be explicit that a lifetime exists but don't care about its name.

### Elision in Structs

Struct definitions always require explicit lifetime annotations (no elision):

```rust
struct Parser<'a> {
    input: &'a str,  // Must be explicit
}
```

However, when using the struct, you can sometimes elide:

```rust
fn parse(input: &str) -> Parser<'_> {
    // The '_ means "use the appropriate lifetime"
    Parser { input }
}
```

### Common Elision Patterns in Textio

**Message accessor:**
```rust
impl SmsMessage {
    fn sender(&self) -> &str {
        // Elision: returns reference to self.sender
        &self.sender
    }
}
```

**Parser with state:**
```rust
impl SmsParser {
    fn remaining(&self) -> &str {
        // Elision: returns reference to internal buffer
        &self.buffer[self.position..]
    }
}
```

**Builder pattern:**
```rust
impl MessageBuilder {
    fn with_body(&mut self, body: &str) -> &mut Self {
        // Elision: returns &mut self
        self.body = Some(body.to_string());
        self
    }
}
```

### When You MUST Be Explicit

1. **Returning a reference from multiple parameters:**
```rust
fn choose(a: &str, b: &str) -> &str { /* ... */ }  // Error!
fn choose<'a>(a: &'a str, b: &'a str) -> &'a str { /* ... */ }  // OK
```

2. **Storing references in structs:**
```rust
struct Holder<'a> {
    data: &'a str,  // Always explicit
}
```

3. **Returning references that outlive inputs:**
```rust
fn get_static() -> &'static str { /* ... */ }  // Must be explicit
```

4. **Complex lifetime relationships:**
```rust
fn complex<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str
//         ^^^^^^^ 'b must outlive 'a
```

### Elision and Trait Bounds

When implementing traits, elision still applies:

```rust
trait Messenger {
    fn send(&self, msg: &str) -> Result<(), &str>;
    // Elision applies here
}

impl Messenger for MyMessenger {
    fn send(&self, msg: &str) -> Result<(), &str> {
        // Elision applies here too
        Ok(())
    }
}
```

### The `'_` Lifetime in Paths

When specifying types with lifetimes, you can use `'_`:

```rust
fn process(msg: &Message<'_>) {
    // "I don't care about the specific lifetime"
}

fn collect(msgs: Vec<Message<'_>>) {
    // Same thing with collections
}
```

This is purely for readability - it makes clear that a lifetime is involved without cluttering code.

### Debugging Elision Errors

When the compiler says "missing lifetime specifier":

1. **Check if you have multiple input references** - If so, you probably need explicit annotations
2. **Check if output could come from different inputs** - The compiler needs to know which one
3. **Consider the relationship** - Does output data actually come from input data?

Example error:
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:38
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, 
          but the signature does not say whether it is borrowed from `x` or `y`
```

### Elision vs. Inference

Elision is a specific set of rules, not general type inference. The compiler doesn't "figure out" lifetimes by analyzing function bodies - it applies these three rules mechanically. If the rules don't cover your case, you must be explicit.

### Best Practices

1. **Use elision when possible** - It makes code cleaner
2. **Be explicit when ambiguous** - Future readers will thank you
3. **Use `'_` for clarity** - When lifetime presence matters but the specific name doesn't
4. **Document non-obvious lifetimes** - A comment explaining why lifetimes relate a certain way is helpful

### Exercise Preview

In this exercise, you'll:
- Identify where elision applies and where it doesn't
- Convert between explicit and elided forms
- Fix functions that need explicit lifetime annotations
- Work with Textio message parsing methods

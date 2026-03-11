# Exercise 3: Multiple Lifetimes

## Why Multiple Lifetimes?

Sometimes references in a function or struct have different valid scopes. Using a single lifetime parameter for all references can be overly restrictive. Multiple lifetime parameters allow you to express precise relationships between different references.

### The Problem with Single Lifetimes

Consider this scenario:

```rust
fn parse_with_context<'a>(message: &'a str, context: &'a str) -> &'a str {
    // Returns something from message
    message
}
```

This forces `message` and `context` to have the same lifetime. But what if `context` lives longer than `message`? The function would still work, but the single lifetime is unnecessarily restrictive.

### Using Multiple Lifetime Parameters

```rust
fn parse_with_context<'a, 'b>(message: &'a str, context: &'b str) -> &'a str {
    // message has lifetime 'a
    // context has lifetime 'b
    // They're independent!
    message
}
```

Now callers can use references with different lifetimes.

### When to Use Different Lifetimes

Use multiple lifetimes when:

1. **References are independent** - One doesn't affect the other's validity
2. **Output comes from one specific input** - Be precise about which one
3. **Structs hold unrelated references** - They might have different scopes

### Lifetime Subtyping

Lifetimes support subtyping: a longer lifetime can substitute for a shorter one.

```rust
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str { x }

// This works even though string literal has 'static lifetime
let result = foo("static", &local_string);
```

The `'static` lifetime (which lives forever) can be coerced to any shorter lifetime.

### Subtype Relationships

If `'long` is a longer lifetime than `'short`, then:
- `&'long T` is a subtype of `&'short T`
- You can use `&'long T` where `&'short T` is expected
- The opposite is NOT true

```rust
fn print_str<'short>(s: &'short str) {
    println!("{}", s);
}

fn main() {
    let long: &'static str = "I live forever";
    
    {
        let short = String::from("I live briefly");
        let short_ref: &str = &short;
        
        print_str(long);      // 'static coerced to 'short
        print_str(short_ref); // Exact match
    }
}
```

### Lifetime Bounds: `T: 'a`

When working with generic types that might contain references, you need lifetime bounds:

```rust
struct Container<'a, T: 'a> {
    value: &'a T,
}
```

`T: 'a` means "T cannot contain any references with lifetimes shorter than 'a". This ensures that if `T` contains references, they live long enough.

### Common Patterns

#### Pattern 1: Independent Inputs

```rust
fn combine<'a, 'b>(a: &'a str, b: &'b str) -> String {
    format!("{} {}", a, b)
}
```

Inputs have independent lifetimes, output owns its data.

#### Pattern 2: Output from One Input

```rust
fn annotate<'a, 'b>(text: &'a str, _context: &'b str) -> &'a str {
    text
}
```

Output lifetime matches `text`, `context` is independent.

#### Pattern 3: Multiple Outputs

```rust
fn split<'a>(text: &'a str, delim: char) -> (&'a str, &'a str) {
    match text.find(delim) {
        Some(pos) => (&text[..pos], &text[pos + 1..]),
        None => (text, ""),
    }
}
```

Both outputs share the input's lifetime.

#### Pattern 4: Struct with Multiple Lifetimes

```rust
struct ContextualMessage<'msg, 'ctx> {
    message: &'msg str,
    context: &'ctx str,
}

impl<'msg, 'ctx> ContextualMessage<'msg, 'ctx> {
    fn get_message(&self) -> &'msg str {
        self.message
    }
    
    fn get_context(&self) -> &'ctx str {
        self.context
    }
}
```

### Subtyping with `where 'a: 'b`

You can specify that one lifetime must outlive another:

```rust
fn select<'a, 'b>(outer: &'a str, inner: &'b str) -> &'a str
where
    'b: 'a,  // 'b must outlive 'a
{
    // Now we know 'b lives at least as long as 'a
    outer
}
```

This syntax `'b: 'a` reads as "'b outlives 'a" or "'b is at least as long as 'a".

### Practical Textio Example

Consider a message parser that maintains a context:

```rust
struct MessageParser<'config, 'cache> {
    config: &'config Config,      // Configuration lives for app lifetime
    cache: &'cache mut Cache,     // Cache might be recreated
}

impl<'config, 'cache> MessageParser<'config, 'cache> {
    fn parse(&mut self, raw: &str) -> ParsedMessage {
        // Use config (long-lived) and cache (might be shorter-lived)
        // to parse the raw message
    }
}
```

Here, `config` might have `'static` lifetime, while `cache` could be recreated periodically.

### Covariance and Contravariance

Rust references are **covariant** in their lifetime parameter:

```rust
// This works:
fn use_longer<'a, 'b: 'a>(x: &'a str, y: &'b str) {
    let _: &'a str = y;  // &'b str can be used as &'a str
}
```

Function pointers are **contravariant** in their argument lifetimes:

```rust
// A function expecting 'long can accept 'short
fn apply<'a, F>(f: F, s: &'a str) 
where 
    F: Fn(&'a str),
{
    f(s);
}
```

### Lifetime Variance in Structs

```rust
struct Container<'a> {
    data: &'a str,  // Covariant
}

fn covariance_example() {
    fn use_container<'short>(c: Container<'short>) {}
    
    let long_data = String::from("long lived");
    let container: Container<'static> = Container { data: &long_data };
    
    // Container<'static> can be used as Container<'short>
    use_container(container);
}
```

### When Multiple Lifetimes Matter

1. **API Flexibility** - Don't force callers to have references with matching lifetimes
2. **Precision** - Express exactly what's required, no more
3. **Performance** - Avoid unnecessary copying when lifetimes allow borrowing
4. **Correctness** - The compiler enforces your stated constraints

### Common Mistakes

**Mistake 1: Using same lifetime when different would work**
```rust
// Too restrictive
fn foo<'a>(x: &'a str, y: &'a str) -> String

// Better
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> String
```

**Mistake 2: Not matching output lifetime to correct input**
```rust
// Wrong - ties output to both inputs
fn first<'a>(x: &'a str, _y: &'a str) -> &'a str { x }

// Correct - output only from x
fn first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str { x }
```

**Mistake 3: Forgetting lifetime bounds on generics**
```rust
// May fail if T contains references
struct Wrapper<'a, T> {
    value: &'a T,
}

// Correct - T must not contain short-lived references
struct Wrapper<'a, T: 'a> {
    value: &'a T,
}
```

### Debugging Multiple Lifetime Issues

When the compiler complains about lifetime mismatches:

1. Draw the lifetime relationships
2. Check which input the output actually comes from
3. Consider if lifetimes should be independent
4. Use `where 'a: 'b` if one must outlive another

### Exercise Preview

In this exercise, you'll:
- Use multiple lifetime parameters in functions
- Create structs with independent lifetime parameters
- Implement methods with complex lifetime relationships
- Build a Textio message processor with context

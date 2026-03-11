# Exercise 4: The `'static` Lifetime

## What is `'static`?

The `'static` lifetime is special: it represents data that lives for the **entire duration of the program**. This is the longest possible lifetime - from program start to termination.

### Two Meanings of `'static`

The term "static" has two related meanings in Rust:

1. **Lifetime** - References with `'static` lifetime are valid for the entire program
2. **Storage** - Static items are stored in the program's binary and always exist

### String Literals are `'static`

Every string literal has `'static` lifetime:

```rust
let s: &'static str = "Hello, world!";
```

The string "Hello, world!" is stored directly in the compiled binary. It exists before `main()` runs and persists after `main()` ends. The reference to it is always valid.

### Where `'static` Appears

#### In String Literals
```rust
fn get_greeting() -> &'static str {
    "Hello!"  // String literals are always 'static
}
```

#### In Static Items
```rust
static GLOBAL_CONFIG: &str = "default";

fn get_config() -> &'static str {
    GLOBAL_CONFIG
}
```

#### In Constants
```rust
const MAX_MESSAGES: usize = 100;

fn get_max() -> usize {
    MAX_MESSAGES  // Constants are inlined but have 'static semantics
}
```

#### In Generic Bounds
```rust
fn store<T: 'static>(value: T) {
    // T cannot contain any non-'static references
}
```

### Understanding `T: 'static`

This bound doesn't mean "T must be `'static`". It means **"T cannot contain any non-`'static` references"**.

```rust
fn print_static<T: Debug + 'static>(value: &T) {
    println!("{:?}", value);
}

// This works: i32 has no references
print_static(&42);

// This works: String owns its data
print_static(&String::from("hello"));

// This FAILS: contains a non-'static reference
let s = String::from("temp");
let slice = &s[..];
print_static(&slice);  // Error: slice contains reference to local variable
```

### When to Use `'static`

#### Good Uses

**1. String constants:**
```rust
const API_VERSION: &'static str = "v1.0";
const ERROR_PREFIX: &'static str = "ERROR: ";
```

**2. Static configuration:**
```rust
static APP_CONFIG: AppConfig = AppConfig {
    name: "Textio",
    version: "2.0",
};
```

**3. Thread-safe shared data:**
```rust
static COUNTER: AtomicUsize = AtomicUsize::new(0);
```

**4. Return literals from functions:**
```rust
fn default_error() -> &'static str {
    "An unknown error occurred"
}
```

#### Problematic Uses

**1. "Fixing" lifetime errors by making things static:**
```rust
// DON'T do this to silence the compiler
fn bad() -> &'static str {
    let s = String::from("created at runtime");
    Box::leak(s.into_boxed_str())  // Memory leak!
}
```

**2. Overly restrictive bounds:**
```rust
// Too restrictive - many valid types won't work
fn process<T: 'static>(data: T) { }
```

### `'static` vs Owned Data

Sometimes returning owned data is better than `'static`:

```rust
// Returns 'static string
fn get_static_message() -> &'static str {
    "Hello"
}

// Returns owned String - more flexible
fn get_dynamic_message(name: &str) -> String {
    format!("Hello, {}", name)
}
```

Owned `String` can contain dynamic content, while `&'static str` can only be compile-time constants.

### Box::leak and Runtime `'static`

You can create `'static` references at runtime using `Box::leak`:

```rust
fn create_static() -> &'static str {
    let s = Box::new(String::from("I live forever"));
    Box::leak(s)  // Leaks memory, returns 'static reference
}
```

This **leaks memory** - the String is never deallocated. Use sparingly, typically for:
- Application-wide configuration loaded once
- Caches that live for program duration
- FFI with C libraries that don't manage memory

### Static Items vs Constants

| Feature | `static` | `const` |
|---------|----------|---------|
| Memory location | Fixed address | Inlined at compile time |
| Mutability | Can be mutable (unsafe) | Always immutable |
| Address identity | Has one address | No fixed address |
| Use case | Global state, FFI | Constants, compile-time values |

```rust
static mut COUNTER: i32 = 0;  // Mutable static (requires unsafe)
const MAX_SIZE: usize = 1024; // Constant
```

### `'static` in Traits and Generics

Some traits require `'static`:

```rust
// std::thread::spawn requires 'static
thread::spawn(|| {
    println!("Running in thread");
});

// Error: closure captures non-'static reference
let s = String::from("hello");
thread::spawn(|| {
    println!("{}", s);  // Error: s is moved or borrowed
});
```

### `'static` with Smart Pointors

```rust
// Arc enables shared ownership across threads
let data = Arc::new(String::from("shared"));
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    println!("{}", data_clone);
});
```

### Common Patterns in Textio

**Error messages:**
```rust
pub const ERR_INVALID_PHONE: &'static str = "Invalid phone number format";
pub const ERR_MESSAGE_TOO_LONG: &'static str = "Message exceeds maximum length";
```

**API endpoints:**
```rust
pub const API_BASE_URL: &'static str = "https://api.textio.com/v2";
pub const SANDBOX_URL: &'static str = "https://sandbox.textio.com/v2";
```

**Status codes:**
```rust
pub fn status_message(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status",
    }
}
```

### When NOT to Use `'static`

1. **Don't use it to silence lifetime errors** - Fix the underlying issue instead
2. **Don't require it unnecessarily** - `T: 'static` restricts what types can be used
3. **Don't leak memory casually** - `Box::leak` should be intentional

### Lifetime Coercion to `'static`

Remember that `'static` can be coerced to any shorter lifetime:

```rust
fn print_str(s: &str) {  // Takes any lifetime
    println!("{}", s);
}

fn main() {
    print_str("static string");  // &'static str coerced to &str
}
```

This is why you don't always need to annotate `'static` explicitly.

### Exercise Preview

In this exercise, you'll:
- Define static constants for Textio
- Use `'static` in function return types
- Understand when `T: 'static` is needed
- Work with static configuration data

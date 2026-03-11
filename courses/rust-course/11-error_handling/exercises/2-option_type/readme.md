# Exercise 2: Option Type in Rust

## Introduction

The `Option<T>` enum is Rust's solution to the billion-dollar mistake: null references. Instead of null, Rust uses `Option` to represent the presence or absence of a value. This makes null-like situations explicit and type-safe.

## The Option Enum

```rust
enum Option<T> {
    Some(T),  // Contains a value
    None,     // Contains no value
}
```

## Why No Null?

Tony Hoare, who introduced null references in 1965, called them his "billion-dollar mistake." Problems with null:

1. **Runtime errors**: Null pointer exceptions crash programs
2. **No type safety**: You can call methods on null
3. **Ambiguity**: Is null an error? Missing value? Not initialized?
4. **Easy to forget**: Null checks are optional

Rust solves this by:
- Making absence explicit in the type system
- Forcing you to handle the None case
- Preventing null pointer dereferences at compile time

## Creating Options

```rust
let some_value: Option<i32> = Some(42);
let no_value: Option<i32> = None;
```

## Pattern Matching Options

```rust
fn describe(option: Option<i32>) {
    match option {
        Some(value) => println!("Got: {}", value),
        None => println!("Got nothing"),
    }
}
```

## Checking Option State

### is_some() and is_none()

```rust
let some = Some(5);
if some.is_some() {
    println!("Has a value!");
}

let none: Option<i32> = None;
if none.is_none() {
    println!("No value!");
}
```

## Extracting Values

### unwrap()

Panics on None:

```rust
let some = Some(42);
let value = some.unwrap(); // Returns 42

let none: Option<i32> = None;
// none.unwrap(); // PANICS!
```

### expect()

Like unwrap, but with a custom message:

```rust
let none: Option<i32> = None;
// none.expect("Value should exist"); // PANICS with your message
```

### unwrap_or()

Provides a default value:

```rust
let none: Option<i32> = None;
let value = none.unwrap_or(0); // Returns 0
```

### unwrap_or_default()

Uses the type's default:

```rust
let none: Option<i32> = None;
let value = none.unwrap_or_default(); // Returns 0 (i32's default)

let none_str: Option<String> = None;
let s = none_str.unwrap_or_default(); // Returns "" (String's default)
```

### unwrap_or_else()

Computes default with a closure:

```rust
let none: Option<i32> = None;
let value = none.unwrap_or_else(|| {
    println!("Computing default");
    42
});
```

## Option Combinators

Combinators let you transform and chain Options without explicit pattern matching.

### map()

Transforms the value inside Some, returns None unchanged:

```rust
let some = Some(5);
let doubled = some.map(|x| x * 2); // Some(10)

let none: Option<i32> = None;
let still_none = none.map(|x| x * 2); // None
```

### and_then()

Chains operations that return Option (also called flatMap):

```rust
fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn half_if_even(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}

let result = parse_int("10")
    .and_then(half_if_even); // Some(5)

let odd = parse_int("5")
    .and_then(half_if_even); // None
```

### or()

Returns the Option if Some, otherwise returns the alternative:

```rust
let none: Option<i32> = None;
let backup = none.or(Some(42)); // Some(42)

let some = Some(5);
let same = some.or(Some(42)); // Some(5)
```

### or_else()

Computes alternative with a closure:

```rust
let none: Option<i32> = None;
let computed = none.or_else(|| Some(42)); // Some(42)
```

### filter()

Keeps the value only if predicate returns true:

```rust
let some = Some(10);
let kept = some.filter(|&x| x > 5); // Some(10)

let filtered = some.filter(|&x| x > 15); // None
```

## Converting Between Option and Result

### ok_or()

Converts Option to Result:

```rust
let some = Some(42);
let result = some.ok_or("Not found"); // Ok(42)

let none: Option<i32> = None;
let error = none.ok_or("Not found"); // Err("Not found")
```

### ok_or_else()

Computes error with a closure:

```rust
let none: Option<i32> = None;
let error = none.ok_or_else(|| format!("Error at line {}", 10));
```

## Map vs and_then

The key difference:

```rust
// map: function returns T
some.map(|x| x * 2)     // Option<T> -> Option<T>

// and_then: function returns Option<T>
some.and_then(|x| Some(x * 2))  // Option<T> -> Option<T>
```

Use `map` when your transformation doesn't fail.
Use `and_then` when your transformation might return None.

## Textio SMS API Examples

### Finding a Contact

```rust
struct Contact {
    name: String,
    phone: String,
}

fn find_contact(name: &str) -> Option<Contact> {
    // Search database...
    if found {
        Some(contact)
    } else {
        None
    }
}

// Usage
let phone = find_contact("Alice")
    .map(|c| c.phone)
    .unwrap_or(String::from("Unknown"));
```

### Message Template

```rust
fn get_template(name: &str) -> Option<String> {
    templates.get(name).cloned()
}

fn format_message(template_name: &str, contact: &Contact) -> String {
    get_template(template_name)
        .map(|t| t.replace("{name}", &contact.name))
        .unwrap_or(format!("Hi {}!", contact.name))
}
```

## Common Patterns

### Chaining Multiple Lookups

```rust
fn find_user(id: u64) -> Option<User> { ... }
fn get_settings(user: &User) -> Option<Settings> { ... }
fn get_theme(settings: &Settings) -> Option<Theme> { ... }

let theme = find_user(id)
    .and_then(|u| get_settings(&u))
    .and_then(|s| get_theme(&s));
```

### Providing Fallbacks

```rust
let value = primary_source
    .or(secondary_source)
    .or_else(|| get_default())
    .unwrap_or(DEFAULT_VALUE);
```

### Conditional Transformation

```rust
let result = parse_id(input)
    .filter(|&id| id > 0)
    .and_then(|id| find_record(id));
```

## Best Practices

1. **Use Option for optional values**, not Result
2. **Use combinators** (map, and_then) instead of nested matches
3. **Prefer unwrap_or and or** over unwrap when you have defaults
4. **Use ok_or to convert** to Result when you need error context
5. **Pattern match** when you need to do different things for Some/None

## Option vs Result

| Use Option when... | Use Result when... |
|-------------------|-------------------|
| Value might be missing | Operation might fail |
| No error context needed | Error information is important |
| "Not found" is normal | "Not found" is an error |
| Examples: lookup, first item | Examples: parsing, I/O, network |

## Exercise Task

In this exercise, you'll implement contact and message lookup for Textio:

1. `find_contact` - Look up contacts by name
2. `get_message_status` - Check message delivery status
3. `format_message` - Create personalized messages using combinators
4. `find_and_send` - Chain lookups with and_then

You'll practice:
- Creating Option values
- Using `map()` and `and_then()`
- Converting Option to Result with `ok_or()`
- Using `unwrap_or()` and `unwrap_or_default()`

Run your code with:
```bash
rustc code.rs && ./code
```

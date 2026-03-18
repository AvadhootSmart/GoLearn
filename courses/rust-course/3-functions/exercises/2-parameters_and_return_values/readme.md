# Function Parameters and Return Values

## Why this matters
Functions are Rust's main abstraction unit. Explicit parameter and return types make code predictable, easier to refactor, and safer to call.

## Core concept
A function signature tells callers exactly:
- what input it needs,
- what output it returns.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

## Worked example
```rust
fn build_message(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let msg = build_message("Textio");
    println!("{}", msg);
}
```

## Common mistakes
- Returning `&str` that points to temporary data created inside the function.
- Adding `return` and semicolon inconsistently (final expression style is often clearer).

## Exercise task

1. Write a function `build_message` that takes `name: &str` and returns `String`.
2. Return `format!("Hello, {}!", name)`.
3. In `main`, call the function with `"Textio"` and print the result.

## Quick recap
Rust function signatures are explicit contracts: inputs in, output out, with type safety at compile time.

# Functions in Rust

## Concept first: defining functions and returning values

Functions are declared with `fn`. In Rust, the last expression in a function can be returned implicitly (without `return` and without a semicolon).

Example:

```rust
fn get_greeting() -> String {
    String::from("Welcome to Textio!")
}

fn main() {
    let greeting = get_greeting();
    println!("{}", greeting);
}
```

Nuances:

- `-> String` declares the return type.
- `String::from("...")` creates an owned `String`.
- Adding a trailing semicolon to the last line would turn it into a statement and break implicit return.

## Exercise task

1. Write a function called `get_greeting` that takes no arguments and returns a `String`.
2. Inside `get_greeting`, return the string `"Welcome to Textio!"`.
3. In `main`, call `get_greeting` and print the result.

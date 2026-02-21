# Structs in Rust

## Concept first: modeling related data

A `struct` groups related fields under one named type. This is useful when several values represent one concept.

Example:

```rust
struct Account {
    id: u32,
    email: String,
}

fn main() {
    let account = Account {
        id: 42,
        email: String::from("dev@example.com"),
    };

    println!("{}", account.email);
}
```

Nuances:

- Field names are required when constructing a normal struct.
- Field access uses dot syntax (`account.email`).
- Struct fields can mix types (numbers, strings, booleans, etc.) as long as each field type is explicit.

## Exercise task

1. Define a struct `Message` with two fields: `body` (a `String`) and `recipient` (a `String`).
2. In `main`, instantiate a `Message` with `body` set to `"Hey there"` and `recipient` to `"555-0199"`.
3. Print the body of the message.

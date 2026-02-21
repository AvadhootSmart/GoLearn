# Structs in Rust

## Concept first: modeling related data

A `struct` groups related fields under one named type. This is useful when several values represent one concept.

Example:

```rust
struct Message {
    body: String,
    recipient: String,
}

fn main() {
    let msg = Message {
        body: String::from("Hey there"),
        recipient: String::from("555-0199"),
    };

    println!("{}", msg.body);
}
```

Nuances:

- Field names are required when constructing a normal struct.
- Field access uses dot syntax (`msg.body`).
- `String` fields own their data, which works well with Rust ownership rules.

## Exercise task

1. Define a struct `Message` with two fields: `body` (a `String`) and `recipient` (a `String`).
2. In `main`, instantiate a `Message` with `body` set to `"Hey there"` and `recipient` to `"555-0199"`.
3. Print the body of the message.

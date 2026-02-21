# Welcome to "Learn Rust"

_This course assumes you're familiar with programming basics. If you're new to coding, consider starting with a beginner-friendly programming course first._

![rust logo](https://rust-lang.org/logos/rust-logo-blk.svg)

## Concept first: printing output in Rust

In Rust, `println!` is a macro used to print text to standard output. You can think of it as the quickest way to verify your code is running and to communicate program state.

Example:

```rust
fn main() {
    println!("starting Textio server");
}
```

A few nuances:

- `println!` ends output with a newline.
- String text goes in double quotes.
- The `!` matters: `println!` is a macro, not a normal function.

## Exercise task

All the code in this course represents parts of an SMS API called **Textio**.

Update the existing program so it logs:

`starting Textio server`

instead of:

`hello world`.

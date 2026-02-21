# Ownership in Rust

## Concept first: moves and single ownership

Ownership is a core Rust rule set that guarantees memory safety without a garbage collector.

For heap-allocated values like `String`, assignment usually **moves** ownership.

Example:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moved from s1 to s2

    println!("{}", s2);
    // println!("{}", s1); // compile error: s1 no longer owns the value
}
```

Nuances:

- Only one owner exists at a time.
- When the owner goes out of scope, Rust drops the value.
- Types like integers are `Copy`, so assignment behaves differently than `String`.

## Exercise task

1. Create a `String` named `s1` with the value `"hello"`.
2. Move the ownership of `s1` to `s2`.
3. Print `s2` to verify it owns the value. (Attempting to use `s1` afterward should fail at compile time.)

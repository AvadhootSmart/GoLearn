# Mutability and Shadowing

## Concept first: controlled change in bindings

By default, Rust variables are immutable. Use `mut` when a value must change. Shadowing (`let name = ...;`) creates a new binding with the same name, often with a different value or type.

Example:

```rust
fn main() {
    let mut attempts = 1;
    attempts += 1;

    let attempts = attempts.to_string();
    println!("{}", attempts);
}
```

Nuances:

- `mut` changes a binding's value in place.
- Shadowing creates a brand-new binding.
- Shadowing can change types, mutation cannot.

## Exercise task

1. Create a mutable variable `count` with value `1`.
2. Increment `count` by `2`.
3. Shadow `count` with a string using `format!("count={}", count)`.
4. Print it.

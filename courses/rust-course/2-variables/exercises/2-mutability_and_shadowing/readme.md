# Mutability and Shadowing

## Why this matters
Rust defaults to immutability so accidental state changes are prevented. You opt into change with `mut`, and you use shadowing when you want a new binding (possibly with a new type) while reusing a familiar name.

## Core concept
- `mut` means the same variable binding can change value.
- Shadowing (`let x = ...;` again) creates a brand-new binding.
- Shadowing can change type; mutation cannot.

## Worked example
```rust
fn main() {
    let mut attempts = 1;
    attempts += 1; // same binding, new value

    let attempts = attempts.to_string(); // new binding, different type
    println!("{}", attempts);
}
```

## Common mistakes
- Forgetting `mut` and then trying to assign (`cannot assign twice to immutable variable`).
- Expecting shadowed and original bindings to both remain mutable in the same way.

## Exercise task

1. Create a mutable variable `count` with value `1`.
2. Increment `count` by `2`.
3. Shadow `count` with a string using `format!("count={}", count)`.
4. Print it.

## Quick recap
Use `mut` for in-place value changes and shadowing for a fresh binding, especially when transforming type or meaning.

# Vectors in Rust

## Concept first: growable lists with `Vec<T>`

A vector stores multiple values of the same type and can grow or shrink at runtime.

Example:

```rust
fn main() {
    let mut user_ids: Vec<i32> = Vec::new();
    user_ids.push(101);
    user_ids.push(102);
    user_ids.push(103);

    for id in user_ids {
        println!("{}", id);
    }
}
```

Nuances:

- Vectors are generic (`Vec<T>`), so all elements share one type.
- You usually need `mut` when pushing/removing elements.
- Iteration can consume, borrow immutably, or borrow mutably depending on how you loop.

## Exercise task

1. In `main`, create a new mutable vector of integers called `user_ids` using `Vec::new()`.
2. Push three IDs to the vector: `101`, `102`, and `103`.
3. Iterate over the vector and print each ID.

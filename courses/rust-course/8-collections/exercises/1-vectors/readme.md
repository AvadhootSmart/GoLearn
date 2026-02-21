# Vectors in Rust

## Concept first: growable lists with `Vec<T>`

A vector stores multiple values of the same type and can grow or shrink at runtime.

Example:

```rust
fn main() {
    let mut scores: Vec<i32> = Vec::new();
    scores.push(10);
    scores.push(20);
    scores.push(30);

    for score in &scores {
        println!("{}", score);
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

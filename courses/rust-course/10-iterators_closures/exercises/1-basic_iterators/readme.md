# Iterators and Closures

## Concept first: lazy iteration and inline behavior

Iterators in Rust are lazy: creating an iterator alone does nothing until a consuming adapter (like `collect`) is called. Closures are inline anonymous functions used with iterator adapters.

Example:

```rust
fn main() {
    let prices = vec![10, 20, 30];
    let discounted: Vec<i32> = prices
        .iter()
        .map(|p| p - 1)
        .collect();

    println!("{:?}", discounted);
}
```

Nuances:

- `.iter()` yields references (`&i32`) for each item, so the original vector is not consumed.
- `map` transforms each element and returns a new iterator.
- `collect` materializes the final collection and often needs a target type.

## Exercise task

1. Create a vector `numbers` containing `1, 2, 3`.
2. Use `.iter().map()` with a closure to add `1` to each number, and `.collect()` it into a new vector `added_one`.
3. Print the `added_one` vector using `println!("{:?}", added_one);`.

# Smart Pointers

## Concept first: heap allocation with `Box<T>`

`Box<T>` stores data on the heap while keeping a pointer on the stack. It's useful when you need a known-size pointer but dynamically allocated data.

Example:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

Nuances:

- `Box<T>` owns its heap value and drops it automatically.
- You can often use `b` like a normal value thanks to deref coercions.
- `Box` is commonly used in recursive data structures and trait objects.

## Exercise task

1. Use `Box::new` to allocate an `i32` with the value `5` on the heap, binding it to the variable `b`.
2. Print the value of `b` by using string interpolation `println!("b = {}", b);`.

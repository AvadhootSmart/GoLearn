# Traits in Rust

## Concept first: shared behavior through traits

A trait defines a shared set of methods that different types can implement.

Example:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Notification;

impl Summary for Notification {
    fn summarize(&self) -> String {
        String::from("Notification summary")
    }
}
```

Nuances:

- Traits specify behavior, not data layout.
- `&self` means the method borrows the instance immutably.
- Trait-based design enables polymorphism while keeping types explicit.

## Exercise task

1. Define a trait `Summary` with a single method signature `fn summarize(&self) -> String;`.
2. Given a `NewsArticle` struct, implement the `Summary` trait for it to return `"Article summary"`.
3. In `main`, call `.summarize()` on an instance of `NewsArticle` and print the result.

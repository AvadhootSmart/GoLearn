# Traits in Rust

Traits are a way to define shared behavior in Rust. They are similar to interfaces in other languages.

## Assignment

1. Define a trait `Summary` with a single method signature `fn summarize(&self) -> String;`.
2. Given a `NewsArticle` struct, implement the `Summary` trait for it to return `"Article summary"`.
3. In `main`, call `.summarize()` on an instance of `NewsArticle` and print the result.

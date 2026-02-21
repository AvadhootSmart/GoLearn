# Ownership in Rust

Ownership is Rust's most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.

## Assignment

1. Create a `String` named `s1` with the value `"hello"`.
2. Move the ownership of `s1` to `s2`.
3. Print `s2` to verify it owns the value. (Note: attempting to print `s1` here would cause a compile-time error!)

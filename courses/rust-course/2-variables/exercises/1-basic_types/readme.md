# Variables in Rust

## Concept first: mutability and type inference

Rust variables are immutable by default. This helps prevent accidental state changes. When you need to change a value, declare it with `mut`.

Example:

```rust
fn main() {
    let server_name = "Textio"; // immutable
    let mut online_users = 0;    // mutable

    online_users = 1;

    println!("{} has {} users online", server_name, online_users);
}
```

Nuances:

- `let` creates a new binding.
- `mut` applies to the binding, not the underlying type.
- Rust often infers types (`0` becomes `i32` by default unless context suggests otherwise).

## Exercise task

1. Declare an immutable variable `server_name` and set it to `"Textio"`.
2. Declare a mutable variable `online_users` and set it to `0`.
3. Update `online_users` to `1`.
4. Print both variables (the `println!` code is provided).

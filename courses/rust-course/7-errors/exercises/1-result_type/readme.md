# Error Handling with `Result`

## Concept first: recoverable errors

Rust uses `Result<T, E>` to represent operations that can succeed (`Ok(T)`) or fail (`Err(E)`). This makes error handling explicit.

Example:

```rust
fn read_config() -> Result<String, String> {
    Ok(String::from("config loaded"))
}

fn main() {
    match read_config() {
        Ok(value) => println!("Success: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
```

Nuances:

- Both success and failure types are part of the function signature.
- `match` forces you to think about both cases.
- As you advance, you'll often use `?` to propagate errors.

## Exercise task

1. Write a function `read_config` that returns `Result<String, String>`.
2. Inside the function, return `Ok(String::from("config loaded"))`.
3. In `main`, call `read_config` and use a `match` expression to print the success or error message.

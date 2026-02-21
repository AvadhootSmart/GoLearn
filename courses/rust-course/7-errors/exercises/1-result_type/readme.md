# Error Handling with `Result`

## Concept first: recoverable errors

Rust uses `Result<T, E>` to represent operations that can succeed (`Ok(T)`) or fail (`Err(E)`). This makes error handling explicit.

Example:

```rust
fn parse_port() -> Result<u16, String> {
    Ok(8080)
}

fn main() {
    match parse_port() {
        Ok(port) => println!("Port: {}", port),
        Err(problem) => println!("Error: {}", problem),
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

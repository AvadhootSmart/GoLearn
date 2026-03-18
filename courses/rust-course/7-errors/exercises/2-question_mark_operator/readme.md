# Propagating Errors with `?`

## Why this matters
Manual `match`-based error handling gets noisy fast. The `?` operator keeps code linear while still returning errors explicitly.

## Core concept
`?` on a `Result<T, E>` does:
- `Ok(v)` -> continues with `v`
- `Err(e)` -> returns early with `Err(e)`

## Worked example
```rust
fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    let port = input.parse::<u16>()?;
    Ok(port)
}
```

## Common mistakes
- Using `?` in a function that does not return `Result`/`Option` compatible types.
- Hiding error context when crossing layers (wrap with context when needed).

## Exercise task

1. Write `parse_port(input: &str) -> Result<u16, std::num::ParseIntError>`.
2. Parse the input with `input.parse::<u16>()?`.
3. In `main`, call `parse_port("8080")` and print `port <value>` on success.

## Quick recap
Use `?` for idiomatic early-return error propagation and cleaner control flow.

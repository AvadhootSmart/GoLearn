# Enums with Associated Data

## Why this matters
Real systems often have one value that can be in multiple states. Enums model those states safely, and associated data lets each variant carry exactly what it needs.

## Core concept
Enums are tagged unions:
- one type,
- multiple variants,
- each variant may hold different data.

## Worked example
```rust
enum ApiResponse {
    Ok(u16),
    Err(String),
}

fn print_response(r: ApiResponse) {
    match r {
        ApiResponse::Ok(code) => println!("status {}", code),
        ApiResponse::Err(msg) => println!("error {}", msg),
    }
}
```

## Common mistakes
- Forgetting to handle all variants in `match`.
- Using many booleans/optional fields where an enum would be clearer.

## Exercise task

1. Define enum `ApiResponse` with variants `Ok(u16)` and `Err(String)`.
2. Write `print_response` using `match` to print `status <code>` or `error <message>`.
3. In `main`, call it with `ApiResponse::Ok(200)`.

## Quick recap
Enums model mutually exclusive states with compile-time exhaustiveness checks.

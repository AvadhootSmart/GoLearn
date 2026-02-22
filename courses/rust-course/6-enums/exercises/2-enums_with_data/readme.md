# Enums with Associated Data

## Concept first: variants can carry payloads

Enum variants can store data, letting one type represent related states with different fields.

## Exercise task

1. Define enum `ApiResponse` with variants `Ok(u16)` and `Err(String)`.
2. Write `print_response` using `match` to print `status <code>` or `error <message>`.
3. In `main`, call it with `ApiResponse::Ok(200)`.

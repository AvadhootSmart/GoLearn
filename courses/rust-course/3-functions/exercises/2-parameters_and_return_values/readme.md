# Function Parameters and Return Values

## Concept first: explicit function signatures

Rust function signatures declare both parameter types and return type. This keeps APIs predictable and readable.

## Exercise task

1. Write a function `build_message` that takes `name: &str` and returns `String`.
2. Return `format!("Hello, {}!", name)`.
3. In `main`, call the function with `"Textio"` and print the result.

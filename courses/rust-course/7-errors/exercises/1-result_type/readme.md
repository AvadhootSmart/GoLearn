# Error Handling with Result

In Rust, recoverable errors are typically handled using the `Result` enum. It has two variants: `Ok` and `Err`.

## Assignment

1. Write a function `read_config` that returns `Result<String, String>`.
2. Inside the function, simply return `Ok(String::from("config loaded"))`.
3. In `main`, call `read_config` and use a `match` expression to print the success or error message.

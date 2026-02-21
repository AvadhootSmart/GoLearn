# Modules and Cargo

Rust's module system lets you organize code into separate files and paths. `cargo` is the package manager and build system. Here, we'll practice defining a nested module.

## Assignment

1. Define a `mod server` block.
2. Inside `server`, define a `pub mod network`.
3. Inside `network`, define a `pub fn connect() { println!("connected"); }`.
4. In `main`, call the `connect` function using its absolute path `crate::server::network::connect()`.

# Modules and Cargo

## Concept first: organizing code with modules

Rust modules (`mod`) create namespaces so related code can be grouped and addressed by paths.

Example:

```rust
mod analytics {
    pub mod metrics {
        pub fn record() {
            println!("metric recorded");
        }
    }
}

fn main() {
    crate::analytics::metrics::record();
}
```

Nuances:

- `pub` controls visibility across module boundaries.
- `crate::` starts from the crate root (an absolute path).
- Good module structure makes larger Rust projects easier to navigate and test.

## Exercise task

1. Define a `mod server` block.
2. Inside `server`, define a `pub mod network`.
3. Inside `network`, define a `pub fn connect() { println!("connected"); }`.
4. In `main`, call the `connect` function using its absolute path `crate::server::network::connect()`.

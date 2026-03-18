# `use` Paths and Module Access

## Why this matters
As projects grow, fully qualified paths become noisy. `use` imports keep code readable while preserving Rust's explicit module system.

## Core concept
`use` brings items into local scope:
```rust
use crate::server::network::connect;
```
This allows `connect()` instead of `crate::server::network::connect()`.

## Worked example
```rust
mod server {
    pub mod network {
        pub fn connect() {
            println!("connected");
        }
    }
}

use crate::server::network::connect;

fn main() {
    connect();
    println!("boot complete");
}
```

## Common mistakes
- Importing private items from another module.
- Confusing `crate::`, `self::`, and `super::` path bases.

## Exercise task

1. Keep the provided module tree.
2. Add a `use crate::server::network::connect;` line.
3. In `main`, call `connect();` and print `boot complete` afterward.

## Quick recap
Use imports to shorten paths, but keep visibility and module boundaries explicit.

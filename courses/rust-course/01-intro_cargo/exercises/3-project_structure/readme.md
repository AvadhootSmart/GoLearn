# Rust Project Structure

## Understanding Code Organization

As projects grow, organizing code becomes critical. Rust provides clear conventions for structuring projects.

## The src/ Directory

### Binary Projects

For executable programs:

```
my_app/
├── Cargo.toml
└── src/
    ├── main.rs      # Entry point for binary
    ├── lib.rs       # Optional: library code
    ├── config.rs    # Module: config
    ├── api/
    │   ├── mod.rs   # Module: api (exposes sub-modules)
    │   ├── routes.rs
    │   └── handlers.rs
    └── utils/
        └── mod.rs   # Module: utils
```

### Library Projects

For libraries (created with `cargo new my_lib --lib`):

```
my_lib/
├── Cargo.toml
└── src/
    └── lib.rs       # Entry point for library
```

## Module Basics

### Creating a Module

**Method 1: File as Module**

```rust
// src/config.rs
pub fn load() -> String {
    String::from("config loaded")
}
```

```rust
// src/main.rs
mod config;  // Declares the module

fn main() {
    let cfg = config::load();
    println!("{}", cfg);
}
```

**Method 2: Directory with mod.rs**

```
src/
├── main.rs
└── api/
    ├── mod.rs       # Declares what's public
    ├── routes.rs    # Private by default
    └── handlers.rs
```

```rust
// src/api/mod.rs
mod routes;
mod handlers;

pub use routes::Routes;
pub use handlers::handle;
```

### Visibility Keywords

```rust
pub fn public() {}        // Visible everywhere
fn private() {}           // Visible only in this module
pub(crate) fn crate_fn()  // Visible within this crate
pub(super) fn parent_fn() // Visible in parent module
```

## The `use` Keyword

Bring items into scope:

```rust
use std::collections::HashMap;
use std::fs::{self, File};  // Multiple items
use std::io::Read as IoRead;  // Alias

fn main() {
    let mut map = HashMap::new();
    // Instead of: std::collections::HashMap::new()
}
```

## Common Standard Library Imports

```rust
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::error::Error;
use std::fmt::{Display, Debug};
use std::clone::Clone;
use std::cmp::{PartialEq, Eq, Ordering};
```

## Project Organization Patterns

### Pattern 1: Small Project

```
src/
├── main.rs
├── config.rs
└── utils.rs
```

Good for: CLI tools, small scripts

### Pattern 2: Medium Project

```
src/
├── main.rs
├── lib.rs
├── api/
│   ├── mod.rs
│   ├── routes.rs
│   └── handlers.rs
├── db/
│   └── mod.rs
└── models/
    └── mod.rs
```

Good for: Web services, larger applications

### Pattern 3: Workspace

```
workspace/
├── Cargo.toml          # Workspace manifest
├── crate_one/
│   ├── Cargo.toml
│   └── src/
└── crate_two/
    ├── Cargo.toml
    └── src/
```

Good for: Multiple related crates

## The `crate` Keyword

Reference items from the crate root:

```rust
// From anywhere in your crate:
crate::api::handlers::process()

// vs relative path:
super::config::load()  // Parent module
self::routes::init()   // Current module
```

## Comments

```rust
// Single line comment

/* Multi-line
   comment */

/// Documentation comment (supports Markdown)
/// Appears in `cargo doc` output
/// 
/// # Examples
/// ```
/// let x = my_function();
/// ```
pub fn my_function() -> i32 {
    42
}

//! Module-level documentation
//! Describes the entire module/crate
```

## Mental Model: Module Tree

```
crate (root)
├── main.rs or lib.rs
├── config (module)
│   └── load() (function)
├── api (module)
│   ├── routes (submodule)
│   └── handlers (submodule)
└── models (module)
    ├── User (struct)
    └── Message (struct)
```

Access path: `crate::api::routes::init()`

## Common Pitfalls

### 1. Forgetting `pub`
```rust
// config.rs
fn load() {}  // Private!

// main.rs
mod config;
fn main() {
    config::load();  // ERROR: function is private
}
```

### 2. Not Declaring Modules
```rust
// main.rs
fn main() {
    config::load();  // ERROR: unresolved name
}

// Fix: add `mod config;`
```

### 3. Circular Dependencies
```
A depends on B, B depends on A
```
Solution: Extract shared code to a third module.

## Real-World Example: Textio Structure

```
textio/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root
│   ├── config.rs        # Configuration handling
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs    # HTTP routes
│   │   └── handlers.rs  # Request handlers
│   ├── db/
│   │   ├── mod.rs
│   │   └── models.rs    # Database models
│   ├── sms/
│   │   └── mod.rs       # SMS sending logic
│   └── utils/
│       └── mod.rs       # Helper functions
├── tests/
│   └── integration.rs   # Integration tests
└── examples/
    └── basic.rs         # Usage example
```

## Exercise Task

You'll create a simple multi-file project structure:

1. Create a module called `greeting` with a function `say_hello()` that returns `"Hello from Textio!"`
2. In `main.rs`, use the `greeting` module
3. Call `say_hello()` and print the result

The code structure shows how modules work. You need to:
1. Define the public function in the greeting module section
2. Call it from main using the proper path

Remember: use `pub fn` to make the function accessible from outside the module.

# Exercise 1: Modules in Rust

## Overview

Modules are Rust's way of organizing code into logical units. They help you manage complexity, control privacy, and create reusable components. In the Textio SMS API project, we'll use modules to organize different parts of our application like message handling, user management, and delivery tracking.

## The Module System

Rust's module system consists of several components:

1. **Modules** - Named organizational units
2. **Crates** - Compilation units (libraries or binaries)
3. **Packages** - Cargo projects containing one or more crates
4. **Paths** - Way to name items in the module tree

## Creating Modules

### Inline Modules

The simplest way to create a module is inline:

```rust
mod messaging {
    fn send_sms() {
        println!("Sending SMS...");
    }
}
```

### External Modules

For larger modules, use separate files:

```rust
// main.rs
mod messaging;  // Looks for messaging.rs or messaging/mod.rs

fn main() {
    messaging::send_sms();
}
```

```
// messaging.rs
pub fn send_sms() {
    println!("Sending SMS...");
}
```

## File Organization Patterns

Rust supports two patterns for organizing modules:

### Pattern 1: File as Module

```
src/
├── main.rs
└── messaging.rs      // mod messaging;
```

### Pattern 2: Directory with mod.rs

```
src/
├── main.rs
└── messaging/
    ├── mod.rs        // mod messaging;
    ├── sms.rs
    └── mms.rs
```

### Pattern 3: Directory with named file (Rust 2018+)

```
src/
├── main.rs
└── messaging/
    ├── mod.rs or messaging.rs (in parent)
    └── sms.rs
```

## Privacy and Visibility

By default, everything in Rust is private. Use `pub` to make items public:

### Visibility Modifiers

```rust
pub fn public_function() {}           // Visible everywhere
fn private_function() {}              // Visible only in current module
pub(crate) fn crate_function() {}     // Visible anywhere in crate
pub(super) fn parent_function() {}    // Visible in parent module
pub(in path::to::module) fn path_fn() {} // Visible in specific path
```

### Visibility Levels

| Modifier | Scope |
|----------|-------|
| `pub` | Everywhere |
| `pub(crate)` | Current crate only |
| `pub(super)` | Parent module only |
| `pub(self)` | Current module (default) |
| `pub(in path)` | Specified module path |

## The `use` Keyword

The `use` keyword creates shortcuts to paths:

### Basic Usage

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```

### Multiple Items

```rust
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
```

### Renaming

```rust
use std::io::Result as IoResult;
```

### Re-exports

```rust
mod internal {
    pub fn deep_function() {}
}

pub use internal::deep_function;  // Re-export at this level
```

## Module Paths

Rust provides keywords for navigating the module tree:

### `self`

Refers to the current module:

```rust
mod messaging {
    pub fn send() {}
    
    pub fn process() {
        self::send();  // Call send in current module
    }
}
```

### `super`

Refers to the parent module:

```rust
mod api {
    pub fn base_url() -> String {
        String::from("https://api.textio.com")
    }
    
    pub mod messaging {
        pub fn endpoint() -> String {
            format!("{}/sms", super::base_url())
        }
    }
}
```

### `crate`

Refers to the crate root:

```rust
mod models {
    pub struct Message {
        pub content: String,
    }
}

mod api {
    use crate::models::Message;  // Start from crate root
    
    pub fn send(msg: Message) {
        println!("Sending: {}", msg.content);
    }
}
```

## The `mod` Keyword

### Declaring Modules

```rust
mod messaging;     // External module (file)
mod analytics {    // Inline module
    pub fn track() {}
}
```

### Conditional Compilation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_send() {}
}
```

## Module Tree Example

Here's how Textio might organize its module tree:

```
textio (crate root)
├── models
│   ├── message
│   └── user
├── api
│   ├── sms
│   └── webhooks
└── utils
    ├── validation
    └── formatting
```

Corresponding code:

```rust
// src/lib.rs or src/main.rs

pub mod models {
    pub mod message {
        pub struct Message {
            pub to: String,
            pub body: String,
        }
    }
    
    pub mod user {
        pub struct User {
            pub id: u64,
            pub phone: String,
        }
    }
}

pub mod api {
    pub mod sms {
        use crate::models::message::Message;
        
        pub fn send(msg: Message) -> Result<(), String> {
            Ok(())
        }
    }
    
    pub mod webhooks {
        pub fn register(url: &str) {
            println!("Registered webhook: {}", url);
        }
    }
}
```

## Re-exports for Clean APIs

Use re-exports to simplify your public API:

```rust
mod internal {
    pub mod deep {
        pub struct ComplexType {}
    }
}

// Users don't need to know the internal structure
pub use internal::deep::ComplexType;
```

## Best Practices

1. **Organize by functionality** - Group related code together
2. **Minimize public API** - Only expose what's necessary
3. **Use re-exports** - Create a clean external interface
4. **Leverage `use`** - Don't repeat long paths
5. **Consider visibility** - Use appropriate `pub` variants

## Common Patterns

### The Prelude Pattern

```rust
// src/lib.rs
mod error;
mod client;
mod types;

pub use error::{Error, Result};
pub use client::Client;
pub use types::*;
```

### The Facade Pattern

```rust
pub mod messaging {
    // Re-export commonly used items
    pub use super::internal::sms::*;
    pub use super::models::Message;
}
```

## Exercise Task

In this exercise, you'll organize a Textio SMS API project into proper modules. You'll:

1. Create module structure for the API
2. Use appropriate visibility modifiers
3. Set up proper `use` declarations
4. Navigate the module tree with `self`, `super`, and `crate`

The starter code contains flat functions and types that need to be reorganized into a proper module structure with correct privacy controls.

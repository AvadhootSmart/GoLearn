# Exercise 3: Publishing Crates

## Overview

Publishing a crate to crates.io makes your code available to the entire Rust community. This exercise covers preparing, documenting, and publishing the Textio SDK crate.

## crates.io

crates.io is the official Rust package registry. It hosts open-source Rust packages that can be easily added to any Rust project.

### Key Features

- Free hosting for open-source packages
- Automatic documentation via docs.rs
- Version management
- Download statistics
- Dependency resolution

## Preparing for Publication

### 1. Cargo.toml Metadata

Every published crate needs proper metadata:

```toml
[package]
name = "textio-sdk"                    # Unique name on crates.io
version = "0.1.0"                      # Semantic version
edition = "2021"                       # Rust edition
authors = ["Your Name <you@example.com>"]
description = "SDK for the Textio SMS API"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/textio-sdk"
homepage = "https://textio.dev"
documentation = "https://docs.rs/textio-sdk"
readme = "README.md"
keywords = ["sms", "api", "messaging", "textio"]
categories = ["api-bindings", "web-programming"]
```

### 2. README.md

A good README is essential:

```markdown
# textio-sdk

A Rust SDK for the Textio SMS API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
textio-sdk = "0.1"
```

## Usage

```rust
use textio_sdk::{Client, Message};

let client = Client::new("your-api-key");
let msg = Message::new("+15551234567", "Hello!");
client.send(msg).await?;
```

## License

MIT OR Apache-2.0
```

### 3. LICENSE

Include a license file. Popular choices:

- MIT
- Apache-2.0
- MIT OR Apache-2.0 (dual licensing)
- BSD-3-Clause

### 4. Documentation

```rust
//! # Textio SDK
//! 
//! A Rust SDK for the Textio SMS messaging API.
//! 
//! ## Features
//! 
//! - Send SMS messages
//! - Track delivery status
//! - Manage webhooks
//! 
//! ## Example
//! 
//! ```rust
//! use textio_sdk::{Client, Message};
//! 
//! let client = Client::new("api-key");
//! let msg = Message::new("+15551234567", "Hello!");
//! ```

/// Represents an SMS message
/// 
/// # Example
/// 
/// ```
/// use textio_sdk::Message;
/// 
/// let msg = Message::new("+15551234567", "Hello!");
/// ```
pub struct Message {
    /// Recipient phone number in E.164 format
    pub to: String,
    /// Message body (max 1600 characters)
    pub body: String,
}
```

## Semantic Versioning

Rust follows Semantic Versioning (SemVer): `MAJOR.MINOR.PATCH`

### Version Rules

- **MAJOR**: Breaking changes (0.x → 1.0, 1.x → 2.0)
- **MINOR**: New features, backward compatible (0.1 → 0.2)
- **PATCH**: Bug fixes, backward compatible (0.1.0 → 0.1.1)

### Version Selection

```toml
[dependencies]
textio-sdk = "0.1.0"      # Exact version
textio-sdk = "0.1"        # Any 0.1.x
textio-sdk = "^0.1.0"     # Same as 0.1 (default)
textio-sdk = "~0.1.0"     # Only 0.1.x
textio-sdk = ">=0.1.0"    # Minimum version
textio-sdk = "*"          # Any version (not recommended)
```

### Pre-release Versions

```toml
textio-sdk = "0.1.0-alpha.1"
textio-sdk = "0.1.0-beta.2"
textio-sdk = "0.1.0-rc.1"
```

## Publishing Process

### 1. Create Account

```bash
# Login to crates.io (opens browser)
cargo login
```

### 2. Verify Package

```bash
# Check for issues before publishing
cargo publish --dry-run

# Run tests
cargo test

# Check documentation
cargo doc --open
```

### 3. Publish

```bash
# Publish to crates.io
cargo publish
```

### 4. Version Updates

```bash
# Update version in Cargo.toml
# version = "0.1.1"

# Publish new version
cargo publish
```

## Cargo.toml Fields Reference

### Required Fields

| Field | Description |
|-------|-------------|
| `name` | Crate name (unique on crates.io) |
| `version` | Semantic version |
| `edition` | Rust edition (2015, 2018, 2021) |

### Recommended Fields

| Field | Description |
|-------|-------------|
| `authors` | List of authors |
| `description` | Short description |
| `license` | SPDX license identifier |
| `repository` | Source repository URL |

### Optional Fields

| Field | Description |
|-------|-------------|
| `homepage` | Project homepage |
| `documentation` | Docs URL |
| `readme` | README file path |
| `keywords` | Search keywords (max 5) |
| `categories` | crates.io categories |
| `exclude` | Files to exclude from package |
| `include` | Files to include in package |

## Excluding Files

Don't include unnecessary files:

```toml
[package]
exclude = [
    "docs/",
    "examples/",
    "tests/",
    ".github/",
    "*.md",
]
```

## CI/CD for Publishing

```yaml
# .github/workflows/publish.yml
name: Publish

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo publish --token ${CRATES_TOKEN}
        env:
          CRATES_TOKEN: ${{ secrets.CRATES_TOKEN }}
```

## Deprecation

Mark old versions as deprecated:

```bash
cargo yank --vers 0.1.0
```

Un-yank if needed:

```bash
cargo yank --vers 0.1.0 --undo
```

## Best Practices

1. **Test thoroughly** before publishing
2. **Document well** with examples
3. **Version carefully** following SemVer
4. **Include examples** in the repository
5. **Keep changelog** for version history
6. **Use CI/CD** for automated publishing
7. **Test as user** with `cargo install`

## Common Issues

### Name Already Taken

```bash
error: crate `textio` already exists
```

Solution: Choose a different name or namespace.

### Missing Metadata

```bash
error: missing field `description` in manifest
```

Solution: Add required fields to Cargo.toml.

### Invalid License

```bash
error: license `MIT/Apache` is invalid
```

Solution: Use SPDX format: `MIT OR Apache-2.0`

## Exercise Task

In this exercise, you'll prepare a Textio SDK crate for publication:

1. Complete Cargo.toml with proper metadata
2. Write comprehensive documentation
3. Create a proper README
4. Follow semantic versioning principles

The starter code provides a partially complete crate that needs proper documentation and metadata.

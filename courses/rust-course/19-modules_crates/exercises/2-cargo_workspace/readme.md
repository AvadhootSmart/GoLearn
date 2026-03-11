# Exercise 2: Cargo Workspaces

## Overview

Cargo workspaces allow you to manage multiple related packages (crates) from a single location. This is essential for larger projects like Textio, where you might have separate crates for the core library, CLI tool, web server, and shared utilities.

## What is a Workspace?

A workspace is a set of packages that share the same `Cargo.lock` and output directory. Benefits include:

1. **Shared dependencies** - All crates use the same versions
2. **Single Cargo.lock** - Ensures version consistency
3. **Faster builds** - Shared target directory for all crates
4. **Easier management** - One place to run tests, builds, etc.

## Workspace Structure

```
textio-workspace/
├── Cargo.toml          # Workspace manifest
├── Cargo.lock          # Shared lock file
├── target/             # Shared build directory
├── crates/
│   ├── textio-core/    # Core library crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── textio-cli/     # CLI binary crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── textio-server/  # Web server crate
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── shared/
    └── textio-types/   # Shared types crate
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

## Workspace Cargo.toml

The root `Cargo.toml` defines the workspace:

```toml
[workspace]
members = [
    "crates/textio-core",
    "crates/textio-cli",
    "crates/textio-server",
    "shared/textio-types",
]

# Optional: exclude certain directories
exclude = [
    "examples/",
    "tools/",
]

# Optional: workspace-wide dependencies
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

## Package Cargo.toml

Each member crate has its own `Cargo.toml`:

```toml
[package]
name = "textio-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Use workspace version
serde = { workspace = true }
textio-types = { path = "../shared/textio-types" }
```

## Workspace Commands

### Build All Crates

```bash
cargo build          # Build all workspace members
cargo build -p textio-core  # Build specific package
```

### Run Tests

```bash
cargo test           # Test all workspace members
cargo test -p textio-cli    # Test specific package
```

### Run a Binary

```bash
cargo run -p textio-cli     # Run specific binary
cargo run -p textio-server  # Run server
```

## Inter-Crate Dependencies

Crates within a workspace can depend on each other:

```toml
# In crates/textio-cli/Cargo.toml
[dependencies]
textio-core = { path = "../textio-core" }
textio-types = { path = "../../shared/textio-types" }
```

## Shared Dependencies

Using workspace dependencies ensures consistency:

```toml
# Root Cargo.toml
[workspace.dependencies]
serde = "1.0"

# Member Cargo.toml
[dependencies]
serde = { workspace = true }
```

## Virtual Workspace Manifest

A workspace without a root package is called a "virtual manifest":

```toml
# Cargo.toml (no [package] section)
[workspace]
members = ["crates/*"]
```

## Workspace with Root Package

You can also have a workspace that includes the root package:

```toml
[package]
name = "textio"
version = "0.1.0"

[workspace]
members = ["crates/*"]
```

## Default Members

Specify which crates to operate on by default:

```toml
[workspace]
members = ["crates/*"]
default-members = ["crates/textio-core", "crates/textio-cli"]
```

## Practical Textio Example

For the Textio SMS API, a workspace might look like:

```
textio/
├── Cargo.toml
├── crates/
│   ├── textio-core/      # Core SMS functionality
│   ├── textio-api/       # REST API server
│   ├── textio-cli/       # Command-line tool
│   ├── textio-sdk/       # Client SDK
│   └── textio-worker/    # Background job processor
└── shared/
    ├── textio-types/     # Shared data types
    └── textio-proto/     # Protocol definitions
```

## Dependency Management

### Version Requirements

```toml
[dependencies]
textio-types = { path = "../shared/textio-types" }  # Path dependency
serde = "1.0"           # From crates.io
tokio = { version = "1.0", features = ["full"] }
```

### Features

Share features across workspace:

```toml
# Workspace level
[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }

# Package level
[dependencies]
tokio = { workspace = true }
```

## Build Optimization

Workspaces share a `target` directory, which:

1. Reduces disk usage
2. Speeds up compilation
3. Shares compiled dependencies

## Testing Strategy

```bash
# Test everything
cargo test --workspace

# Test specific package
cargo test -p textio-core

# Test with features
cargo test -p textio-api --features "postgres"
```

## CI/CD with Workspaces

```yaml
# Example GitHub Actions
- name: Build
  run: cargo build --workspace

- name: Test
  run: cargo test --workspace

- name: Check specific crate
  run: cargo clippy -p textio-core
```

## Common Patterns

### Library + Binary Pattern

```
my-app/
├── Cargo.toml
├── src/
│   ├── lib.rs    # Library code
│   └── main.rs   # Binary (uses lib)
```

### Multi-Crate Pattern

```
my-workspace/
├── Cargo.toml
└── crates/
    ├── lib/
    ├── cli/
    └── server/
```

### Shared Types Pattern

```
workspace/
├── Cargo.toml
└── crates/
    ├── shared-types/  # Common types
    ├── api/           # Uses shared-types
    └── client/        # Uses shared-types
```

## Best Practices

1. **Use workspace dependencies** - Ensures version consistency
2. **Keep shared code in separate crates** - Avoids circular dependencies
3. **Use path dependencies** - For local crates
4. **Version independently** - Each crate has its own version
5. **Document dependencies** - Comment why each dependency exists

## Exercise Task

In this exercise, you'll create a Cargo workspace for Textio with multiple crates:

1. Create workspace root Cargo.toml
2. Create member crates with proper dependencies
3. Set up shared dependencies
4. Configure inter-crate dependencies

The starter code provides the structure you need to complete.

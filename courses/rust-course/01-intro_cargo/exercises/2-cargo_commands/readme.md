# Essential Cargo Commands

## The Cargo Workflow

Understanding Cargo's commands is fundamental to Rust development. Here's what you'll use daily:

## Building Your Project

### `cargo build` - Compile Your Code

```bash
cargo build
```

Creates an executable in `target/debug/your_project_name`

This is a **debug build**:
- Fast compilation
- Includes debug symbols (for debugging)
- No optimizations
- Larger binary size
- Slower runtime

### `cargo build --release` - Optimized Build

```bash
cargo build --release
```

Creates an executable in `target/release/your_project_name`

This is a **release build**:
- Slower compilation
- Aggressive optimizations
- Strips debug symbols
- Smaller binary size
- Much faster runtime

**When to use which:**
- Debug during development
- Release for production deployment

## Running Your Project

### `cargo run` - Build and Run

```bash
cargo run
```

Equivalent to `cargo build` + `./target/debug/your_project_name`

Cargo is smart:
- Only recompiles changed files
- Uses incremental compilation
- Checks timestamps

```bash
cargo run --release  # Runs release build
```

### `cargo run -- args` - Pass Arguments

```bash
cargo run -- --help
cargo run -- input.txt output.txt
```

The `--` separates Cargo flags from your program's arguments.

## Checking Your Code

### `cargo check` - Fast Verification

```bash
cargo check
```

Quickly checks if your code compiles **without** producing an executable.

**Why use it?**
- Much faster than `cargo build`
- Perfect for iterative development
- Catches errors early
- Use it constantly while coding

**Typical workflow:**
```
Write code → cargo check → Fix errors → cargo check → cargo run
```

## Project Management

### `cargo new` - Create New Project

```bash
cargo new my_project          # Binary (executable) project
cargo new my_lib --lib        # Library project
```

### `cargo init` - Initialize in Current Directory

```bash
mkdir existing_folder
cd existing_folder
cargo init                    # Creates Cargo.toml and src/
```

## Understanding Cargo.toml

```toml
[package]
name = "textio"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "An SMS API service"
license = "MIT"

[dependencies]
serde = "1.0"                 # From crates.io
tokio = { version = "1", features = ["full"] }  # With features
```

### Adding Dependencies

**Method 1: Manual**
Edit `Cargo.toml`:
```toml
[dependencies]
rand = "0.8"
```

**Method 2: Command Line**
```bash
cargo add rand
cargo add tokio --features full
cargo add serde --features derive
```

## Dependency Versions

```toml
[dependencies]
rand = "0.8"           # Compatible with 0.8.x (most common)
rand = "0.8.5"         # Exact version
rand = "^0.8.5"        # Same as "0.8.5"
rand = ">=0.8.0"       # Minimum version
rand = "*"             # Any version (dangerous!)
```

Cargo uses **Semantic Versioning**:
- `MAJOR.MINOR.PATCH` (e.g., `1.4.2`)
- `^1.4.2` means `>=1.4.2, <2.0.0`
- Breaking changes require major version bump

## Lock File: Cargo.lock

Cargo automatically creates `Cargo.lock`:
- Records exact versions of all dependencies
- Ensures reproducible builds
- **Commit this file for applications**
- **Don't commit for libraries**

## Project Structure

```
textio/
├── Cargo.toml          # Manifest
├── Cargo.lock          # Dependency lock (auto-generated)
├── src/
│   ├── main.rs         # Binary entry point
│   └── lib.rs          # Library root (optional)
├── tests/              # Integration tests
├── benches/            # Benchmarks
└── examples/           # Example programs
```

## Mental Model: Build Process

```
1. Parse Cargo.toml
      ↓
2. Resolve dependencies (what versions?)
      ↓
3. Download crates (~/.cargo/registry)
      ↓
4. Compile dependencies (cached)
      ↓
5. Compile your code
      ↓
6. Link everything
      ↓
7. Output binary to target/
```

## Common Commands Summary

| Command | Purpose | Frequency |
|---------|---------|-----------|
| `cargo check` | Verify compilation | Constantly |
| `cargo run` | Build and run | Often |
| `cargo build` | Build debug binary | Sometimes |
| `cargo build --release` | Build optimized binary | Before deployment |
| `cargo test` | Run tests | Often |
| `cargo doc` | Generate documentation | Occasionally |
| `cargo clean` | Remove target/ | Rarely |

## Common Pitfalls

### 1. Not Using `cargo check` Enough
Always run `cargo check` while developing. It's 10-100x faster than `cargo build`.

### 2. Forgetting `--release`
Production builds should use `--release`. Debug builds can be 10-100x slower.

### 3. Ignoring `Cargo.lock`
For applications, commit `Cargo.lock` to ensure all environments use the same dependency versions.

## Exercise Task

You'll practice the essential Cargo workflow:

1. Print a message that shows:
   - App name: `Textio`
   - Version: `1.0`
   - Status: `initializing`
   
2. Format your output to look like:
   ```
   Textio v1.0 - initializing
   ```

3. Use `println!` with multiple placeholders (`{}`) to practice string formatting.

After writing the code, verify it works with `cargo run`.

# Hello, Cargo!

## Why Cargo?

Cargo is Rust's build system and package manager. Most Rustaceans use Cargo to manage their Rust projects because it handles:

- Building your code
- Downloading dependencies (libraries your code depends on)
- Building those dependencies
- Running tests
- Generating documentation

Think of Cargo as Rust's equivalent to `npm` (Node.js), `pip` (Python), or `go mod` (Go) - but it also handles compilation.

## Your First Cargo Project

Let's understand what `cargo new` creates:

```bash
cargo new hello_cargo
```

This creates a directory structure:

```
hello_cargo/
├── Cargo.toml
└── src/
    └── main.rs
```

### Cargo.toml - The Manifest File

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

This is your project's configuration:
- **name**: Your package name (used when publishing to crates.io)
- **version**: Semantic versioning (major.minor.patch)
- **edition**: Rust edition (2021 is current, determines language features)
- **dependencies**: External crates your project needs

### src/main.rs - Your Code

```rust
fn main() {
    println!("Hello, world!");
}
```

## Printing in Rust

`println!` is a **macro** (not a function). The `!` is the telltale sign.

```rust
fn main() {
    println!("Hello, world!");
    println!("The answer is {}", 42);
    println!("{} plus {} equals {}", 1, 2, 3);
}
```

The `{}` is a placeholder that gets replaced by arguments. This is called "string interpolation" or "formatting."

### Common Formatting Options

```rust
fn main() {
    let name = "Textio";
    let version = 1.0;
    
    println!("App: {}", name);                    // Basic
    println!("Version: {:.2}", version);          // 2 decimal places
    println!("{0} is {1}, {0}!", "Rust", "fun");  // Positional
    println!("{name} v{version}", name="Textio", version=1); // Named
    println!("Debug: {:?}", vec![1, 2, 3]);       // Debug format
}
```

## The `fn main()` Function

Every Rust executable starts with `main`:

```rust
fn main() {
    // Your code here
}
```

- `fn` declares a function
- `main` is the entry point (special name)
- No parameters (for now)
- No return type specified (returns unit type `()`)

## Mental Model: Compilation

```
Your Code (.rs files)
        ↓
    Rust Compiler (rustc)
        ↓
    Binary Executable
        ↓
    Operating System runs it
```

Cargo wraps `rustc` and adds dependency management:

```
Your Code + Dependencies
        ↓
    Cargo (build system)
        ↓
    rustc (compiler)
        ↓
    Binary Executable
```

## Common Pitfalls

### 1. Forgetting the `!` in `println!`
```rust
println("Hello");  // ERROR: expected function, found macro
println!("Hello"); // CORRECT
```

### 2. Mismatched Braces
```rust
fn main() {
    println!("Missing closing brace"
}  // ERROR: expected expression, found `}`
```

### 3. Wrong Quote Types
```rust
println!("Hello");  // CORRECT: double quotes for strings
println!('Hello');  // ERROR: character literals are single chars only
```

## Under the Hood

When you run `cargo build`:

1. Cargo reads `Cargo.toml`
2. Resolves dependencies
3. Downloads missing crates to `~/.cargo/registry`
4. Compiles dependencies first (cached in `target/`)
5. Compiles your code
6. Links everything into a binary

The `target/` directory contains:
- `target/debug/` - Development builds (faster to compile, slower to run)
- `target/release/` - Optimized builds (slower to compile, faster to run)

## Exercise Task

You're building **Textio**, an SMS API. This will be our example project throughout the course.

1. Write code that prints: `starting Textio server`
2. Use the `println!` macro with the message as a string literal

Run your code with `cargo run` to verify it works.

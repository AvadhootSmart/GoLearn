# Exercise 1: Clap Basics - Arguments and Options

## Overview

Command-line interfaces (CLIs) are essential tools for developers and system administrators. In Rust, the `clap` crate is the de facto standard for building powerful, user-friendly CLIs. This exercise introduces you to clap's derive API, which uses Rust's procedural macros to generate argument parsing code automatically.

## Learning Objectives

By the end of this exercise, you will be able to:
- Use `clap` with the derive API
- Define positional arguments
- Create flags and options with short and long forms
- Set required vs optional arguments
- Provide default values for optional arguments
- Access parsed values in your program

## Introduction to Clap

Clap (Command Line Argument Parser) is a feature-rich library for parsing command line arguments. It provides two APIs:

1. **Derive API**: Uses Rust's `#[derive]` macros - simpler and more idiomatic
2. **Builder API**: Uses method chaining - more flexible but verbose

We'll focus on the Derive API as it's more beginner-friendly and works well for most use cases.

## Adding Clap to Your Project

Add this to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
```

The `derive` feature is required to use the `Parser`, `Args`, and `Subcommand` derive macros.

## The Parser Derive Macro

The foundation of clap's derive API is the `Parser` trait. Any struct that derives `Parser` can parse command-line arguments:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A sample CLI application", long_about = None)]
struct Cli {
    // Your arguments go here
}

fn main() {
    let cli = Cli::parse();
    // Use the parsed values
}
```

## Positional Arguments

Positional arguments are specified by their position in the command line, not by a flag:

```rust
#[derive(Parser)]
struct Cli {
    /// The input file to process
    input: String,
    
    /// The output file destination
    output: String,
}
```

Usage: `myapp input.txt output.txt`

The doc comments (`///`) automatically become help text for the argument.

## Options (Named Arguments with Values)

Options have names and require values. Use `Option<T>` for optional options:

```rust
#[derive(Parser)]
struct Cli {
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
    
    /// Number of retries
    #[arg(short = 'r', long = "retries")]
    retry_count: Option<u32>,
}
```

Usage: `myapp --config settings.toml --retries 5`

The `short` attribute creates a short flag (first letter of field name by default).
The `long` attribute creates a long flag (full field name with underscores as dashes).

## Flags (Boolean Switches)

Flags are boolean switches that don't take values:

```rust
#[derive(Parser)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Force overwrite existing files
    #[arg(short, long)]
    force: bool,
}
```

Usage: `myapp --verbose --force` or `myapp -v -f`

## Required vs Optional Arguments

- Required arguments: Use the type directly (e.g., `String`, `PathBuf`)
- Optional arguments: Use `Option<T>` (e.g., `Option<String>`)

```rust
#[derive(Parser)]
struct Cli {
    // Required positional argument
    message: String,
    
    // Optional flag with value
    #[arg(short, long)]
    recipient: Option<String>,
    
    // Required flag with value
    #[arg(short, long)]
    api_key: String,
}
```

## Default Values

For optional arguments, you can provide default values:

```rust
#[derive(Parser)]
struct Cli {
    /// Port number to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Host address
    #[arg(long, default_value = "localhost")]
    host: String,
}
```

With `default_value`, the field type doesn't need to be `Option<T>`.

## The #[arg] Attribute

The `#[arg]` attribute configures individual arguments:

| Attribute | Description | Example |
|-----------|-------------|---------|
| `short` | Enable short flag | `#[arg(short)]` |
| `long` | Enable long flag | `#[arg(long)]` |
| `short = 'x'` | Custom short flag | `#[arg(short = 'x')]` |
| `long = "custom"` | Custom long flag | `#[arg(long = "custom-name")]` |
| `default_value` | Default value | `#[arg(default_value = "10")]` |
| `value_name` | Name in help | `#[arg(value_name = "FILE")]` |
| `help` | Custom help text | `#[arg(help = "Custom help")]` |
| `required` | Force required | `#[arg(required = true)]` |

## The #[command] Attribute

The `#[command]` attribute configures the overall CLI:

```rust
#[derive(Parser)]
#[command(name = "textio")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Textio SMS CLI", long_about = None)]
struct Cli {
    // arguments
}
```

## Accessing Parsed Values

After calling `Cli::parse()`, you can access all values:

```rust
fn main() {
    let cli = Cli::parse();
    
    println!("Input: {}", cli.input);
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    if let Some(config) = cli.config {
        println!("Config: {}", config);
    }
}
```

## Textio CLI Example

For our Textio SMS API, we'll build a CLI to send messages:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "textio")]
#[command(about = "Textio SMS CLI - Send messages from the command line")]
struct Cli {
    /// The phone number to send to
    #[arg(short, long)]
    to: String,
    
    /// The message content
    #[arg(short, long)]
    message: String,
    
    /// API key for authentication
    #[arg(short, long, env = "TEXTIO_API_KEY")]
    api_key: String,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Maximum characters per SMS
    #[arg(long, default_value = "160")]
    max_chars: usize,
}
```

Usage examples:
```bash
# Send a message
textio --to +1234567890 --message "Hello, World!" --api-key secret

# With verbose output
textio -t +1234567890 -m "Test message" -k secret --verbose

# Using environment variable for API key
export TEXTIO_API_KEY=secret
textio --to +1234567890 --message "Hello"
```

## Help Generation

Clap automatically generates help text:

```bash
textio --help
```

Output:
```
Textio SMS CLI - Send messages from the command line

Usage: textio [OPTIONS] --to <TO> --message <MESSAGE> --api-key <API_KEY>

Options:
  -t, --to <TO>              The phone number to send to
  -m, --message <MESSAGE>    The message content
  -k, --api-key <API_KEY>    API key for authentication [env: TEXTIO_API_KEY=]
  -v, --verbose              Enable verbose output
      --max-chars <MAX_CHARS>  Maximum characters per SMS [default: 160]
  -h, --help                 Print help
  -V, --version              Print version
```

## Best Practices

1. **Use descriptive field names**: They become the long flag name
2. **Add doc comments**: They become help text
3. **Use appropriate types**: clap supports String, PathBuf, numbers, etc.
4. **Set environment variables**: Use `env` for sensitive data like API keys
5. **Provide defaults**: For commonly used values

## Common Patterns

### Multiple Values
```rust
/// List of recipients
#[arg(short, long)]
to: Vec<String>,
```

Usage: `textio --to alice --to bob --to charlie`

### Exclusive Arguments
```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long, conflicts_with = "decrypt")]
    encrypt: bool,
    
    #[arg(short, long, conflicts_with = "encrypt")]
    decrypt: bool,
}
```

## Exercise Task

Create a CLI for Textio that:
1. Takes a required message recipient (`--to`)
2. Takes optional message content (`--message`) or reads from stdin
3. Has an API key option with environment variable fallback
4. Supports verbose mode flag
5. Has a configurable timeout with a default value

## Resources

- [Clap Documentation](https://docs.rs/clap/)
- [Clap Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [Rust CLI Book](https://rust-cli.github.io/book/)

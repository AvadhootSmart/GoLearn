# Exercise 3: CLI Arguments

## Overview

Command-line interfaces (CLI) are a fundamental way to interact with programs. Rust's `std::env::args` provides access to command-line arguments. Understanding how to parse and validate these arguments is essential for building useful CLI tools.

## Learning Objectives

By the end of this exercise, you will be able to:

- Access command-line arguments with `std::env::args`
- Use `args_os` for handling non-UTF8 arguments
- Parse arguments into structured data
- Implement common CLI patterns (flags, options, positional)
- Build a functional CLI for the Textio system
- Handle invalid input gracefully

## Core Concepts

### Basic Argument Access

```rust
use std::env;

fn main() {
    // Get all arguments as an iterator
    let args: Vec<String> = env::args().collect();
    
    // args[0] is the program name
    println!("Program: {}", args[0]);
    
    // args[1..] are the actual arguments
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }
}
```

### Using the Iterator

Instead of collecting into a Vec, you can process arguments as an iterator:

```rust
use std::env;

fn main() {
    let mut args = env::args();
    
    // Skip the program name
    args.next();
    
    // Process remaining arguments
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => print_help(),
            "-v" | "--version" => print_version(),
            "--file" => {
                let filename = args.next().expect("Expected filename after --file");
                println!("File: {}", filename);
            }
            arg => println!("Unknown argument: {}", arg),
        }
    }
}
```

### args vs args_os

```rust
use std::env;

// args() - Returns String (panics on invalid UTF-8)
let args: Vec<String> = env::args().collect();

// args_os() - Returns OsString (handles any bytes)
let args_os: Vec<std::ffi::OsString> = env::args_os().collect();
```

Use `args_os` when:
- Dealing with non-UTF8 filenames
- Maximum compatibility is needed
- You need to handle paths on any OS

## Common CLI Patterns

### Flag Arguments

Flags are boolean switches:

```rust
struct Flags {
    verbose: bool,
    quiet: bool,
    help: bool,
}

fn parse_flags(args: &[String]) -> Flags {
    let mut flags = Flags {
        verbose: false,
        quiet: false,
        help: false,
    };
    
    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => flags.verbose = true,
            "-q" | "--quiet" => flags.quiet = true,
            "-h" | "--help" => flags.help = true,
            _ => {}
        }
    }
    
    flags
}
```

### Option Arguments

Options take a value:

```rust
struct Options {
    output: Option<String>,
    config: Option<String>,
}

fn parse_options(args: &[String]) -> Options {
    let mut options = Options {
        output: None,
        config: None,
    };
    
    let mut args_iter = args.iter().peekable();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                options.output = args_iter.next().cloned();
            }
            "-c" | "--config" => {
                options.config = args_iter.next().cloned();
            }
            _ => {}
        }
    }
    
    options
}
```

### Positional Arguments

Positional arguments are values without a flag:

```rust
struct Args {
    command: String,
    files: Vec<String>,
}

fn parse_positional(args: &[String]) -> Option<Args> {
    let mut positional: Vec<&String> = Vec::new();
    let mut skip_next = false;
    
    for arg in args.iter() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        if arg.starts_with('-') && arg.len() > 1 {
            // It's a flag or option, skip its value if it takes one
            skip_next = takes_value(arg);
        } else {
            positional.push(arg);
        }
    }
    
    if positional.is_empty() {
        return None;
    }
    
    Some(Args {
        command: positional[0].clone(),
        files: positional[1..].to_vec(),
    })
}

fn takes_value(flag: &str) -> bool {
    matches!(flag, "-o" | "--output" | "-c" | "--config" | "-f" | "--file")
}
```

### Combined Short Flags

Handle `-abc` as `-a -b -c`:

```rust
fn expand_short_flags(arg: &str) -> Vec<String> {
    if arg.starts_with("--") || !arg.starts_with('-') {
        return vec![arg.to_string()];
    }
    
    let chars: Vec<char> = arg.chars().skip(1).collect();
    if chars.len() == 1 {
        return vec![arg.to_string()];
    }
    
    chars.iter().map(|c| format!("-{}", c)).collect()
}
```

### Subcommand Pattern

```rust
enum Command {
    Send { to: String, message: String },
    List { filter: Option<String> },
    Config { key: String, value: Option<String> },
}

fn parse_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("No command provided".to_string());
    }
    
    match args[0].as_str() {
        "send" => {
            let to = args.get(1).ok_or("Missing recipient")?.clone();
            let message = args.get(2).ok_or("Missing message")?.clone();
            Ok(Command::Send { to, message })
        }
        "list" => {
            let filter = args.get(1).cloned();
            Ok(Command::List { filter })
        }
        "config" => {
            let key = args.get(1).ok_or("Missing key")?.clone();
            let value = args.get(2).cloned();
            Ok(Command::Config { key, value })
        }
        cmd => Err(format!("Unknown command: {}", cmd)),
    }
}
```

### Value Validation

```rust
fn parse_number(arg: &str) -> Result<u32, String> {
    arg.parse::<u32>()
        .map_err(|_| format!("'{}' is not a valid number", arg))
}

fn parse_phone_number(arg: &str) -> Result<String, String> {
    let cleaned: String = arg.chars().filter(|c| c.is_digit(10) || *c == '+').collect();
    
    if cleaned.starts_with('+') && cleaned.len() >= 10 {
        Ok(cleaned)
    } else {
        Err(format!("Invalid phone number: {}", arg))
    }
}
```

## Textio Context

Building a CLI for the Textio SMS system:

### Textio CLI Structure

```rust
enum TextioCommand {
    Send {
        to: String,
        message: String,
        from: Option<String>,
    },
    Status {
        message_id: String,
    },
    Contacts {
        action: ContactAction,
    },
    Config {
        key: Option<String>,
        value: Option<String>,
    },
}

enum ContactAction {
    List,
    Add { name: String, phone: String },
    Remove { phone: String },
}
```

### Usage Examples

```bash
# Send a message
textio send +1234567890 "Hello, World!"
textio send +1234567890 "Hello" --from Textio

# Check message status
textio status msg_abc123

# Manage contacts
textio contacts list
textio contacts add "John Doe" +1234567890
textio contacts remove +1234567890

# Configuration
textio config
textio config api_key
textio config api_key your_key_here
```

### Help Text

```rust
fn print_help() {
    println!("Textio - SMS Messaging CLI");
    println!();
    println!("USAGE:");
    println!("    textio <command> [options]");
    println!();
    println!("COMMANDS:");
    println!("    send <to> <message>  Send an SMS message");
    println!("    status <id>          Check message status");
    println!("    contacts <action>    Manage contacts");
    println!("    config [key] [value] View or set config");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help           Print this help message");
    println!("    -v, --version        Print version info");
    println!("    --from <id>          Set sender ID for send command");
    println!("    -f, --file <path>    Use config file");
}
```

## Best Practices

1. **Always handle the help flag**: `-h` and `--help`
2. **Validate input early**: Fail fast with clear messages
3. **Provide meaningful error messages**: Include what was expected
4. **Support common conventions**: `-v` for verbose, `-q` for quiet
5. **Consider a CLI library**: For complex CLIs, use `clap` or `structopt`
6. **Test edge cases**: Empty args, missing values, invalid input

## Exercise Instructions

You will implement a CLI parser for the Textio messaging system:

1. **parse_args**: Parse raw arguments into a structured format
2. **parse_command**: Determine which command was requested
3. **validate_phone**: Validate phone number format
4. **build_help**: Generate help text dynamically

## Key Takeaways

- `std::env::args()` returns an iterator over arguments
- The first argument is always the program name
- Use `args_os()` for non-UTF8 compatible arguments
- Parse iteratively, handling options that take values
- Always validate user input before using it

## Further Reading

- [std::env documentation](https://doc.rust-lang.org/std/env/)
- [Rust Book: Command Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
- [clap crate](https://docs.rs/clap/) - Full-featured argument parser

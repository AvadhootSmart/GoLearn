# Exercise 2: Subcommands and Command Enum Pattern

## Overview

Real-world CLI applications often have multiple commands, each with its own set of arguments. Think of `git commit`, `git push`, and `git pull` - these are all subcommands of `git`. In this exercise, you'll learn how to implement subcommands using clap's derive API.

## Learning Objectives

By the end of this exercise, you will be able to:
- Define subcommands using enums
- Use the `#[derive(Subcommand)]` macro
- Nest subcommands for complex CLI structures
- Handle subcommands in your main function
- Share common arguments between subcommands

## Why Subcommands?

Subcommands organize related functionality under a single CLI tool:

```bash
git status          # 'status' is a subcommand
git commit -m "msg" # 'commit' is a subcommand with its own flags
docker run -it ubuntu bash  # 'run' is a subcommand
kubectl get pods    # 'get' is a subcommand
```

Without subcommands, you'd need separate binaries or complex flag combinations.

## Basic Subcommand Structure

Use an enum to define subcommands:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new contact
    Add {
        /// Contact name
        #[arg(short, long)]
        name: String,
        
        /// Phone number
        #[arg(short, long)]
        phone: String,
    },
    
    /// List all contacts
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Remove a contact
    Remove {
        /// Contact ID or name
        name: String,
    },
}
```

## The #[derive(Subcommand)] Macro

The `Subcommand` derive macro works similarly to `Parser`:

- Enum variants become subcommand names
- Variant names are converted to kebab-case (e.g., `ListAll` becomes `list-all`)
- Doc comments become subcommand descriptions
- Each variant can have its own arguments

## Handling Subcommands

Match on the enum to handle each subcommand:

```rust
fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { name, phone } => {
            println!("Adding contact: {} - {}", name, phone);
        }
        Commands::List { verbose } => {
            println!("Listing contacts...");
            if verbose {
                println!("(detailed view)");
            }
        }
        Commands::Remove { name } => {
            println!("Removing contact: {}", name);
        }
    }
}
```

## Renaming Subcommands

Control the subcommand name with attributes:

```rust
#[derive(Subcommand)]
enum Commands {
    /// Send an SMS message
    #[command(name = "send")]
    SendMessage {
        // ...
    },
    
    /// View message history
    #[command(name = "history")]
    ViewHistory {
        // ...
    },
}
```

## Tuple vs Struct Variants

Subcommands can use tuple or struct variants:

```rust
#[derive(Subcommand)]
enum Commands {
    // Struct variant - named fields
    Send {
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        message: String,
    },
    
    // Tuple variant - positional arguments
    Delete(String),  // Just one argument: the ID
    
    // Unit variant - no arguments
    Status,
}
```

Usage:
```bash
app send --to +123 --message "Hello"
app delete contact-id
app status
```

## Combining Global and Subcommand Arguments

Often you want some flags to apply globally:

```rust
#[derive(Parser)]
struct Cli {
    /// Global verbose flag
    #[arg(short, long, global = true)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send {
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        message: String,
    },
    List,
}
```

The `global = true` attribute makes the flag available to all subcommands.

## Accessing Global Args

```rust
fn main() {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    
    match cli.command {
        Commands::Send { to, message } => {
            // ...
        }
        Commands::List => {
            // ...
        }
    }
}
```

## Nested Subcommands

For complex CLIs, you can nest subcommands:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Contact management
    #[command(subcommand)]
    Contact(ContactCommands),
    
    /// Message operations
    #[command(subcommand)]
    Message(MessageCommands),
}

#[derive(Subcommand)]
enum ContactCommands {
    /// Add a new contact
    Add {
        name: String,
        phone: String,
    },
    /// List all contacts
    List,
    /// Delete a contact
    Delete {
        name: String,
    },
}

#[derive(Subcommand)]
enum MessageCommands {
    /// Send a message
    Send {
        to: String,
        message: String,
    },
    /// View message history
    History {
        #[arg(short, long)]
        limit: Option<usize>,
    },
}
```

Usage:
```bash
app contact add John +1234567890
app contact list
app message send --to +123 --message "Hello"
app message history --limit 10
```

## Handling Nested Commands

```rust
fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Contact(cmd) => match cmd {
            ContactCommands::Add { name, phone } => {
                println!("Adding contact: {} - {}", name, phone);
            }
            ContactCommands::List => {
                println!("Listing contacts...");
            }
            ContactCommands::Delete { name } => {
                println!("Deleting contact: {}", name);
            }
        },
        Commands::Message(cmd) => match cmd {
            MessageCommands::Send { to, message } => {
                println!("Sending to {}: {}", to, message);
            }
            MessageCommands::History { limit } => {
                println!("Message history (limit: {:?})", limit);
            }
        },
    }
}
```

## The Args Derive for Shared Arguments

When subcommands share arguments, extract them to a struct:

```rust
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send(SendArgs),
    Schedule(ScheduleArgs),
}

#[derive(Args)]
struct SendArgs {
    #[command(flatten)]
    common: MessageArgs,
    
    /// Send immediately
    #[arg(short, long)]
    now: bool,
}

#[derive(Args)]
struct ScheduleArgs {
    #[command(flatten)]
    common: MessageArgs,
    
    /// Schedule time (ISO 8601)
    #[arg(short, long)]
    at: String,
}

#[derive(Args)]
struct MessageArgs {
    /// Recipient phone number
    #[arg(short, long)]
    to: String,
    
    /// Message content
    #[arg(short, long)]
    message: String,
}
```

## Textio CLI Structure

For our Textio SMS API, we'll create:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "textio")]
struct Cli {
    /// API key for authentication
    #[arg(short, long, global = true)]
    api_key: Option<String>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send an SMS message
    Send {
        /// Recipient phone number
        #[arg(short, long)]
        to: String,
        
        /// Message content
        #[arg(short, long)]
        message: String,
    },
    
    /// View sent messages
    History {
        /// Maximum number of messages to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Manage contacts
    #[command(subcommand)]
    Contact(ContactCommands),
}

#[derive(Subcommand)]
enum ContactCommands {
    /// Add a new contact
    Add {
        name: String,
        phone: String,
    },
    /// List all contacts
    List,
    /// Remove a contact
    Remove {
        name: String,
    },
}
```

## Exercise Task

Create a Textio CLI with:
1. A `send` subcommand to send messages
2. A `history` subcommand to view sent messages
3. A `contact` subcommand with `add`, `list`, and `remove` nested subcommands
4. A global `--api-key` flag
5. A global `--verbose` flag

## Best Practices

1. **Use descriptive subcommand names**: They should be verbs (send, list, delete)
2. **Group related commands**: Use nested subcommands for organization
3. **Keep subcommands focused**: Each should do one thing well
4. **Provide helpful descriptions**: Users should understand what each command does
5. **Use global flags sparingly**: Only for truly universal options

## Resources

- [Clap Subcommand Documentation](https://docs.rs/clap/latest/clap/Subcommand.html)
- [Clap Derive Tutorial - Subcommands](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_1.html)

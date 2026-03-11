# Exercise 4: CLI Patterns - Exit Codes, Colors, Progress Bars, and I/O

## Overview

Professional CLI applications do more than just parse arguments. They provide colored output for better readability, show progress for long operations, read from stdin for pipeline integration, and use proper exit codes for scripting. This exercise covers these essential CLI patterns.

## Learning Objectives

By the end of this exercise, you will be able to:
- Use proper exit codes for success and failure
- Add colors to terminal output with the `colored` crate
- Display progress bars with the `indicatif` crate
- Read from stdin and write to stdout
- Create pipeline-friendly CLI tools
- Handle signals gracefully

## Exit Codes

Exit codes communicate success or failure to the shell and other programs:

```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    if success {
        ExitCode::SUCCESS  // 0
    } else {
        ExitCode::FAILURE  // 1
    }
}
```

### Standard Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Misuse of command |
| 126 | Command not executable |
| 127 | Command not found |
| 130 | Interrupted (Ctrl+C) |

### Custom Exit Codes

```rust
#[derive(Debug)]
enum TextioError {
    NetworkError,
    InvalidInput,
    AuthenticationFailed,
    RateLimited,
}

impl From<TextioError> for ExitCode {
    fn from(error: TextioError) -> Self {
        match error {
            TextioError::NetworkError => ExitCode::from(10),
            TextioError::InvalidInput => ExitCode::from(20),
            TextioError::AuthenticationFailed => ExitCode::from(30),
            TextioError::RateLimited => ExitCode::from(40),
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            e.into()
        }
    }
}
```

## Terminal Colors with `colored`

Add colors to make output more readable:

```toml
[dependencies]
colored = "2.1"
```

### Basic Usage

```rust
use colored::Colorize;

fn main() {
    println!("{}", "Success!".green());
    println!("{}", "Warning!".yellow());
    println!("{}", "Error!".red());
    println!("{}", "Info:".blue().bold());
}
```

### Styling Options

```rust
use colored::Colorize;

// Colors
"green".green();
"red".red();
"blue".blue();
"yellow".yellow();
"magenta".magenta();
"cyan".cyan();
"white".white();
"black".black();

// Bright variants
"bright green".bright_green();
"bright red".bright_red();

// Styles
"bold".bold();
"italic".italic();
"underline".underline();
"blink".blink();
"dimmed".dimmed();

// Combinations
let msg = "Important".red().bold().underline();
println!("{}", msg);

// Background colors
"black on white".black().on_white();
"white on red".white().on_red();
```

### Conditional Coloring

Check if stdout is a terminal:

```rust
use colored::Colorize;
use std::io::IsTerminal;

fn main() {
    let use_color = std::io::stdout().is_terminal();
    
    colored::control::set_override(use_color);
    
    if use_color {
        println!("{}", "Success!".green());
    } else {
        println!("Success!");
    }
}
```

## Progress Bars with `indicatif`

Show progress for long operations:

```toml
[dependencies]
indicatif = "0.17"
```

### Basic Progress Bar

```rust
use indicatif::ProgressBar;

fn main() {
    let bar = ProgressBar::new(100);
    
    for _ in 0..100 {
        bar.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    bar.finish();
}
```

### Progress Bar with Style

```rust
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})"
        )
        .unwrap()
        .progress_chars("#>-")
    );
    
    for _ in 0..100 {
        bar.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    bar.finish_with_message("Done!");
}
```

### Spinner

```rust
use indicatif::ProgressBar;

fn main() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Loading...");
    
    for _ in 0..100 {
        spinner.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    spinner.finish_with_message("Done!");
}
```

### Multi-Progress

```rust
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let m = MultiProgress::new();
    
    let style = ProgressStyle::with_template("{msg} [{bar:40}] {pos}/{len}")
        .unwrap();
    
    let pb1 = m.add(ProgressBar::new(100));
    pb1.set_style(style.clone());
    pb1.set_message("Task 1");
    
    let pb2 = m.add(ProgressBar::new(200));
    pb2.set_style(style);
    pb2.set_message("Task 2");
    
    // ... update progress bars
}
```

## Reading from Stdin

Process input from pipes or user input:

```rust
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    
    // Read line by line
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => println!("Read: {}", text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
```

### Read All Input

```rust
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    println!("Input length: {} bytes", input.len());
}
```

### Conditional Stdin

```rust
use std::io::{self, IsTerminal};

fn main() {
    if io::stdin().is_terminal() {
        // Interactive input
        println!("Enter your message:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    } else {
        // Piped input
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
    }
}
```

## Writing to Stdout

Write output for pipelines:

```rust
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    writeln!(handle, "Line 1").unwrap();
    writeln!(handle, "Line 2").unwrap();
}
```

### Structured Output

```rust
fn output_json(data: &Data) {
    println!("{}", serde_json::to_string(data).unwrap());
}

fn output_csv(data: &[Data]) {
    for item in data {
        println!("{},{},{}", item.id, item.name, item.value);
    }
}
```

## Complete Example: Textio CLI

```rust
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Read};
use std::process::ExitCode;

#[derive(Parser)]
struct Cli {
    /// Read message from stdin
    #[arg(short, long)]
    stdin: bool,
    
    /// Output format
    #[arg(long, default_value = "text")]
    format: String,
    
    /// Show progress
    #[arg(long)]
    progress: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    match run(cli) {
        Ok(()) => {
            println!("{}", "✓ Done!".green());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}
```

## Exercise Task

Create a Textio CLI that:
1. Uses proper exit codes for different error types
2. Colors output (green for success, red for errors, yellow for warnings)
3. Shows a progress bar when sending multiple messages
4. Reads message content from stdin when `--stdin` flag is used
5. Supports JSON output format with `--format json`
6. Handles Ctrl+C gracefully

## Dependencies

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
colored = "2.1"
indicatif = "0.17"
serde_json = "1.0"
ctrlc = "3.4"
```

## Best Practices

1. **Use exit codes**: Scripts depend on them
2. **Support pipes**: Read stdin, write stdout
3. **Color responsibly**: Check if terminal supports it
4. **Show progress**: For operations taking more than 1 second
5. **Handle signals**: Clean up on Ctrl+C
6. **Provide structured output**: JSON for scripting

## Resources

- [colored crate](https://docs.rs/colored/)
- [indicatif crate](https://docs.rs/indicatif/)
- [Rust CLI Book - Signals](https://rust-cli.github.io/book/in-depth/signals.html)
- [Exit Codes Convention](https://tldp.org/LDP/abs/html/exitcodes.html)

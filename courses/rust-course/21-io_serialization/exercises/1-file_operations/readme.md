# Exercise 1: File Operations

## Overview

File I/O (Input/Output) is fundamental to most applications. Rust provides powerful, safe abstractions for working with files through `std::fs` and `std::io`. In this exercise, you'll learn how to read from and write to files, handle errors properly, and work with paths.

## Learning Objectives

By the end of this exercise, you will be able to:

- Read file contents using `std::fs::read_to_string`
- Write to files using `std::fs::write`
- Use `OpenOptions` for fine-grained file control
- Work with `BufReader` and `BufWriter` for efficient I/O
- Handle I/O errors using `Result` and `?` operator
- Use `Path` and `PathBuf` for path manipulation

## Core Concepts

### The std::fs Module

The `std::fs` module provides filesystem operations. The most commonly used functions are:

```rust
use std::fs;

// Read entire file into a String
let content = fs::read_to_string("file.txt")?;

// Write content to a file (creates or overwrites)
fs::write("output.txt", "Hello, World!")?;

// Create a new file
let file = fs::File::create("new.txt")?;

// Check if a file exists
let exists = fs::metadata("file.txt").is_ok();
```

### Reading Files

#### Simple Reading with read_to_string

The easiest way to read a text file:

```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
```

#### Reading with BufReader

For more control, especially with large files:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}
```

#### Reading Bytes

For binary files:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
```

### Writing Files

#### Simple Writing

```rust
use std::fs;

fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}
```

#### Writing with BufWriter

For efficient writing of multiple operations:

```rust
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn write_lines(path: &str, lines: &[&str]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    
    writer.flush()?;
    Ok(())
}
```

### OpenOptions for Fine Control

`OpenOptions` gives you precise control over how files are opened:

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn append_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)      // Open for writing
        .append(true)     // Append to file
        .create(true)     // Create if doesn't exist
        .open(path)?;
    
    writeln!(file, "{}", content)?;
    Ok(())
}
```

Common `OpenOptions` methods:

- `.read(true)` - Open for reading
- `.write(true)` - Open for writing
- `.append(true)` - Append mode
- `.create(true)` - Create file if it doesn't exist
- `.create_new(true)` - Create new file, error if exists
- `.truncate(true)` - Clear file contents on open

### Path and PathBuf

`Path` and `PathBuf` are like `str` and `String` but for filesystem paths:

```rust
use std::path::{Path, PathBuf};

// Path is a slice (borrowed)
let path = Path::new("src/main.rs");

// PathBuf is owned
let mut path_buf = PathBuf::new();
path_buf.push("src");
path_buf.push("main.rs");

// Common operations
let extension = path.extension();      // Some("rs")
let file_name = path.file_name();      // Some("main.rs")
let parent = path.parent();            // Some("src")
let exists = path.exists();            // bool

// Joining paths
let full_path = Path::new("project").join("src").join("main.rs");

// Convert to string
let path_str = path.to_str();          // Option<&str>
```

### Error Handling

File operations return `io::Result<T>`, which is `Result<T, io::Error>`:

```rust
use std::fs;
use std::io;

fn process_file(path: &str) -> io::Result<()> {
    // The ? operator propagates errors
    let content = fs::read_to_string(path)?;
    
    // Handle the success case
    println!("Read {} bytes", content.len());
    
    Ok(())
}

// Custom error messages
fn read_with_context(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
        .map_err(|e| io::Error::new(
            e.kind(),
            format!("Failed to read {}: {}", path, e)
        ))
}
```

Common error kinds (`io::ErrorKind`):

- `NotFound` - File doesn't exist
- `PermissionDenied` - Insufficient permissions
- `AlreadyExists` - File already exists
- `InvalidInput` - Invalid parameter
- `UnexpectedEof` - Unexpected end of file

## Textio Context

In the Textio SMS API, you need to handle various file operations:

### Reading Contact Lists

```rust
use std::fs;
use std::io;

struct Contact {
    name: String,
    phone: String,
}

fn load_contacts(path: &str) -> io::Result<Vec<Contact>> {
    let content = fs::read_to_string(path)?;
    
    let contacts: Vec<Contact> = content
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                Some(Contact {
                    name: parts[0].trim().to_string(),
                    phone: parts[1].trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    
    Ok(contacts)
}
```

### Writing Message Logs

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn log_message(log_path: &str, to: &str, message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_path)?;
    
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "[{}] To: {} | Message: {}", timestamp, to, message)?;
    
    Ok(())
}
```

### Managing Templates

```rust
use std::fs;
use std::path::Path;

fn load_template(template_dir: &Path, name: &str) -> io::Result<String> {
    let template_path = template_dir.join(format!("{}.txt", name));
    fs::read_to_string(template_path)
}
```

## Best Practices

1. **Always handle errors**: Use `Result` and the `?` operator
2. **Use buffered I/O**: `BufReader` and `BufWriter` for large files
3. **Check file existence**: Before operations that require it
4. **Use Path/PathBuf**: For path manipulation, not string concatenation
5. **Close resources explicitly**: Call `flush()` on writers
6. **Consider encoding**: `read_to_string` expects UTF-8

## Exercise Instructions

You will implement file operations for the Textio messaging system:

1. **load_contacts**: Read a contacts file and parse into a vector
2. **save_contacts**: Write contacts back to a file
3. **append_to_log**: Append messages to a log file
4. **copy_template**: Copy a template file to a new location

The contacts file format is:
```
# Comments start with #
Name, phone_number
John Doe, +1234567890
Jane Smith, +0987654321
```

## Key Takeaways

- `std::fs::read_to_string` and `write` are simplest for text files
- `OpenOptions` provides fine-grained control over file access
- `BufReader`/`BufWriter` improve performance for large files
- `Path` and `PathBuf` are the proper types for filesystem paths
- Always handle `io::Result` errors appropriately

## Further Reading

- [std::fs documentation](https://doc.rust-lang.org/std/fs/)
- [std::io documentation](https://doc.rust-lang.org/std/io/)
- [Rust Book Chapter 12: I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

# Exercise 1: Threads in Rust

## Overview

Rust's standard library provides powerful threading capabilities through `std::thread`. This module explores how to create, manage, and coordinate threads in Rust, with a focus on Textio's parallel message processing needs.

## Why Threading Matters for Textio

Textio processes thousands of SMS messages daily. Sequential processing would be too slow for our users. Threading allows us to:
- Send multiple messages in parallel
- Process webhook callbacks concurrently
- Aggregate status reports from multiple sources
- Handle user requests without blocking

## Basic Thread Creation

### Using `thread::spawn`

The `thread::spawn` function creates a new operating system thread:

```rust
use std::thread;

thread::spawn(|| {
    println!("Hello from a spawned thread!");
});
```

This creates a new thread that runs the closure. However, the main thread doesn't wait for it to finish. The spawned thread might not even get a chance to run before the program exits.

### JoinHandle

`thread::spawn` returns a `JoinHandle<T>` which allows you to wait for the thread to complete:

```rust
use std::thread;

let handle = thread::spawn(|| {
    "Thread completed".to_string()
});

let result = handle.join().unwrap();
println!("{}", result);
```

Calling `join()` blocks the current thread until the spawned thread finishes. It returns a `Result` containing the value the thread returned (or an error if the thread panicked).

## The `move` Keyword

Closures capture variables from their environment. When passing closures to threads, we often need `move`:

```rust
let message = String::from("Hello, Textio!");
let handle = thread::spawn(move || {
    println!("Sending: {}", message);
});
handle.join().unwrap();
```

Without `move`, the closure would borrow `message`. But the thread might outlive the current function, making the borrow invalid. `move` transfers ownership of captured variables into the closure.

### Why `move` is Necessary

```rust
fn send_messages() {
    let messages = vec!["msg1", "msg2", "msg3"];
    
    // This won't compile without 'move'
    let handle = thread::spawn(move || {
        for msg in messages {
            println!("Sending: {}", msg);
        }
    });
    
    handle.join().unwrap();
}
```

The `messages` vector is moved into the thread. The main thread no longer has access to it.

## Thread Builder

For more control, use `thread::Builder`:

```rust
use std::thread;

let handle = thread::Builder::new()
    .name("textio-sender".to_string())
    .stack_size(4 * 1024 * 1024)
    .spawn(|| {
        println!("Custom thread running");
    })
    .unwrap();

handle.join().unwrap();
```

This allows you to:
- Name the thread (useful for debugging)
- Set a custom stack size
- Handle spawn errors (returns `Result` instead of panicking)

## Scoped Threads

A common pattern is spawning threads that borrow from the parent scope. Previously, this required complex lifetime management. Rust 1.63 introduced `std::thread::scope`:

```rust
use std::thread;

let messages = vec!["msg1", "msg2", "msg3"];

thread::scope(|s| {
    for msg in &messages {
        s.spawn(|| {
            println!("Processing: {}", msg);
        });
    }
});

// All scoped threads are guaranteed to have joined here
println!("All messages processed");
```

### How Scoped Threads Work

The `scope` function guarantees that all spawned threads complete before the scope exits. This means:
- Threads can safely borrow from the enclosing scope
- No need for `Arc` or `move` when borrowing read-only data
- The closure passed to `scope` blocks until all threads finish

### Scoped Threads vs Regular Threads

| Feature | `thread::spawn` | `thread::scope` |
|---------|-----------------|-----------------|
| Can outlive caller | Yes | No |
| Can borrow from scope | No (without `move`) | Yes |
| Manual `join` required | Yes | No (automatic) |
| Use case | Long-running tasks | Parallel processing |

## Thread Safety and Panics

### Handling Thread Panics

When a thread panics, `join()` returns `Err`:

```rust
let handle = thread::spawn(|| {
    panic!("Something went wrong!");
});

match handle.join() {
    Ok(_) => println!("Thread completed successfully"),
    Err(e) => println!("Thread panicked: {:?}", e),
}
```

### Unwinding vs Aborting

By default, Rust unwinds the stack when a thread panics. This:
- Runs destructors for local variables
- Allows other threads to continue
- Can be caught via `join()`

If you want panics to abort the entire program, set `panic = "abort"` in `Cargo.toml`.

## Practical Textio Example: Parallel Message Sending

```rust
use std::thread;

fn send_sms_batch(messages: Vec<String>) -> Vec<String> {
    let mut handles = Vec::new();
    
    for (i, msg) in messages.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            // Simulate sending
            thread::sleep(std::time::Duration::from_millis(100));
            format!("Message {} sent: {}", i, msg)
        });
        handles.push(handle);
    }
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}
```

## Thread Sleep and Timing

Use `thread::sleep` to pause a thread:

```rust
use std::thread;
use std::time::Duration;

thread::sleep(Duration::from_millis(100));
thread::sleep(Duration::from_secs(1));
```

This is useful for:
- Rate limiting API calls
- Simulating network latency
- Implementing retry delays

## Common Pitfalls

### 1. Forgetting to Join

```rust
thread::spawn(|| {
    println!("This might never print!");
});
// Main thread exits immediately
```

### 2. Use After Move

```rust
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
    data.len()
});
// println!("{:?}", data); // Error: data was moved
handle.join().unwrap();
```

### 3. Borrow in Thread Without Scope

```rust
let data = vec![1, 2, 3];
// This won't compile:
// thread::spawn(|| {
//     println!("{:?}", data);
// });
```

Use `move` or `thread::scope` instead.

## Performance Considerations

### Thread Creation Overhead

Creating threads has overhead. For many small tasks, consider:
- Thread pools (see `rayon` crate)
- Async/await (for I/O-bound tasks)
- Work stealing

### Number of Threads

A common pattern is to match the number of CPU cores:

```rust
use std::thread;

let num_threads = thread::available_parallelism()
    .map(|p| p.get())
    .unwrap_or(4);
```

## Summary

- `thread::spawn` creates new OS threads
- `JoinHandle::join()` waits for thread completion
- `move` transfers ownership into thread closures
- `thread::scope` allows borrowing from parent scope
- Thread panics can be caught via `join()`
- Use threads for CPU-bound parallelism

## Next Steps

In the next exercise, we'll explore `Arc<Mutex<T>>` for sharing state between threads safely.

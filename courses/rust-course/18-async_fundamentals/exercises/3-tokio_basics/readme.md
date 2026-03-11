# Exercise 3: Tokio Runtime Basics

## Introduction to Tokio

Tokio is Rust's most popular async runtime. It provides the infrastructure needed to execute async code, including:

- A multi-threaded work-stealing scheduler
- An I/O driver for async networking
- A time driver for timers and intervals
- Utilities for building async applications

## Why You Need a Runtime

Rust's async is zero-cost and doesn't include a built-in runtime. The compiler transforms async code into state machines, but something needs to execute them:

```rust
async fn my_function() -> i32 {
    42
}

fn main() {
    let future = my_function();
    future.await; // ERROR: .await is only allowed inside async functions
}
```

## The #[tokio::main] Macro

The most common way to use Tokio:

```rust
#[tokio::main]
async fn main() {
    println!("Hello from async!");
}
```

This expands to:

```rust
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            println!("Hello from async!");
        })
}
```

## Runtime Configuration

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Runs on 4 worker threads
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Single-threaded runtime (good for I/O-bound work)
}
```

### Multi-threaded vs Current-thread

- **Multi-threaded**: Best for CPU-bound work mixed with I/O
- **Current-thread**: Lower overhead for pure I/O-bound workloads

## tokio::spawn vs std::thread::spawn

### std::thread::spawn

```rust
std::thread::spawn(|| {
    std::thread::sleep(Duration::from_secs(1));
    println!("Thread complete");
});
```

- Creates a new OS thread
- Expensive (~1MB stack per thread)
- Limited by OS thread count

### tokio::spawn

```rust
tokio::spawn(async {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Task complete");
});
```

- Creates a lightweight task (green thread)
- Very cheap (~1KB initial allocation)
- Can spawn millions of tasks

### Comparison Table

| Feature | std::thread::spawn | tokio::spawn |
|---------|-------------------|--------------|
| Memory | ~1MB per thread | ~1KB per task |
| Creation | Slow (system call) | Fast (allocation) |
| Context switch | OS-managed | User-space |
| Max count | ~thousands | ~millions |
| Blocking | Blocks thread OK | Must not block |

## The join! Macro

Run multiple futures concurrently and wait for all:

```rust
use tokio::join;

async fn main() {
    let (result1, result2, result3) = join!(
        fetch_user(1),
        fetch_messages(1),
        fetch_settings(1),
    );
}
```

### Sequential vs Concurrent

```rust
// Sequential: takes ~300ms total
async fn sequential() {
    let a = delay_100ms().await;
    let b = delay_100ms().await;
    let c = delay_100ms().await;
}

// Concurrent: takes ~100ms total
async fn concurrent() {
    let (a, b, c) = join!(delay_100ms(), delay_100ms(), delay_100ms());
}
```

### Error Handling with join!

All branches run to completion regardless of failures:

```rust
let result = join!(
    async { Ok::<_, Error>(1) },
    async { Err(Error::Failed) },
    async { Ok(3) },
);
// result = (Ok(1), Err(Error::Failed), Ok(3))
```

## Task Handle

`tokio::spawn` returns a `JoinHandle`:

```rust
let handle: JoinHandle<Result<Message, Error>> = tokio::spawn(async {
    fetch_message(1).await
});

match handle.await {
    Ok(Ok(message)) => println!("Got message: {:?}", message),
    Ok(Err(e)) => println!("Task failed: {}", e),
    Err(e) => println!("Task panicked or was cancelled: {}", e),
}
```

## Blocking in Async Code

### The Problem

```rust
async fn bad_example() {
    std::thread::sleep(Duration::from_secs(1)); // Blocks the runtime thread!
}
```

This blocks the entire worker thread, preventing other tasks from running.

### The Solution: spawn_blocking

For unavoidable blocking operations:

```rust
async fn good_example() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_secs(1));
    }).await.unwrap();
}
```

## Tokio Tasks in Textio

### Concurrent Message Sending

```rust
async fn broadcast_sms(recipients: Vec<String>, message: String) -> Vec<DeliveryReport> {
    let handles: Vec<_> = recipients
        .into_iter()
        .map(|phone| {
            let msg = message.clone();
            tokio::spawn(async move {
                send_single_sms(&phone, &msg).await
            })
        })
        .collect();
    
    let mut results = Vec::new();
    for handle in handles {
        if let Ok(report) = handle.await.unwrap() {
            results.push(report);
        }
    }
    results
}
```

### Concurrent API Calls

```rust
async fn get_dashboard_data(user_id: u32) -> Dashboard {
    let (user, messages, stats) = join!(
        api_get_user(user_id),
        api_get_messages(user_id),
        api_get_stats(user_id),
    );
    
    Dashboard { user, messages, stats }
}
```

## Task Groups

For managing groups of related tasks:

```rust
async fn process_batch(messages: Vec<Message>) {
    let mut handles = Vec::new();
    
    for msg in messages {
        let handle = tokio::spawn(async move {
            process_message(msg).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.await;
    }
}
```

## Cancellation

Tasks can be cancelled by dropping their handle:

```rust
let handle = tokio::spawn(async {
    loop {
        work().await;
    }
});

drop(handle); // Cancels the task
```

## Best Practices

1. **Never block in async code** - Use `spawn_blocking` for blocking operations
2. **Use join! for known concurrent operations** - More efficient than spawn for few futures
3. **Handle JoinHandle errors** - Tasks can panic
4. **Consider task granularity** - Don't spawn tasks that are too small
5. **Use appropriate runtime** - `current_thread` for I/O-bound, `multi_thread` for mixed workloads

## Exercise Overview

In this exercise, you'll:

1. Use `#[tokio::main]` to run async code
2. Spawn tasks with `tokio::spawn`
3. Use `join!` for concurrent operations
4. Compare sequential vs concurrent execution
5. Build Textio's concurrent message delivery system

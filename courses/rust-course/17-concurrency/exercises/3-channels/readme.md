# Exercise 3: Channels - Message Passing Concurrency

## Overview

Channels provide a way for threads to communicate by sending messages. Rust's standard library includes `mpsc` (multiple producer, single consumer) channels. This approach follows the Go proverb: "Do not communicate by sharing memory; instead, share memory by communicating."

## Why Channels Matter for Textio

Textio uses channels for:
- Distributing messages to worker threads
- Collecting results from parallel processing
- Implementing event notification systems
- Building pipeline architectures for message processing

## MPSC Channels

The `std::sync::mpsc` module provides multiple-producer, single-consumer channels:

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread".to_string()).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

### Channel Components

1. **Transmitter (`tx`)**: Sends messages via `send()`
2. **Receiver (`rx`)**: Receives messages via `recv()` or iteration

## Sending Messages

### Basic Send

```rust
tx.send(value).unwrap();
```

`send()` returns `Result<(), SendError<T>>`. It fails if the receiver has been dropped.

### Non-Blocking Send

For bounded channels (created with `sync_channel`), `send()` can block:

```rust
let (tx, rx) = mpsc::sync_channel(3); // Buffer size of 3

tx.send(1).unwrap(); // Succeeds immediately
tx.send(2).unwrap();
tx.send(3).unwrap();
tx.send(4); // Blocks until receiver reads (or returns error if dropped)
```

### Try Send

```rust
match tx.try_send(value) {
    Ok(()) => println!("Sent"),
    Err(mpsc::TrySendError::Full(_)) => println!("Channel full"),
    Err(mpsc::TrySendError::Disconnected(_)) => println!("Receiver dropped"),
}
```

## Receiving Messages

### Blocking Receive

```rust
let value = rx.recv().unwrap();
```

`recv()` blocks until a message arrives or the sender is dropped. Returns `Result<T, RecvError>`.

### Non-Blocking Receive

```rust
match rx.try_recv() {
    Ok(value) => println!("Got: {}", value),
    Err(mpsc::TryRecvError::Empty) => println!("No message yet"),
    Err(mpsc::TryRecvError::Disconnected) => println!("Senders dropped"),
}
```

### Iterating Over Messages

```rust
for message in rx {
    println!("Received: {}", message);
}
```

The iterator blocks waiting for messages and ends when all senders are dropped.

### Non-Blocking Iteration

```rust
for message in rx.try_iter() {
    println!("Got: {}", message);
}
```

`try_iter()` returns an iterator over currently available messages without blocking.

## Multiple Producers

Clone the transmitter to create multiple producers:

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();

thread::spawn(move || {
    tx.send("From thread 1").unwrap();
});

thread::spawn(move || {
    tx2.send("From thread 2").unwrap();
});

for msg in rx {
    println!("Received: {}", msg);
}
```

## Channel Types

### Unbounded Channel

```rust
let (tx, rx) = mpsc::channel();
```

- No limit on buffered messages
- `send()` never blocks
- Can cause memory exhaustion if sender is faster than receiver

### Bounded (Synchronous) Channel

```rust
let (tx, rx) = mpsc::sync_channel(5); // Buffer size 5
```

- Limited buffer
- `send()` blocks when buffer is full
- Provides backpressure to slow down fast senders

## Practical Textio Example: Worker Pool

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // Spawn workers
    let mut workers = vec![];
    for _ in 0..4 {
        let tx = tx.clone();
        workers.push(thread::spawn(move || {
            // Simulate work
            tx.send("Task completed").unwrap();
        }));
    }
    
    // Drop the original sender so rx.iter() terminates
    drop(tx);
    
    // Collect results
    for result in rx {
        println!("Worker: {}", result);
    }
    
    for worker in workers {
        worker.join().unwrap();
    }
}
```

## Channel Patterns

### Pipeline Pattern

```rust
fn pipeline() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    // Stage 1: Parse
    thread::spawn(move || {
        for msg in rx1 {
            tx2.send(format!("Parsed: {}", msg)).unwrap();
        }
    });
    
    // Stage 2: Process
    thread::spawn(move || {
        for msg in rx2 {
            println!("Final: {}", msg);
        }
    });
    
    tx1.send("raw message".to_string()).unwrap();
}
```

### Fan-Out Pattern

Multiple workers receive from the same channel:

```rust
let (tx, rx) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));

for _ in 0..4 {
    let rx = Arc::clone(&rx);
    thread::spawn(move || {
        loop {
            let msg = rx.lock().unwrap().recv();
            match msg {
                Ok(m) => println!("Worker got: {}", m),
                Err(_) => break,
            }
        }
    });
}
```

### Fan-In Pattern

Multiple senders, one receiver:

```rust
let (tx, rx) = mpsc::channel();

for i in 0..4 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(format!("From sender {}", i)).unwrap();
    });
}

drop(tx); // Important: drop original sender

for msg in rx {
    println!("Received: {}", msg);
}
```

## Tokio Channels (Brief Overview)

For async Rust, `tokio::sync` provides additional channel types:

### Broadcast Channel

Multiple consumers, each gets a copy of every message:

```rust
let tx = tokio::sync::broadcast::channel(16).0;
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

tx.send("message").unwrap();
// Both rx1 and rx2 receive "message"
```

### Oneshot Channel

Single message, single receiver:

```rust
let (tx, rx) = tokio::sync::oneshot::channel();
tx.send("result".to_string()).unwrap();
let result = rx.await.unwrap();
```

## Error Handling

### SendError

```rust
match tx.send(value) {
    Ok(()) => {}
    Err(SendError(v)) => {
        // Receiver was dropped, v is the failed value
        println!("Failed to send: {}", v);
    }
}
```

### RecvError

```rust
match rx.recv() {
    Ok(v) => println!("Got: {}", v),
    Err(RecvError) => println!("All senders dropped"),
}
```

## Best Practices

1. **Drop senders when done**: The receiver's iterator only terminates when all senders are dropped
2. **Use bounded channels**: Prevent memory exhaustion in production
3. **Handle errors**: Channels can disconnect unexpectedly
4. **Consider capacity**: Match buffer size to your workload

## Summary

- `mpsc::channel()` creates unbounded channels
- `mpsc::sync_channel(n)` creates bounded channels with buffer size n
- Clone transmitters for multiple producers
- `recv()` blocks, `try_recv()` doesn't
- Iterate over `rx` for continuous receiving
- Drop all senders to terminate receiver iteration

## Next Steps

In the next exercise, we'll explore the `Send` and `Sync` traits that make thread safety possible.

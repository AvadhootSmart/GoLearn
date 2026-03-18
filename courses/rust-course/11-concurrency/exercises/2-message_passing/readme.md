# Message Passing with Channels

## Why this matters
Rust encourages "share memory by communicating." Channels let threads exchange ownership safely without data races.

## Core concept
`std::sync::mpsc::channel()` creates:
- sender (`tx`) used by producer thread,
- receiver (`rx`) used by consumer thread.

Values sent through channels are moved to the receiver.

## Worked example
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("ready")).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("message {}", msg);
}
```

## Common mistakes
- Forgetting `move` in spawned closure when transferring sender ownership.
- Using `recv` blindly without handling channel closure in larger programs.

## Exercise task

1. Create a channel with `std::sync::mpsc::channel()`.
2. Spawn a thread that sends `String::from("ready")`.
3. Receive in main thread and print `message <value>`.

## Quick recap
Channels provide safe ownership transfer between threads and clearer concurrency design.

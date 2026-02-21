# Concurrency in Rust

## Concept first: running work in parallel with threads

Rust lets you spawn OS threads using `std::thread::spawn`. A spawned thread runs concurrently with the main thread.

Example:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=2 {
            println!("worker step {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..=2 {
        println!("main step {}", i);
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();
}
```

Nuances:

- Output order can vary because scheduling is nondeterministic.
- `join()` waits for the spawned thread to finish.
- Forgetting to join can let `main` exit before spawned work completes.

## Exercise task

1. Import `std::thread` and `std::time::Duration`.
2. Spawn a thread that loops 3 times, prints `"hi number {} from the spawned thread!"`, and sleeps for 1 millisecond.
3. In the main thread, loop 3 times, print `"hi number {} from the main thread!"`, and sleep for 1 millisecond.
4. Call `.join().unwrap()` on the handle returned by `thread::spawn` at the end of `main` to ensure the spawned thread finishes.

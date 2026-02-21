# Concurrency in Rust

Rust provides safe concurrency by design. You can spawn threads to run code simultaneously and use channels or mutexes to share state safely.

## Assignment

1. Import `std::thread` and `std::time::Duration`.
2. Spawn a thread that loops 3 times, prints `"hi number {} from the spawned thread!"`, and sleeps for 1 millisecond.
3. In the main thread, loop 3 times, print `"hi number {} from the main thread!"`, and sleep for 1 millisecond.
4. Call `.join().unwrap()` on the handle returned by `thread::spawn` at the end of `main` to ensure the spawned thread finishes.

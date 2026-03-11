# Exercise 2: Arc<Mutex<T>> - Shared State Concurrency

## Overview

When multiple threads need to access and modify shared data, Rust requires safe synchronization. The combination of `Arc<Mutex<T>>` is the standard pattern for shared mutable state across threads.

## Why Shared State Matters for Textio

Textio needs shared state for:
- Tracking message counts across worker threads
- Maintaining a shared queue of pending messages
- Aggregating delivery status from multiple sources
- Rate limiting across concurrent requests

## Arc: Atomic Reference Counting

`Arc<T>` enables multiple ownership of the same data across threads:

```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);

// Both data and data_clone point to the same vector
```

### Arc vs Rc

| Feature | `Rc<T>` | `Arc<T>` |
|---------|---------|----------|
| Thread-safe | No | Yes |
| Performance | Faster | Slightly slower (atomic ops) |
| Use case | Single-threaded | Multi-threaded |

`Arc` uses atomic operations for reference counting, making it safe across threads. `Rc` is not thread-safe and will cause a compile error if used across threads.

### Cloning Arc

`Arc::clone(&arc)` increments the reference count atomically. When all references are dropped, the data is deallocated:

```rust
let arc1 = Arc::new(String::from("shared"));
let arc2 = Arc::clone(&arc1);  // ref count = 2
let arc3 = Arc::clone(&arc1);  // ref count = 3
drop(arc2);                     // ref count = 2
drop(arc3);                     // ref count = 1
drop(arc1);                     // ref count = 0, data freed
```

## Mutex: Mutual Exclusion

`Mutex<T>` ensures only one thread can access the data at a time:

```rust
use std::sync::Mutex;

let mutex = Mutex::new(0);
{
    let mut num = mutex.lock().unwrap();
    *num += 1;
} // Lock is automatically released here
```

### How Mutex Works

1. `lock()` acquires the lock, blocking if another thread holds it
2. Returns a `MutexGuard<T>` which dereferences to `&mut T`
3. The lock is released when the guard is dropped (at end of scope)

### Why Mutex is Necessary

Without synchronization, concurrent access causes data races:

```rust
// This would be unsafe without Mutex:
let mut counter = 0;
thread::spawn(|| counter += 1);  // Won't compile!
thread::spawn(|| counter += 1);
```

## Combining Arc and Mutex

`Arc<Mutex<T>>` gives us:
- `Arc`: Multiple owners across threads
- `Mutex`: Safe mutable access

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Counter: {}", *counter.lock().unwrap()); // 10
```

## Lock Poisoning

When a thread panics while holding a lock, the mutex becomes "poisoned":

```rust
let mutex = Mutex::new(0);

let handle = thread::spawn(|| {
    let _guard = mutex.lock().unwrap();
    panic!("Oops!");
});

handle.join().unwrap_err(); // Thread panicked

// The mutex is now poisoned
match mutex.lock() {
    Ok(guard) => println!("Got the lock"),
    Err(poison_error) => {
        // Can still recover the data:
        let guard = poison_error.into_inner();
        println!("Recovered: {}", *guard);
    }
}
```

### Handling Poison

1. `lock()` returns `Result<MutexGuard, PoisonError>`
2. `PoisonError::into_inner()` recovers the guard
3. `PoisonError::get_ref()` accesses the inner mutex

## RwLock: Read-Write Lock

For read-heavy workloads, `RwLock<T>` allows multiple readers or one writer:

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// Multiple readers can hold the lock simultaneously
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    println!("Read: {} and {}", *r1, *r2);
} // Both read locks released

// Only one writer at a time, blocks all readers
{
    let mut w = lock.write().unwrap();
    *w += 1;
}
```

### RwLock vs Mutex

| Scenario | Best Choice |
|----------|-------------|
| Frequent writes | `Mutex` |
| Frequent reads | `RwLock` |
| Short critical sections | `Mutex` |
| Long read operations | `RwLock` |

## Practical Textio Example: Message Counter

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct MessageCounter {
    sent: usize,
    failed: usize,
}

fn main() {
    let counter = Arc::new(Mutex::new(MessageCounter { sent: 0, failed: 0 }));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut c = counter.lock().unwrap();
            c.sent += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.lock().unwrap();
    println!("Messages sent: {}", final_count.sent);
}
```

## Deadlock Prevention

### Common Deadlock Patterns

1. **Lock ordering**: Always acquire locks in the same order
2. **Holding locks too long**: Release before calling unknown code
3. **Re-entrant locking**: Rust's Mutex is not re-entrant

### Example of Deadlock

```rust
let a = Arc::new(Mutex::new(0));
let b = Arc::new(Mutex::new(0));

// Thread 1: locks a, then tries to lock b
// Thread 2: locks b, then tries to lock a
// DEADLOCK!
```

### Prevention

```rust
// Always lock in consistent order
let (lock_a, lock_b) = if a.as_ref() as *const _ < b.as_ref() as *const _ {
    (a.lock().unwrap(), b.lock().unwrap())
} else {
    (b.lock().unwrap(), a.lock().unwrap())
};
```

## Interior Mutability Patterns

| Type | Thread-Safe | Use Case |
|------|-------------|----------|
| `RefCell<T>` | No | Single-threaded mutation |
| `Mutex<T>` | Yes | Multi-threaded mutation |
| `RwLock<T>` | Yes | Read-heavy multi-threaded |
| `Cell<T>` | No | Copy types, single-threaded |

## Performance Tips

1. **Minimize lock duration**: Hold locks for the shortest time possible
2. **Use fine-grained locks**: Multiple small locks vs one big lock
3. **Consider lock-free structures**: For high-contention scenarios
4. **Batch operations**: Reduce lock/unlock cycles

## Summary

- `Arc<T>` enables shared ownership across threads
- `Mutex<T>` provides mutual exclusion for safe mutation
- `Arc<Mutex<T>>` combines both for thread-safe shared state
- Lock poisoning occurs when a thread panics while holding a lock
- `RwLock<T>` allows multiple readers or one writer
- Always be aware of potential deadlocks

## Next Steps

In the next exercise, we'll explore channels for message-passing concurrency, an alternative to shared state.

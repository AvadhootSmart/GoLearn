# Exercise 3: Arc<T> - Atomic Reference Counting and Thread Safety

## Overview

`Arc<T>` (Atomically Reference Counted) is the thread-safe version of `Rc<T>`. It enables safe shared ownership across multiple threads, making it essential for concurrent programming in Rust.

## What is Arc<T>?

`Arc<T>` provides:
- Thread-safe reference counting using atomic operations
- Multiple ownership across threads
- Automatic deallocation when last reference drops
- Same API as `Rc<T>` but with thread safety guarantees

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    println!("Thread sees: {:?}", data_clone);
});

println!("Main sees: {:?}", data);
```

## Arc vs Rc

| Feature | Rc | Arc |
|---------|-----|-----|
| Thread-safe | No | Yes |
| Performance | Faster | Slower (atomic ops) |
| Send trait | No | Yes (if T: Send + Sync) |
| Sync trait | No | Yes (if T: Send + Sync) |
| Use case | Single-threaded | Multi-threaded |

## Atomic Operations

Arc uses atomic operations for reference counting:
- `fetch_add` - atomically increment
- `fetch_sub` - atomically decrement
- These ensure no data races in counting

```rust
// Arc internally uses atomic operations
let arc = Arc::new(42);
let arc2 = Arc::clone(&arc);  // Atomic increment
drop(arc2);                    // Atomic decrement
```

## Send and Sync Traits

For `Arc<T>` to be sent between threads:
- `T` must be `Send` (safe to transfer ownership)
- `T` must be `Sync` (safe to share references)

```rust
// Arc<T> implements Send when T: Send + Sync
let arc: Arc<String> = Arc::new("hello".to_string());
thread::spawn(move || {
    println!("{}", arc);
});
```

## Common Patterns in Textio

### Shared Configuration Across Workers

```rust
use std::sync::Arc;
use std::thread;

struct Config {
    api_endpoint: String,
    timeout_ms: u64,
}

fn spawn_workers(config: Arc<Config>, worker_count: usize) {
    let handles: Vec<_> = (0..worker_count)
        .map(|id| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                println!("Worker {} using {}", id, config.api_endpoint);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Shared Message Queue

```rust
use std::sync::Arc;
use std::sync::Mutex;

let queue = Arc::new(Mutex::new(Vec::new()));

let producer = {
    let queue = Arc::clone(&queue);
    thread::spawn(move || {
        queue.lock().unwrap().push("message".to_string());
    })
};

let consumer = {
    let queue = Arc::clone(&queue);
    thread::spawn(move || {
        if let Some(msg) = queue.lock().unwrap().pop() {
            println!("Received: {}", msg);
        }
    })
};
```

### Shared Statistics Counter

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

struct Stats {
    messages_sent: AtomicU64,
    errors: AtomicU64,
}

let stats = Arc::new(Stats {
    messages_sent: AtomicU64::new(0),
    errors: AtomicU64::new(0),
});

// Thread 1
stats.messages_sent.fetch_add(1, Ordering::SeqCst);

// Thread 2
let count = stats.messages_sent.load(Ordering::SeqCst);
```

## Arc<Mutex<T>> Pattern

The most common pattern for shared mutable state:

```rust
use std::sync::{Arc, Mutex};

let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

let mut handles = vec![];

for i in 0..3 {
    let data = Arc::clone(&shared_data);
    handles.push(thread::spawn(move || {
        let mut data = data.lock().unwrap();
        data.push(i);
    }));
}

for handle in handles {
    handle.join().unwrap();
}

println!("{:?}", shared_data.lock().unwrap());
```

## Arc with RwLock

For read-heavy workloads:

```rust
use std::sync::{Arc, RwLock};

let cache = Arc::new(RwLock::new(HashMap::new()));

// Multiple readers can access simultaneously
let r1 = cache.read().unwrap();
let r2 = cache.read().unwrap();

// Only one writer at a time
let mut w = cache.write().unwrap();
```

## Performance Considerations

### Atomic Operation Cost

```rust
// Arc clone is relatively cheap but not free
// Each clone involves an atomic increment
let arc = Arc::new(data);

// Batch operations when possible
let clones: Vec<_> = (0..100).map(|_| Arc::clone(&arc)).collect();
```

### Memory Layout

```
Arc<T> Structure:
                    
Thread 1 ──┐
Thread 2 ──┼──> [atomic_strong_count: 3]
Thread 3 ──┘    [atomic_weak_count: 0]
                [data: T]
```

## When to Use Arc

1. **Shared configuration** - Multiple threads need same config
2. **Worker pools** - Share state among workers
3. **Caches** - Shared read-only or read-mostly data
4. **Metrics** - Collecting stats from multiple threads
5. **Circuit breakers** - Shared state for fault tolerance

## Textio Example: Multi-threaded Message Processor

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct MessageProcessor {
    config: Arc<Config>,
    stats: Arc<Stats>,
    queue: Arc<Mutex<Vec<Message>>>,
}

impl MessageProcessor {
    fn spawn_workers(&self, count: usize) -> Vec<thread::JoinHandle<()>> {
        (0..count)
            .map(|id| {
                let config = Arc::clone(&self.config);
                let stats = Arc::clone(&self.stats);
                let queue = Arc::clone(&self.queue);
                
                thread::spawn(move || {
                    loop {
                        let msg = {
                            let mut q = queue.lock().unwrap();
                            q.pop()
                        };
                        
                        match msg {
                            Some(m) => {
                                process_message(&m, &config);
                                stats.processed.fetch_add(1, Ordering::SeqCst);
                            }
                            None => break,
                        }
                    }
                })
            })
            .collect()
    }
}
```

## Common Pitfalls

### 1. Using Rc Instead of Arc

```rust
// WRONG - Won't compile
use std::rc::Rc;
let rc = Rc::new(42);
thread::spawn(move || { println!("{}", rc); });

// CORRECT
use std::sync::Arc;
let arc = Arc::new(42);
thread::spawn(move || { println!("{}", arc); });
```

### 2. Deadlocks with Mutex

```rust
// Potential deadlock
let data1 = Arc::new(Mutex::new(1));
let data2 = Arc::new(Mutex::new(2));

// Thread 1: locks data1, then data2
// Thread 2: locks data2, then data1
// DEADLOCK!
```

### 3. Holding Locks Too Long

```rust
// BAD - Holding lock during slow operation
let data = arc.lock().unwrap();
send_to_network(&data);  // Slow!

// GOOD - Clone before releasing lock
let data = arc.lock().unwrap().clone();
drop(data);  // Release lock
send_to_network(&data);
```

## Arc Weak References

Like Rc, Arc has weak references:

```rust
use std::sync::{Arc, Weak};

let strong = Arc::new(42);
let weak: Weak<i32> = Arc::downgrade(&strong);

if let Some(value) = weak.upgrade() {
    println!("Still alive: {}", value);
}

drop(strong);
assert!(weak.upgrade().is_none());
```

## Best Practices

1. **Use Arc only when needed** - Rc is faster for single-threaded
2. **Minimize lock time** - Hold locks for shortest possible duration
3. **Prefer immutable data** - Arc<T> without Mutex when possible
4. **Use RwLock for read-heavy** - Multiple readers, single writer
5. **Clone Arc, not the data** - Arc::clone is cheap

## Exercise Task

In this exercise, you will implement:
1. A thread-safe configuration system
2. A parallel message processor with shared stats
3. A thread-safe message cache
4. A worker pool pattern

Focus on understanding how Arc enables safe sharing across threads and when to combine it with synchronization primitives.

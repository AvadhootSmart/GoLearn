# Exercise 4: Send and Sync Traits

## Overview

The `Send` and `Sync` traits are marker traits that form the foundation of Rust's thread safety guarantees. Understanding these traits is essential for writing correct concurrent code and for implementing thread-safe types.

## Why Send and Sync Matter for Textio

Textio's concurrent architecture requires:
- Safe transfer of data between threads
- Thread-safe shared references
- Custom types that work correctly in multi-threaded contexts
- Understanding why certain types can't cross thread boundaries

## The Send Trait

`Send` marks types that can be safely transferred to another thread:

```rust
pub unsafe trait Send {}
```

A type is `Send` if it's safe for another thread to own it. Most types in Rust are `Send`.

### Types That Are Send

- All primitive types (`i32`, `bool`, `char`, etc.)
- Most standard library types (`Vec<T>`, `String`, `Option<T>`)
- Types composed of `Send` types
- `Arc<T>` (but not `Rc<T>`)
- `Mutex<T>` and `RwLock<T>`

### Types That Are NOT Send

- `Rc<T>` - reference counting is not atomic
- `*const T` and `*mut T` - raw pointers
- Types containing non-`Send` fields

### Example: Rc is Not Send

```rust
use std::rc::Rc;
use std::thread;

let rc = Rc::new(5);
thread::spawn(move || {  // Compile error!
    println!("{}", rc);
});
```

Error: `Rc<i32>` cannot be sent between threads safely. Use `Arc` instead.

## The Sync Trait

`Sync` marks types that can be safely shared between threads via references:

```rust
pub unsafe trait Sync {}
```

A type is `Sync` if it's safe for multiple threads to have `&T` references to it simultaneously.

### Types That Are Sync

- All primitive types
- Most immutable standard library types
- `Mutex<T>` and `RwLock<T>`
- `Arc<T>` where `T: Sync`

### Types That Are NOT Sync

- `Cell<T>` and `RefCell<T>` - interior mutability without synchronization
- `Rc<T>` - not thread-safe reference counting
- Raw pointers

### Example: RefCell is Not Sync

```rust
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

let data = Arc::new(RefCell::new(5));
let data2 = Arc::clone(&data);
thread::spawn(move || {  // Compile error!
    *data2.borrow_mut() += 1;
});
```

Error: `RefCell<i32>` cannot be shared between threads safely. Use `Mutex` instead.

## The Relationship Between Send and Sync

There's an important relationship:

```
T: Sync  <=>  &T: Send
```

If a type is `Sync`, references to it can be sent to other threads. This makes sense: if it's safe for multiple threads to have references, it's safe to send those references.

### Examples

| Type | Send | Sync | Reason |
|------|------|------|--------|
| `i32` | Yes | Yes | Primitive, no interior mutability |
| `String` | Yes | Yes | Heap-allocated, owned |
| `Vec<T>` | Yes | Yes | If `T: Send` |
| `Rc<T>` | No | No | Non-atomic ref count |
| `Arc<T>` | Yes | Yes (if T: Sync) | Atomic ref count |
| `Mutex<T>` | Yes | Yes | Synchronized access |
| `RefCell<T>` | Yes | No | Unsynchronized mutation |
| `Cell<T>` | Yes | No | Unsynchronized mutation |

## Implementing Send and Sync

### Automatic Derivation

Most types automatically implement `Send` and `Sync`:

```rust
struct MyStruct {
    id: u32,
    name: String,
}
// MyStruct is automatically Send and Sync
```

### Manual Implementation (Unsafe)

Sometimes you need to manually implement these traits. This is `unsafe`:

```rust
struct RawPointerWrapper {
    ptr: *mut i32,
}

// SAFETY: We ensure ptr is only accessed from one thread at a time
unsafe impl Send for RawPointerWrapper {}
```

**Warning**: Incorrect implementations can cause undefined behavior!

### When to Implement

Only implement `Send` or `Sync` manually when:
1. You're wrapping a thread-safe primitive
2. You can guarantee the safety invariants
3. You understand the implications fully

## Data Race Prevention

Rust's type system prevents data races at compile time:

### What is a Data Race?

A data race occurs when:
1. Two or more threads access the same memory concurrently
2. At least one access is a write
3. There's no synchronization

### How Rust Prevents Data Races

```rust
let mut data = vec![1, 2, 3];

// This won't compile - can't have mutable and immutable references
// across threads without synchronization
thread::spawn(|| {
    data.push(4);  // mutable borrow
});
println!("{:?}", data);  // immutable borrow
```

### The Three Ways to Share Data

1. **Immutable sharing**: Multiple `&T` references (requires `T: Sync`)
2. **Mutex**: `Arc<Mutex<T>>` for mutable sharing
3. **Message passing**: Send data through channels (requires `T: Send`)

## Practical Textio Example: Thread-Safe Message Queue

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct MessageQueue {
    queue: Mutex<Vec<String>>,
}

// MessageQueue is Send + Sync because Mutex<T> is Send + Sync
impl MessageQueue {
    fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
        }
    }
    
    fn push(&self, msg: String) {
        self.queue.lock().unwrap().push(msg);
    }
    
    fn pop(&self) -> Option<String> {
        self.queue.lock().unwrap().pop()
    }
}

fn main() {
    let mq = Arc::new(MessageQueue::new());
    
    let producer = {
        let mq = Arc::clone(&mq);
        thread::spawn(move || {
            mq.push("Hello".to_string());
        })
    };
    
    let consumer = {
        let mq = Arc::clone(&mq);
        thread::spawn(move || {
            if let Some(msg) = mq.pop() {
                println!("Got: {}", msg);
            }
        })
    };
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

## Common Patterns and Pitfalls

### Pattern: Thread-Local Storage

For data that shouldn't cross threads:

```rust
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<i32> = RefCell::new(0);
}

fn increment() {
    COUNTER.with(|c| {
        *c.borrow_mut() += 1;
    });
}
```

### Pitfall: Capturing Rc in Threads

```rust
use std::rc::Rc;
use std::thread;

let data = Rc::new(5);
// This won't compile:
// thread::spawn(move || {
//     println!("{}", data);
// });

// Solution: Use Arc
use std::sync::Arc;
let data = Arc::new(5);
thread::spawn(move || {
    println!("{}", data);
});
```

### Pitfall: Interior Mutability Across Threads

```rust
use std::cell::Cell;
use std::sync::Arc;

let data = Arc::new(Cell::new(5));
// This won't compile for Sync requirement:
// thread::spawn(move || {
//     data.set(10);
// });

// Solution: Use Mutex or Atomic types
use std::sync::atomic::{AtomicI32, Ordering};
let data = Arc::new(AtomicI32::new(5));
thread::spawn(move || {
    data.store(10, Ordering::SeqCst);
});
```

## Atomic Types

For simple values, atomic types provide lock-free thread safety:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

let counter = Arc::new(AtomicUsize::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    }));
}

for h in handles {
    h.join().unwrap();
}

println!("Count: {}", counter.load(Ordering::SeqCst));
```

### Memory Orderings

- `Relaxed`: No ordering guarantees, only atomicity
- `Release`/`Acquire`: Establishes happens-before relationship
- `SeqCst`: Sequentially consistent, strongest guarantee

## Summary

- `Send`: Safe to transfer ownership to another thread
- `Sync`: Safe to share references between threads
- `T: Sync` implies `&T: Send`
- Most types are automatically `Send + Sync`
- `Rc`, `RefCell`, `Cell` are NOT thread-safe
- Use `Arc`, `Mutex`, atomics for thread-safe alternatives
- Data races are prevented at compile time

## Conclusion

Understanding `Send` and `Sync` is crucial for writing correct concurrent Rust code. These traits ensure that data races are caught at compile time, making Rust's concurrency guarantees possible.

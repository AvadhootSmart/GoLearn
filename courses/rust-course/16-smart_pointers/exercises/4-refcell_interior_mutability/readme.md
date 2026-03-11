# Exercise 4: RefCell<T> - Interior Mutability and Runtime Borrowing

## Overview

`RefCell<T>` provides interior mutability - the ability to mutate data even when there are immutable references to it. This moves Rust's borrow checking from compile time to runtime.

## What is RefCell<T>?

`RefCell<T>` wraps a value and provides:
- Runtime borrow checking instead of compile-time
- Interior mutability through `borrow()` and `borrow_mut()`
- Single-threaded use only (not thread-safe)
- Panics on borrow rule violations

```rust
use std::cell::RefCell;

let data = RefCell::new(5);

// Borrow immutably
let value = *data.borrow();
println!("{}", value);  // 5

// Borrow mutably
*data.borrow_mut() = 10;
println!("{}", *data.borrow());  // 10
```

## Interior Mutability Pattern

The interior mutability pattern allows mutation through immutable references:

```rust
// Normally, this wouldn't compile
fn modify(data: &i32) {
    *data = 5;  // Error: cannot mutate through &i32
}

// With RefCell, it works at runtime
fn modify(data: &RefCell<i32>) {
    *data.borrow_mut() = 5;  // OK: interior mutability
}
```

## Borrow Rules at Runtime

RefCell enforces the same rules as Rust, but at runtime:

1. **Many immutable borrows OR one mutable borrow**
2. **Violation causes panic**

```rust
let data = RefCell::new(5);

let ref1 = data.borrow();     // OK: 1 immutable
let ref2 = data.borrow();     // OK: 2 immutable
// let mut_ref = data.borrow_mut();  // PANIC: can't have mutable with immutable
drop(ref1);
drop(ref2);
let mut_ref = data.borrow_mut();  // OK: no other borrows
```

## When to Use RefCell

### 1. Mock Objects for Testing

```rust
trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(msg.to_string());
    }
}
```

### 2. Implementing Traits with Mutable State

```rust
trait Cache {
    fn get(&self, key: &str) -> Option<&String>;
    fn put(&self, key: &str, value: String);
}

struct SimpleCache {
    data: RefCell<HashMap<String, String>>,
}

impl Cache for SimpleCache {
    fn get(&self, key: &str) -> Option<&String> {
        // Can't return reference to RefCell contents
        // Would need different approach
        self.data.borrow().get(key).cloned()
    }

    fn put(&self, key: &str, value: String) {
        self.data.borrow_mut().insert(key.to_string(), value);
    }
}
```

### 3. Graph Structures with Cycles

```rust
struct Node {
    value: i32,
    edges: RefCell<Vec<Rc<Node>>>,
}
```

## Common Patterns in Textio

### Mutable Statistics in Immutable Context

```rust
struct MessageHandler {
    stats: RefCell<HandlerStats>,
}

struct HandlerStats {
    messages_processed: u32,
    errors: u32,
}

impl MessageHandler {
    fn handle(&self, message: &str) -> Result<(), Error> {
        match process(message) {
            Ok(_) => {
                self.stats.borrow_mut().messages_processed += 1;
                Ok(())
            }
            Err(e) => {
                self.stats.borrow_mut().errors += 1;
                Err(e)
            }
        }
    }
}
```

### Lazy Initialization

```rust
struct ExpensiveService {
    cache: RefCell<Option<HashMap<String, Data>>>,
}

impl ExpensiveService {
    fn get(&self, key: &str) -> Option<Data> {
        let cache = self.cache.borrow();
        if let Some(map) = cache.as_ref() {
            return map.get(key).cloned();
        }
        drop(cache);
        
        // Initialize lazily
        let mut cache = self.cache.borrow_mut();
        let map = self.load_cache();
        *cache = Some(map);
        cache.as_ref()?.get(key).cloned()
    }
}
```

### Observer Pattern

```rust
struct EventEmitter {
    listeners: RefCell<Vec<Box<dyn Fn(&str)>>>,
}

impl EventEmitter {
    fn subscribe(&self, listener: Box<dyn Fn(&str)>) {
        self.listeners.borrow_mut().push(listener);
    }

    fn emit(&self, event: &str) {
        for listener in self.listeners.borrow().iter() {
            listener(event);
        }
    }
}
```

## borrow() vs borrow_mut()

```rust
let data = RefCell::new(vec![1, 2, 3]);

// borrow() - returns Ref<T> (immutable reference)
let borrowed: Ref<Vec<i32>> = data.borrow();
println!("{:?}", *borrowed);
drop(borrowed);  // Must drop before borrow_mut

// borrow_mut() - returns RefMut<T> (mutable reference)
let mut borrowed_mut: RefMut<Vec<i32>> = data.borrow_mut();
borrowed_mut.push(4);
drop(borrowed_mut);

// try_borrow() and try_borrow_mut() - return Result instead of panicking
if let Ok(mut data) = data.try_borrow_mut() {
    data.push(5);
}
```

## RefCell with Rc

The common `Rc<RefCell<T>>` pattern:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

let ref1 = Rc::clone(&shared);
let ref2 = Rc::clone(&shared);

// Both can mutate
ref1.borrow_mut().push(4);
ref2.borrow_mut().push(5);

println!("{:?}", shared.borrow());  // [1, 2, 3, 4, 5]
```

## Graph with Shared Mutable Nodes

```rust
struct GraphNode {
    value: i32,
    neighbors: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(GraphNode {
            value,
            neighbors: RefCell::new(Vec::new()),
        })
    }

    fn add_neighbor(&self, node: Rc<GraphNode>) {
        self.neighbors.borrow_mut().push(node);
    }
}

let a = GraphNode::new(1);
let b = GraphNode::new(2);

a.add_neighbor(Rc::clone(&b));
b.add_neighbor(Rc::clone(&a));  // Cycle is OK with RefCell
```

## Common Pitfalls

### 1. Holding Borrows Too Long

```rust
let data = RefCell::new(5);

let borrow = data.borrow();
let sum = *borrow + data.borrow();  // PANIC: already borrowed
```

### 2. Creating Cycles That Leak

```rust
let a = Rc::new(RefCell::new(Node::new()));
let b = Rc::new(RefCell::new(Node::new()));

a.borrow_mut().next = Some(Rc::clone(&b));
b.borrow_mut().next = Some(Rc::clone(&a));  // Memory leak!
```

### 3. Using in Multi-threaded Context

```rust
// WON'T COMPILE - RefCell is not Send
let data = RefCell::new(5);
thread::spawn(move || {
    data.borrow();
});
```

## RefCell vs Cell

| Type | Works with | Copy required | Use case |
|------|------------|---------------|----------|
| Cell | Copy types | Yes | Simple replacement |
| RefCell | Any type | No | Complex mutation |

```rust
// Cell - for Copy types, simpler
let cell = Cell::new(5);
cell.set(10);

// RefCell - for any type, more flexible
let refcell = RefCell::new(vec![1, 2, 3]);
refcell.borrow_mut().push(4);
```

## Debugging Borrow Panics

```rust
let data = RefCell::new(5);

// Use try_borrow to avoid panics
match data.try_borrow() {
    Ok(value) => println!("{}", value),
    Err(_) => println!("Already borrowed mutably!"),
}

// Check borrow state (unstable, but useful for debugging)
// data.borrow_state()  // Only in unstable Rust
```

## Best Practices

1. **Keep borrows short** - Drop borrows as soon as possible
2. **Use try_borrow for safety** - Avoid panics in production
3. **Document interior mutability** - Make it clear in your API
4. **Consider Cell for Copy types** - Simpler and faster
5. **Combine with Rc for shared mutation** - Rc<RefCell<T>> pattern
6. **Use Arc<Mutex<T>> for threads** - Thread-safe alternative

## Exercise Task

In this exercise, you will implement:
1. A message handler with interior mutability for stats
2. A lazy-loaded configuration cache
3. An observable event system
4. A graph structure with mutable edges

Focus on understanding when RefCell enables patterns that wouldn't otherwise be possible.

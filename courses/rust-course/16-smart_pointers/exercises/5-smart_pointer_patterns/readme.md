# Exercise 5: Smart Pointer Patterns - Combining Types

## Overview

This exercise brings together all the smart pointer concepts to solve real-world problems. You'll learn to combine `Rc<RefCell<T>>`, use `Weak<T>` to break cycles, and choose the right pattern for each scenario.

## The Rc<RefCell<T>> Pattern

The most common combination for shared mutable state in single-threaded Rust:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

let ref1 = Rc::clone(&shared);
let ref2 = Rc::clone(&shared);

ref1.borrow_mut().push(4);
ref2.borrow_mut().push(5);

// All see the same data
assert_eq!(*shared.borrow(), vec![1, 2, 3, 4, 5]);
```

### When to Use Rc<RefCell<T>>

- Multiple owners need to mutate shared data
- Single-threaded context only
- Graph structures with mutable nodes
- Observer/listener patterns
- Shared caches with updates

## Weak<T> for Breaking Cycles

Reference cycles cause memory leaks. `Weak<T>` breaks them:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,  // Weak reference to parent
}

let root = Rc::new(Node {
    value: 1,
    children: RefCell::new(vec![]),
    parent: RefCell::new(Weak::new()),
});

let child = Rc::new(Node {
    value: 2,
    children: RefCell::new(vec![]),
    parent: RefCell::new(Rc::downgrade(&root)),
});

root.children.borrow_mut().push(Rc::clone(&child));

// No cycle because parent is Weak
```

## Common Patterns in Textio

### 1. Shared Mutable Configuration

```rust
struct AppConfig {
    settings: Rc<RefCell<HashMap<String, String>>>,
}

impl AppConfig {
    fn get(&self, key: &str) -> Option<String> {
        self.settings.borrow().get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) {
        self.settings.borrow_mut().insert(key.to_string(), value.to_string());
    }
}
```

### 2. Observable Data Store

```rust
struct DataStore<T> {
    data: Rc<RefCell<T>>,
    observers: RefCell<Vec<Box<dyn Fn(&T)>>>,
}

impl<T: Clone> DataStore<T> {
    fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.data.borrow_mut());
        let data = self.data.borrow();
        for observer in self.observers.borrow().iter() {
            observer(&data);
        }
    }
}
```

### 3. Bidirectional Graph

```rust
struct MessageNode {
    id: u64,
    content: String,
    replies: RefCell<Vec<Rc<MessageNode>>>,
    parent: RefCell<Weak<MessageNode>>,
}
```

## Pattern Selection Guide

| Requirement | Pattern |
|-------------|---------|
| Heap allocation only | Box<T> |
| Shared read-only | Rc<T> |
| Shared read-only, threads | Arc<T> |
| Interior mutability | RefCell<T> or Cell<T> |
| Shared mutable | Rc<RefCell<T>> |
| Thread-safe shared mutable | Arc<Mutex<T>> or Arc<RwLock<T>> |
| Avoid cycles | Weak<T> |
| Trait objects | Box<dyn Trait> or Arc<dyn Trait> |

## Advanced Patterns

### Self-Referential Structures

```rust
struct SelfRef {
    data: String,
    // Can't directly have &String pointing to data
    // Use patterns like ouroboros crate or Pin
}
```

### Arena Allocation

```rust
struct Arena<T> {
    items: RefCell<Vec<Rc<T>>>,
}

impl<T> Arena<T> {
    fn alloc(&self, item: T) -> Rc<T> {
        let rc = Rc::new(item);
        self.items.borrow_mut().push(Rc::clone(&rc));
        rc
    }
}
```

### Copy-on-Write

```rust
struct CowData {
    data: Rc<RefCell<Vec<i32>>>,
}

impl CowData {
    fn modify(&mut self, index: usize, value: i32) {
        let mut data = self.data.borrow_mut();
        if Rc::strong_count(&self.data) > 1 {
            // Clone before modifying if shared
            *self.data = Rc::new(RefCell::new(data.clone()));
        }
        self.data.borrow_mut()[index] = value;
    }
}
```

## Memory Management Patterns

### Parent-Child with Back-Reference

```rust
struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    parent: Weak<Parent>,
}

impl Parent {
    fn add_child(&self, child: Rc<Child>) {
        self.children.borrow_mut().push(child);
    }
}
```

### Object Pool

```rust
struct Pool<T> {
    available: RefCell<Vec<Rc<T>>>,
    in_use: RefCell<Vec<Rc<T>>>,
}

impl<T> Pool<T> {
    fn acquire(&self) -> Option<Rc<T>> {
        let item = self.available.borrow_mut().pop()?;
        self.in_use.borrow_mut().push(Rc::clone(&item));
        Some(item)
    }

    fn release(&self, item: Rc<T>) {
        self.in_use.borrow_mut().retain(|i| !Rc::ptr_eq(i, &item));
        self.available.borrow_mut().push(item);
    }
}
```

## Error Handling with Smart Pointers

### Handling Borrow Errors

```rust
let data = RefCell::new(vec![1, 2, 3]);

match data.try_borrow_mut() {
    Ok(mut vec) => vec.push(4),
    Err(_) => println!("Already borrowed!"),
}
```

### Handling Weak Upgrades

```rust
let weak: Weak<Vec<i32>> = Rc::downgrade(&Rc::new(vec![1, 2, 3]));

match weak.upgrade() {
    Some(rc) => println!("Still alive: {:?}", *rc),
    None => println!("Data was dropped"),
}
```

## Performance Considerations

### Rc vs Arc Overhead

```rust
// Rc: ~single instruction for increment/decrement
let rc = Rc::new(42);

// Arc: atomic operations, more expensive
let arc = Arc::new(42);
```

### RefCell Runtime Checks

```rust
// RefCell checks at runtime - small but nonzero cost
let cell = RefCell::new(42);
let _ = cell.borrow();  // Runtime check
let _ = cell.borrow();  // Another check
```

### Minimize Lock Time

```rust
// BAD - holding lock during computation
let data = rc.borrow_mut();
process_data(&data);  // Slow computation while locked

// GOOD - extract data, release lock
let data = rc.borrow_mut().clone();
drop(data);  // Explicit drop
process_data(&data);  // Compute without lock
```

## Debugging Tips

### Finding Leaks

```rust
// Check strong counts
println!("Strong count: {}", Rc::strong_count(&rc));
println!("Weak count: {}", Rc::weak_count(&rc));

// If strong count never reaches 0, you have a cycle
```

### Tracking Borrows

```rust
let data = RefCell::new(42);

// Use try_borrow to debug
if data.try_borrow().is_err() {
    println!("Already borrowed mutably!");
}
```

## Best Practices Summary

1. **Use the simplest type that works** - Box before Rc before Arc
2. **Avoid interior mutability when possible** - Pure functions are easier to reason about
3. **Use Weak for back-references** - Prevents reference cycles
4. **Keep borrows short** - Release locks quickly
5. **Document ownership patterns** - Help future readers understand your design
6. **Test for cycles** - Check strong counts in tests
7. **Consider thread safety early** - Rc to Arc refactoring is painful

## Exercise Task

In this exercise, you will implement:
1. A doubly-linked message thread with parent references
2. A shared state manager with observers
3. A message pool for reusing allocations
4. A reference-counted cache with eviction

Focus on combining smart pointers effectively and understanding when each pattern is appropriate.

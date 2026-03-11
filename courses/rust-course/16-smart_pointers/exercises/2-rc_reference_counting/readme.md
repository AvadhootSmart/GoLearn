# Exercise 2: Rc<T> - Reference Counting and Multiple Ownership

## Overview

`Rc<T>` (Reference Counted) enables multiple ownership of the same data. When you need several parts of your program to access and share ownership of read-only data, `Rc<T>` is the solution.

## What is Rc<T>?

`Rc<T>` stands for Reference Counted smart pointer:
- Tracks the number of references to a value
- Allows multiple owners of the same data
- Automatically deallocates when the last reference is dropped
- Only works in single-threaded contexts

```rust
use std::rc::Rc;

let data = Rc::new(String::from("shared data"));
let reference1 = Rc::clone(&data);  // Creates new reference
let reference2 = Rc::clone(&data);

println!("Reference count: {}", Rc::strong_count(&data));  // 3
```

## Rc::clone vs clone()

Understanding the difference is critical:

```rust
let data = Rc::new(vec![1, 2, 3]);

// Rc::clone - increments reference count, cheap pointer copy
let ref1 = Rc::clone(&data);

// clone() on the inner value - creates deep copy of data
let deep_copy = (*data).clone();
```

## When to Use Rc<T>

### 1. Shared Configuration
```rust
struct AppConfig {
    api_key: String,
    rate_limit: u32,
}

struct SmsService {
    config: Rc<AppConfig>,
}

struct EmailService {
    config: Rc<AppConfig>,
}

let config = Rc::new(AppConfig {
    api_key: "secret".to_string(),
    rate_limit: 100,
});

let sms = SmsService { config: Rc::clone(&config) };
let email = EmailService { config: Rc::clone(&config) };
```

### 2. Graph Structures
```rust
struct Node {
    value: i32,
    neighbors: Vec<Rc<Node>>,
}

let node_a = Rc::new(Node { value: 1, neighbors: vec![] });
let node_b = Rc::new(Node { value: 2, neighbors: vec![Rc::clone(&node_a)] });
```

### 3. Tree with Parent References
```rust
struct TreeNode {
    value: String,
    children: Vec<Rc<TreeNode>>,
}
```

## Reference Counting Details

### Strong vs Weak References

```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(42);
let weak: Weak<i32> = Rc::downgrade(&strong);

println!("Strong count: {}", Rc::strong_count(&strong));  // 1
println!("Weak count: {}", Rc::weak_count(&strong));      // 1

// Weak references don't keep data alive
if let Some(value) = weak.upgrade() {
    println!("Value still exists: {}", value);
}
```

### Memory Layout

```
Rc<T> Structure:
                    
Reference 1 ──┐
Reference 2 ──┼──> [strong_count: 3]
Reference 3 ──┘    [weak_count: 0]
                   [data: T]
```

## Common Patterns in Textio

### Shared Message Templates

```rust
struct MessageTemplate {
    name: String,
    content: String,
    variables: Vec<String>,
}

struct Campaign {
    name: String,
    template: Rc<MessageTemplate>,
}

struct ScheduledMessage {
    recipient: String,
    template: Rc<MessageTemplate>,
    scheduled_time: DateTime,
}

let welcome_template = Rc::new(MessageTemplate {
    name: "welcome".to_string(),
    content: "Hello {name}!".to_string(),
    variables: vec!["name".to_string()],
});

let campaign = Campaign {
    name: "Spring Promo".to_string(),
    template: Rc::clone(&welcome_template),
};

let scheduled = ScheduledMessage {
    recipient: "+1234567890".to_string(),
    template: Rc::clone(&welcome_template),
    scheduled_time: now(),
};
```

### Shared Rate Limiter

```rust
struct RateLimiter {
    max_requests: u32,
    current_count: std::cell::Cell<u32>,
}

struct ApiEndpoint {
    path: String,
    limiter: Rc<RateLimiter>,
}

let limiter = Rc::new(RateLimiter {
    max_requests: 100,
    current_count: std::cell::Cell::new(0),
});

let endpoints = vec![
    ApiEndpoint { path: "/send".to_string(), limiter: Rc::clone(&limiter) },
    ApiEndpoint { path: "/status".to_string(), limiter: Rc::clone(&limiter) },
];
```

### Graph of Related Messages

```rust
struct Message {
    id: u64,
    content: String,
    related: Vec<Rc<Message>>,
}

impl Message {
    fn add_related(&mut self, other: Rc<Message>) {
        self.related.push(other);
    }
}

let msg1 = Rc::new(Message { id: 1, content: "Hello".to_string(), related: vec![] });
let msg2 = Rc::new(Message { id: 2, content: "Reply".to_string(), related: vec![Rc::clone(&msg1)] });
```

## Rc with Interior Mutability

For shared mutable state, combine with RefCell:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared_vec: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![1, 2, 3]));

let ref1 = Rc::clone(&shared_vec);
let ref2 = Rc::clone(&shared_vec);

ref1.borrow_mut().push(4);  // Modify through first reference
ref2.borrow_mut().push(5);  // Modify through second reference

println!("{:?}", shared_vec.borrow());  // [1, 2, 3, 4, 5]
```

## Comparing Ownership Models

```rust
// Single owner - data moves
let v = vec![1, 2, 3];
let v2 = v;  // v is moved, can't use v anymore

// Multiple owners with Rc
let v = Rc::new(vec![1, 2, 3]);
let v2 = Rc::clone(&v);  // v is still valid
let v3 = Rc::clone(&v);  // All three share ownership
```

## Rc<T> Limitations

1. **Not thread-safe** - Cannot be sent between threads
2. **Immutable by default** - Need RefCell for mutation
3. **Runtime overhead** - Reference counting has cost
4. **Reference cycles** - Can cause memory leaks

## Avoiding Reference Cycles

```rust
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: Weak<Node>,      // Weak to avoid cycles
    children: Vec<Rc<Node>>,
}

let root = Rc::new(Node {
    value: 1,
    parent: Weak::new(),
    children: vec![],
});

let child = Rc::new(Node {
    value: 2,
    parent: Rc::downgrade(&root),  // Weak reference to parent
    children: vec![],
});
```

## Best Practices

1. **Use Rc::clone for clarity** - Makes reference counting explicit
2. **Document shared ownership** - Explain why multiple owners are needed
3. **Watch for cycles** - Use Weak for back-references
4. **Consider Arc for threads** - If you need thread safety
5. **Measure performance** - Rc has overhead vs Box

## Rc<T> vs Box<T> vs Arc<T>

| Type | Ownership | Thread-safe | Use Case |
|------|-----------|-------------|----------|
| Box | Single | Yes | Heap allocation, recursive types |
| Rc | Multiple | No | Shared read-only data |
| Arc | Multiple | Yes | Shared data across threads |

## Exercise Task

In this exercise, you will implement:
1. A shared configuration system for Textio services
2. A message graph where messages can reference each other
3. A notification system with shared templates
4. A DAG (directed acyclic graph) for message workflows

Focus on understanding when Rc enables multiple ownership and how to use it correctly.

# Closure Capture Modes

## Introduction

Understanding how closures capture variables is crucial for writing correct and efficient Rust code. The `move` keyword and the `Fn`, `FnMut`, and `FnOnce` traits determine ownership and borrowing behavior of captured variables.

## How Closures Capture Variables

Rust closures capture variables differently based on how the variables are used inside the closure body:

1. **By Reference (`&T`)** - If the closure only reads the variable
2. **By Mutable Reference (`&mut T`)** - If the closure modifies the variable
3. **By Value (`T`)** - If the closure takes ownership of the variable

```rust
let list = vec![1, 2, 3];

// Captures by immutable reference
let only_reads = || println!("List: {:?}", list);

// Captures by mutable reference
let mut modifies = || list.push(4);

// Captures by value (takes ownership)
let consumes = || drop(list);
```

## The `move` Keyword

The `move` keyword forces the closure to take ownership of all captured variables:

```rust
let name = String::from("Alice");

// Without move: borrows name
let greet = || println!("Hello, {}", name);

// With move: takes ownership of name
let greet_moved = move || println!("Hello, {}", name);

// name is still valid here
println!("{}", name);  // OK

// After greet_moved, the moved name is gone
```

### When to Use `move`

#### 1. Returning Closures

```rust
fn create_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}
```

Without `move`, the closure would try to borrow `count`, but `count` is dropped when `create_counter` returns.

#### 2. Spawning Threads

```rust
let data = vec![1, 2, 3];

thread::spawn(move || {
    println!("Data in thread: {:?}", data);
});
```

The thread may outlive the current function, so we must move ownership.

#### 3. Storing in Structs

```rust
struct Handler<F> {
    callback: F,
}

let value = 42;
let handler = Handler {
    callback: move || value,  // Must own value
};
```

## Fn Trait Hierarchy

### `FnOnce`

Implemented by closures that might consume captured variables:

```rust
fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();  // Can only be called once
}

let data = vec![1, 2, 3];
call_once(move || {
    let _ = data;  // Consumes data
});
```

Characteristics:
- Can be called at most once
- Takes ownership of captured variables
- All closures implement this trait

### `FnMut`

Implemented by closures that modify captured variables but don't consume them:

```rust
fn call_repeatedly<F>(mut f: F, times: usize)
where
    F: FnMut(),
{
    for _ in 0..times {
        f();  // Can be called multiple times
    }
}

let mut count = 0;
call_repeatedly(|| count += 1, 5);
println!("Count: {}", count);  // 5
```

Characteristics:
- Can be called multiple times
- Mutably borrows captured variables
- Cannot be called while other borrows exist

### `Fn`

Implemented by closures that only read captured variables:

```rust
fn call_concurrently<F>(f: F)
where
    F: Fn() + Clone + Send + 'static,
{
    // Can be called from multiple threads
    let f1 = f.clone();
    let f2 = f.clone();
    
    thread::spawn(move || f1());
    thread::spawn(move || f2());
}

let message = "Hello".to_string();
let print_message = move || println!("{}", message);
call_concurrently(print_message);
```

Characteristics:
- Can be called multiple times
- Only immutably borrows captured variables
- Can be called while other immutable borrows exist

## Capture Modes in Detail

### Mode 1: By Reference

```rust
let text = String::from("hello");
let print_text = || {
    println!("{}", text.len());  // Only reads
};
// text is still accessible, closure only borrowed it
println!("{}", text);
print_text();
```

### Mode 2: By Mutable Reference

```rust
let mut text = String::from("hello");
let mut append_world = || {
    text.push_str(" world");  // Modifies
};
// text is not accessible here - borrowed mutably
append_world();
append_world();
println!("{}", text);  // Now accessible
```

### Mode 3: By Value

```rust
let text = String::from("hello");
let consume_text = || {
    let _owned = text;  // Takes ownership
};
consume_text();
// text is no longer valid
```

## Forced Capture with `move`

Even if a closure doesn't need ownership, `move` forces it:

```rust
let x = 5;  // i32 is Copy
let closure = move || x;  // Copies x into closure

println!("{}", x);  // Still works - x was copied, not moved
let y = closure();
```

For `Copy` types, `move` creates a copy inside the closure.

## Practical Examples

### Example 1: Configuration Capture

```rust
struct Config {
    threshold: i32,
    prefix: String,
}

fn create_validator(config: Config) -> impl Fn(&str) -> bool + 'static {
    let threshold = config.threshold;
    let prefix = config.prefix;
    
    move |input: &str| {
        input.starts_with(&prefix) && input.len() >= threshold as usize
    }
}
```

### Example 2: Thread-Safe Message Handler

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let messages = Arc::new(Mutex::new(Vec::new()));
let messages_clone = Arc::clone(&messages);

let handler = move |msg: String| {
    let mut msgs = messages_clone.lock().unwrap();
    msgs.push(msg);
};

thread::spawn(move || {
    handler("Hello from thread".to_string());
});
```

### Example 3: Iterator with Captured State

```rust
let mut multiplier = 2;
let numbers = vec![1, 2, 3, 4, 5];

let multiplied: Vec<i32> = numbers
    .iter()
    .map(|&n| {
        let result = n * multiplier;
        multiplier += 1;  // Can't do this! map takes FnMut
        result
    })
    .collect();
```

## Textio Example: Message Filters

```rust
struct MessageFilter {
    blocked_numbers: Vec<String>,
    max_length: usize,
}

impl MessageFilter {
    fn create_filter(&self) -> impl Fn(&Message) -> bool + '_ {
        move |msg: &Message| {
            !self.blocked_numbers.contains(&msg.recipient) 
                && msg.content.len() <= self.max_length
        }
    }
}

// Using move for thread-safe filtering
fn spawn_filter_thread(blocked: Vec<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let is_blocked = |number: &str| blocked.contains(&number.to_string());
        // Process messages...
    })
}
```

## Common Pitfalls

### Pitfall 1: Borrow Conflicts

```rust
let mut v = vec![1, 2, 3];
let mut push_closure = || v.push(4);

println!("{:?}", v);  // Error: v is borrowed
push_closure();
```

### Pitfall 2: Dropped Values

```rust
fn broken_closure() -> impl Fn() {
    let x = String::from("hello");
    || println!("{}", x)  // Error: x is dropped
}

fn working_closure() -> impl Fn() {
    let x = String::from("hello");
    move || println!("{}", x)  // OK: x is moved into closure
}
```

### Pitfall 3: FnMut in map

```rust
let mut count = 0;
let v = vec![1, 2, 3];

// Error: map expects Fn, not FnMut
let _: Vec<i32> = v.iter().map(|_| {
    count += 1;
    count
}).collect();
```

## Exercise Overview

In this exercise, you will:
1. Use `move` to transfer ownership into closures
2. Understand when to use `Fn`, `FnMut`, and `FnOnce`
3. Fix borrow checker errors with closures
4. Create closures that work across threads
5. Implement Textio message handlers with proper capture modes

## Key Takeaways

- Closures capture by reference, mutable reference, or value based on usage
- `move` forces ownership transfer into closures
- `FnOnce`: may consume captures, callable once
- `FnMut`: mutably borrows captures, callable multiple times
- `Fn`: immutably borrows captures, callable multiple times concurrently
- Use `move` when the closure outlives the current scope
- `Copy` types are copied, not moved, with `move`

## Further Reading

- [Rust Book: Closures and Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits)
- [Fn Traits Documentation](https://doc.rust-lang.org/std/ops/trait.Fn.html)

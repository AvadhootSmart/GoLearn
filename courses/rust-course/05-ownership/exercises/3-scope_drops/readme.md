# Scope, Drops, and RAII: Automatic Resource Management

## The RAII Pattern

RAII (Resource Acquisition Is Initialization) is the foundation of Rust's memory safety.

```
┌──────────────────────────────────────────────────────────────┐
│                    RAII EXPLAINED                            │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  RAII = Resource Acquisition Is Initialization               │
│                                                              │
│  Core principle:                                             │
│  • When a value is created, it acquires resources            │
│  • When a value goes out of scope, it releases resources     │
│  • This happens AUTOMATICALLY and GUARANTEED                 │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                    SCOPE                            │    │
│  │  ┌───────────────────────────────────────────────┐ │    │
│  │  │  let s = String::from("hello");               │ │    │
│  │  │           │                                   │ │    │
│  │  │           ▼                                   │ │    │
│  │  │  ┌─────────────────────┐                     │ │    │
│  │  │  │ ACQUIRE RESOURCE    │                     │ │    │
│  │  │  │ - Allocate heap     │                     │ │    │
│  │  │  │ - Store pointer     │                     │ │    │
│  │  │  └─────────────────────┘                     │ │    │
│  │  │           │                                   │ │    │
│  │  │           ▼                                   │ │    │
│  │  │  // use s                                     │ │    │
│  │  │           │                                   │ │    │
│  │  │           ▼                                   │ │    │
│  │  │  ┌─────────────────────────────────────────┐ │ │    │
│  │  │  │ END OF SCOPE - DROP AUTOMATICALLY       │ │ │    │
│  │  │  │ - Free heap memory                      │ │ │    │
│  │  │  │ - Clean up resources                    │ │ │    │
│  │  │  └─────────────────────────────────────────┘ │ │    │
│  │  └───────────────────────────────────────────────┘ │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Scope in Rust

```
┌──────────────────────────────────────────────────────────────┐
│                    SCOPE BASICS                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Scope = the region where a binding is valid                 │
│                                                              │
│  fn main() {                         SCOPE DIAGRAM:         │
│      // s not valid here            ┌─────────────────────┐ │
│                                        │ main scope         │ │
│      let s = "hello";                │                     │ │
│      // s is valid here              │   ┌───────────────┐ │ │
│                                        │   │ s is valid   │ │ │
│      {                               │   └───────────────┘ │ │
│          let x = 5;                  │                     │ │
│          // s and x valid            │   ┌───────────────┐ │ │
│      }                               │   │ inner scope   │ │ │
│      // x not valid (dropped)        │   │ s & x valid   │ │ │
│      // s still valid                │   └───────────────┘ │ │
│  }                                   │                     │ │
│  // s dropped here                   └─────────────────────┘ │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## The Drop Trait

The Drop trait lets you customize what happens when a value goes out of scope.

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    DROP TRAIT                                │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Drop is called AUTOMATICALLY when:                          │
│  • A value goes out of scope                                 │
│  • A value is reassigned (old value dropped)                 │
│  • A vector is resized (excess elements dropped)             │
│  • A value is explicitly dropped with drop(value)            │
│                                                              │
│  Drop is NEVER called when:                                  │
│  • A value is moved to another owner                         │
│  • A value is a reference (references don't own)             │
│                                                              │
│  IMPORTANT: You never call .drop() yourself!                 │
│  Use std::mem::drop() if you need early cleanup.             │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Example: Custom Drop Implementation

```rust
struct SmsMessage {
    id: u32,
    body: String,
}

impl Drop for SmsMessage {
    fn drop(&mut self) {
        println!("Dropping message {}: '{}'", self.id, self.body);
    }
}

fn main() {
    println!("Creating message...");
    let msg = SmsMessage {
        id: 1,
        body: String::from("Hello"),
    };
    println!("Message created");
    // Drop happens automatically here!
}
```

Output:
```
Creating message...
Message created
Dropping message 1: 'Hello'
```

## Visualizing Drop Order

```
┌──────────────────────────────────────────────────────────────┐
│                    DROP ORDER                                │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  fn main() {                                                 │
│      let a = String::from("a");                             │
│      let b = String::from("b");                             │
│      let c = String::from("c");                             │
│  }                                                           │
│                                                              │
│  Stack:                                                      │
│  ┌────────────────────────────────────────┐                 │
│  │ c ──────► "c"                          │                 │
│  │ b ──────► "b"                          │                 │
│  │ a ──────► "a"                          │                 │
│  └────────────────────────────────────────┘                 │
│                                                              │
│  Drop order: REVERSE of creation                             │
│  1. c dropped first                                          │
│  2. b dropped second                                         │
│  3. a dropped last                                           │
│                                                              │
│  Like a stack: LIFO (Last In, First Out)                    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Early Drop with std::mem::drop

```rust
fn main() {
    let msg = SmsMessage {
        id: 1,
        body: String::from("Hello"),
    };
    
    println!("Before explicit drop");
    drop(msg);  // Force early drop
    println!("After explicit drop");
    
    // msg is no longer valid here!
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    EXPLICIT DROP                             │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Why use explicit drop?                                      │
│                                                              │
│  1. Release resources early (files, locks, connections)      │
│  2. Free memory before expensive operations                  │
│  3. End a scope's lifetime explicitly                        │
│                                                              │
│  std::mem::drop(x) is just:                                  │
│  pub fn drop<T>(_x: T) { }                                   │
│                                                              │
│  It takes ownership and does nothing -                       │
│  the value naturally drops at end of function!               │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## What Rust Prevents: Memory Leaks

```
┌──────────────────────────────────────────────────────────────┐
│              MEMORY LEAKS RUST PREVENTS                      │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  In C/C++, memory leaks are easy:                            │
│                                                              │
│  // C++ - MEMORY LEAK                                        │
│  void leak() {                                               │
│      int* ptr = new int(42);                                 │
│      // Oops, forgot delete!                                 │
│  }  // ptr goes out of scope, but heap memory leaked!        │
│                                                              │
│  In Rust, this is IMPOSSIBLE with safe code:                 │
│                                                              │
│  // Rust - NO LEAK POSSIBLE                                  │
│  fn no_leak() {                                              │
│      let v = vec![1, 2, 3];                                  │
│      // No need to remember cleanup!                         │
│  }  // v automatically drops, heap freed!                    │
│                                                              │
│  RAII guarantee: If you acquire it, you WILL release it.     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## What Rust Prevents: Double Free

```
┌──────────────────────────────────────────────────────────────┐
│              DOUBLE FREE RUST PREVENTS                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  In C++, double free crashes:                                │
│                                                              │
│  // C++ - CRASH                                              │
│  void crash() {                                              │
│      char* s1 = strdup("hello");                             │
│      char* s2 = s1;  // Both point to same memory            │
│      free(s1);                                               │
│      free(s2);  // CRASH! Double free!                       │
│  }                                                           │
│                                                              │
│  In Rust, this WON'T COMPILE:                                │
│                                                              │
│  // Rust - COMPILE ERROR                                     │
│  fn safe() {                                                 │
│      let s1 = String::from("hello");                         │
│      let s2 = s1;  // s1 moved                               │
│      drop(s1);  // ERROR: use of moved value!                │
│      drop(s2);                                               │
│  }                                                           │
│                                                              │
│  The compiler catches it BEFORE it crashes!                  │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## What Rust Prevents: Use After Free

```
┌──────────────────────────────────────────────────────────────┐
│              USE AFTER FREE RUST PREVENTS                    │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  In C++, use after free is undefined behavior:               │
│                                                              │
│  // C++ - UNDEFINED BEHAVIOR                                 │
│  void danger() {                                             │
│      int* ptr = new int(42);                                 │
│      delete ptr;                                             │
│      *ptr = 100;  // Writing to freed memory!                │
│      // Could corrupt heap, crash, or "work"                 │
│  }                                                           │
│                                                              │
│  In Rust, this WON'T COMPILE:                                │
│                                                              │
│  // Rust - COMPILE ERROR                                     │
│  fn safe() {                                                 │
│      let s = String::from("hello");                          │
│      drop(s);                                                │
│      println!("{}", s);  // ERROR: use after drop!           │
│  }                                                           │
│                                                              │
│  The compiler prevents use after free!                       │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Drop with Complex Types

### Vec<T> Drop

```rust
fn main() {
    let messages = vec![
        SmsMessage { id: 1, body: String::from("Hello") },
        SmsMessage { id: 2, body: String::from("World") },
    ];
    
    // When messages drops:
    // 1. Each SmsMessage's Drop is called (in reverse order)
    // 2. Each String inside drops
    // 3. Vec's heap memory drops
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    VEC DROP ORDER                            │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  let v = vec![msg1, msg2, msg3];                             │
│                                                              │
│  Stack:              Heap:                                   │
│  ┌──────────┐       ┌─────────────────────────┐             │
│  │ v        │       │ msg1 ──► "hello"        │             │
│  │ ptr ─────┼──────►│ msg2 ──► "world"        │             │
│  │ len = 3  │       │ msg3 ──► "foo"          │             │
│  │ cap = 3  │       └─────────────────────────┘             │
│  └──────────┘                                               │
│                                                              │
│  Drop order:                                                 │
│  1. msg3 drops                                               │
│  2. msg2 drops                                               │
│  3. msg1 drops                                               │
│  4. Vec's buffer is deallocated                              │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## RAII in Action: File Handle

```rust
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("log.txt")?;
        file.write_all(b"Hello, Textio!")?;
        // file automatically closed here!
    }  // File::drop() is called, closing the file handle
    
    println!("File is closed!");
    Ok(())
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    FILE HANDLE RAII                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  C++ approach (manual):                                      │
│  FILE* f = fopen("log.txt", "w");                           │
│  fprintf(f, "Hello");                                        │
│  fclose(f);  // Must remember this!                          │
│                                                              │
│  Rust approach (RAII):                                       │
│  let mut f = File::create("log.txt")?;                      │
│  f.write_all(b"Hello")?;                                    │
│  // Automatically closed when f drops!                       │
│                                                              │
│  Benefits:                                                   │
│  • No forgotten close()                                      │
│  • Works even on early return or panic                       │
│  • Guaranteed cleanup                                        │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Scope and Shadowing

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
    
    {
        let x = x + 1;  // Shadows outer x
        println!("Inner x = {}", x);  // 6
    }  // Inner x drops here
    
    println!("Outer x = {}", x);  // 5
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    SHADOWING AND SCOPE                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Shadowing creates a NEW binding:                            │
│                                                              │
│  let x = 5;          // x_1 created                         │
│  {                                                   │
│      let x = x + 1;  // x_2 created (different x!)          │
│      println!("{}", x);  // Uses x_2                        │
│  }                   // x_2 dropped                          │
│  println!("{}", x);  // Uses x_1                            │
│                      // x_1 dropped at end of main          │
│                                                              │
│  This is DIFFERENT from assignment:                          │
│  let mut x = 5;                                              │
│  x = x + 1;  // Same x, just modified                        │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Nested Scopes and Early Returns

```rust
fn process_message(id: u32) -> Result<String, &'static str> {
    let msg = create_message(id)?;
    
    if msg.is_empty() {
        return Err("Empty message");  // msg drops here!
    }
    
    let processed = msg.to_uppercase();
    
    Ok(processed)
}  // All values drop here on success path
```

```
┌──────────────────────────────────────────────────────────────┐
│                    EARLY RETURN DROPS                        │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  fn example() {                                              │
│      let a = String::from("a");                             │
│      let b = String::from("b");                             │
│                                                              │
│      if some_condition {                                    │
│          return;  // Both a and b drop here!                │
│      }                                                       │
│                                                              │
│      let c = String::from("c");                             │
│      // c drops at end                                       │
│  }  // a, b, c all drop here (normal path)                  │
│                                                              │
│  Rust tracks all exit paths and ensures cleanup!             │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Drop with Cycles (Rc/Arc)

```
┌──────────────────────────────────────────────────────────────┐
│                    CYCLE WARNING                             │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  RAII can leak memory with reference cycles:                 │
│                                                              │
│  use std::rc::Rc;                                            │
│  use std::cell::RefCell;                                     │
│                                                              │
│  struct Node {                                               │
│      next: RefCell<Option<Rc<Node>>>,                       │
│  }                                                           │
│                                                              │
│  let a = Rc::new(Node { next: RefCell::new(None) });        │
│  let b = Rc::new(Node { next: RefCell::new(None) });        │
│                                                              │
│  *a.next.borrow_mut() = Some(b.clone());                    │
│  *b.next.borrow_mut() = Some(a.clone());  // CYCLE!         │
│                                                              │
│  // Neither a nor b will ever drop!                          │
│                                                              │
│  Solution: Use Weak<T> for back-references                   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Textio Example: Connection Pool

```rust
struct Connection {
    id: u32,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("Connection {} closed", self.id);
    }
}

struct ConnectionPool {
    connections: Vec<Connection>,
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        println!("Dropping pool with {} connections", self.connections.len());
        // Each connection's Drop will be called automatically
    }
}

fn main() {
    println!("Creating pool...");
    {
        let pool = ConnectionPool {
            connections: vec![
                Connection { id: 1 },
                Connection { id: 2 },
                Connection { id: 3 },
            ],
        };
        println!("Pool created with {} connections", pool.connections.len());
    }  // Pool drops here
    
    println!("Pool dropped");
}
```

Output:
```
Creating pool...
Pool created with 3 connections
Dropping pool with 3 connections
Connection 3 closed
Connection 2 closed
Connection 1 closed
Pool dropped
```

## Exercises

In this exercise, you'll work with Textio's resource management:

1. Implement custom Drop for SMS messages
2. Observe drop order in various scenarios
3. Use explicit drop for early cleanup
4. Understand scope boundaries

Complete the tasks in `code.rs` to master RAII and Drop!

## Key Takeaways

```
┌──────────────────────────────────────────────────────────────┐
│                    REMEMBER                                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  1. RAII = resources tied to object lifetime                 │
│                                                              │
│  2. Drop is called automatically at scope end                │
│                                                              │
│  3. Drop order is reverse of creation (LIFO)                 │
│                                                              │
│  4. Use std::mem::drop() for early cleanup                   │
│                                                              │
│  5. Rust prevents: memory leaks, double free, use after free │
│                                                              │
│  6. Never call .drop() directly - use drop(value)            │
│                                                              │
│  7. Early returns still trigger proper cleanup               │
│                                                              │
│  8. Cycles with Rc can cause leaks (use Weak)                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

# Exercise 1: Box<T> - Heap Allocation and Recursive Types

## Overview

`Box<T>` is Rust's simplest smart pointer. It provides heap allocation with a fixed-size pointer, enabling recursive data types and efficient data storage when stack allocation isn't suitable.

## What is Box<T>?

A `Box<T>` is a smart pointer that:
- Allocates data on the heap rather than the stack
- Has a known, fixed size at compile time (size of a pointer)
- Provides ownership semantics like regular values
- Automatically deallocates when dropped

```rust
let boxed_value = Box::new(5);  // Integer stored on heap
println!("{}", boxed_value);     // Dereferences automatically
```

## Why Use Box<T>?

### 1. Recursive Types
When a type needs to contain itself, the size becomes infinite. Box breaks this cycle:

```rust
// This WON'T compile - infinite size
enum List {
    Cons(i32, List),  // List contains itself
    Nil,
}

// This WORKS - Box has known size (pointer size)
enum List {
    Cons(i32, Box<List>),  // Box<List> is just a pointer
    Nil,
}
```

### 2. Large Data Transfer
Moving large structures can be expensive. Boxing makes moves cheap (just pointer copy):

```rust
struct LargeConfig {
    data: [u8; 10000],
}

let config = Box::new(LargeConfig { data: [0; 10000] });
let moved = config;  // Only pointer is copied
```

### 3. Trait Objects
Box enables dynamic dispatch through trait objects:

```rust
trait MessageHandler {
    fn handle(&self, msg: &str);
}

struct SmsHandler;
struct EmailHandler;

impl MessageHandler for SmsHandler {
    fn handle(&self, msg: &str) { println!("SMS: {}", msg); }
}

impl MessageHandler for EmailHandler {
    fn handle(&self, msg: &str) { println!("Email: {}", msg); }
}

let handlers: Vec<Box<dyn MessageHandler>> = vec![
    Box::new(SmsHandler),
    Box::new(EmailHandler),
];
```

## The Deref Trait

Box implements `Deref`, allowing automatic dereferencing:

```rust
let x = Box::new(5);
let y = *x;  // Explicit dereference
let z = x + 1;  // Automatic dereference via Deref

struct Message {
    text: String,
}

let msg = Box::new(Message { text: String::from("Hello") });
println!("{}", msg.text);  // Deref coercion
```

## The Drop Trait

Box implements `Drop` for automatic cleanup:

```rust
{
    let boxed = Box::new(String::from("temporary"));
    // boxed goes out of scope
}  // Drop is called, memory freed
```

## Recursive Data Structures in Textio

### Message Thread Tree

```rust
enum MessageNode {
    Message {
        id: u64,
        content: String,
        replies: Vec<Box<MessageNode>>,
    },
    Deleted,
}

fn build_thread() -> Box<MessageNode> {
    Box::new(MessageNode::Message {
        id: 1,
        content: String::from("Welcome to Textio!"),
        replies: vec![
            Box::new(MessageNode::Message {
                id: 2,
                content: String::from("Thanks!"),
                replies: vec![],
            }),
        ],
    })
}
```

### Linked List for Message Queue

```rust
enum MessageQueue {
    Item {
        message: String,
        next: Box<MessageQueue>,
    },
    Empty,
}

impl MessageQueue {
    fn push(self, message: String) -> Box<MessageQueue> {
        Box::new(MessageQueue::Item {
            message,
            next: Box::new(self),
        })
    }

    fn pop(&mut self) -> Option<String> {
        match std::mem::replace(self, MessageQueue::Empty) {
            MessageQueue::Item { message, next } => {
                *self = *next;
                Some(message)
            }
            MessageQueue::Empty => None,
        }
    }
}
```

### Binary Tree for Contact Search

```rust
struct ContactNode {
    name: String,
    phone: String,
    left: Option<Box<ContactNode>>,
    right: Option<Box<ContactNode>>,
}

impl ContactNode {
    fn new(name: String, phone: String) -> Box<Self> {
        Box::new(ContactNode {
            name,
            phone,
            left: None,
            right: None,
        })
    }

    fn insert(&mut self, name: String, phone: String) {
        if name < self.name {
            match &mut self.left {
                Some(node) => node.insert(name, phone),
                None => self.left = Some(ContactNode::new(name, phone)),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(name, phone),
                None => self.right = Some(ContactNode::new(name, phone)),
            }
        }
    }

    fn find(&self, name: &str) -> Option<&String> {
        match name.cmp(&self.name) {
            std::cmp::Ordering::Less => self.left.as_ref()?.find(name),
            std::cmp::Ordering::Equal => Some(&self.phone),
            std::cmp::Ordering::Greater => self.right.as_ref()?.find(name),
        }
    }
}
```

## Memory Layout

```
Stack:                    Heap:
+--------+               +------------------+
| Box<T> | ------------> |      Data        |
| ptr    |               | (actual value)   |
+--------+               +------------------+
   8 bytes                  size_of::<T>()
```

## Box vs Stack Allocation

```rust
// Stack allocation
let stack_value: i32 = 42;
let stack_array: [i32; 1000] = [0; 1000];

// Heap allocation
let heap_value: Box<i32> = Box::new(42);
let heap_array: Box<[i32; 1000]> = Box::new([0; 1000]);
```

## Common Patterns

### Recursive Enum with Multiple Variants

```rust
enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Negate(Box<Expression>),
}

fn evaluate(expr: &Expression) -> i64 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Add(a, b) => evaluate(a) + evaluate(b),
        Expression::Multiply(a, b) => evaluate(a) * evaluate(b),
        Expression::Negate(e) => -evaluate(e),
    }
}
```

### Builder Pattern with Box

```rust
struct MessageBuilder {
    content: String,
    metadata: Option<Box<Metadata>>,
}

struct Metadata {
    priority: u8,
    encoding: String,
}

impl MessageBuilder {
    fn new(content: &str) -> Self {
        MessageBuilder {
            content: content.to_string(),
            metadata: None,
        }
    }

    fn with_metadata(mut self, priority: u8, encoding: &str) -> Self {
        self.metadata = Some(Box::new(Metadata {
            priority,
            encoding: encoding.to_string(),
        }));
        self
    }

    fn build(self) -> Message {
        Message {
            content: self.content,
            metadata: self.metadata,
        }
    }
}

struct Message {
    content: String,
    metadata: Option<Box<Metadata>>,
}
```

## When to Use Box

| Use Case | Recommended |
|----------|-------------|
| Recursive types | Yes - essential |
| Large data transfers | Yes - cheap moves |
| Trait objects | Yes - dyn Trait |
| Simple heap allocation | Consider - often Rc/Arc better |
| Shared ownership | No - use Rc/Arc |
| Interior mutability | No - use RefCell |

## Best Practices

1. **Use Box for recursive types** - It's the idiomatic solution
2. **Don't Box small values** - Stack is faster for small data
3. **Consider layout** - Box adds indirection, may hurt cache
4. **Document ownership** - Box implies unique ownership
5. **Use with traits** - Box<dyn Trait> for dynamic dispatch

## Exercise Task

In this exercise, you will implement:
1. A recursive message thread structure
2. A binary search tree for contacts
3. A simple expression evaluator
4. Trait objects for message handlers

Focus on understanding when Box is necessary and how it enables recursive data structures.

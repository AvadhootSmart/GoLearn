# Exercise 3: Borrowing Rules

## Learning Objectives

By the end of this exercise, you will understand:
- The complete borrowing rules and their rationale
- How the "one mutable OR many immutable" rule works
- Common borrow checker errors and how to fix them
- How to restructure code to satisfy the borrow checker
- Why these rules prevent data races at compile time
- Non-Lexical Lifetimes (NLL) and reference lifetimes

---

## The Borrowing Rules: A Complete Picture

Rust's borrowing rules can be summarized in two key principles:

### Rule 1: One Mutable OR Many Immutable

At any given time, you can have **either**:
- Exactly one mutable reference (`&mut T`)
- Any number of immutable references (`&T`)

But **never both at the same time**.

```
┌─────────────────────────────────────────────────────┐
│                  BORROWING RULES                     │
├─────────────────────────────────────────────────────┤
│                                                      │
│   Multiple &T    ✓    ✗    ✗                        │
│   Single &mut T  ✗    ✓    ✗                        │
│   Both at once   ✗    ✗    ✗                        │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### Rule 2: References Must Always Be Valid

References must never outlive the data they refer to.

---

## Why These Rules Exist: Preventing Data Races

### What Is a Data Race?

A data race occurs when:
1. Two or more threads access the same memory concurrently
2. At least one access is a write
3. There's no synchronization

Data races cause undefined behavior - bugs that are:
- Hard to reproduce
- Hard to debug
- Potentially catastrophic

### How Rust Prevents Data Races

```rust
// This would be a data race in other languages
fn problematic_code() {
    let mut data = vec![1, 2, 3];
    
    let read_ref = &data[0];     // Immutable reference
    let write_ref = &mut data;   // Mutable reference (NOT ALLOWED!)
    
    write_ref.push(4);           // Might reallocate the vector
    
    // read_ref might now point to freed memory!
    println!("{}", read_ref);
}
```

Rust prevents this at compile time:

```
error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:5
  |
4 |     let read_ref = &data[0];
  |                    ---- immutable borrow occurs here
5 |     let write_ref = &mut data;
  |                     ^^^^^^^^^ mutable borrow occurs here
6 |     
7 |     write_ref.push(4);
8 |     println!("{}", read_ref);
  |                    -------- immutable borrow later used here
```

---

## Understanding Reference Lifetimes

### When Does a Borrow End?

With **Non-Lexical Lifetimes (NLL)**, introduced in Rust 2018, a borrow ends when it's **last used**, not when it goes out of scope.

### Example: NLL in Action

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;
    println!("{}", r1);  // r1 is last used here
    
    // r1's borrow effectively ends above
    // So this is allowed:
    let r2 = &mut s;
    r2.push_str(", World!");
    
    println!("{}", s);
}
```

### Without NLL (Old Behavior)

Before NLL, the above would error because `r1` would live until the end of its scope:

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &s;         // ------------------+-- borrow starts
    println!("{}", r1);  //                   |
                         //                   |
    let r2 = &mut s;     // ERROR!            |  borrow still active
    r2.push_str("...");  //                   |
                         //                   |
}                        // <-----------------+-- borrow ends (scope end)
```

---

## Common Borrow Checker Errors

### Error 1: Immutable Borrow Followed by Mutable

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    
    let first = &data[0];  // Immutable borrow
    
    data.push(4);  // Error: mutable borrow while immutable exists
    
    println!("{}", first);
}
```

**Error Message:**
```
error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
```

**Fix 1: Clone the Value**

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    
    let first = data[0];  // Copy the value (i32 is Copy)
    
    data.push(4);  // Now OK
    
    println!("{}", first);
}
```

**Fix 2: Reorder Operations**

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    
    let first = &data[0];
    println!("{}", first);  // Use the reference first
    
    data.push(4);  // Now OK - first is no longer used
}
```

### Error 2: Multiple Mutable Borrows

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &mut s;
    let r2 = &mut s;  // Error!
    
    println!("{} {}", r1, r2);
}
```

**Error Message:**
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

**Fix 1: Use Scopes**

```rust
fn main() {
    let mut s = String::from("Hello");
    
    {
        let r1 = &mut s;
        r1.push_str(" there");
    }  // r1 ends here
    
    {
        let r2 = &mut s;
        r2.push('!');
    }  // r2 ends here
    
    println!("{}", s);
}
```

**Fix 2: Sequential Access with NLL**

```rust
fn main() {
    let mut s = String::from("Hello");
    
    let r1 = &mut s;
    r1.push_str(" there");
    // r1 is no longer used after this point
    
    let r2 = &mut s;  // OK with NLL
    r2.push('!');
    
    println!("{}", s);
}
```

### Error 3: Borrow Across Function Calls

```rust
fn main() {
    let mut messages = vec![
        String::from("Hello"),
        String::from("World"),
    ];
    
    let first = &messages[0];
    
    messages.push(String::from("New"));  // Error!
    
    println!("{}", first);
}
```

**Fix: Reorder**

```rust
fn main() {
    let mut messages = vec![
        String::from("Hello"),
        String::from("World"),
    ];
    
    let first = &messages[0];
    println!("{}", first);  // Use first before modifying
    
    messages.push(String::from("New"));  // Now OK
}
```

---

## The Interior Mutability Pattern

Sometimes you need to mutate data even when there are immutable references to it. Rust provides **interior mutability** patterns for these cases.

### `RefCell<T>` for Runtime Checking

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Borrow immutably
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("{} {}", r1, r2);
    
    // Borrow mutably (after immutable borrows are done)
    let mut r3 = data.borrow_mut();
    *r3 += 1;
    
    println!("{}", data.borrow());
}
```

Note: `RefCell` moves the borrow checking from compile time to runtime. If you violate the rules at runtime, the program will panic.

---

## Working With the Borrow Checker

### Strategy 1: Extract Values Early

```rust
fn main() {
    let mut messages = vec![
        String::from("Hello"),
        String::from("World"),
    ];
    
    // Instead of keeping a reference...
    // let first = &messages[0];
    
    // Extract the value
    let first = messages[0].clone();
    
    // Now we can modify
    messages.push(String::from("New"));
    
    println!("First: {}", first);
}
```

### Strategy 2: Restructure Your Code

```rust
fn process_data(data: &mut Vec<i32>) {
    // Bad: Trying to borrow immutably and mutably
    // let first = &data[0];
    // data.push(4);
    // println!("{}", first);
    
    // Good: Do reads first, then writes
    let first = data[0];
    println!("First: {}", first);
    
    data.push(4);
}
```

### Strategy 3: Use Indices Instead of References

```rust
fn main() {
    let mut messages = vec![
        String::from("Hello"),
        String::from("World"),
    ];
    
    let index = 0;  // Store index, not reference
    
    messages.push(String::from("New"));
    
    println!("{}", messages[index]);
}
```

---

## Textio Example: Message Queue

```rust
struct MessageQueue {
    messages: Vec<Message>,
}

struct Message {
    id: u64,
    body: String,
    priority: u8,
}

impl MessageQueue {
    fn new() -> Self {
        MessageQueue {
            messages: Vec::new(),
        }
    }
    
    fn add_message(&mut self, body: &str, priority: u8) -> u64 {
        let id = self.messages.len() as u64 + 1;
        self.messages.push(Message {
            id,
            body: body.to_string(),
            priority,
        });
        id
    }
    
    fn get_message(&self, id: u64) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }
    
    fn get_highest_priority(&self) -> Option<&Message> {
        self.messages.iter().max_by_key(|m| m.priority)
    }
    
    fn remove_message(&mut self, id: u64) -> bool {
        if let Some(pos) = self.messages.iter().position(|m| m.id == id) {
            self.messages.remove(pos);
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut queue = MessageQueue::new();
    
    queue.add_message("Hello", 1);
    queue.add_message("Urgent: Verify now!", 10);
    queue.add_message("Reminder", 5);
    
    // Get highest priority message (immutable borrow)
    if let Some(msg) = queue.get_highest_priority() {
        println!("Highest priority: {}", msg.body);
    }
    // Immutable borrow ends here
    
    // Now we can modify
    queue.remove_message(2);
    
    // Check remaining messages
    println!("Remaining messages: {}", queue.messages.len());
}
```

---

## Advanced: Splitting Borrows

Sometimes you need multiple mutable references to different parts of a data structure.

### Splitting a Slice

```rust
fn main() {
    let mut data = [1, 2, 3, 4, 5];
    
    // Split into two mutable slices
    let (left, right) = data.split_at_mut(2);
    
    // Now we can modify both independently
    left[0] = 100;
    right[0] = 200;
    
    println!("{:?}", data);  // [100, 2, 200, 4, 5]
}
```

### Splitting a Vector

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = &mut v[0];  // Borrow first element
    let last = &mut v[4];   // Borrow last element
    
    // These don't overlap, so it's OK
    *first = 100;
    *last = 500;
    
    println!("{:?}", v);
}
```

Note: The borrow checker is smart enough to see that these borrows don't overlap.

---

## Summary: The Borrow Checker Is Your Friend

### What the Borrow Checker Does

1. **Prevents data races** at compile time
2. **Ensures memory safety** without garbage collection
3. **Catches bugs** before they reach production

### How to Work With It

1. **Think about ownership**: Who owns this data? Who's reading? Who's writing?
2. **Order your operations**: Do reads before writes
3. **Use scopes**: Limit the lifetime of references
4. **Clone when necessary**: Sometimes copying is the right answer
5. **Use indices**: Store positions instead of references

### Rules Summary Table

| Scenario | Allowed? | Reason |
|----------|----------|--------|
| Multiple `&T` | ✓ | No mutation, safe to share |
| One `&mut T` | ✓ | Mutation without sharing |
| Multiple `&mut T` | ✗ | Would cause data races |
| `&T` + `&mut T` | ✗ | Reader might see invalid data |

---

## Next Steps

In the next exercise, you'll learn about **dangling references** and why Rust's design makes them impossible in safe code.

# Move Semantics: The Heart of Ownership

## The Problem Ownership Solves

Before Rust, you had two choices for memory management:

```
┌─────────────────────────────────────────────────────────────────┐
│                    MEMORY MANAGEMENT OPTIONS                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   C/C++ (Manual)          Rust (Ownership)        Java/Go/JS    │
│   ┌──────────┐           ┌──────────────┐       ┌──────────┐   │
│   │ malloc() │           │  Ownership   │       │   GC     │   │
│   │ free()   │           │  + Borrowing │       │ Runtime  │   │
│   └──────────┘           └──────────────┘       └──────────┘   │
│       │                        │                      │         │
│       ▼                        ▼                      ▼         │
│   • Fast                   • Fast                 • Slower     │
│   • Unsafe                 • Safe                 • Safe       │
│   • Bugs!                  • No GC pause          • GC pauses  │
│   • Memory leaks           • Compile-time         • Runtime    │
│   • Dangling pointers      • Zero-cost            • Overhead   │
│   • Double free            • Predictable                         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Why Does Rust Need Ownership?

### The Double-Free Problem

In C++, this code compiles but crashes:

```cpp
// C++ - DANGEROUS!
#include <string>
void main() {
    std::string s1 = "hello";
    std::string s2 = s1;  // Shallow copy - both point to same data
    
    // When s1 and s2 go out of scope, BOTH try to free the same memory!
    // CRASH: Double free corruption
}
```

Visual representation of the problem:

```
┌──────────────────────────────────────────────────────────────┐
│                    C++ DOUBLE FREE BUG                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  After shallow copy:        At scope end:                    │
│                                                              │
│  Stack:                     Both destructors run:            │
│  ┌──────┐                   ┌──────┐                         │
│  │  s1 ─┼──┐                │  s1 ─┼──┐ free() ──┐           │
│  └──────┘  │                └──────┘  │          │           │
│  ┌──────┐  │                ┌──────┐  │          ▼           │
│  │  s2 ─┼──┼──┐             │  s2 ─┼──┼──┐   ┌────────┐     │
│  └──────┘  │  │             └──────┘  │  │   │ CRASH! │     │
│            │  │                       │  │   │ Double │     │
│            ▼  ▼                       ▼  ▼   │  Free! │     │
│  Heap:   ┌───────┐                 ┌───────┐ └────────┘     │
│          │"hello"│                 │ ???   │                │
│          └───────┘                 └───────┘                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Rust's Solution: Move Semantics

```rust
// Rust - SAFE!
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2, s1 is no longer valid
    
    // println!("{}", s1);  // ERROR! Won't compile
    println!("{}", s2);     // OK!
    
    // Only s2 will free the memory - exactly once!
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    RUST MOVE SEMANTICS                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  After move:                At scope end:                    │
│                                                              │
│  Stack:                     Only s2's destructor runs:       │
│  ┌──────┐                                                   │
│  │  s1 ─┼─── INVALID        ┌──────┐                        │
│  └──────┘   (moved)         │  s1 ─┼─── (invalidated)       │
│  ┌──────┐                   └──────┘                        │
│  │  s2 ─┼──┐                ┌──────┐        ┌────────┐      │
│  └──────┘  │                │  s2 ─┼────────│ free() │      │
│            │                └──────┘        └────────┘      │
│            ▼                                   │            │
│  Heap:   ┌───────┐                             ▼            │
│          │"hello"│                         ┌───────┐        │
│          └───────┘                         │ Clean │        │
│                                            └───────┘        │
└──────────────────────────────────────────────────────────────┘
```

## Stack vs Heap Memory

Understanding ownership requires understanding stack vs heap:

```
┌──────────────────────────────────────────────────────────────┐
│                      STACK vs HEAP                           │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  STACK                           HEAP                        │
│  ┌─────────────────────┐        ┌─────────────────────┐     │
│  │ • Fixed size        │        │ • Dynamic size      │     │
│  │ • Fast access       │        │ • Slower access     │     │
│  │ • Automatic cleanup │        │ • Manual/automatic  │     │
│  │ • Compile-time size │        │   cleanup           │     │
│  │ • LIFO order        │        │ • Runtime allocated │     │
│  └─────────────────────┘        └────────────────────┘      │
│                                                              │
│  Example:                       Example:                    │
│  • i32, f64, bool               • String                    │
│  • Arrays [i32; 5]              • Vec<T>                    │
│  • &str (reference)             • Box<T>                    │
│  • Structs with Copy            • HashMap                   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Stack-Only Types (Copy Types)

```
┌──────────────────────────────────────────────────────────────┐
│                    STACK-ONLY VALUES                         │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  let x = 5;                                                  │
│  let y = x;  // x is COPIED, both are valid                  │
│                                                              │
│  Stack memory:                                               │
│                                                              │
│  ┌─────────────────────┐    ┌─────────────────────┐         │
│  │ Before:             │    │ After:              │         │
│  │                     │    │                     │         │
│  │  ┌─────┐            │    │  ┌─────┐ ┌─────┐   │         │
│  │  │ x=5 │            │    │  │ x=5 │ │ y=5 │   │         │
│  │  └─────┘            │    │  └─────┘ └─────┘   │         │
│  │                     │    │                     │         │
│  └─────────────────────┘    └─────────────────────┘         │
│                                                              │
│  Why copy? Because:                                          │
│  • Fixed size known at compile time                          │
│  • Very cheap to copy (just a few bytes)                     │
│  • No heap allocation involved                               │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Heap-Allocated Types (Move Types)

```
┌──────────────────────────────────────────────────────────────┐
│                    HEAP-ALLOCATED VALUES                     │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  let s1 = String::from("hello");                             │
│  let s2 = s1;  // s1 is MOVED, only s2 is valid              │
│                                                              │
│  Memory layout:                                              │
│                                                              │
│      STACK                   HEAP                            │
│  ┌──────────────┐       ┌─────────────────┐                 │
│  │ s1 (invalid) │       │                 │                 │
│  │ ┌──────────┐ │       │    ┌───────┐    │                 │
│  │ │ ptr ──────┼───────┼───►│ hello │    │                 │
│  │ │ len = 5   │ │       │    └───────┘    │                 │
│  │ │ cap = 5   │ │       │                 │                 │
│  │ └──────────┘ │       └─────────────────┘                 │
│  └──────────────┘                ▲                          │
│  ┌──────────────┐                │                          │
│  │ s2 (valid)   │                │                          │
│  │ ┌──────────┐ │       Ownership transferred!              │
│  │ │ ptr ──────┼───────┘                                      │
│  │ │ len = 5   │                                              │
│  │ │ cap = 5   │                                              │
│  │ └──────────┘ │                                              │
│  └──────────────┘                                              │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## The String Type Deep Dive

String is the quintessential owned type:

```
┌──────────────────────────────────────────────────────────────┐
│                    String ANATOMY                            │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  let s = String::from("hello");                              │
│                                                              │
│  String is a "smart pointer" that owns heap data:            │
│                                                              │
│  Stack (24 bytes):            Heap:                          │
│  ┌────────────────┐          ┌───────────────────┐          │
│  │ ptr: *const u8 ──────────►│ h e l l o \0      │          │
│  ├────────────────┤          └───────────────────┘          │
│  │ length: 5      │                 ▲                        │
│  ├────────────────┤                 │                        │
│  │ capacity: 5    │          Index: 0 1 2 3 4                │
│  └────────────────┘                                          │
│                                                              │
│  Three fields:                                               │
│  • ptr - pointer to heap data                                │
│  • length - current number of bytes                          │
│  • capacity - total allocated space                          │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### String vs &str

```
┌──────────────────────────────────────────────────────────────┐
│                    String vs &str                            │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  String (Owned):                                             │
│  • Heap allocated                                            │
│  • Can grow/shrink                                           │
│  • Owns its data                                             │
│  • Mutable                                                   │
│                                                              │
│  let mut s: String = String::from("hello");                  │
│  s.push_str(" world");  // OK - we own it                    │
│                                                              │
│  ─────────────────────────────────────────────────────────   │
│                                                              │
│  &str (Borrowed string slice):                               │
│  • Can point to stack OR heap                                │
│  • Cannot grow                                               │
│  • Borrows data (doesn't own)                                │
│  • Immutable view                                            │
│                                                              │
│  let s: &str = "hello";        // Points to static memory    │
│  let s2: &str = &String::from("hello");  // Points to heap   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Ownership Rules

```
┌──────────────────────────────────────────────────────────────┐
│                    OWNERSHIP RULES                           │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  RULE 1: Each value has ONE owner                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ let s = String::from("hello");  // s owns "hello"    │   │
│  │ // "hello" has exactly one owner: s                  │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  RULE 2: When owner goes out of scope, value is dropped      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ {                                                    │   │
│  │     let s = String::from("hello");  // s created     │   │
│  │ }  // s goes out of scope, "hello" is dropped        │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  RULE 3: Ownership can be transferred (moved)                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ let s1 = String::from("hello");                      │   │
│  │ let s2 = s1;  // s1 moved to s2                      │   │
│  │ // s1 is now INVALID                                 │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  RULE 4: Some types are copied instead of moved              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ let x = 5;                                           │   │
│  │ let y = x;  // x is copied, both valid               │   │
│  │ // x is STILL VALID (Copy trait)                     │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Move Semantics in Action

### Example 1: Basic Move

```rust
fn main() {
    let message = String::from("Hello, Textio!");
    
    // Transfer ownership
    let transferred = message;
    
    // ERROR: message was moved
    // println!("{}", message);
    
    // OK: transferred now owns the data
    println!("{}", transferred);
}
```

### Example 2: Move with Reassignment

```rust
fn main() {
    let mut message = String::from("Hello");
    
    // Original message is dropped when reassigned
    message = String::from("World");
    
    // Old "Hello" is automatically freed!
    println!("{}", message);  // "World"
}
```

## Why Not Just Copy Everything?

```
┌──────────────────────────────────────────────────────────────┐
│                WHY MOVE INSTEAD OF COPY?                     │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Performance comparison for large data:                      │
│                                                              │
│  MOVE (transfer pointer):                                    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ let s1 = String::from("x".repeat(1_000_000));       │    │
│  │ let s2 = s1;  // O(1) - just copy 24 bytes          │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  COPY (if it existed):                                       │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ let s1 = String::from("x".repeat(1_000_000));       │    │
│  │ let s2 = s1.clone();  // O(n) - copy 1MB of data!   │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Move is ALWAYS O(1), regardless of data size!               │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## The Clone Trait

When you DO need to copy heap data:

```rust
fn main() {
    let original = String::from("Hello, Textio!");
    
    // Explicit deep copy using clone
    let clone = original.clone();
    
    // Both are valid - we have TWO copies of the data
    println!("Original: {}", original);
    println!("Clone: {}", clone);
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                    CLONE vs MOVE                             │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  MOVE:                       CLONE:                          │
│                                                              │
│  let s1 = String...          let s1 = String...              │
│  let s2 = s1;                let s2 = s1.clone();            │
│                                                              │
│  Stack:        Heap:         Stack:           Heap:          │
│  ┌────┐      ┌───────┐      ┌────┐         ┌───────┐        │
│  │s1 ╳│      │       │      │s1 ─┼──►      │ data  │        │
│  └────┘      │ data  │      └────┘         └───────┘        │
│  ┌────┐      │       │      ┌────┐         ┌───────┐        │
│  │s2 ─┼─────►│       │      │s2 ─┼──►      │ data  │        │
│  └────┘      └───────┘      └────┘         └───────┘        │
│                                                              │
│  Cheap (O(1))               Expensive (O(n))                 │
│  s1 invalid                 Both valid                       │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Textio Example: SMS Message Ownership

```rust
struct SmsMessage {
    to: String,      // Owned string
    from: String,    // Owned string
    body: String,    // Owned string
}

fn main() {
    let message = SmsMessage {
        to: String::from("+1234567890"),
        from: String::from("+0987654321"),
        body: String::from("Hello from Textio!"),
    };
    
    // Move the entire struct
    let sent_message = message;
    
    // ERROR: message was moved
    // println!("{:?}", message.to);
    
    // OK: sent_message owns all the data now
    println!("Sent to: {}", sent_message.to);
}
```

## Common Mistakes

### Mistake 1: Using After Move

```rust
fn main() {
    let s = String::from("hello");
    let s2 = s;
    
    // ERROR: value borrowed here after move
    // println!("{}", s);
}
```

### Mistake 2: Partial Move

```rust
fn main() {
    let data = (String::from("hello"), 42);
    
    // Partial move: only the String is moved
    let text = data.0;
    
    // ERROR: data.0 was moved
    // println!("{}", data.0);
    
    // OK: data.1 (i32) is Copy, still valid
    println!("{}", data.1);
}
```

## Exercises

In this exercise, you'll work with Textio's SMS message system to understand:

1. How ownership transfers between variables
2. When values are moved vs copied
3. How to use clone for explicit copies
4. Why the borrow checker prevents use-after-move

Complete the tasks in `code.rs` to see move semantics in action!

## Key Takeaways

```
┌──────────────────────────────────────────────────────────────┐
│                    REMEMBER                                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Ownership = single responsibility for cleanup            │
│                                                              │
│  2. Move = transfer ownership (O(1), cheap)                  │
│                                                              │
│  3. Clone = explicit deep copy (O(n), expensive)             │
│                                                              │
│  4. Copy types = implicitly copied (stack-only types)        │
│                                                              │
│  5. Use after move = compile error (not runtime crash!)      │
│                                                              │
│  6. String = owned, &str = borrowed                          │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

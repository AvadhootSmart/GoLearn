# The Copy Trait: Automatic Duplication

## What is the Copy Trait?

The Copy trait tells Rust: "This type is safe to automatically copy byte-by-byte."

```
┌──────────────────────────────────────────────────────────────┐
│                    COPY TRAIT ESSENCE                        │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  When a type implements Copy:                                │
│  • Assignment creates a COPY, not a move                     │
│  • Passing to functions creates a COPY                       │
│  • Returning from functions creates a COPY                   │
│  • Original value remains VALID after assignment             │
│                                                              │
│  Copy types are:                                             │
│  • Stored entirely on the stack                              │
│  • No heap allocation                                        │
│  • No owned resources (files, sockets, etc.)                 │
│  • Cheap to duplicate (just copy bytes)                      │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Stack vs Heap: The Key Distinction

```
┌──────────────────────────────────────────────────────────────┐
│                STACK vs HEAP VISUALIZATION                   │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  COPY TYPE (Stack only):                                     │
│                                                              │
│  let x: i32 = 42;                                            │
│  let y = x;  // Copy - both valid                            │
│                                                              │
│  ┌─────────────────────────────────────────┐                │
│  │              STACK                       │                │
│  │  ┌─────┐ ┌─────┐                        │                │
│  │  │x=42 │ │y=42 │  ← Two independent     │                │
│  │  └─────┘ └─────┘    copies on stack     │                │
│  └─────────────────────────────────────────┘                │
│                                                              │
│  NO HEAP ALLOCATION - Everything on stack                    │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  NON-COPY TYPE (Heap allocated):                             │
│                                                              │
│  let s1 = String::from("hello");                             │
│  let s2 = s1;  // Move - s1 invalid                          │
│                                                              │
│  ┌─────────────────────────────────────────┐                │
│  │              STACK                       │                │
│  │  ┌─────────┐ ┌─────────┐                │                │
│  │  │s1 ╳     │ │s2 ──────┼──┐             │                │
│  │  └─────────┘ └─────────┘  │             │                │
│  └─────────────────────────────────────────┘                │
│                               │                              │
│  ┌─────────────────────────────────────────┐                │
│  │              HEAP                        │                │
│  │  ┌───────────────────────┐              │                │
│  │  │   h e l l o           │◄─────────────┘                │
│  │  └───────────────────────┘                               │
│  └─────────────────────────────────────────┘                │
│                                                              │
│  HEAP DATA - Cannot just copy bytes, must transfer ownership │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Which Types Are Copy?

```
┌──────────────────────────────────────────────────────────────┐
│                    COPY TYPES REFERENCE                      │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ✅ ALL INTEGER TYPES                                        │
│     i8, i16, i32, i64, i128, isize                          │
│     u8, u16, u32, u64, u128, usize                          │
│                                                              │
│  ✅ FLOATING POINT TYPES                                     │
│     f32, f64                                                 │
│                                                              │
│  ✅ BOOLEAN TYPE                                             │
│     bool                                                     │
│                                                              │
│  ✅ CHARACTER TYPE                                           │
│     char (Unicode scalar value, 4 bytes)                     │
│                                                              │
│  ✅ TUPLE TYPES (if all elements are Copy)                   │
│     (i32, i32) ✅                                            │
│     (i32, String) ❌                                         │
│     (i32, f64, bool) ✅                                      │
│                                                              │
│  ✅ ARRAY TYPES (if element type is Copy)                    │
│     [i32; 5] ✅                                              │
│     [String; 5] ❌                                           │
│     [[u8; 3]; 4] ✅                                          │
│                                                              │
│  ✅ SHARED REFERENCES (always Copy)                          │
│     &T (for any T)                                           │
│     &mut T is NOT Copy (but &T is)                           │
│                                                              │
│  ✅ FUNCTION POINTERS                                        │
│     fn() -> i32                                              │
│                                                              │
│  ✅ NEVER TYPE                                               │
│     !                                                        │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Types That Are NOT Copy

```
┌──────────────────────────────────────────────────────────────┐
│                  NON-COPY TYPES                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ❌ OWNED HEAP TYPES                                         │
│     String, Vec<T>, Box<T>, HashMap<K,V>, etc.               │
│                                                              │
│  ❌ TYPES WITH RESOURCES                                     │
│     File, TcpStream, etc.                                    │
│                                                              │
│  ❌ TYPES WITH OWNERSHIP SEMANTICS                           │
│     Box<T>, Rc<T>, Arc<T>                                    │
│                                                              │
│  ❌ MUTABLE REFERENCES                                       │
│     &mut T                                                   │
│                                                              │
│  ❌ TUPLES WITH NON-COPY ELEMENTS                            │
│     (String, i32)                                            │
│     (Vec<u8>, bool)                                          │
│                                                              │
│  ❌ STRUCTS WITH NON-COPY FIELDS                             │
│     struct User { name: String }                             │
│                                                              │
│  ❌ ENUMS WITH NON-COPY VARIANTS                             │
│     enum Result<T, E> when T or E is not Copy                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## The Copy Trait Definition

```rust
// Simplified view of the Copy trait
pub trait Copy: Clone { }
```

Key insight: Copy is a "marker trait" with no methods!

```
┌──────────────────────────────────────────────────────────────┐
│                    COPY TRAIT RULES                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  To implement Copy, a type MUST:                             │
│                                                              │
│  1. Implement Clone (Copy requires Clone)                    │
│     - Clone::clone() does the actual copying                 │
│     - Copy just says "call clone implicitly"                 │
│                                                              │
│  2. Be "trivially copyable"                                  │
│     - All bits can be copied directly                        │
│     - No custom destructor (no Drop trait)                   │
│     - All fields must also be Copy                           │
│                                                              │
│  3. Not manage any resources                                 │
│     - No heap allocation                                     │
│     - No file handles, sockets, etc.                         │
│                                                              │
│  DERIVE MACRO:                                               │
│  #[derive(Copy, Clone)]                                      │
│  struct Point { x: i32, y: i32 }                             │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Copy in Action

### Example 1: Primitive Types

```rust
fn main() {
    let x: i32 = 42;
    let y = x;  // Copy happens here
    
    // Both are valid
    println!("x = {}, y = {}", x, y);
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                  PRIMITIVE COPY EXAMPLE                      │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  let x: i32 = 42;                                            │
│                                                              │
│  Stack:                                                      │
│  ┌─────────┐                                                │
│  │  x = 42 │                                                │
│  └─────────┘                                                │
│                                                              │
│  let y = x;  // Copy                                         │
│                                                              │
│  Stack:                                                      │
│  ┌─────────┐ ┌─────────┐                                    │
│  │  x = 42 │ │  y = 42 │                                    │
│  └─────────┘ └─────────┘                                    │
│                                                              │
│  Both valid!                                                 │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Example 2: Arrays of Copy Types

```rust
fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;  // Copy the entire array
    
    println!("arr1: {:?}", arr1);  // Still valid!
    println!("arr2: {:?}", arr2);
}
```

```
┌──────────────────────────────────────────────────────────────┐
│                  ARRAY COPY EXAMPLE                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  let arr1 = [1, 2, 3, 4, 5];                                 │
│                                                              │
│  Stack (20 bytes for 5 i32s):                               │
│  ┌─────────────────────────┐                                │
│  │ arr1: [1, 2, 3, 4, 5]   │                                │
│  └─────────────────────────┘                                │
│                                                              │
│  let arr2 = arr1;  // Copy all 20 bytes                      │
│                                                              │
│  Stack:                                                      │
│  ┌─────────────────────────┐                                │
│  │ arr1: [1, 2, 3, 4, 5]   │                                │
│  └─────────────────────────┘                                │
│  ┌─────────────────────────┐                                │
│  │ arr2: [1, 2, 3, 4, 5]   │                                │
│  └─────────────────────────┘                                │
│                                                              │
│  Both valid! Array is on stack, so it's Copy!               │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Example 3: Custom Copy Types

```rust
#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;  // Copy!
    
    println!("p1: {:?}", p1);  // Still valid!
    println!("p2: {:?}", p2);
}
```

## Why Can't String Be Copy?

```
┌──────────────────────────────────────────────────────────────┐
│              WHY STRING CANNOT BE COPY                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Imagine if String WAS Copy:                                 │
│                                                              │
│  let s1 = String::from("hello");                             │
│  let s2 = s1;  // If this was a copy...                     │
│                                                              │
│  Stack:                    Heap:                             │
│  ┌──────────┐             ┌───────────┐                     │
│  │ s1 ──────┼──────┐      │ "hello"   │                     │
│  └──────────┘      │      └───────────┘                     │
│  ┌──────────┐      │           ▲                            │
│  │ s2 ──────┼──────┘           │                            │
│  └──────────┘                  │                            │
│                          Two pointers to same heap!          │
│                                                              │
│  PROBLEM: When s1 AND s2 both go out of scope:              │
│  • Both try to free the same heap memory                     │
│  • Double free = memory corruption!                          │
│  • This is the bug Rust prevents!                            │
│                                                              │
│  That's why String MOVES instead of copies!                  │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Copy vs Clone: The Difference

```
┌──────────────────────────────────────────────────────────────┐
│                  COPY vs CLONE                               │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  COPY:                          CLONE:                       │
│  ┌─────────────────────┐        ┌─────────────────────┐     │
│  │ • Implicit          │        │ • Explicit          │     │
│  │ • Automatic         │        │ • Manual .clone()   │     │
│  │ • Cheap (stack)     │        │ • Can be expensive  │     │
│  │ • Always bitwise    │        │ • Can be custom     │     │
│  │ • No code needed    │        │ • Deep copy         │     │
│  └─────────────────────┘        └─────────────────────┘     │
│                                                              │
│  // Copy example                                             │
│  let x = 5;                                                  │
│  let y = x;  // Implicitly copied                            │
│                                                              │
│  // Clone example                                            │
│  let s1 = String::from("hello");                             │
│  let s2 = s1.clone();  // Explicitly cloned                  │
│                                                              │
│  ─────────────────────────────────────────────────────────   │
│                                                              │
│  Copy types can still use .clone():                          │
│                                                              │
│  let x = 5;                                                  │
│  let y = x.clone();  // Works, same as assignment            │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Copy and Drop: Mutually Exclusive

```
┌──────────────────────────────────────────────────────────────┐
│              COPY AND DROP ARE INCOMPATIBLE                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  A type CANNOT implement both Copy and Drop!                 │
│                                                              │
│  // This won't compile:                                      │
│  #[derive(Copy, Clone)]                                      │
│  struct MyType {                                             │
│      data: i32,                                              │
│  }                                                           │
│                                                              │
│  impl Drop for MyType {                                      │
│      fn drop(&mut self) {                                    │
│          println!("Dropping!");                              │
│      }                                                       │
│  }                                                           │
│                                                              │
│  ERROR: cannot implement both Copy and Drop                  │
│                                                              │
│  WHY?                                                        │
│  • Copy implies trivial duplication                          │
│  • Drop implies custom cleanup logic                         │
│  • If we copy a value, which copy runs Drop?                 │
│  • This would lead to double-free or resource leaks!         │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Function Calls with Copy Types

```rust
fn process(n: i32) -> i32 {
    n + 1
}

fn main() {
    let x = 5;
    let result = process(x);  // x is copied into function
    
    println!("x is still valid: {}", x);  // OK!
    println!("result: {}", result);
}
```

```
┌──────────────────────────────────────────────────────────────┐
│            COPY THROUGH FUNCTION CALLS                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  fn process(n: i32) -> i32 { n + 1 }                         │
│                                                              │
│  let x = 5;                                                  │
│                                                              │
│  Stack in main():                                            │
│  ┌─────────┐                                                │
│  │  x = 5  │                                                │
│  └─────────┘                                                │
│                                                              │
│  process(x)  // Copy x into parameter n                      │
│                                                              │
│  Stack in process():                                         │
│  ┌─────────┐                                                │
│  │  n = 5  │  (copy of x)                                   │
│  └─────────┘                                                │
│                                                              │
│  Stack in main() after call:                                 │
│  ┌─────────┐                                                │
│  │  x = 5  │  (still valid!)                                │
│  └─────────┘                                                │
│  ┌─────────┐                                                │
│  │result=6 │                                                │
│  └─────────┘                                                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Textio Example: Phone Number Type

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
struct PhoneNumber {
    country_code: u16,  // e.g., 1 for US
    area_code: u16,     // e.g., 555
    exchange: u16,      // e.g., 123
    line: u16,          // e.g., 4567
}

fn main() {
    let number1 = PhoneNumber {
        country_code: 1,
        area_code: 555,
        exchange: 123,
        line: 4567,
    };
    
    let number2 = number1;  // Copy!
    
    // Both valid
    println!("Number 1: {:?}", number1);
    println!("Number 2: {:?}", number2);
    
    // Can pass to function and still use after
    send_sms(number1);
    println!("Still have: {:?}", number1);  // OK!
}

fn send_sms(number: PhoneNumber) {
    println!("Sending to: {:?}", number);
}
```

## Memory Layout Comparison

```
┌──────────────────────────────────────────────────────────────┐
│               MEMORY LAYOUT: COPY vs NON-COPY                │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  COPY TYPE: Point { x: i32, y: i32 }                         │
│  Total size: 8 bytes                                         │
│                                                              │
│  Stack:                                                      │
│  ┌────────────────────────────────────┐                     │
│  │ x (4 bytes) │ y (4 bytes)          │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  Copying: Just duplicate 8 bytes on stack. Done!             │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  NON-COPY TYPE: struct Message { text: String }              │
│  Total size: 24 bytes (for String field)                     │
│                                                              │
│  Stack:                      Heap:                           │
│  ┌────────────────────┐     ┌─────────────────────┐         │
│  │ String struct      │     │ Actual text data    │         │
│  │ (ptr, len, cap)    │────►│ (variable size)     │         │
│  │ 24 bytes total     │     └─────────────────────┘         │
│  └────────────────────┘                                     │
│                                                              │
│  Copying: Can't just copy 24 bytes - would duplicate        │
│  the pointer! Must either MOVE or CLONE (deep copy).         │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## When to Make Your Type Copy

```
┌──────────────────────────────────────────────────────────────┐
│              WHEN TO IMPLEMENT COPY                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ✅ GOOD CANDIDATES:                                         │
│  • Small structs (just primitives)                           │
│  • Mathematical types (Point, Vector, Color)                 │
│  • Identifiers and handles that don't own resources          │
│  • Configuration structs with Copy fields                    │
│                                                              │
│  ❌ BAD CANDIDATES:                                          │
│  • Types with String or Vec fields                           │
│  • Types with Box, Rc, Arc                                   │
│  • Types that manage resources (File, Connection)            │
│  • Types with custom Drop implementations                    │
│  • Large structs (expensive to copy)                         │
│                                                              │
│  REMEMBER: Just because you CAN Copy doesn't mean you SHOULD │
│  • Large Copy types waste stack space                        │
│  • Consider references or Box for large types                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Exercises

In this exercise, you'll work with Copy types in Textio's system:

1. Identify which types are Copy and which aren't
2. Create custom Copy types for phone numbers and IDs
3. Understand when Copy happens implicitly
4. Compare Copy vs Clone performance

Complete the tasks in `code.rs` to master the Copy trait!

## Key Takeaways

```
┌──────────────────────────────────────────────────────────────┐
│                    REMEMBER                                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Copy = automatic bitwise duplication                     │
│                                                              │
│  2. Only for stack-only types (no heap, no resources)        │
│                                                              │
│  3. Copy and Drop are mutually exclusive                     │
│                                                              │
│  4. Copy is implicit, Clone is explicit                      │
│                                                              │
│  5. Primitives, tuples of Copy types, and arrays of Copy     │
│     types are all Copy                                       │
│                                                              │
│  6. String, Vec, Box are NOT Copy - they manage heap         │
│                                                              │
│  7. Copy through functions is cheap for small types          │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

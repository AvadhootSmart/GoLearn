# Exercise 2: Array Slices in Rust

## Overview

Array slices (`&[T]`) extend the concept of string slices to any type of data. In the Textio SMS API, you'll use array slices to work with collections of messages, phone numbers, delivery statuses, and more—all without copying data or taking ownership.

## What is an Array Slice?

An **array slice** is a reference to a contiguous portion of an array, vector, or other sequence. It's written as `&[T]` where `T` is the element type.

```rust
let numbers = [1, 2, 3, 4, 5];
let slice: &[i32] = &numbers[1..4]; // [2, 3, 4]
```

## The Fat Pointer Concept

Like string slices, array slices are **fat pointers** containing two pieces of information:

```
Memory Layout:
┌─────────────────────────────────────────────┐
│  &[T] Slice                                 │
├─────────────────┬───────────────────────────┤
│  Pointer (8B)   │  Length (8B)              │
│  0x1000         │  5                        │
└─────────────────┴───────────────────────────┘
         │
         ▼
┌────┬────┬────┬────┬────┐
│ 10 │ 20 │ 30 │ 40 │ 50 │  <- Actual data
└────┴────┴────┴────┴────┘
  [0]  [1]  [2]  [3]  [4]
```

This means:
- **Size is known at runtime** (stored in the fat pointer)
- **No copying** of elements
- **Efficient passing** to functions (just 16 bytes)

## Slicing Arrays vs Vectors

Both arrays and vectors can be sliced with the same syntax:

```rust
// Array (fixed size, stack-allocated)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let arr_slice: &[i32] = &arr[1..3];

// Vector (dynamic size, heap-allocated)
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
let vec_slice: &[i32] = &vec[1..3];

// Both slices have the same type: &[i32]
```

This is why slices are so powerful—a single function can work with both arrays and vectors!

## Slicing Syntax

### Full Slice: `&arr[..]`

```rust
let numbers = [1, 2, 3, 4, 5];
let all: &[i32] = &numbers[..];
// all == [1, 2, 3, 4, 5]
```

### From Start: `&arr[..end]`

```rust
let numbers = [1, 2, 3, 4, 5];
let first_three: &[i32] = &numbers[..3];
// first_three == [1, 2, 3]
```

### To End: `&arr[start..]`

```rust
let numbers = [1, 2, 3, 4, 5];
let last_three: &[i32] = &numbers[2..];
// last_three == [3, 4, 5]
```

### Range: `&arr[start..end]`

```rust
let numbers = [1, 2, 3, 4, 5];
let middle: &[i32] = &numbers[1..4];
// middle == [2, 3, 4]
```

### Inclusive Range: `&arr[start..=end]`

```rust
let numbers = [1, 2, 3, 4, 5];
let inclusive: &[i32] = &numbers[1..=3];
// inclusive == [2, 3, 4]
```

## The `.as_slice()` Method

Vectors provide `.as_slice()` for explicit conversion:

```rust
let vec = vec![1, 2, 3, 4, 5];

// These are equivalent:
let slice1: &[i32] = &vec[..];
let slice2: &[i32] = vec.as_slice();
```

Arrays can also use `.as_slice()`:

```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = arr.as_slice();
```

## Working with Slice Elements

### Accessing Elements

```rust
let numbers = [10, 20, 30, 40, 50];
let slice = &numbers[1..4];

// Index access (can panic)
let first = slice[0]; // 20
let second = slice[1]; // 30

// Safe access with .get()
let safe = slice.get(0); // Some(&20)
let out_of_bounds = slice.get(10); // None
```

### Iterating Over Slices

```rust
let numbers = [1, 2, 3, 4, 5];
let slice = &numbers[1..4];

// Immutable iteration
for num in slice {
    println!("{}", num);
}

// Mutable iteration (on &mut [T])
let mut values = [1, 2, 3, 4, 5];
let slice_mut = &mut values[1..4];
for num in slice_mut.iter_mut() {
    *num *= 2;
}
// values is now [1, 4, 6, 8, 5]
```

### Slice Methods

Slices have many useful methods:

```rust
let numbers = [1, 2, 3, 4, 5];
let slice = &numbers[..];

slice.len();           // 5
slice.is_empty();      // false
slice.first();         // Some(&1)
slice.last();          // Some(&5)
slice.contains(&3);    // true
slice.iter();          // Iterator over &i32
slice.windows(2);      // Sliding windows: [1,2], [2,3], [3,4], [4,5]
slice.chunks(2);       // Non-overlapping: [1,2], [3,4], [5]
slice.split_at(2);     // ([1,2], [3,4,5])
```

## Slices in Function Parameters

Using slices as function parameters provides maximum flexibility:

```rust
// This function works with arrays, vectors, and slices!
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

let arr = [1, 2, 3];
let vec = vec![4, 5, 6];

sum(&arr);      // Works with array
sum(&vec);      // Works with vector
sum(&arr[0..2]); // Works with slice
```

### Why Not Use `&Vec<T>`?

Using `&Vec<T>` is more restrictive than `&[T]`:

```rust
// Less flexible - only accepts Vec
fn less_flexible(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

// More flexible - accepts any slice
fn more_flexible(s: &[i32]) -> i32 {
    s.iter().sum()
}

let arr = [1, 2, 3];
// less_flexible(&arr); // Won't compile!
more_flexible(&arr);    // Works!
```

## Mutable Slices: `&mut [T]`

You can create mutable slices to modify elements:

```rust
let mut numbers = [1, 2, 3, 4, 5];
let slice = &mut numbers[1..4];

// Modify elements
slice[0] = 20;
slice[1] = 30;
slice[2] = 40;

// numbers is now [1, 20, 30, 40, 5]
```

### Common Mutating Operations

```rust
let mut numbers = [3, 1, 4, 1, 5];
let slice = &mut numbers[..];

slice.sort();          // [1, 1, 3, 4, 5]
slice.reverse();       // [5, 4, 3, 1, 1]
slice.rotate_left(2);  // [3, 1, 1, 5, 4]
slice.fill(0);         // [0, 0, 0, 0, 0]
slice.copy_from_slice(&[1, 2, 3, 4, 5]); // [1, 2, 3, 4, 5]
```

## Why Slices Don't Have Ownership

Slices are **borrowed references**:

1. **No Allocation**: Slices don't allocate memory
2. **No Deallocation**: Dropping a slice does nothing
3. **Lifetime Dependent**: Cannot outlive the source
4. **Borrow Checker**: Ensures safety at compile time

```rust
fn example() {
    let data = vec![1, 2, 3, 4, 5]; // Owner
    
    {
        let slice: &[i32] = &data[1..3]; // Borrower
        println!("{:?}", slice);
    } // slice dropped here (no memory freed)
    
    // data is still valid!
    println!("{:?}", data);
    
} // data dropped here (memory freed)
```

## Range Types

Rust's range types enable flexible slicing:

| Type | Syntax | Use Case |
|------|--------|----------|
| `Range<usize>` | `1..4` | Most common, exclusive end |
| `RangeFrom<usize>` | `2..` | From index to end |
| `RangeTo<usize>` | `..3` | From start to index |
| `RangeFull` | `..` | Entire sequence |
| `RangeInclusive<usize>` | `1..=4` | Inclusive end |
| `RangeToInclusive<usize>` | `..=3` | Start to inclusive end |

## Performance Benefits

### Zero-Cost Abstraction

```rust
// This function call has no overhead
fn first_three(data: &[i32]) -> &[i32] {
    &data[..3]
}
```

The compiler optimizes slices to direct memory access.

### Cache Efficiency

Slices reference contiguous memory, which is cache-friendly:

```rust
// Good: Contiguous memory access
let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let slice = &data[..];
for num in slice {
    println!("{}", num); // Cache-friendly sequential access
}
```

### Small Size

A slice is just 16 bytes (on 64-bit systems):
- 8 bytes for the pointer
- 8 bytes for the length

This makes passing slices very cheap.

## Textio Use Cases

### Processing Message Queue

```rust
fn process_batch(messages: &[SmsMessage]) {
    for msg in messages {
        send_message(msg);
    }
}
```

### Phone Number Validation

```rust
fn validate_prefix(digits: &[u8]) -> bool {
    matches!(digits, [1, ..] | [4, 4, ..])
}
```

### Status Tracking

```rust
fn get_failed_indices(statuses: &[DeliveryStatus]) -> Vec<usize> {
    statuses
        .iter()
        .enumerate()
        .filter(|(_, s)| s == &DeliveryStatus::Failed)
        .map(|(i, _)| i)
        .collect()
}
```

## Exercise Instructions

In this exercise, you'll implement functions for Textio's message queue:

1. **`get_recent_messages`**: Get the last N messages from the queue
2. **`calculate_average_length`**: Calculate the average message length
3. **`find_message_by_id`**: Find a message by its ID
4. **`partition_messages`**: Split messages at a given index

Run the code and verify it produces the expected output.

## Key Takeaways

- `&[T]` is a borrowed view into a sequence
- Works with both arrays and vectors
- Fat pointer: pointer + length (16 bytes)
- Use `.as_slice()` for explicit conversion
- Slices are Copy, making them cheap to pass around
- Mutable slices (`&mut [T]`) allow modification
- Slices have many useful methods (sort, reverse, etc.)
- Always prefer `&[T]` over `&Vec<T>` in function parameters

## Common Pitfalls

1. **Out of bounds slicing** (panics!)
2. **Using slices after source is dropped** (compile error)
3. **Holding mutable slice while accessing source** (borrow checker)
4. **Confusing slice length with source length**

## Further Reading

- [Rust Book: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust Documentation: Primitive Type Slice](https://doc.rust-lang.org/std/primitive.slice.html)
- [Rust By Example: Slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html#arrays-and-slices)

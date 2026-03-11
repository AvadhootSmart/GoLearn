# Iterator Adapters

## Introduction

Iterator adapters are methods that transform an iterator into another iterator. They are the building blocks of iterator chains, allowing you to express complex data transformations declaratively. Because iterators are lazy, these transformations don't execute until a consuming adapter is called.

## The Power of Chaining

Iterator adapters can be chained together to create powerful data pipelines:

```rust
let result: Vec<i32> = (1..=100)
    .filter(|x| x % 2 == 0)    // Keep even numbers
    .map(|x| x * x)            // Square them
    .take(5)                   // Take first 5
    .collect();                // Collect into Vec
// Result: [4, 16, 36, 64, 100]
```

## Transforming Adapters

### `map()` - Transform Each Element

Applies a function to each element:

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
// [2, 4, 6]

let words = vec!["hello", "world"];
let lengths: Vec<usize> = words.iter().map(|s| s.len()).collect();
// [5, 5]
```

### `filter()` - Keep Matching Elements

Keeps elements that satisfy a predicate:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
// [&2, &4, &6]
```

### `filter_map()` - Filter and Transform Combined

Combines filter and map; returns `Some(value)` to keep, `None` to discard:

```rust
let strings = vec!["1", "two", "3", "four"];
let numbers: Vec<i32> = strings
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
// [1, 3]
```

### `inspect()` - Peek at Elements (Debugging)

Executes a function on each element without modifying it:

```rust
let sum: i32 = (1..=5)
    .inspect(|x| println!("Processing: {}", x))
    .sum();
```

## Limiting Adapters

### `take()` - Take First N Elements

Limits iteration to the first N elements:

```rust
let first_three: Vec<i32> = (1..).take(3).collect();
// [1, 2, 3]
```

### `skip()` - Skip First N Elements

Skips the first N elements:

```rust
let after_two: Vec<i32> = (1..=5).skip(2).collect();
// [3, 4, 5]
```

### `take_while()` - Take While Condition is True

Takes elements while predicate returns true, stops at first false:

```rust
let numbers = vec![1, 2, 3, 4, 1, 2];
let taken: Vec<&i32> = numbers.iter().take_while(|x| **x < 4).collect();
// [&1, &2, &3]
```

### `skip_while()` - Skip While Condition is True

Skips elements while predicate returns true:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let remaining: Vec<&i32> = numbers.iter().skip_while(|x| **x < 3).collect();
// [&3, &4, &5]
```

## Combining Adapters

### `zip()` - Pair Elements from Two Iterators

Combines two iterators into pairs:

```rust
let names = vec!["Alice", "Bob", "Charlie"];
let ages = vec![30, 25, 35];
let pairs: Vec<(&str, i32)> = names.iter().zip(ages.iter()).map(|(n, a)| (*n, *a)).collect();
// [("Alice", 30), ("Bob", 25), ("Charlie", 35)]
```

The resulting iterator length is the minimum of the two input iterators.

### `chain()` - Concatenate Iterators

Joins two iterators sequentially:

```rust
let v1 = vec![1, 2, 3];
let v2 = vec![4, 5, 6];
let combined: Vec<i32> = v1.iter().chain(v2.iter()).cloned().collect();
// [1, 2, 3, 4, 5, 6]
```

### `flatten()` - Flatten Nested Iterators

Flattens a structure of iterators into one:

```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
let flat: Vec<i32> = nested.iter().flatten().cloned().collect();
// [1, 2, 3, 4, 5]
```

### `flat_map()` - Map Then Flatten

Combines map and flatten:

```rust
let words = vec!["Hello", "World"];
let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
// ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd']
```

## Indexing Adapters

### `enumerate()` - Add Index to Each Element

Returns tuples of (index, element):

```rust
let items = vec!["a", "b", "c"];
for (index, value) in items.iter().enumerate() {
    println!("{}: {}", index, value);
}
// 0: a
// 1: b
// 2: c
```

## Ordering Adapters

### `rev()` - Reverse the Iterator

Reverses the order (requires DoubleEndedIterator):

```rust
let reversed: Vec<i32> = (1..=5).rev().collect();
// [5, 4, 3, 2, 1]
```

### `sorted()` and `sorted_by()` - Sort Elements

Requires the `itertools` crate or use `.collect::<Vec<_>>().sort()`:

```rust
let mut v: Vec<i32> = vec![3, 1, 4, 1, 5].iter().cloned().collect();
v.sort();
```

## Performance Considerations

Iterator adapters are zero-cost abstractions. The compiler optimizes chained iterators to be as efficient as hand-written loops:

```rust
// This high-level code:
let sum: i32 = (1..=1000).filter(|x| x % 2 == 0).map(|x| x * 2).sum();

// Compiles to code as efficient as:
let mut sum = 0;
for x in 1..=1000 {
    if x % 2 == 0 {
        sum += x * 2;
    }
}
```

## Textio Example: Message Processing Pipeline

```rust
struct Message {
    id: u32,
    phone: String,
    content: String,
    priority: u8,
}

let messages = vec![/* ... */];

// Get high-priority message summaries
let urgent_summaries: Vec<String> = messages
    .iter()
    .filter(|m| m.priority >= 8)
    .map(|m| format!("To {}: {}", m.phone, m.content))
    .take(10)
    .collect();

// Create message batches with IDs
let with_indices: Vec<(usize, &Message)> = messages
    .iter()
    .enumerate()
    .collect();

// Combine two message queues
let all_messages = queue1.iter()
    .chain(queue2.iter())
    .collect::<Vec<_>>();
```

## Common Patterns

### Pagination with `skip()` and `take()`

```rust
fn get_page(messages: &[Message], page: usize, per_page: usize) -> Vec<&Message> {
    messages
        .iter()
        .skip(page * per_page)
        .take(per_page)
        .collect()
}
```

### Deduplication

```rust
let numbers = vec![1, 1, 2, 2, 3, 3];
let deduped: Vec<i32> = numbers
    .iter()
    .dedup()  // Requires itertools crate
    .cloned()
    .collect();
```

### Grouping

```rust
use std::collections::HashMap;
let grouped: HashMap<char, Vec<&str>> = words
    .iter()
    .fold(HashMap::new(), |mut acc, word| {
        let first_char = word.chars().next().unwrap();
        acc.entry(first_char).or_insert_with(Vec::new).push(*word);
        acc
    });
```

## Exercise Overview

In this exercise, you will:
1. Use `map()` and `filter()` to transform data
2. Apply `take()`, `skip()`, and `enumerate()` for pagination
3. Combine iterators with `zip()` and `chain()`
4. Flatten nested structures
5. Build a message processing pipeline for Textio

## Key Takeaways

- Iterator adapters are lazy - they don't execute until consumed
- Adapters can be chained for complex transformations
- `map()` transforms, `filter()` selects, `filter_map()` does both
- `zip()` pairs, `chain()` concatenates, `flatten()` flattens
- `enumerate()` adds indices for tracking position
- These abstractions have zero runtime cost

## Further Reading

- [Rust Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Iterator Adapters in the Book](https://doc.rust-lang.org/book/ch13-02-iterators.html)

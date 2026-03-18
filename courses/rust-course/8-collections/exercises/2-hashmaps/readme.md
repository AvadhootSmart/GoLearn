# HashMaps in Rust

## Why this matters
Many tasks need fast key-based lookup (scores, counts, indexes). `HashMap<K, V>` is Rust's standard hash table for this job.

## Core concept
A `HashMap` stores key-value pairs where keys are unique. Insertions can overwrite existing keys, and lookups return `Option<&V>`.

## Worked example
```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("alice".to_string(), 10);
    scores.insert("bob".to_string(), 20);

    if let Some(score) = scores.get("alice") {
        println!("alice {}", score);
    }
}
```

## Common mistakes
- Forgetting `mut` before inserting.
- Expecting `get` to return a value instead of `Option`.

## Exercise task

1. Create a mutable `HashMap<String, i32>` called `scores`.
2. Insert `("alice", 10)` and `("bob", 20)`.
3. Read alice's score and print `alice <score>`.

## Quick recap
Use `HashMap` for efficient key-value access and handle missing keys with `Option`.

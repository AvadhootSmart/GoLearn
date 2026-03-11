# HashMap in Rust

## Introduction

A `HashMap<K, V>` stores key-value pairs with fast lookup, insertion, and deletion. In our Textio SMS API, HashMaps are perfect for mapping phone numbers to delivery statuses, storing message metadata, and caching API responses.

## What is a HashMap?

A HashMap uses a hash function to compute an index into an array of buckets, from which the desired value can be found:

```
HashMap Structure:

┌─────────────────────────────────────────────────────────────┐
│                      HashMap<K, V>                           │
│                                                              │
│  ┌─────┐                                                     │
│  │  0  │──► None                                             │
│  ├─────┤                                                     │
│  │  1  │──► Some(Bucket { hash, key, value })               │
│  ├─────┤                                                     │
│  │  2  │──► Some(Bucket) ──► Some(Bucket)  (collision chain)│
│  ├─────┤                                                     │
│  │  3  │──► None                                             │
│  ├─────┤                                                     │
│  │ ... │                                                     │
│  └─────┘                                                     │
│                                                              │
│  Buckets array (capacity determines size)                    │
└─────────────────────────────────────────────────────────────┘

Key → Hash Function → Index → Bucket → Value
```

## Creating HashMaps

### Using `new`

```rust
use std::collections::HashMap;

let mut map: HashMap<String, i32> = HashMap::new();
```

### Using `with_capacity`

```rust
let mut map = HashMap::with_capacity(100);
```

### From Arrays

```rust
let map: HashMap<_, _> = [
    ("apple", 3),
    ("banana", 2),
    ("orange", 5),
].into_iter().collect();
```

## Basic Operations

### Inserting Values

```rust
let mut map = HashMap::new();
map.insert("key1", "value1");
map.insert("key2", "value2");
```

### Getting Values

```rust
let mut map = HashMap::new();
map.insert("key1", "value1");

let value = map.get("key1");  // Some(&"value1")
let missing = map.get("key3");  // None
```

### Checking if Key Exists

```rust
if map.contains_key("key1") {
    println!("Key exists!");
}
```

### Removing Entries

```rust
let removed = map.remove("key1");  // Some("value1")
let not_found = map.remove("key3");  // None
```

## The Entry API

The `entry` API is a powerful way to handle the common pattern of "insert if missing, modify if present":

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// or_insert: Insert if key doesn't exist
scores.entry("blue").or_insert(10);
scores.entry("blue").or_insert(50);  // Won't change, already exists

// or_insert_with: Insert with a function if key doesn't exist
scores.entry("red").or_insert_with(|| {
    // Expensive computation only runs if needed
    100
});
```

### Entry Methods

| Method | Behavior |
|--------|----------|
| `or_insert(value)` | Insert if missing, return mutable ref |
| `or_insert_with(f)` | Insert with function if missing |
| `or_default()` | Insert default value if missing |
| `and_modify(f)` | Modify existing value |
| `key()` | Get reference to key |

### Combining Operations

```rust
let mut map: HashMap<&str, u32> = HashMap::new();

// Insert 0 if missing, then increment
*map.entry("count").or_insert(0) += 1;

// Modify if exists, otherwise insert default
map.entry("count")
    .and_modify(|v| *v += 10)
    .or_insert(10);
```

## Mutable Access

### get_mut

```rust
let mut map = HashMap::new();
map.insert("key", 10);

if let Some(value) = map.get_mut("key") {
    *value += 5;
}
```

### Entry-based Mutation

```rust
// Preferred: Use entry for in-place modification
map.entry("key")
    .and_modify(|v| *v += 5)
    .or_insert(5);
```

## Iteration

### Over Key-Value Pairs

```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

### Over Keys Only

```rust
for key in map.keys() {
    println!("{}", key);
}
```

### Over Values Only

```rust
for value in map.values() {
    println!("{}", value);
}
```

### Mutable Iteration

```rust
for (key, value) in &mut map {
    *value += 1;
}
```

## Textio Example: Delivery Status Tracking

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum DeliveryStatus {
    Pending,
    Sent,
    Delivered,
    Failed(String),
}

struct MessageTracker {
    statuses: HashMap<String, DeliveryStatus>,
    retry_counts: HashMap<String, u32>,
}

impl MessageTracker {
    fn new() -> Self {
        MessageTracker {
            statuses: HashMap::new(),
            retry_counts: HashMap::new(),
        }
    }

    fn track(&mut self, message_id: String) {
        self.statuses.insert(message_id.clone(), DeliveryStatus::Pending);
        self.retry_counts.insert(message_id, 0);
    }

    fn mark_sent(&mut self, message_id: &str) {
        if let Some(status) = self.statuses.get_mut(message_id) {
            *status = DeliveryStatus::Sent;
        }
    }

    fn mark_delivered(&mut self, message_id: &str) {
        self.statuses.insert(
            message_id.to_string(),
            DeliveryStatus::Delivered
        );
    }

    fn mark_failed(&mut self, message_id: &str, reason: &str) {
        self.statuses.insert(
            message_id.to_string(),
            DeliveryStatus::Failed(reason.to_string())
        );
        *self.retry_counts.entry(message_id.to_string()).or_insert(0) += 1;
    }

    fn get_status(&self, message_id: &str) -> Option<&DeliveryStatus> {
        self.statuses.get(message_id)
    }

    fn get_retry_count(&self, message_id: &str) -> u32 {
        *self.retry_counts.get(message_id).unwrap_or(&0)
    }

    fn pending_messages(&self) -> Vec<&String> {
        self.statuses
            .iter()
            .filter(|(_, status)| matches!(status, DeliveryStatus::Pending))
            .map(|(id, _)| id)
            .collect()
    }
}
```

## Memory and Performance

### Load Factor

HashMap automatically resizes when the load factor exceeds a threshold:

```
Before resize:
┌───┬───┬───┬───┐
│ A │ B │   │ C │  3/4 = 75% load
└───┴───┴───┴───┘

After resize (doubled):
┌───┬───┬───┬───┬───┬───┬───┬───┐
│ A │   │ B │   │   │ C │   │   │  3/8 = 37.5% load
└───┴───┴───┴───┴───┴───┴───┴───┘
```

### Hash Function

By default, Rust uses SipHash, which is cryptographically strong but slower. For non-cryptographic use:

```rust
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

let mut map: HashMap<_, _, BuildHasherDefault<XxHash64>> = HashMap::default();
```

## Common Patterns

### Counting Occurrences

```rust
let text = "hello world hello rust";
let mut counts: HashMap<&str, u32> = HashMap::new();

for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}
```

### Caching Computed Values

```rust
let mut cache: HashMap<String, ExpensiveResult> = HashMap::new();

fn get_or_compute(cache: &mut HashMap<String, ExpensiveResult>, key: &str) -> &ExpensiveResult {
    cache.entry(key.to_string()).or_insert_with(|| {
        // Expensive computation
        compute_expensive_result(key)
    })
}
```

### Grouping Data

```rust
let data = vec![
    ("fruit", "apple"),
    ("vegetable", "carrot"),
    ("fruit", "banana"),
];

let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();

for (category, item) in data {
    groups.entry(category).or_insert_with(Vec::new).push(item);
}
```

## Common Pitfalls

### Borrow Checker Issues

```rust
let mut map = HashMap::new();
map.insert("key", "value");

let value = map.get("key");
map.insert("other", "new");  // Error: already borrowed
println!("{:?}", value);
```

### Key Type Requirements

Keys must implement `Hash` and `Eq`:

```rust
// Works: String implements Hash + Eq
let mut map: HashMap<String, i32> = HashMap::new();

// Won't compile: Vec doesn't implement Hash
// let mut map: HashMap<Vec<i32>, i32> = HashMap::new();
```

### Ownership with get

```rust
let map = HashMap::from([("key".to_string(), "value".to_string())]);
let value = map.get("key");  // Returns Option<&String>
// map.remove("key");  // Can't mutate while borrowed
```

## Best Practices

1. Use `entry` API for insert-or-update patterns
2. Pre-allocate with `with_capacity` for known sizes
3. Use `get_mut` for simple modifications
4. Consider custom hashers for performance-critical code
5. Use `retain` to filter entries in place

## Summary

HashMaps provide O(1) average-case performance for lookups, insertions, and deletions. The entry API is particularly powerful for Textio's use cases like tracking message statuses and counting retry attempts efficiently.

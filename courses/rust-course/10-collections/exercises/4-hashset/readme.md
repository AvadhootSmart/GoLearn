# HashSet in Rust

## Introduction

A `HashSet<T>` is a collection of unique values with fast lookup, insertion, and deletion. Unlike a `Vec` which can contain duplicates, a `HashSet` guarantees uniqueness. In Textio, HashSets are perfect for tracking unique phone numbers, managing contact lists, and performing set operations like finding common recipients.

## What is a HashSet?

A HashSet is essentially a HashMap where we only care about the keys, not the values:

```
HashSet Structure:

┌─────────────────────────────────────────────────────────────┐
│                       HashSet<T>                             │
│                                                              │
│  ┌─────┐                                                     │
│  │  0  │──► None                                             │
│  ├─────┤                                                     │
│  │  1  │──► Some("+1234567890")                              │
│  ├─────┤                                                     │
│  │  2  │──► Some("+1111111111") ──► Some("+2222222222")      │
│  ├─────┤                                                     │
│  │  3  │──► None                                             │
│  └─────┘                                                     │
│                                                              │
│  Only stores unique values - duplicates are ignored          │
└─────────────────────────────────────────────────────────────┘
```

## Creating HashSets

### Using `new`

```rust
use std::collections::HashSet;

let mut set: HashSet<i32> = HashSet::new();
```

### With Initial Values

```rust
let set: HashSet<_> = [1, 2, 3, 4, 5].into_iter().collect();
let set: HashSet<&str> = ["apple", "banana", "orange"].into_iter().collect();
```

### With Capacity

```rust
let set = HashSet::with_capacity(100);
```

## Basic Operations

### Inserting Values

```rust
let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
set.insert("apple");  // Duplicate - ignored!

println!("{:?}", set);  // {"apple", "banana"}
```

The `insert` method returns `true` if the value was newly inserted, and `false` if it already existed.

### Checking for Membership

```rust
if set.contains(&"apple") {
    println!("Apple is in the set!");
}
```

### Removing Values

```rust
let removed = set.remove(&"apple");  // true if it was present
```

### Getting Length

```rust
let count = set.len();
let is_empty = set.is_empty();
```

## Set Operations

HashSet provides powerful mathematical set operations:

### Union (∪)

Returns all elements from both sets:

```rust
let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let b: HashSet<_> = [3, 4, 5].into_iter().collect();

let union: HashSet<_> = a.union(&b).collect();
// {1, 2, 3, 4, 5}
```

```
Set A:    ┌───┬───┬───┐
          │ 1 │ 2 │ 3 │
          └───┴───┴───┘
                          
Set B:              ┌───┬───┬───┐
                    │ 3 │ 4 │ 5 │
                    └───┴───┴───┘

Union:    ┌───┬───┬───┬───┬───┐
          │ 1 │ 2 │ 3 │ 4 │ 5 │
          └───┴───┴───┴───┴───┘
```

### Intersection (∩)

Returns elements present in both sets:

```rust
let intersection: HashSet<_> = a.intersection(&b).collect();
// {3}
```

```
Set A:    ┌───┬───┬───┐
          │ 1 │ 2 │ 3 │
          └───┴───┴───┘
                  │
Set B:            ▼
          ┌───┬───┬───┐
          │ 3 │ 4 │ 5 │
          └───┴───┴───┘

Intersection: ┌───┐
              │ 3 │
              └───┘
```

### Difference (-)

Returns elements in A but not in B:

```rust
let difference: HashSet<_> = a.difference(&b).collect();
// {1, 2}
```

```
Set A:    ┌───┬───┬───┐
          │ 1 │ 2 │ 3 │
          └───┴───┴───┘
            │   │   ✗ (also in B)
            │   │
Difference: ▼   ▼
          ┌───┬───┐
          │ 1 │ 2 │
          └───┴───┘
```

### Symmetric Difference

Returns elements in either set, but not in both (XOR):

```rust
let sym_diff: HashSet<_> = a.symmetric_difference(&b).collect();
// {1, 2, 4, 5}
```

```
Set A:    ┌───┬───┬───┐
          │ 1 │ 2 │ 3 │  ← 3 is in both
          └───┴───┴───┘

Set B:    ┌───┬───┬───┐
          │ 3 │ 4 │ 5 │  ← 3 is in both
          └───┴───┴───┘

Sym Diff: ┌───┬───┬───┬───┐
          │ 1 │ 2 │ 4 │ 5 │  (not 3!)
          └───┴───┴───┴───┘
```

## Subset Relationships

### is_subset

```rust
let a: HashSet<_> = [1, 2].into_iter().collect();
let b: HashSet<_> = [1, 2, 3, 4].into_iter().collect();

a.is_subset(&b);  // true
```

### is_superset

```rust
b.is_superset(&a);  // true
```

### is_disjoint

Returns true if sets have no elements in common:

```rust
let a: HashSet<_> = [1, 2].into_iter().collect();
let b: HashSet<_> = [3, 4].into_iter().collect();

a.is_disjoint(&b);  // true
```

## Textio Example: Contact Management

```rust
use std::collections::HashSet;

struct ContactList {
    contacts: HashSet<String>,
    blocked: HashSet<String>,
    vip: HashSet<String>,
}

impl ContactList {
    fn new() -> Self {
        ContactList {
            contacts: HashSet::new(),
            blocked: HashSet::new(),
            vip: HashSet::new(),
        }
    }

    fn add_contact(&mut self, phone: String) -> bool {
        self.contacts.insert(phone)
    }

    fn remove_contact(&mut self, phone: &str) -> bool {
        self.contacts.remove(phone)
    }

    fn block_number(&mut self, phone: String) {
        self.blocked.insert(phone);
    }

    fn unblock_number(&mut self, phone: &str) {
        self.blocked.remove(phone);
    }

    fn is_blocked(&self, phone: &str) -> bool {
        self.blocked.contains(phone)
    }

    fn mark_vip(&mut self, phone: String) {
        if self.contacts.contains(&phone) {
            self.vip.insert(phone);
        }
    }

    fn can_send_to(&self, phone: &str) -> bool {
        !self.is_blocked(phone)
    }

    fn merge_contacts(&mut self, other: HashSet<String>) {
        self.contacts.extend(other);
    }

    fn get_blocked_contacts(&self) -> HashSet<&String> {
        self.contacts.intersection(&self.blocked).collect()
    }

    fn get_reachable_contacts(&self) -> HashSet<&String> {
        self.contacts.difference(&self.blocked).collect()
    }
}
```

## Iteration

### Basic Iteration

```rust
for phone in &set {
    println!("{}", phone);
}
```

### Converting to Vec

```rust
let vec: Vec<_> = set.into_iter().collect();
```

## Modifying While Iterating

You cannot modify a set while iterating over it:

```rust
// This won't compile:
for item in &set {
    set.remove(item);  // Error!
}

// Instead, collect items to remove first:
let to_remove: Vec<_> = set.iter()
    .filter(|item| /* condition */)
    .cloned()
    .collect();

for item in to_remove {
    set.remove(&item);
}

// Or use retain:
set.retain(|item| /* keep if true */);
```

## Performance Characteristics

| Operation | Average | Worst Case |
|-----------|---------|------------|
| Insert | O(1) | O(n) |
| Remove | O(1) | O(n) |
| Contains | O(1) | O(n) |
| Union | O(len(s) + len(t)) | - |
| Intersection | O(min(len(s), len(t))) | - |

## Common Patterns

### Deduplicating a Vec

```rust
let numbers = vec![1, 2, 2, 3, 3, 3, 4];
let unique: HashSet<_> = numbers.into_iter().collect();
let deduped: Vec<_> = unique.into_iter().collect();
```

### Finding Duplicates

```rust
fn find_duplicates(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    
    for &item in items {
        if !seen.insert(item) {
            duplicates.insert(item);
        }
    }
    
    duplicates.into_iter().collect()
}
```

### Tag/Category System

```rust
struct Message {
    id: u64,
    tags: HashSet<String>,
}

impl Message {
    fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
    
    fn has_any_tag(&self, tags: &HashSet<String>) -> bool {
        !self.tags.is_disjoint(tags)
    }
    
    fn has_all_tags(&self, tags: &HashSet<String>) -> bool {
        tags.is_subset(&self.tags)
    }
}
```

## Best Practices

1. Use `HashSet` when uniqueness is the primary concern
2. Pre-allocate with `with_capacity` for known sizes
3. Use set operations for combining/comparing collections
4. Use `retain` for efficient filtering in place
5. Consider `BTreeSet` if you need ordered iteration

## Summary

HashSets provide efficient uniqueness guarantees and powerful set operations. For Textio, they're essential for managing contact lists, blocking phone numbers, and performing recipient analysis across multiple campaigns.

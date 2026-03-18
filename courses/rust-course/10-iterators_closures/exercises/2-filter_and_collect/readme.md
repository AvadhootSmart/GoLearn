# Filtering Iterators

## Why this matters
Iterator pipelines make data processing concise, composable, and expressive without manual indexing loops.

## Core concept
`filter` keeps matching elements; `collect` materializes them into a concrete collection.

## Worked example
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let evens: Vec<i32> = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect();

    println!("{:?}", evens);
}
```

## Common mistakes
- Using `.iter()` and then trying to collect into owned values without cloning.
- Forgetting type annotation when inference is ambiguous.

## Exercise task

1. Create `numbers = vec![1, 2, 3, 4, 5]`.
2. Build `evens` using `.into_iter().filter(...)` to keep even values.
3. Collect into `Vec<i32>` and print it.

## Quick recap
Think in stages: source iterator -> transform/filter -> collect result.

# Filtering Iterators

## Concept first: selecting matching elements

Use `filter` to keep elements that match a predicate, then `collect` into a concrete collection.

## Exercise task

1. Create `numbers = vec![1, 2, 3, 4, 5]`.
2. Build `evens` using `.into_iter().filter(...)` to keep even values.
3. Collect into `Vec<i32>` and print it.

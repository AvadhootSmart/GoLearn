# Struct Methods with `impl`

## Why this matters
Structs model data, and methods model behavior tied to that data. Grouping both together with `impl` improves readability and API discoverability.

## Core concept
Method receivers define access level:
- `self` consumes,
- `&self` reads,
- `&mut self` mutates.

## Worked example
```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();
    c.increment();
    println!("{}", c.value);
}
```

## Common mistakes
- Forgetting `mut` on the variable when calling a `&mut self` method.
- Using associated functions and methods interchangeably (they are invoked differently).

## Exercise task

1. Define a struct `Counter` with field `value: i32`.
2. Add an `impl` method `increment(&mut self)` that increases `value` by 1.
3. In `main`, create a mutable `Counter` with `value: 0`, call `increment()` twice, and print `value`.

## Quick recap
Use `impl` to place behavior next to data and choose receiver types intentionally.

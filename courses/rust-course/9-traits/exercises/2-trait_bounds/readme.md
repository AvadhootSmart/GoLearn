# Trait Bounds in Functions

## Why this matters
Generics alone say "any type." Trait bounds say "any type with required behavior," enabling reusable but constrained APIs.

## Core concept
Trait bounds attach capabilities to generic parameters:
```rust
fn print_log<T: Loggable>(item: &T) {
    println!("{}", item.log_line());
}
```
Only types implementing `Loggable` can be used.

## Worked example
```rust
trait Loggable {
    fn log_line(&self) -> String;
}

struct Event { id: u32 }

impl Loggable for Event {
    fn log_line(&self) -> String {
        format!("event:{}", self.id)
    }
}
```

## Common mistakes
- Forgetting to implement the trait before using bounded functions.
- Over-constraining bounds and reducing API flexibility.

## Exercise task

1. Define trait `Loggable` with `fn log_line(&self) -> String;`.
2. Implement `Loggable` for struct `Event { id: u32 }`.
3. Write `print_log<T: Loggable>(item: &T)` that prints `item.log_line()`.
4. Use it in `main` with `Event { id: 7 }`.

## Quick recap
Trait bounds make generic code both reusable and behaviorally precise.

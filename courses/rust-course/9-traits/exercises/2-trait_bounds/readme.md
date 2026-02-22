# Trait Bounds in Functions

## Concept first: generic behavior constraints

Trait bounds let generic functions require specific capabilities.

## Exercise task

1. Define trait `Loggable` with `fn log_line(&self) -> String;`.
2. Implement `Loggable` for struct `Event { id: u32 }`.
3. Write `print_log<T: Loggable>(item: &T)` that prints `item.log_line()`.
4. Use it in `main` with `Event { id: 7 }`.

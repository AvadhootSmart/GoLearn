# Enums in Rust

## Concept first: modeling one-of-many states

Enums let you represent values that can be exactly one of several variants.

Example:

```rust
enum MessageStatus {
    Sent,
    Failed,
}

fn print_status(status: MessageStatus) {
    match status {
        MessageStatus::Sent => println!("Message was sent"),
        MessageStatus::Failed => println!("Message failed"),
    }
}
```

Nuances:

- `match` on enums is exhaustive; every variant must be handled.
- Enums are ideal for state machines and API outcomes.
- Variant names are namespaced (e.g., `MessageStatus::Sent`).

## Exercise task

1. Define an enum `MessageStatus` with variants `Sent` and `Failed`.
2. Write a function `print_status` that takes a `MessageStatus` and uses a `match` expression to print `"Message was sent"` if it's `Sent`, or `"Message failed"` if it's `Failed`.
3. Call `print_status` with `MessageStatus::Sent` in `main`.

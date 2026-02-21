# Enums in Rust

## Concept first: modeling one-of-many states

Enums let you represent values that can be exactly one of several variants.

Example:

```rust
enum ConnectionState {
    Connected,
    Disconnected,
}

fn print_state(state: ConnectionState) {
    match state {
        ConnectionState::Connected => println!("Connected"),
        ConnectionState::Disconnected => println!("Disconnected"),
    }
}
```

Nuances:

- `match` on enums is exhaustive; every variant must be handled.
- Enums are ideal for state machines and API outcomes.
- Variant names are namespaced (e.g., `ConnectionState::Connected`).

## Exercise task

1. Define an enum `MessageStatus` with variants `Sent` and `Failed`.
2. Write a function `print_status` that takes a `MessageStatus` and uses a `match` expression to print `"Message was sent"` if it's `Sent`, or `"Message failed"` if it's `Failed`.
3. Call `print_status` with `MessageStatus::Sent` in `main`.

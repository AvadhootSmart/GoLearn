# Borrowing with References

## Why this matters
Ownership prevents memory bugs, but moving values everywhere is inconvenient. Borrowing lets you read data without taking ownership away from the caller.

## Core concept
- `String` owns heap data.
- `&String` is a shared borrow (read-only reference).
- Borrowing avoids cloning and preserves ownership at call sites.

## Worked example
```rust
fn message_len(msg: &String) -> usize {
    msg.len()
}

fn main() {
    let msg = String::from("status");
    let len = message_len(&msg);
    println!("len={}, msg={}", len, msg); // msg still usable
}
```

## Common mistakes
- Passing `msg` instead of `&msg`, which moves ownership.
- Overusing `&String` in APIs where `&str` would be more flexible.

## Exercise task

1. Write a function `message_len` that takes `&String` and returns `usize`.
2. In `main`, create `msg = String::from("status")`.
3. Call `message_len(&msg)` and print both the length and original message.

## Quick recap
Borrow references for read access and keep ownership with the original variable.

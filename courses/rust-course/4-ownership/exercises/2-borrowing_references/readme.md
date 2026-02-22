# Borrowing with References

## Concept first: read access without taking ownership

Borrowing lets a function inspect data using references (`&T`) without moving ownership.

## Exercise task

1. Write a function `message_len` that takes `&String` and returns `usize`.
2. In `main`, create `msg = String::from("status")`.
3. Call `message_len(&msg)` and print both the length and original message.

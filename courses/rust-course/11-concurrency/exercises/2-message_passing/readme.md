# Message Passing with Channels

## Concept first: thread communication

Channels allow one thread to send values to another safely.

## Exercise task

1. Create a channel with `std::sync::mpsc::channel()`.
2. Spawn a thread that sends `String::from("ready")`.
3. Receive in main thread and print `message <value>`.

# Enums in Rust

Enums allow you to define a type by enumerating its possible variants. When dealing with web traffic or APIs in Textio, you'll frequently encounter different states or types of messages.

## Assignment

1. Define an enum `MessageStatus` with variants `Sent` and `Failed`.
2. Write a function `print_status` that takes a `MessageStatus` and uses a `match` expression to print "Message was sent" if it's `Sent`, or "Message failed" if it's `Failed`.
3. Call `print_status` with `MessageStatus::Sent` in `main`.

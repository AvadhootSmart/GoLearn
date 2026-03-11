# Exercise 1: Async/Await Fundamentals

## Introduction to Asynchronous Programming in Rust

Asynchronous programming allows you to write code that can perform multiple operations concurrently without blocking threads. In Rust, async programming is built on zero-cost abstractions, meaning you only pay for what you use.

### Why Async?

Textio needs to handle thousands of SMS API requests simultaneously. With synchronous code, each request would block a thread, requiring thousands of threads for thousands of concurrent requests. Async code allows handling many operations on fewer threads.

## The async Keyword

The `async` keyword transforms a function into one that returns a `Future`.

```rust
fn synchronous_send() -> Message {
    Message { id: 1, content: "Hello".to_string() }
}

async fn async_send() -> Message {
    Message { id: 1, content: "Hello".to_string() }
}
```

### What async fn Actually Returns

An `async fn` doesn't execute immediately. Instead, it returns a `Future` that will eventually produce the result.

```rust
async fn fetch_message(id: u32) -> Message {
    Message { id, content: "Hello".to_string() }
}

fn main() {
    let future = fetch_message(1);
    
    println!("Nothing has happened yet!");
    
    future.await;
}
```

### async fn vs impl Future

These two signatures are equivalent:

```rust
async fn fetch_user(id: u32) -> User {
    User { id, name: "Alice".to_string() }
}

fn fetch_user_explicit(id: u32) -> impl Future<Output = User> {
    async move {
        User { id, name: "Alice".to_string() }
    }
}
```

The explicit form is useful when you need more control over the returned type or when implementing traits.

## The .await Operator

The `.await` operator suspends execution of the current async function until the future completes.

```rust
async fn send_sms(phone: &str, message: &str) -> Result<(), Error> {
    let response = http_post("/sms/send", &format!("{{\"to\": \"{}\", \"message\": \"{}\"}}", phone, message)).await?;
    
    if response.success {
        Ok(())
    } else {
        Err(Error::SendFailed)
    }
}
```

### How .await Works

When you call `.await`:

1. The current function pauses execution
2. Control returns to the runtime (allowing other tasks to run)
3. When the awaited future completes, execution resumes
4. The result of the future is returned

### Blocking vs Non-Blocking

```rust
async fn blocking_example() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn non_blocking_example() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

The first blocks the thread entirely. The second yields control, allowing other tasks to use the thread.

## Async Blocks

You can create anonymous futures using `async { }` blocks:

```rust
fn main() {
    let future = async {
        let user = fetch_user(1).await;
        let messages = fetch_messages(user.id).await;
        messages
    };
    
    let result = future.await;
}
```

### Async Block Captures

Async blocks capture their environment:

```rust
fn create_sender(phone: String) -> impl Future<Output = Result<(), Error>> {
    async move {
        send_sms(&phone, "Hello!").await
    }
}
```

The `move` keyword ensures `phone` is moved into the async block.

## Zero-Cost Async

Rust's async is "zero-cost" because:

1. **No runtime by default**: You choose your async runtime (Tokio, async-std, smol)
2. **State machine compilation**: Async functions compile to state machines
3. **No hidden allocations**: Memory layout is known at compile time

```rust
async fn zero_cost_example() -> i32 {
    let a = compute_value().await;
    let b = another_value().await;
    a + b
}
```

This compiles to an efficient state machine with no heap allocation required for the future itself.

## Async Context

You can only use `.await` inside async functions or blocks:

```rust
fn not_async() {
    let future = fetch_message(1);
    future.await; // ERROR: await is only allowed inside async functions
}

async fn is_async() {
    let future = fetch_message(1);
    future.await; // OK
}
```

## Composing Futures

Async functions compose naturally:

```rust
async fn process_message(id: u32) -> Result<ProcessedMessage, Error> {
    let message = fetch_message(id).await?;
    let validated = validate_message(message).await?;
    let result = send_message(validated).await?;
    Ok(result)
}
```

## Error Handling in Async

Use `?` operator just like synchronous code:

```rust
async fn send_with_retry(phone: &str, message: &str) -> Result<(), Error> {
    let response = http_post("/send", message).await?;
    
    if response.status == 429 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let retry_response = http_post("/send", message).await?;
        Ok(())
    } else {
        Ok(())
    }
}
```

## Async Traits

As of Rust 1.75, you can use `async fn` in traits:

```rust
trait MessageSender {
    async fn send(&self, message: &str) -> Result<(), Error>;
}

struct SmsSender;

impl MessageSender for SmsSender {
    async fn send(&self, message: &str) -> Result<(), Error> {
        Ok(())
    }
}
```

## Common Patterns in Textio

### Sending Multiple Messages

```rust
async fn broadcast_message(phone_numbers: &[String], message: &str) {
    for phone in phone_numbers {
        match send_sms(phone, message).await {
            Ok(_) => println!("Sent to {}", phone),
            Err(e) => println!("Failed: {}", e),
        }
    }
}
```

### Chaining Operations

```rust
async fn handle_incoming_sms(from: String, body: String) -> Result<Reply, Error> {
    let user = lookup_user(&from).await?;
    let subscription = check_subscription(&user).await?;
    
    if subscription.active {
        let reply = generate_reply(&body).await?;
        Ok(reply)
    } else {
        Err(Error::SubscriptionExpired)
    }
}
```

## Key Takeaways

1. `async fn` returns a `Future`, not the actual result
2. `.await` suspends execution until the future completes
3. Async blocks create anonymous futures
4. Rust's async is zero-cost (no built-in runtime)
5. Only use `.await` inside async contexts
6. Error handling works the same with `?` operator

## Exercise Overview

In this exercise, you'll implement async functions for Textio's SMS API:

1. Convert synchronous functions to async
2. Use `.await` to chain async operations
3. Create async blocks for complex operations
4. Handle errors in async contexts

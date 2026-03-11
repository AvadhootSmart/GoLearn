# Exercise 4: Async Patterns

## Advanced Async Patterns in Rust

This exercise covers essential patterns for building robust async applications: select!, channels, timeouts, and cancellation.

## The select! Macro

`select!` waits for the first of several futures to complete:

```rust
use tokio::select;

async fn race_operations() -> String {
    select! {
        result = operation_a() => format!("A: {}", result),
        result = operation_b() => format!("B: {}", result),
    }
}
```

### How select! Works

1. All futures are polled simultaneously
2. When one completes, its branch executes
3. Other futures are dropped (cancelled)

### Pattern Matching in select!

```rust
select! {
    Ok(data) = fetch_data() => process(data),
    Err(e) = fetch_data() => handle_error(e),
    
    Some(msg) = receiver.recv() => handle_message(msg),
    None = receiver.recv() => println!("Channel closed"),
}
```

### Default and Guarded Patterns

```rust
select! {
    Some(msg) = receiver.recv() => handle(msg),
    _ = timeout(Duration::from_secs(5)) => {
        println!("Timeout!");
    }
}
```

### The else Branch

```rust
select! {
    result = operation() => process(result),
    else => {
        println!("All branches are disabled");
    }
}
```

## Async Channels

Tokio provides async-friendly channels for communication between tasks.

### mpsc - Multi-Producer, Single-Consumer

```rust
use tokio::sync::mpsc;

async fn channel_example() {
    let (tx, mut rx) = mpsc::channel(100);
    
    tokio::spawn(async move {
        tx.send(Message { id: 1, body: "Hello".to_string() }).await.unwrap();
    });
    
    while let Some(msg) = rx.recv().await {
        println!("Received: {:?}", msg);
    }
}
```

### oneshot - Single Message

```rust
use tokio::sync::oneshot;

async fn oneshot_example() {
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        tx.send("Response".to_string()).unwrap();
    });
    
    let response = rx.await.unwrap();
}
```

### broadcast - Multiple Receivers

```rust
use tokio::sync::broadcast;

async fn broadcast_example() {
    let (tx, _) = broadcast::channel(16);
    
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    
    tx.send("Broadcast message".to_string()).unwrap();
    
    println!("Rx1: {:?}", rx1.recv().await);
    println!("Rx2: {:?}", rx2.recv().await);
}
```

## Timeouts

### Basic Timeout

```rust
use tokio::time::{timeout, Duration};

async fn with_timeout() -> Result<Data, TimeoutError> {
    match timeout(Duration::from_secs(5), fetch_data()).await {
        Ok(Ok(data)) => Ok(data),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(TimeoutError),
    }
}
```

### Timeout Pattern with select!

```rust
select! {
    result = fetch_data() => {
        match result {
            Ok(data) => process(data),
            Err(e) => handle_error(e),
        }
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!("Request timed out");
    }
}
```

## Cancellation

### Implicit Cancellation (Drop)

Futures are cancelled when dropped:

```rust
let handle = tokio::spawn(async {
    loop {
        work().await;
    }
});

drop(handle); // Task is cancelled immediately
```

### Cooperative Cancellation

Use a cancellation token for graceful shutdown:

```rust
use tokio_util::sync::CancellationToken;

async fn worker(token: CancellationToken) {
    loop {
        select! {
            _ = token.cancelled() => {
                println!("Shutting down gracefully");
                break;
            }
            result = do_work() => {
                process(result);
            }
        }
    }
}
```

### Cancellation-Safe Code

Operations before `.await` complete; operations after may not:

```rust
async fn cancellation_safe() {
    let data = prepare_data(); // Always runs
    
    save_to_db(data).await;    // May be cancelled here
    
    log_success();             // May not run if cancelled
}
```

## Textio Patterns

### Message Processing with Timeout

```rust
async fn process_with_timeout(msg: Message) -> Result<DeliveryReport, Error> {
    match timeout(Duration::from_secs(30), send_sms(msg)).await {
        Ok(Ok(report)) => Ok(report),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(Error::Timeout),
    }
}
```

### Worker Pool Pattern

```rust
async fn worker_pool(messages: mpsc::Receiver<Message>, workers: usize) {
    let mut handles = Vec::new();
    
    for _ in 0..workers {
        let mut rx = messages.clone();
        let handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                process_message(msg).await;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.await;
    }
}
```

### Graceful Shutdown

```rust
async fn graceful_shutdown(
    mut receiver: mpsc::Receiver<Message>,
    shutdown: CancellationToken,
) {
    loop {
        select! {
            _ = shutdown.cancelled() => {
                while let Ok(msg) = receiver.try_recv() {
                    process_final(msg).await;
                }
                break;
            }
            msg = receiver.recv() => {
                if let Some(m) = msg {
                    process_message(m).await;
                }
            }
        }
    }
}
```

### Rate Limiting with select!

```rust
async fn rate_limited_send(
    messages: &mut mpsc::Receiver<Message>,
    rate: Duration,
) {
    let mut interval = tokio::time::interval(rate);
    
    loop {
        select! {
            Some(msg) = messages.recv() => {
                send(msg).await;
                interval.tick().await;
            }
            else => break,
        }
    }
}
```

## Common Pitfalls

### 1. Forgetting .await

```rust
let future = async_operation(); // Not started
async_operation().await;         // Started and completed
```

### 2. Blocking in select!

```rust
select! {
    _ = std::thread::sleep(Duration::from_secs(1)) => {} // WRONG!
    _ = tokio::time::sleep(Duration::from_secs(1)) => {} // Correct
}
```

### 3. Channel Buffer Too Small

```rust
let (tx, rx) = mpsc::channel(0); // Will deadlock!
let (tx, rx) = mpsc::channel(100); // Better
```

### 4. Not Handling Channel Closure

```rust
while let Some(msg) = rx.recv().await {
    process(msg);
}
// Handle channel closure here
```

## Best Practices

1. **Use select! for racing** - When you need the first result
2. **Use join! for waiting** - When you need all results
3. **Handle timeouts** - Network operations can hang
4. **Implement graceful shutdown** - Don't lose in-flight messages
5. **Choose right channel type** - mpsc, oneshot, or broadcast
6. **Be cancellation-safe** - Assume any .await can be the last

## Exercise Overview

In this exercise, you'll:

1. Use `select!` to race multiple operations
2. Implement async channels for message passing
3. Add timeouts to network operations
4. Implement graceful cancellation
5. Build a complete Textio message processor

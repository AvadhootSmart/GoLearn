# Exercise 2: Futures Deep Dive

## Understanding Futures

A `Future` is a value that represents the result of an asynchronous computation that may not be available yet. In Rust, futures are implemented as state machines.

## The Future Trait

The core of async Rust is the `Future` trait:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### The poll Method

The `poll` method is called by the runtime to check if the future is complete:

```rust
pub enum Poll<T> {
    Ready(T),   // The future has completed with a value
    Pending,    // The future is not complete yet
}
```

### Pin and Self-Referential Futures

The `Pin<&mut Self>` type ensures the future cannot be moved in memory after polling starts. This is crucial for self-referential structs:

```rust
struct SelfReferential {
    data: String,
    pointer: *const str,  // Points to data field
}
```

If this struct were moved, the pointer would become invalid. Pin prevents this.

## Implementing Future Manually

Here's a simple future that completes after being polled twice:

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDownFuture {
    count: u32,
}

impl Future for CountDownFuture {
    type Output = String;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        if self.count == 0 {
            Poll::Ready("Done!".to_string())
        } else {
            self.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```

## How Async Functions Become State Machines

Consider this async function:

```rust
async fn fetch_and_process(id: u32) -> ProcessedData {
    let raw = fetch_raw(id).await;     // Await point 1
    let validated = validate(raw).await; // Await point 2
    process(validated)                  // Final state
}
```

The compiler transforms this into a state machine with three states:

```rust
enum FetchAndProcess {
    Initial { id: u32 },
    WaitingForFetch { id: u32, fetch_future: impl Future<Output=RawData> },
    WaitingForValidate { validate_future: impl Future<Output=ValidatedData> },
    Completed,
}
```

Each `.await` creates a suspension point where the future yields control.

## Async Blocks as Futures

Async blocks create anonymous future types:

```rust
let future = async {
    let a = operation_a().await;
    let b = operation_b().await;
    a + b
};
```

The compiler generates a unique type for each async block.

## The Waker

When a future returns `Poll::Pending`, it must arrange to wake the task when it can make progress:

```rust
struct DelayFuture {
    delay: Duration,
    started: Option<Instant>,
    waker: Option<Waker>,
}

impl Future for DelayFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Some(started) = self.started {
            if started.elapsed() >= self.delay {
                return Poll::Ready(());
            }
        } else {
            self.started = Some(Instant::now());
        }
        
        let waker = cx.waker().clone();
        let delay = self.delay;
        std::thread::spawn(move || {
            std::thread::sleep(delay);
            waker.wake();
        });
        
        Poll::Pending
    }
}
```

## State Machine Size

The size of a future depends on all values it captures:

```rust
async fn example() {
    let a = [0u8; 1000];  // 1000 bytes
    let b = [0u8; 2000];  // 2000 bytes
    let _ = async {
        drop(a);
        drop(b);
    }.await;
}
```

The generated state machine must store all live values at each await point.

## Combining Futures

Futures can be combined without awaiting each individually:

```rust
use std::future::join;

async fn fetch_all() -> (User, Messages, Settings) {
    let user_future = fetch_user();
    let messages_future = fetch_messages();
    let settings_future = fetch_settings();
    
    join!(user_future, messages_future, settings_future)
}
```

## Future Traits and Bounds

Common bounds for futures:

```rust
fn execute<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
{
    runtime.block_on(future)
}
```

### Send Bound

A future is `Send` if all its captures are `Send`:

```rust
async fn send_example() {
    let data = vec
![1, 2, 3];
    async { data }.await;  // Send-safe
}

async fn not_send_example(rc: Rc<i32>) {
    async { *rc }.await;   // Not Send-safe (Rc is not Send)
}
```

## Async in Traits

```rust
trait AsyncProcessor {
    async fn process(&self, data: Data) -> Result<(), Error>;
}

struct MessageProcessor;

impl AsyncProcessor for MessageProcessor {
    async fn process(&self, data: Data) -> Result<(), Error> {
        Ok(())
    }
}
```

## Textio Example: Message Pipeline

```rust
struct MessagePipeline {
    fetcher: FetchFuture,
    validator: Option<ValidateFuture>,
    sender: Option<SendFuture>,
}

impl Future for MessagePipeline {
    type Output = Result<DeliveryReport, Error>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(ref mut sender) = self.sender {
                match sender.as_mut().poll(cx) {
                    Poll::Ready(result) => return Poll::Ready(result),
                    Poll::Pending => return Poll::Pending,
                }
            }
            
            if let Some(ref mut validator) = self.validator {
                match validator.as_mut().poll(cx) {
                    Poll::Ready(Ok(validated)) => {
                        self.sender = Some(send(validated));
                        continue;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Pending => return Poll::Pending,
                }
            }
            
            match self.fetcher.as_mut().poll(cx) {
                Poll::Ready(message) => {
                    self.validator = Some(validate(message));
                    continue;
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
```

## Key Concepts

1. **Poll-based**: Futures are polled by the runtime
2. **State machines**: Async functions compile to state machines
3. **Pin**: Ensures futures aren't moved after polling starts
4. **Waker**: Mechanism for futures to signal readiness
5. **Zero-cost**: No hidden allocations or runtime overhead
6. **Composability**: Futures naturally chain and combine

## Exercise Overview

In this exercise, you'll:

1. Implement a custom `Future` from scratch
2. Understand the `poll` method and `Poll` enum
3. Use `Pin` and `Context` correctly
4. Build a state machine for Textio's message pipeline

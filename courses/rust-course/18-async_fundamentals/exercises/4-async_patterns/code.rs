use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u32,
    pub recipient: String,
    pub body: String,
}

#[derive(Debug)]
pub enum WorkerCommand {
    Send(Message),
    Shutdown,
}

#[derive(Debug)]
pub struct DeliveryResult {
    pub message_id: u32,
    pub success: bool,
}

pub async fn send_with_timeout(message: Message, timeout_duration: Duration) -> Result<DeliveryResult, String> {
    match tokio::time::timeout(timeout_duration, simulate_send(message.clone())).await {
        Ok(result) => Ok(result),
        Err(_) => Err(format!("Message {} timed out", message.id)),
    }
}

async fn simulate_send(message: Message) -> DeliveryResult {
    tokio::time::sleep(Duration::from_millis(50)).await;
    DeliveryResult {
        message_id: message.id,
        success: true,
    }
}

pub async fn race_first<F1, F2>(future1: F1, future2: F2) -> &'static str
where
    F1: std::future::Future,
    F2: std::future::Future,
{
    tokio::select! {
        _ = future1 => "First completed",
        _ = future2 => "Second completed",
    }
}

pub async fn worker(
    mut receiver: mpsc::Receiver<WorkerCommand>,
    results: mpsc::Sender<DeliveryResult>,
) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            WorkerCommand::Send(msg) => {
                let result = simulate_send(msg).await;
                let _ = results.send(result).await;
            }
            WorkerCommand::Shutdown => {
                println!("   Worker shutting down gracefully");
                break;
            }
        }
    }
}

pub async fn message_processor(
    mut incoming: mpsc::Receiver<Message>,
    results: mpsc::Sender<DeliveryResult>,
    timeout_duration: Duration,
) {
    while let Some(message) = incoming.recv().await {
        match send_with_timeout(message.clone(), timeout_duration).await {
            Ok(result) => {
                let _ = results.send(result).await;
            }
            Err(e) => {
                println!("   Error: {}", e);
                let _ = results.send(DeliveryResult {
                    message_id: message.id,
                    success: false,
                }).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Async Patterns for Textio ===\n");
    
    println!("1. Timeout Pattern:");
    let message = Message { id: 1, recipient: "+1555001".to_string(), body: "Test".to_string() };
    let result = send_with_timeout(message, Duration::from_millis(100)).await;
    println!("   Result: {:?}\n", result);
    
    println!("2. select! - Racing Futures:");
    let slow = tokio::time::sleep(Duration::from_millis(100));
    let fast = tokio::time::sleep(Duration::from_millis(50));
    let winner = tokio::select! {
        _ = slow => "slow",
        _ = fast => "fast",
    };
    println!("   Winner: {}\n", winner);
    
    println!("3. Async Channels:");
    let (cmd_tx, cmd_rx) = mpsc::channel::<WorkerCommand>(10);
    let (result_tx, mut result_rx) = mpsc::channel::<DeliveryResult>(10);
    
    let worker_handle = tokio::spawn(worker(cmd_rx, result_tx));
    
    cmd_tx.send(WorkerCommand::Send(Message {
        id: 1,
        recipient: "+1555001".to_string(),
        body: "Hello".to_string(),
    })).await.unwrap();
    
    cmd_tx.send(WorkerCommand::Send(Message {
        id: 2,
        recipient: "+1555002".to_string(),
        body: "World".to_string(),
    })).await.unwrap();
    
    cmd_tx.send(WorkerCommand::Shutdown).await.unwrap();
    
    worker_handle.await.unwrap();
    
    while let Ok(result) = result_rx.try_recv() {
        println!("   Delivered message {}: {}", result.message_id, result.success);
    }
    println!();
    
    println!("4. Cancellation with select!:");
    let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(1);
    
    let task = tokio::spawn(async move {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("   Task completed normally");
            }
            _ = cancel_rx.recv() => {
                println!("   Task cancelled!");
            }
        }
    });
    
    tokio::time::sleep(Duration::from_millis(10)).await;
    cancel_tx.send(()).await.unwrap();
    task.await.unwrap();
    
    println!("\n=== Async patterns demonstrated ===");
}

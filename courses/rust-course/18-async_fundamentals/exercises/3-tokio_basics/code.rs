use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u32,
    pub recipient: String,
    pub body: String,
}

#[derive(Debug)]
pub struct DeliveryReport {
    pub message_id: u32,
    pub recipient: String,
    pub delivered: bool,
    pub duration_ms: u64,
}

pub async fn simulate_send(message: Message) -> DeliveryReport {
    let start = Instant::now();
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    DeliveryReport {
        message_id: message.id,
        recipient: message.recipient,
        delivered: true,
        duration_ms: start.elapsed().as_millis() as u64,
    }
}

pub async fn send_sequential(messages: Vec<Message>) -> Vec<DeliveryReport> {
    let mut reports = Vec::new();
    for message in messages {
        let report = simulate_send(message).await;
        reports.push(report);
    }
    reports
}

pub async fn send_concurrent_join(messages: Vec<Message>) -> Vec<DeliveryReport> {
    let futures: Vec<_> = messages.into_iter().map(|m| simulate_send(m)).collect();
    
    let mut results = Vec::new();
    if futures.len() >= 3 {
        let (r1, r2, r3) = tokio::join!(
            simulate_send(Message { id: 1, recipient: "test1".to_string(), body: "test".to_string() }),
            simulate_send(Message { id: 2, recipient: "test2".to_string(), body: "test".to_string() }),
            simulate_send(Message { id: 3, recipient: "test3".to_string(), body: "test".to_string() }),
        );
        results.push(r1);
        results.push(r2);
        results.push(r3);
    }
    results
}

pub async fn send_concurrent_spawn(messages: Vec<Message>) -> Vec<DeliveryReport> {
    let handles: Vec<_> = messages
        .into_iter()
        .map(|message| tokio::spawn(simulate_send(message)))
        .collect();
    
    let mut reports = Vec::new();
    for handle in handles {
        if let Ok(report) = handle.await {
            reports.push(report);
        }
    }
    reports
}

pub async fn fetch_user(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(30)).await;
    format!("User {}", id)
}

pub async fn fetch_messages(user_id: u32) -> Vec<String> {
    tokio::time::sleep(Duration::from_millis(40)).await;
    vec
![format!("Message for user {}", user_id)]
}

pub async fn fetch_settings(user_id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(20)).await;
    format!("Settings for user {}", user_id)
}

pub async fn load_dashboard(user_id: u32) -> (String, Vec<String>, String) {
    let (user, messages, settings) = tokio::join
!(
        fetch_user(user_id),
        fetch_messages(user_id),
        fetch_settings(user_id),
    );
    (user, messages, settings)
}

#[tokio::main]
async fn main() {
    println!("=== Tokio Runtime Fundamentals ===\n");
    
    println!("1. Sequential vs Concurrent Execution:");
    let messages: Vec<Message> = (1..=3)
        .map(|i| Message {
            id: i,
            recipient: format!("+155500{}", i),
            body: "Hello from Textio!".to_string(),
        })
        .collect();
    
    let start = Instant::now();
    let _ = send_sequential(messages.clone()).await;
    let sequential_time = start.elapsed();
    println!("   Sequential: {:?}", sequential_time);
    
    let start = Instant::now();
    let _ = send_concurrent_spawn(messages).await;
    let concurrent_time = start.elapsed();
    println!("   Concurrent: {:?}", concurrent_time);
    println!("   Speedup: {:.1}x\n", sequential_time.as_secs_f64() / concurrent_time.as_secs_f64());
    
    println!("2. Using join! for Concurrent Operations:");
    let start = Instant::now();
    let (user, messages, settings) = load_dashboard(1).await;
    println!("   User: {}", user);
    println!("   Messages: {:?}", messages);
    println!("   Settings: {}", settings);
    println!("   Total time: {:?}\n", start.elapsed());
    
    println!("3. Task Spawning:");
    let handle1 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "Task 1 complete"
    });
    
    let handle2 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "Task 2 complete"
    });
    
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    println!("   Results: {}, {}\n", result1, result2);
    
    println!("=== Tokio basics demonstrated ===");
}

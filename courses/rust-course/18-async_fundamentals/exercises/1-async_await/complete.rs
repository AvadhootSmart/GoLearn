use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: u32,
    pub to: String,
    pub body: String,
    pub status: MessageStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

#[derive(Debug)]
pub struct DeliveryReport {
    pub message_id: u32,
    pub delivered: bool,
    pub timestamp: u64,
}

impl std::future::Future for Message {
    type Output = Self;
    
    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(self.clone())
    }
}

pub fn simulate_network_delay() {
    std::thread::sleep(Duration::from_millis(10));
}

pub async fn create_message(to: String, body: String) -> Message {
    simulate_network_delay();
    Message {
        id: rand_id(),
        to,
        body,
        status: MessageStatus::Pending,
    }
}

pub async fn send_message(mut message: Message) -> Message {
    simulate_network_delay();
    message.status = MessageStatus::Sent;
    message
}

pub async fn confirm_delivery(message: Message) -> DeliveryReport {
    simulate_network_delay();
    DeliveryReport {
        message_id: message.id,
        delivered: true,
        timestamp: current_timestamp(),
    }
}

pub async fn send_sms(to: &str, body: &str) -> Result<DeliveryReport, String> {
    let message = create_message(to.to_string(), body.to_string()).await;
    let sent = send_message(message).await;
    let report = confirm_delivery(sent).await;
    Ok(report)
}

pub async fn broadcast_sms(recipients: &[&str], body: &str) -> Vec<Result<DeliveryReport, String>> {
    let mut results = Vec::new();
    for recipient in recipients {
        let result = send_sms(recipient, body).await;
        results.push(result);
    }
    results
}

pub async fn send_with_callback<F>(to: String, body: String, callback: F) -> DeliveryReport 
where 
    F: FnOnce(&DeliveryReport) + Send,
{
    let result = send_sms(&to, &body).await.unwrap();
    callback(&result);
    result
}

pub fn rand_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("=== Textio Async SMS API ===\n");
        
        println!("1. Sending single SMS:");
        let report = send_sms("+1555001", "Hello from Textio!").await.unwrap();
        println!("   Message {} delivered: {}\n", report.message_id, report.delivered);
        
        println!("2. Broadcasting to multiple recipients:");
        let recipients = ["+1555002", "+1555003", "+1555004"];
        let results = broadcast_sms(&recipients, "Group message").await;
        println!("   Sent {} messages\n", results.len());
        
        println!("3. Send with callback:");
        let report = send_with_callback(
            "+1555005".to_string(),
            "Callback test".to_string(),
            |r| println!("   Callback: Message {} delivered!", r.message_id),
        ).await;
        println!("   Final status: delivered={}\n", report.delivered);
        
        println!("=== All operations completed ===");
    });
}

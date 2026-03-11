// Exercise 1: Iterator Basics - Complete Solution

#[derive(Debug, Clone)]
struct Message {
    id: u32,
    content: String,
    recipient: String,
    sent: bool,
}

impl Message {
    fn new(id: u32, content: &str, recipient: &str) -> Self {
        Message {
            id,
            content: content.to_string(),
            recipient: recipient.to_string(),
            sent: false,
        }
    }
}

fn main() {
    println!("=== Part 1: iter() - Immutable References ===\n");
    
    let messages = vec![
        Message::new(1, "Hello there!", "Alice"),
        Message::new(2, "Meeting at 3pm", "Bob"),
        Message::new(3, "Don't forget the report", "Charlie"),
    ];
    
    let message_count = messages.iter().count();
    println!("Total messages: {}", message_count);
    
    let bob_message: Option<&Message> = messages.iter().find(|m| m.recipient == "Bob");
    println!("Message to Bob: {:?}", bob_message);
    
    println!("Messages still accessible: {} messages\n", messages.len());
    
    println!("=== Part 2: iter_mut() - Mutable References ===\n");
    
    let mut status_messages = vec![
        Message::new(101, "Order shipped", "Customer1"),
        Message::new(102, "Payment received", "Customer2"),
        Message::new(103, "Delivery confirmed", "Customer3"),
    ];
    
    for msg in status_messages.iter_mut() {
        msg.sent = true;
    }
    
    println!("Marked messages as sent:");
    for msg in status_messages.iter() {
        println!("  ID {}: sent = {}", msg.id, msg.sent);
    }
    
    println!("\n=== Part 3: into_iter() - Ownership Transfer ===\n");
    
    let owned_messages = vec![
        Message::new(201, "Welcome!", "NewUser1"),
        Message::new(202, "Verify email", "NewUser2"),
    ];
    
    let message_ids: Vec<u32> = owned_messages.into_iter().map(|m| m.id).collect();
    println!("Extracted message IDs: {:?}", message_ids);
    
    println!("\n=== Part 4: Consuming Adaptors ===\n");
    
    let numbers = vec![10, 20, 30, 40, 50];
    
    let total: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", total);
    
    let has_large = numbers.iter().any(|n| *n > 25);
    println!("Has number > 25: {}", has_large);
    
    let all_positive = numbers.iter().all(|n| *n > 0);
    println!("All positive: {}", all_positive);
    
    let position: Option<usize> = numbers.iter().position(|n| *n == 30);
    println!("Position of 30: {:?}", position);
    
    println!("\n=== Part 5: Textio Message Analysis ===\n");
    
    let sms_queue = vec![
        Message::new(1, "Your code is 1234", "+15550001"),
        Message::new(2, "Your code is 5678", "+15550002"),
        Message::new(3, "Delivery scheduled", "+15550001"),
        Message::new(4, "Your code is 9999", "+15550003"),
        Message::new(5, "Promo: 50% off!", "+15550001"),
    ];
    
    let code_messages = sms_queue.iter().filter(|m| m.content.contains("code")).count();
    println!("Messages with verification codes: {}", code_messages);
    
    let recipients: Vec<&String> = sms_queue.iter().map(|m| &m.recipient).collect();
    println!("All recipients: {:?}", recipients);
    
    let first_to_001: Option<&Message> = sms_queue.iter().find(|m| m.recipient == "+15550001");
    println!("First message to +15550001: {:?}", first_to_001);
}

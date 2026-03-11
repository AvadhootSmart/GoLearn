// Exercise 1: Iterator Basics
// Learn the three ways to create iterators and consuming adaptors

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
    
    // TODO: Use iter() to count messages
    // Hint: messages.iter().count()
    let message_count = 0; // Replace with your code
    println!("Total messages: {}", message_count);
    
    // TODO: Use iter() to find the first message to "Bob"
    // Hint: use find() with a closure |m| m.recipient == "Bob"
    let bob_message: Option<&Message> = None; // Replace with your code
    println!("Message to Bob: {:?}", bob_message);
    
    // messages is still valid here because iter() only borrows
    println!("Messages still accessible: {} messages\n", messages.len());
    
    println!("=== Part 2: iter_mut() - Mutable References ===\n");
    
    let mut status_messages = vec![
        Message::new(101, "Order shipped", "Customer1"),
        Message::new(102, "Payment received", "Customer2"),
        Message::new(103, "Delivery confirmed", "Customer3"),
    ];
    
    // TODO: Use iter_mut() to mark all messages as sent
    // Hint: for msg in status_messages.iter_mut() { msg.sent = true; }
    
    println!("Marked messages as sent:");
    // TODO: Print all messages with their sent status
    // Use iter() and a for loop
    
    println!("\n=== Part 3: into_iter() - Ownership Transfer ===\n");
    
    let owned_messages = vec![
        Message::new(201, "Welcome!", "NewUser1"),
        Message::new(202, "Verify email", "NewUser2"),
    ];
    
    // TODO: Use into_iter() to process messages and collect IDs
    // Hint: let ids: Vec<u32> = owned_messages.into_iter().map(|m| m.id).collect();
    let message_ids: Vec<u32> = vec![]; // Replace with your code
    println!("Extracted message IDs: {:?}", message_ids);
    
    // TODO: Uncommenting this would cause an error - why?
    // println!("{:?}", owned_messages);
    
    println!("\n=== Part 4: Consuming Adaptors ===\n");
    
    let numbers = vec![10, 20, 30, 40, 50];
    
    // TODO: Calculate the sum using iter() and sum()
    let total: i32 = 0; // Replace with your code
    println!("Sum of numbers: {}", total);
    
    // TODO: Check if any number is greater than 25 using any()
    let has_large = false; // Replace with your code
    println!("Has number > 25: {}", has_large);
    
    // TODO: Check if all numbers are positive using all()
    let all_positive = false; // Replace with your code
    println!("All positive: {}", all_positive);
    
    // TODO: Find the position of the first number equal to 30
    let position: Option<usize> = None; // Replace with your code
    println!("Position of 30: {:?}", position);
    
    println!("\n=== Part 5: Textio Message Analysis ===\n");
    
    let sms_queue = vec![
        Message::new(1, "Your code is 1234", "+15550001"),
        Message::new(2, "Your code is 5678", "+15550002"),
        Message::new(3, "Delivery scheduled", "+15550001"),
        Message::new(4, "Your code is 9999", "+15550003"),
        Message::new(5, "Promo: 50% off!", "+15550001"),
    ];
    
    // TODO: Count messages containing "code"
    let code_messages = 0; // Replace: use filter() and count()
    println!("Messages with verification codes: {}", code_messages);
    
    // TODO: Collect all recipients into a Vec
    let recipients: Vec<&String> = vec![]; // Replace with your code
    println!("All recipients: {:?}", recipients);
    
    // TODO: Find first message to "+15550001"
    let first_to_001: Option<&Message> = None; // Replace with your code
    println!("First message to +15550001: {:?}", first_to_001);
}

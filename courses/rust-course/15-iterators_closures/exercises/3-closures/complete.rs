// Exercise 3: Closures - Complete Solution

#[derive(Debug, Clone)]
struct Message {
    id: u32,
    content: String,
    recipient: String,
    status: MessageStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

impl Message {
    fn new(id: u32, content: &str, recipient: &str) -> Self {
        Message {
            id,
            content: content.to_string(),
            recipient: recipient.to_string(),
            status: MessageStatus::Pending,
        }
    }
    
    fn mark_sent(&mut self) {
        self.status = MessageStatus::Sent;
    }
}

fn filter_messages<F>(messages: Vec<Message>, predicate: F) -> Vec<Message>
where
    F: Fn(&Message) -> bool,
{
    messages.into_iter().filter(predicate).collect()
}

fn transform_messages<F>(messages: Vec<Message>, transformer: F) -> Vec<String>
where
    F: Fn(Message) -> String,
{
    messages.into_iter().map(transformer).collect()
}

fn process_with_counter<F>(messages: Vec<Message>, mut processor: F) -> (Vec<Message>, usize)
where
    F: FnMut(&Message) -> bool,
{
    let mut count = 0;
    let matched: Vec<Message> = messages
        .into_iter()
        .filter(|m| {
            let result = processor(m);
            if result {
                count += 1;
            }
            result
        })
        .collect();
    (matched, count)
}

fn main() {
    println!("=== Part 1: Basic Closure Syntax ===\n");
    
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));
    
    let concat = |a: &str, b: &str| format!("{}{}", a, b);
    println!("concat: {}", concat("Hello, ", "World!"));
    
    let describe = |n: i32| {
        if n > 0 {
            "positive".to_string()
        } else if n < 0 {
            "negative".to_string()
        } else {
            "zero".to_string()
        }
    };
    println!("5 is {}", describe(5));
    println!("-3 is {}", describe(-3));
    println!("0 is {}", describe(0));
    
    println!("\n=== Part 2: Type Inference ===\n");
    
    let multiply = |x, y| x * y;
    let product = multiply(4, 5);
    println!("4 * 5 = {}", product);
    
    let float_multiply = |x, y| x * y;
    let float_product = float_multiply(2.5, 4.0);
    println!("2.5 * 4.0 = {}", float_product);
    
    println!("\n=== Part 3: Capturing the Environment ===\n");
    
    let prefix = "[TEXTIO] ";
    let max_length = 20;
    
    let add_prefix = |msg: &str| format!("{}{}", prefix, msg);
    println!("With prefix: {}", add_prefix("Hello"));
    
    let is_valid_length = |msg: &str| msg.len() <= max_length;
    println!("'Short' is valid: {}", is_valid_length("Short"));
    println!("'This is a very long message' is valid: {}", is_valid_length("This is a very long message"));
    
    println!("\n=== Part 4: Closures with Iterators ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let greater_than_five: Vec<i32> = numbers.iter().filter(|n| **n > 5).copied().collect();
    println!("Numbers > 5: {:?}", greater_than_five);
    
    let formatted: Vec<String> = numbers.iter().map(|n| format!("Num: {}", n)).collect();
    println!("Formatted (first 3): {:?}", &formatted[..3]);
    
    let has_seven = numbers.iter().any(|n| *n == 7);
    println!("Has 7: {}", has_seven);
    
    println!("\n=== Part 5: Closures as Function Parameters ===\n");
    
    let messages = vec![
        Message::new(1, "Hello Alice", "+15550001"),
        Message::new(2, "Meeting at 3pm", "+15550002"),
        Message::new(3, "Your code is 1234", "+15550001"),
        Message::new(4, "Don't forget!", "+15550003"),
        Message::new(5, "Hello Bob", "+15550001"),
    ];
    
    let to_001: Vec<Message> = filter_messages(messages.clone(), |m| m.recipient == "+15550001");
    println!("Messages to +15550001:");
    for msg in &to_001 {
        println!("  {:?}", msg);
    }
    
    let summaries: Vec<String> = transform_messages(messages, |m| format!("[{}] -> {}: {}", m.id, m.recipient, m.content));
    println!("\nMessage summaries:");
    for summary in &summaries {
        println!("  {}", summary);
    }
    
    println!("\n=== Part 6: FnMut - Mutable Captures ===\n");
    
    let mut counter = 0;
    
    let mut count_and_check = |_: &Message| {
        counter += 1;
        counter % 2 == 0
    };
    
    let test_messages = vec![
        Message::new(1, "A", "+1"),
        Message::new(2, "B", "+2"),
        Message::new(3, "C", "+3"),
    ];
    
    let (matched, count) = process_with_counter(test_messages, &mut count_and_check);
    println!("Matched {} messages out of {}", count, count);
    
    println!("\n=== Part 7: Textio Message Router ===\n");
    
    let incoming = vec![
        Message::new(101, "Verify your account", "+15550001"),
        Message::new(102, "Your order shipped", "+15550002"),
        Message::new(103, "Meeting reminder", "+15550003"),
        Message::new(104, "Your code: 9999", "+15550001"),
        Message::new(105, "Delivery tomorrow", "+15550002"),
    ];
    
    let priority_recipients = vec!["+15550001", "+15550003"];
    
    let priority_messages: Vec<Message> = filter_messages(incoming.clone(), |m| {
        priority_recipients.contains(&m.recipient.as_str())
    });
    println!("Priority messages:");
    for msg in &priority_messages {
        println!("  ID {} to {}", msg.id, msg.recipient);
    }
    
    let verification_count = incoming.iter()
        .filter(|m| m.content.to_lowercase().contains("code") || m.content.to_lowercase().contains("verify"))
        .count();
    println!("\nVerification-related messages: {}", verification_count);
}

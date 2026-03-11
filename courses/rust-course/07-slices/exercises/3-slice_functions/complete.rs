// Textio SMS API - Slice Functions Exercise
// Complete the functions below to process message analytics using flexible slice parameters

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DeliveryStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

#[derive(Debug, Clone)]
struct SmsMessage {
    id: u32,
    recipient: String,
    content: String,
    status: DeliveryStatus,
}

impl SmsMessage {
    fn new(id: u32, recipient: &str, content: &str, status: DeliveryStatus) -> Self {
        SmsMessage {
            id,
            recipient: recipient.to_string(),
            content: content.to_string(),
            status,
        }
    }
    
    fn content_length(&self) -> usize {
        self.content.len()
    }
}

fn main() {
    println!("=== Textio Slice Functions Exercise ===\n");
    
    let messages = vec![
        SmsMessage::new(1, "+1-555-0100", "Hi", DeliveryStatus::Delivered),
        SmsMessage::new(2, "+1-555-0101", "Hello there!", DeliveryStatus::Sent),
        SmsMessage::new(3, "+1-555-0102", "Your verification code is 123456", DeliveryStatus::Delivered),
        SmsMessage::new(4, "+1-555-0103", "OK", DeliveryStatus::Failed),
        SmsMessage::new(5, "+44-555-0104", "Welcome to Textio!", DeliveryStatus::Pending),
        SmsMessage::new(6, "+44-555-0105", "Thanks!", DeliveryStatus::Delivered),
        SmsMessage::new(7, "+1-555-0106", "Your order shipped", DeliveryStatus::Delivered),
        SmsMessage::new(8, "+44-555-0107", "Delivery failed", DeliveryStatus::Failed),
    ];
    
    println!("Test 1: Count by Status");
    let counts = count_by_status(&messages);
    println!("Status counts:");
    for (status, count) in &counts {
        println!("  {:?}: {}", status, count);
    }
    println!();
    
    println!("Test 2: Filter Short Messages");
    let short = filter_short_messages(&messages, 10);
    println!("Messages shorter than 10 chars:");
    for msg in &short {
        println!("  ID {}: \"{}\" ({} chars)", msg.id, msg.content, msg.content_length());
    }
    println!("Found {} short messages", short.len());
    println!();
    
    println!("Test 3: Find Longest Message");
    match find_longest_message(&messages) {
        Some(msg) => println!("Longest: ID {} - \"{}\" ({} chars)", 
            msg.id, msg.content, msg.content_length()),
        None => println!("No messages found"),
    }
    println!();
    
    println!("Test 4: Group by Recipient Prefix");
    let groups = group_by_recipient(&messages);
    for (prefix, msgs) in &groups {
        println!("Prefix {}: {} messages", prefix, msgs.len());
        for msg in msgs {
            println!("  ID {} to {}", msg.id, msg.recipient);
        }
    }
    println!();
    
    demonstrate_function_flexibility();
}

fn count_by_status(messages: &[SmsMessage]) -> HashMap<DeliveryStatus, usize> {
    let mut counts = HashMap::new();
    for msg in messages {
        *counts.entry(msg.status.clone()).or_insert(0) += 1;
    }
    counts
}

fn filter_short_messages(messages: &[SmsMessage], max_length: usize) -> Vec<&SmsMessage> {
    messages
        .iter()
        .filter(|m| m.content_length() < max_length)
        .collect()
}

fn find_longest_message(messages: &[SmsMessage]) -> Option<&SmsMessage> {
    messages.iter().max_by_key(|m| m.content_length())
}

fn group_by_recipient(messages: &[SmsMessage]) -> HashMap<String, Vec<&SmsMessage>> {
    let mut groups: HashMap<String, Vec<&SmsMessage>> = HashMap::new();
    for msg in messages {
        let prefix = if msg.recipient.len() >= 2 {
            &msg.recipient[..2]
        } else {
            &msg.recipient[..]
        };
        groups.entry(prefix.to_string()).or_default().push(msg);
    }
    groups
}

fn demonstrate_function_flexibility() {
    println!("Test 5: Function Flexibility");
    
    let array = [1, 2, 3, 4, 5];
    let vector = vec![10, 20, 30, 40, 50];
    
    println!("Sum of array: {}", calculate_sum(&array));
    println!("Sum of vector: {}", calculate_sum(&vector));
    println!("Sum of slice: {}", calculate_sum(&vector[1..4]));
    
    let strings_arr = ["hello", "world"];
    let strings_vec = vec!["foo".to_string(), "bar".to_string()];
    
    println!("Longest in array: {:?}", find_longest_str(&strings_arr));
    println!("Longest in vector: {:?}", find_longest_str(&strings_vec));
    
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Before sort: {:?}", numbers);
    sort_slice(&mut numbers);
    println!("After sort: {:?}", numbers);
}

fn calculate_sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn find_longest_str(strings: &[impl AsRef<str>]) -> Option<&str> {
    strings
        .iter()
        .map(|s| s.as_ref())
        .max_by_key(|s| s.len())
}

fn sort_slice(numbers: &mut [i32]) {
    numbers.sort();
}

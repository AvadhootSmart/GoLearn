// Textio SMS API - Array Slices Exercise
// Complete the functions below to process message queues using array slices

#[derive(Debug, Clone)]
struct SmsMessage {
    id: u32,
    recipient: String,
    content: String,
}

impl SmsMessage {
    fn new(id: u32, recipient: &str, content: &str) -> Self {
        SmsMessage {
            id,
            recipient: recipient.to_string(),
            content: content.to_string(),
        }
    }
    
    fn content_length(&self) -> usize {
        self.content.len()
    }
}

fn main() {
    println!("=== Textio Array Slices Exercise ===\n");
    
    let messages = vec![
        SmsMessage::new(1, "+1-555-0100", "Hello from Textio!"),
        SmsMessage::new(2, "+1-555-0101", "Your code is 12345"),
        SmsMessage::new(3, "+1-555-0102", "Delivery confirmed"),
        SmsMessage::new(4, "+1-555-0103", "Meeting at 3pm"),
        SmsMessage::new(5, "+1-555-0104", "Thanks for subscribing!"),
        SmsMessage::new(6, "+1-555-0105", "Your balance is $100"),
    ];
    
    println!("Test 1: Get Recent Messages");
    println!("All messages: {} total", messages.len());
    let recent = get_recent_messages(&messages, 3);
    println!("Last 3 messages:");
    for msg in recent {
        println!("  ID {}: to {}", msg.id, msg.recipient);
    }
    println!();
    
    println!("Test 2: Average Message Length");
    let avg = calculate_average_length(&messages);
    println!("Average content length: {:.2} characters", avg);
    
    let empty: Vec<SmsMessage> = vec![];
    let empty_avg = calculate_average_length(&empty);
    println!("Empty slice average: {:.2}", empty_avg);
    println!();
    
    println!("Test 3: Find Message by ID");
    let found = find_message_by_id(&messages, 3);
    println!("Found ID 3: {:?}", found.map(|m| &m.content));
    let not_found = find_message_by_id(&messages, 99);
    println!("Found ID 99: {:?}", not_found.map(|m| &m.content));
    println!();
    
    println!("Test 4: Partition Messages");
    let (first_half, second_half) = partition_messages(&messages, 3);
    println!("First half ({} messages):", first_half.len());
    for msg in first_half {
        println!("  ID {}", msg.id);
    }
    println!("Second half ({} messages):", second_half.len());
    for msg in second_half {
        println!("  ID {}", msg.id);
    }
    println!();
    
    demonstrate_slice_flexibility();
}

fn get_recent_messages(messages: &[SmsMessage], n: usize) -> &[SmsMessage] {
    let start = if n >= messages.len() { 0 } else { messages.len() - n };
    &messages[start..]
}

fn calculate_average_length(messages: &[SmsMessage]) -> f64 {
    if messages.is_empty() {
        return 0.0;
    }
    let total: usize = messages.iter().map(|m| m.content_length()).sum();
    total as f64 / messages.len() as f64
}

fn find_message_by_id(messages: &[SmsMessage], id: u32) -> Option<&SmsMessage> {
    messages.iter().find(|m| m.id == id)
}

fn partition_messages(messages: &[SmsMessage], mid: usize) -> (&[SmsMessage], &[SmsMessage]) {
    messages.split_at(mid.min(messages.len()))
}

fn demonstrate_slice_flexibility() {
    println!("Test 5: Slice Flexibility");
    
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_slice: &[i32] = &arr[1..3];
    println!("Array slice: {:?}", arr_slice);
    
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let vec_slice: &[i32] = &vec[2..];
    println!("Vector slice: {:?}", vec_slice);
    
    let vec2 = vec![100, 200, 300];
    println!("as_slice(): {:?}", vec2.as_slice());
    
    let mut data = [5, 2, 8, 1, 9];
    let mutable_slice = &mut data[..];
    mutable_slice.sort();
    println!("Sorted mutable slice: {:?}", data);
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("Chunks of 2:");
    for chunk in numbers.chunks(2) {
        println!("  {:?}", chunk);
    }
    
    println!("Windows of 3:");
    for window in numbers.windows(3) {
        println!("  {:?}", window);
    }
}

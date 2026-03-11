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
    
    // Create test messages
    let messages = vec![
        SmsMessage::new(1, "+1-555-0100", "Hello from Textio!"),
        SmsMessage::new(2, "+1-555-0101", "Your code is 12345"),
        SmsMessage::new(3, "+1-555-0102", "Delivery confirmed"),
        SmsMessage::new(4, "+1-555-0103", "Meeting at 3pm"),
        SmsMessage::new(5, "+1-555-0104", "Thanks for subscribing!"),
        SmsMessage::new(6, "+1-555-0105", "Your balance is $100"),
    ];
    
    // Test 1: Get recent messages
    println!("Test 1: Get Recent Messages");
    println!("All messages: {} total", messages.len());
    let recent = get_recent_messages(&messages, 3);
    println!("Last 3 messages:");
    for msg in recent {
        println!("  ID {}: to {}", msg.id, msg.recipient);
    }
    println!();
    
    // Test 2: Calculate average length
    println!("Test 2: Average Message Length");
    let avg = calculate_average_length(&messages);
    println!("Average content length: {:.2} characters", avg);
    
    // Test with empty slice
    let empty: Vec<SmsMessage> = vec![];
    let empty_avg = calculate_average_length(&empty);
    println!("Empty slice average: {:.2}", empty_avg);
    println!();
    
    // Test 3: Find message by ID
    println!("Test 3: Find Message by ID");
    let found = find_message_by_id(&messages, 3);
    println!("Found ID 3: {:?}", found.map(|m| &m.content));
    let not_found = find_message_by_id(&messages, 99);
    println!("Found ID 99: {:?}", not_found.map(|m| &m.content));
    println!();
    
    // Test 4: Partition messages
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
    
    // Test 5: Demonstrate slice flexibility
    demonstrate_slice_flexibility();
}

/// Get the last n messages from the queue
/// Returns fewer messages if n > total messages
fn get_recent_messages(messages: &[SmsMessage], n: usize) -> &[SmsMessage] {
    // TODO: Return a slice containing the last n messages
    // Hint: Handle the case where n > messages.len()
    
    &messages[..0]
}

/// Calculate the average content length of all messages
/// Returns 0.0 for empty slices
fn calculate_average_length(messages: &[SmsMessage]) -> f64 {
    // TODO: Calculate the average of all message content lengths
    // Hint: Use iter() and content_length()
    
    0.0
}

/// Find a message by its ID
/// Returns a reference to the message if found
fn find_message_by_id(messages: &[SmsMessage], id: u32) -> Option<&SmsMessage> {
    // TODO: Find and return the message with the given ID
    // Hint: Use iter().find()
    
    None
}

/// Partition messages at the given index
/// Returns two slices: [0..mid) and [mid..end)
fn partition_messages(messages: &[SmsMessage], mid: usize) -> (&[SmsMessage], &[SmsMessage]) {
    // TODO: Split the slice at the given index
    // Hint: Use split_at()
    
    (&messages[..0], &messages[..0])
}

/// Demonstrates how array slices work with different types
fn demonstrate_slice_flexibility() {
    println!("Test 5: Slice Flexibility");
    
    // Array slice
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_slice: &[i32] = &arr[1..3];
    println!("Array slice: {:?}", arr_slice);
    
    // Vector slice
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let vec_slice: &[i32] = &vec[2..];
    println!("Vector slice: {:?}", vec_slice);
    
    // Using as_slice()
    let vec2 = vec![100, 200, 300];
    println!("as_slice(): {:?}", vec2.as_slice());
    
    // Mutable slice operations
    let mut data = [5, 2, 8, 1, 9];
    let mutable_slice = &mut data[..];
    mutable_slice.sort();
    println!("Sorted mutable slice: {:?}", data);
    
    // Chunk iteration
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("Chunks of 2:");
    for chunk in numbers.chunks(2) {
        println!("  {:?}", chunk);
    }
    
    // Window iteration
    println!("Windows of 3:");
    for window in numbers.windows(3) {
        println!("  {:?}", window);
    }
}

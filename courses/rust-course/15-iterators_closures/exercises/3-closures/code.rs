// Exercise 3: Closures
// Learn closure syntax, type inference, and Fn traits

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

// TODO: Define a function that accepts a closure
// The closure should take a &Message and return bool
fn filter_messages<F>(messages: Vec<Message>, predicate: F) -> Vec<Message>
where
    F: Fn(&Message) -> bool,
{
    // TODO: Use iterator filter to return matching messages
    vec![] // Replace with your implementation
}

// TODO: Define a function that transforms messages using a closure
fn transform_messages<F>(messages: Vec<Message>, transformer: F) -> Vec<String>
where
    F: Fn(Message) -> String,
{
    // TODO: Use iterator map to transform messages
    vec![] // Replace with your implementation
}

// TODO: Define a function that uses FnMut (modifies captured state)
fn process_with_counter<F>(messages: Vec<Message>, mut processor: F) -> (Vec<Message>, usize)
where
    F: FnMut(&Message) -> bool,
{
    // TODO: Process messages and count how many matched
    // Return (matching_messages, count)
    (vec![], 0) // Replace with your implementation
}

fn main() {
    println!("=== Part 1: Basic Closure Syntax ===\n");
    
    // TODO: Create a closure that adds 1 to a number
    let add_one = |x: i32| x; // Fix this closure
    println!("add_one(5) = {}", add_one(5));
    
    // TODO: Create a closure that concatenates two strings
    // Hint: |a: &str, b: &str| format!("{}{}", a, b)
    let concat = |a: &str, _b: &str| a.to_string(); // Fix this
    println!("concat: {}", concat("Hello, ", "World!"));
    
    // TODO: Create a multi-line closure that describes a number
    let describe = |n: i32| {
        // Add logic to return "positive", "negative", or "zero"
        "unknown".to_string() // Fix this
    };
    println!("5 is {}", describe(5));
    println!("-3 is {}", describe(-3));
    println!("0 is {}", describe(0));
    
    println!("\n=== Part 2: Type Inference ===\n");
    
    // Closures infer types from usage
    let multiply = |x, y| x * y;  // Compiler infers i32
    
    // TODO: Use the multiply closure
    let product = 0; // Replace: multiply(4, 5)
    println!("4 * 5 = {}", product);
    
    // TODO: Create a closure that works with floats
    let float_multiply = |x, y| x * y;
    let float_product = 0.0; // Replace: float_multiply(2.5, 4.0)
    println!("2.5 * 4.0 = {}", float_product);
    
    println!("\n=== Part 3: Capturing the Environment ===\n");
    
    let prefix = "[TEXTIO] ";
    let max_length = 20;
    
    // TODO: Create a closure that captures `prefix`
    let add_prefix = |msg: &str| msg.to_string(); // Fix to use prefix
    println!("With prefix: {}", add_prefix("Hello"));
    
    // TODO: Create a closure that checks if message is within max_length
    let is_valid_length = |msg: &str| true; // Fix to use max_length
    println!("'Short' is valid: {}", is_valid_length("Short"));
    println!("'This is a very long message' is valid: {}", is_valid_length("This is a very long message"));
    
    println!("\n=== Part 4: Closures with Iterators ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // TODO: Use a closure with filter to get numbers > 5
    let greater_than_five: Vec<i32> = vec![]; // Replace with your code
    println!("Numbers > 5: {:?}", greater_than_five);
    
    // TODO: Use a closure with map to create formatted strings
    let formatted: Vec<String> = vec![]; // Replace: numbers.iter().map(|n| format!("Num: {}", n)).collect()
    println!("Formatted (first 3): {:?}", &formatted[..3]);
    
    // TODO: Use a closure with any to check if any number equals 7
    let has_seven = false; // Replace with your code
    println!("Has 7: {}", has_seven);
    
    println!("\n=== Part 5: Closures as Function Parameters ===\n");
    
    let messages = vec![
        Message::new(1, "Hello Alice", "+15550001"),
        Message::new(2, "Meeting at 3pm", "+15550002"),
        Message::new(3, "Your code is 1234", "+15550001"),
        Message::new(4, "Don't forget!", "+15550003"),
        Message::new(5, "Hello Bob", "+15550001"),
    ];
    
    // TODO: Use filter_messages with a closure to get messages to +15550001
    let to_001: Vec<Message> = vec![]; // Replace with filter_messages call
    println!("Messages to +15550001:");
    for msg in &to_001 {
        println!("  {:?}", msg);
    }
    
    // TODO: Use transform_messages to create summaries
    let summaries: Vec<String> = vec![]; // Replace with transform_messages call
    println!("\nMessage summaries:");
    for summary in &summaries {
        println!("  {}", summary);
    }
    
    println!("\n=== Part 6: FnMut - Mutable Captures ===\n");
    
    let mut counter = 0;
    
    // TODO: Create a closure that increments and returns counter
    // Note: This closure needs to mutate `counter`
    let mut count_and_check = |_msg: &Message| {
        // Increment counter and return true if counter is even
        false // Fix this
    };
    
    let test_messages = vec![
        Message::new(1, "A", "+1"),
        Message::new(2, "B", "+2"),
        Message::new(3, "C", "+3"),
    ];
    
    // TODO: Use process_with_counter with the closure
    let (matched, count) = (vec![], 0); // Replace with process_with_counter call
    println!("Matched {} messages out of {}", count, test_messages.len());
    
    println!("\n=== Part 7: Textio Message Router ===\n");
    
    let incoming = vec![
        Message::new(101, "Verify your account", "+15550001"),
        Message::new(102, "Your order shipped", "+15550002"),
        Message::new(103, "Meeting reminder", "+15550003"),
        Message::new(104, "Your code: 9999", "+15550001"),
        Message::new(105, "Delivery tomorrow", "+15550002"),
    ];
    
    // TODO: Create a router that filters by a list of recipients
    let priority_recipients = vec!["+15550001", "+15550003"];
    
    // Use a closure that captures priority_recipients
    let priority_messages: Vec<Message> = vec![]; // Replace with filter_messages call
    println!("Priority messages:");
    for msg in &priority_messages {
        println!("  ID {} to {}", msg.id, msg.recipient);
    }
    
    // TODO: Count messages containing "code" or "verify"
    let verification_count = 0; // Replace with your code
    println!("\nVerification-related messages: {}", verification_count);
}

// Associated Functions - Textio Constructors
// 
// Implement associated functions (constructors) for the
// Message struct using Self and various patterns.

#[derive(Debug)]
struct Message {
    to: String,
    from: String,
    body: String,
    priority: u8,
}

// TODO: Add associated constants to Message
// - MAX_BODY_LENGTH: usize = 160
// - MAX_PRIORITY: u8 = 5
// - DEFAULT_PRIORITY: u8 = 1



impl Message {
    // TODO: Implement new() constructor
    // Parameters: to (String), from (String), body (String)
    // Set priority to DEFAULT_PRIORITY
    // Return Self (not Message)
    fn new(to: String, from: String, body: String) -> Self {
        // Your code here
    }
    
    // TODO: Implement empty() constructor
    // All strings empty, priority is DEFAULT_PRIORITY
    fn empty() -> Self {
        // Your code here
    }
    
    // TODO: Implement notification() constructor
    // Parameters: to (String), body (String)
    // from should be "Textio"
    // priority should be DEFAULT_PRIORITY
    fn notification(to: String, body: String) -> Self {
        // Your code here
    }
    
    // TODO: Implement urgent() constructor
    // Parameters: to (String), from (String), body (String)
    // priority should be MAX_PRIORITY
    fn urgent(to: String, from: String, body: String) -> Self {
        // Your code here
    }
    
    // TODO: Implement with_priority() constructor
    // Parameters: to, from, body, priority
    // Validate priority doesn't exceed MAX_PRIORITY
    // If it does, cap it at MAX_PRIORITY
    fn with_priority(to: String, from: String, body: String, priority: u8) -> Self {
        // Your code here
    }
    
    // TODO: Implement a method to check if body is too long
    fn is_valid(&self) -> bool {
        // Return true if body length <= MAX_BODY_LENGTH
    }
    
    // TODO: Implement a method to get remaining characters
    fn remaining_chars(&self) -> i32 {
        // Return MAX_BODY_LENGTH - body length as i32
        // Can be negative if message is too long
    }
}

fn main() {
    // TODO: Print the MAX_BODY_LENGTH constant
    

    // TODO: Create a basic message using new()
    let msg1 = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Hello from Textio!"),
    );
    println!("Basic message: {:?}", msg1);
    

    // TODO: Create an empty message
    

    // TODO: Create a notification
    

    // TODO: Create an urgent message
    

    // TODO: Create a message with custom priority (3)
    

    // TODO: Create a message with priority exceeding max (10)
    // It should be capped at MAX_PRIORITY
    

    // TODO: Test is_valid() on a short message
    

    // TODO: Test is_valid() on a long message
    let long_body = "x".repeat(200);
    let long_msg = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        long_body,
    );
    

    // TODO: Test remaining_chars()
    let msg5 = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Short"),
    );
    
}

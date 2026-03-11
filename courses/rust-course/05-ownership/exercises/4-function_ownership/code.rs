#[derive(Debug)]
struct SmsMessage {
    id: u32,
    to: String,
    body: String,
}

#[derive(Debug)]
struct PhoneNumber {
    number: String,
}

// TASK: Implement these functions based on their signatures

// Exercise 1: Takes ownership, doesn't return
fn send_message(message: SmsMessage) {
    // Print the message being sent
    // Message will be dropped at end of function
}

// Exercise 2: Creates and returns ownership
fn create_message(id: u32, to: &str, body: &str) -> SmsMessage {
    // Create and return a new SmsMessage
    // This gives ownership to the caller
}

// Exercise 3: Takes ownership, validates, returns ownership
fn validate_message(message: SmsMessage) -> Result<SmsMessage, String> {
    // If to starts with '+', return Ok(message)
    // Otherwise, return Err with "Invalid phone number"
}

// Exercise 4: Takes ownership, modifies, returns ownership
fn add_signature(mut message: SmsMessage) -> SmsMessage {
    // Append "\n\nSent via Textio" to body
    // Return the modified message
}

// Exercise 5: Takes ownership of String, returns new String
fn uppercase_message(message: String) -> String {
    // Convert message to uppercase and return
}

// Exercise 6: Takes ownership, splits into two parts
fn split_message(message: SmsMessage) -> (u32, String, String) {
    // Return (id, to, body) as a tuple
    // This "destructures" the message
}

// Exercise 7: Takes multiple owned values, returns one
fn merge_messages(msg1: SmsMessage, msg2: SmsMessage) -> SmsMessage {
    // Create a new message with combined bodies
    // Use msg1's id and to, combine bodies with " | "
}

// Exercise 8: Builder-style method chain
struct MessageBuilder {
    to: Option<String>,
    body: Option<String>,
}

impl MessageBuilder {
    fn new() -> Self {
        MessageBuilder {
            to: None,
            body: None,
        }
    }
    
    fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }
    
    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
    
    fn build(self) -> Result<SmsMessage, &'static str> {
        // Return Ok with SmsMessage if to and body are set
        // Otherwise return Err with appropriate message
    }
}

// Exercise 9: Process pipeline
fn process_pipeline(id: u32, to: &str, body: &str) -> Result<String, String> {
    // Chain: create -> validate -> add_signature -> format
    // Return formatted string or error
    // This combines all the previous functions!
}

// Exercise 10: Option ownership transfer
fn take_first(messages: &mut Vec<SmsMessage>) -> Option<SmsMessage> {
    // Remove and return the first message from the Vec
    // Return None if Vec is empty
}

fn main() {
    println!("=== Exercise 1: Taking Ownership ===\n");
    
    // TASK 1: Create a message and send it
    // Show that you can't use the message after sending
    
    let msg = SmsMessage {
        id: 1,
        to: String::from("+1234567890"),
        body: String::from("Hello, Textio!"),
    };
    
    send_message(msg);
    // println!("{:?}", msg);  // This would be an error!
    println!("Message was sent and consumed");
    
    println!("\n=== Exercise 2: Creating and Returning ===\n");
    
    // TASK 2: Use create_message to get ownership of a new message
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 3: Validate with Result ===\n");
    
    // TASK 3: Validate a good message and a bad message
    
    let good_msg = create_message(2, "+1234567890", "Valid");
    let bad_msg = create_message(3, "1234567890", "Invalid");
    
    // Validate both and print results
    // YOUR CODE HERE
    
    println!("\n=== Exercise 4: Transform and Return ===\n");
    
    // TASK 4: Add signature to a message
    
    let msg = create_message(4, "+1234567890", "Hello!");
    // Add signature and print result
    // YOUR CODE HERE
    
    println!("\n=== Exercise 5: String Transformation ===\n");
    
    // TASK 5: Uppercase a String
    
    let text = String::from("hello world");
    // Transform and print
    // Show original is consumed
    // YOUR CODE HERE
    
    println!("\n=== Exercise 6: Destructuring ===\n");
    
    // TASK 6: Split message into parts
    
    let msg = create_message(6, "+1234567890", "Hello World");
    // Split and print each part
    // YOUR CODE HERE
    
    println!("\n=== Exercise 7: Merging ===\n");
    
    // TASK 7: Merge two messages
    
    let msg1 = create_message(7, "+1234567890", "First");
    let msg2 = create_message(8, "+1234567890", "Second");
    // Merge and print
    // YOUR CODE HERE
    
    println!("\n=== Exercise 8: Builder Pattern ===\n");
    
    // TASK 8: Use MessageBuilder
    
    let msg = MessageBuilder::new()
        .to("+1234567890")
        .body("Built message")
        .build();
    
    match msg {
        Ok(m) => println!("Built: {:?}", m),
        Err(e) => println!("Error: {}", e),
    }
    
    // Try building incomplete message
    let incomplete = MessageBuilder::new()
        .to("+1234567890")
        .build();
    
    match incomplete {
        Ok(m) => println!("Built: {:?}", m),
        Err(e) => println!("Expected error: {}", e),
    }
    
    println!("\n=== Exercise 9: Full Pipeline ===\n");
    
    // TASK 9: Use the pipeline function
    
    match process_pipeline(100, "+1234567890", "Pipeline test") {
        Ok(formatted) => println!("Pipeline result: {}", formatted),
        Err(e) => println!("Pipeline error: {}", e),
    }
    
    // Try with invalid phone
    match process_pipeline(101, "invalid", "Bad phone") {
        Ok(formatted) => println!("Pipeline result: {}", formatted),
        Err(e) => println!("Pipeline error: {}", e),
    }
    
    println!("\n=== Exercise 10: Option Transfer ===\n");
    
    // TASK 10: Take messages from Vec
    
    let mut messages = vec![
        create_message(201, "+1", "First"),
        create_message(202, "+2", "Second"),
        create_message(203, "+3", "Third"),
    ];
    
    println!("Starting with {} messages", messages.len());
    
    // Take first message
    if let Some(msg) = take_first(&mut messages) {
        println!("Took: {:?}", msg);
    }
    
    println!("Now have {} messages", messages.len());
    
    // Take remaining messages
    while let Some(msg) = take_first(&mut messages) {
        println!("Taking: id={}, to={}", msg.id, msg.to);
    }
    
    println!("Final count: {} messages", messages.len());
}

// Builder Pattern - Textio Message Builder
// 
// Implement the builder pattern for constructing Message
// objects with optional fields and validation.

#[derive(Debug)]
struct Message {
    to: String,
    from: String,
    body: String,
    priority: u8,
    delay_seconds: u32,
    callback_url: Option<String>,
}

// TODO: Create a MessageBuilder struct
// All fields should be Option<T>
// priority and delay_seconds should be Option<u32> initially (we'll set defaults)




impl MessageBuilder {
    // TODO: Implement new() that creates an empty builder
    fn new() -> Self {
        // All fields None
    }
    
    // TODO: Implement setter methods that return Self for chaining
    // - to(mut self, to: String) -> Self
    // - from(mut self, from: String) -> Self
    // - body(mut self, body: String) -> Self
    // - priority(mut self, priority: u8) -> Self
    // - delay(mut self, seconds: u32) -> Self
    // - callback(mut self, url: String) -> Self
    
    
    
    
    
    
    // TODO: Implement build() that validates and creates Message
    // Validation rules:
    // - to is required (return error if missing)
    // - to must start with '+' (return error if not)
    // - body is required (return error if missing)
    // - body must not exceed 160 chars (return error if too long)
    // - from defaults to "Textio" if not set
    // - priority defaults to 1 if not set
    // - delay_seconds defaults to 0 if not set
    // - callback_url stays None if not set
    fn build(self) -> Result<Message, String> {
        // Your validation and construction code here
    }
}

fn main() {
    // TODO: Build a complete message with all fields
    let msg1 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .from(String::from("+15550002"))
        .body(String::from("Hello from Textio!"))
        .priority(3)
        .delay(60)
        .callback(String::from("https://example.com/callback"))
        .build();
    
    match msg1 {
        Ok(msg) => println!("Message built: {:?}", msg),
        Err(e) => println!("Error: {}", e),
    }
    

    // TODO: Build a minimal message (only required fields)
    let msg2 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .body(String::from("Minimal message"))
        .build();
    
    match msg2 {
        Ok(msg) => println!("Minimal message: {:?}", msg),
        Err(e) => println!("Error: {}", e),
    }
    

    // TODO: Try to build without 'to' (should fail)
    let msg3 = MessageBuilder::new()
        .body(String::from("Missing recipient"))
        .build();
    
    match msg3 {
        Ok(msg) => println!("Unexpected success: {:?}", msg),
        Err(e) => println!("Expected error: {}", e),
    }
    

    // TODO: Try to build with invalid 'to' (no + prefix)
    let msg4 = MessageBuilder::new()
        .to(String::from("15550001"))
        .body(String::from("Invalid phone"))
        .build();
    
    match msg4 {
        Ok(msg) => println!("Unexpected success: {:?}", msg),
        Err(e) => println!("Expected error: {}", e),
    }
    

    // TODO: Try to build with body too long
    let long_body = "x".repeat(200);
    let msg5 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .body(long_body)
        .build();
    
    match msg5 {
        Ok(msg) => println!("Unexpected success: {:?}", msg),
        Err(e) => println!("Expected error: {}", e),
    }
    

    // TODO: Build with custom priority and verify default from
    let msg6 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .body(String::from("With priority"))
        .priority(5)
        .build();
    
    match msg6 {
        Ok(msg) => {
            println!("Priority message: {:?}", msg);
            println!("From field defaulted to: {}", msg.from);
        },
        Err(e) => println!("Error: {}", e),
    }
    
}

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

fn send_message(message: SmsMessage) {
    println!("Sending message {} to {}: {}", message.id, message.to, message.body);
}

fn create_message(id: u32, to: &str, body: &str) -> SmsMessage {
    SmsMessage {
        id,
        to: to.to_string(),
        body: body.to_string(),
    }
}

fn validate_message(message: SmsMessage) -> Result<SmsMessage, String> {
    if message.to.starts_with('+') {
        Ok(message)
    } else {
        Err(format!("Invalid phone number: {}", message.to))
    }
}

fn add_signature(mut message: SmsMessage) -> SmsMessage {
    message.body.push_str("\n\nSent via Textio");
    message
}

fn uppercase_message(message: String) -> String {
    message.to_uppercase()
}

fn split_message(message: SmsMessage) -> (u32, String, String) {
    (message.id, message.to, message.body)
}

fn merge_messages(msg1: SmsMessage, _msg2: SmsMessage) -> SmsMessage {
    SmsMessage {
        id: msg1.id,
        to: msg1.to,
        body: format!("{} | {}", msg1.body, _msg2.body),
    }
}

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
        match (self.to, self.body) {
            (Some(to), Some(body)) => Ok(SmsMessage {
                id: 0,
                to,
                body,
            }),
            (None, _) => Err("to is required"),
            (_, None) => Err("body is required"),
        }
    }
}

fn process_pipeline(id: u32, to: &str, body: &str) -> Result<String, String> {
    let msg = create_message(id, to, body);
    let msg = validate_message(msg)?;
    let msg = add_signature(msg);
    Ok(format!("[{}] To: {} | {}", msg.id, msg.to, msg.body))
}

fn take_first(messages: &mut Vec<SmsMessage>) -> Option<SmsMessage> {
    if messages.is_empty() {
        None
    } else {
        Some(messages.remove(0))
    }
}

fn main() {
    println!("=== Exercise 1: Taking Ownership ===\n");
    
    let msg = SmsMessage {
        id: 1,
        to: String::from("+1234567890"),
        body: String::from("Hello, Textio!"),
    };
    
    send_message(msg);
    println!("Message was sent and consumed");
    
    println!("\n=== Exercise 2: Creating and Returning ===\n");
    
    let msg = create_message(2, "+1234567890", "Created message");
    println!("Created: {:?}", msg);
    
    println!("\n=== Exercise 3: Validate with Result ===\n");
    
    let good_msg = create_message(3, "+1234567890", "Valid");
    let bad_msg = create_message(4, "1234567890", "Invalid");
    
    match validate_message(good_msg) {
        Ok(m) => println!("Valid message: {:?}", m),
        Err(e) => println!("Error: {}", e),
    }
    
    match validate_message(bad_msg) {
        Ok(m) => println!("Valid message: {:?}", m),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Exercise 4: Transform and Return ===\n");
    
    let msg = create_message(5, "+1234567890", "Hello!");
    let signed = add_signature(msg);
    println!("Signed message: {:?}", signed);
    
    println!("\n=== Exercise 5: String Transformation ===\n");
    
    let text = String::from("hello world");
    let upper = uppercase_message(text);
    println!("Uppercased: {}", upper);
    
    println!("\n=== Exercise 6: Destructuring ===\n");
    
    let msg = create_message(6, "+1234567890", "Hello World");
    let (id, to, body) = split_message(msg);
    println!("Split: id={}, to={}, body={}", id, to, body);
    
    println!("\n=== Exercise 7: Merging ===\n");
    
    let msg1 = create_message(7, "+1234567890", "First");
    let msg2 = create_message(8, "+1234567890", "Second");
    let merged = merge_messages(msg1, msg2);
    println!("Merged: {:?}", merged);
    
    println!("\n=== Exercise 8: Builder Pattern ===\n");
    
    let msg = MessageBuilder::new()
        .to("+1234567890")
        .body("Built message")
        .build();
    
    match msg {
        Ok(m) => println!("Built: {:?}", m),
        Err(e) => println!("Error: {}", e),
    }
    
    let incomplete = MessageBuilder::new()
        .to("+1234567890")
        .build();
    
    match incomplete {
        Ok(m) => println!("Built: {:?}", m),
        Err(e) => println!("Expected error: {}", e),
    }
    
    println!("\n=== Exercise 9: Full Pipeline ===\n");
    
    match process_pipeline(100, "+1234567890", "Pipeline test") {
        Ok(formatted) => println!("Pipeline result: {}", formatted),
        Err(e) => println!("Pipeline error: {}", e),
    }
    
    match process_pipeline(101, "invalid", "Bad phone") {
        Ok(formatted) => println!("Pipeline result: {}", formatted),
        Err(e) => println!("Pipeline error: {}", e),
    }
    
    println!("\n=== Exercise 10: Option Transfer ===\n");
    
    let mut messages = vec![
        create_message(201, "+1", "First"),
        create_message(202, "+2", "Second"),
        create_message(203, "+3", "Third"),
    ];
    
    println!("Starting with {} messages", messages.len());
    
    if let Some(msg) = take_first(&mut messages) {
        println!("Took: {:?}", msg);
    }
    
    println!("Now have {} messages", messages.len());
    
    while let Some(msg) = take_first(&mut messages) {
        println!("Taking: id={}, to={}", msg.id, msg.to);
    }
    
    println!("Final count: {} messages", messages.len());
}

// Exercise 1: Lifetime Basics
// 
// In this exercise, you'll learn why lifetimes exist and how to annotate them.
// The Textio SMS API needs to efficiently parse messages without copying strings.
// 
// TODO: Fix all the lifetime errors in this file.

// A parsed SMS message that holds references to the original string.
// This avoids copying - we just point into the original message.
struct ParsedSms {
    sender: &str,
    recipient: &str,
    body: &str,
}

impl ParsedSms {
    // TODO: Add lifetime parameter to the impl block
    
    fn new(sender: &str, recipient: &str, body: &str) -> Self {
        ParsedSms {
            sender,
            recipient,
            body,
        }
    }
    
    // Returns a reference to the sender
    fn get_sender(&self) -> &str {
        self.sender
    }
    
    // Returns a reference to the body
    fn get_body(&self) -> &str {
        self.body
    }
    
    // Returns a formatted display string
    // TODO: This needs lifetime annotations
    fn format(&self) -> &str {
        // We'll return the body for now
        self.body
    }
}

// Parses a raw SMS message in the format "SENDER|RECIPIENT|BODY"
// Returns None if the format is invalid
// 
// TODO: Add lifetime annotations
fn parse_sms(raw_message: &str) -> Option<ParsedSms> {
    let parts: Vec<&str> = raw_message.splitn(3, '|').collect();
    
    if parts.len() == 3 {
        Some(ParsedSms::new(parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

// Returns the first word of a message
// The returned slice lives as long as the input
// 
// TODO: Add lifetime annotations
fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }
    
    &text[..]
}

// Returns the longest of two message slices
// The result lives as long as the shorter input could allow
// 
// TODO: Add lifetime annotations
fn longest_message<'a>(msg1: &str, msg2: &str) -> &str {
    if msg1.len() > msg2.len() {
        msg1
    } else {
        msg2
    }
}

// A cache entry for storing message metadata
struct MessageCache {
    message_id: &str,
    status: &str,
}

impl MessageCache {
    // TODO: Add lifetimes
    fn new(id: &str, status: &str) -> Self {
        MessageCache {
            message_id: id,
            status: status,
        }
    }
    
    // TODO: Fix the return type
    fn get_status(&self) -> &str {
        self.status
    }
}

// Validates a phone number format
// Returns a slice of the valid number, or an error message
// 
// TODO: Add lifetime annotations
fn validate_phone(phone: &str) -> Result<&str, &str> {
    if phone.len() < 10 {
        Err("Phone number too short")
    } else if !phone.chars().all(|c| c.is_numeric()) {
        Err("Phone number must contain only digits")
    } else {
        Ok(phone)
    }
}

// Extract the area code (first 3 digits) from a phone number
// 
// TODO: Add lifetime annotations
fn extract_area_code(phone: &str) -> Option<&str> {
    if phone.len() >= 3 {
        Some(&phone[0..3])
    } else {
        None
    }
}

fn main() {
    println!("=== Textio SMS Parser ===\n");
    
    // Test parsing
    let raw = "5551234567|5559876543|Hello, this is a test message!";
    match parse_sms(raw) {
        Some(sms) => {
            println!("Parsed SMS:");
            println!("  Sender: {}", sms.get_sender());
            println!("  Recipient: {}", sms.get_recipient());
            println!("  Body: {}", sms.get_body());
            println!("  First word: {}", first_word(sms.get_body()));
        }
        None => println!("Failed to parse SMS"),
    }
    
    println!("\n=== Longest Message ===");
    let msg1 = "Short";
    let msg2 = "This is a longer message";
    println!("Longest: {}", longest_message(msg1, msg2));
    
    println!("\n=== Phone Validation ===");
    match validate_phone("5551234567") {
        Ok(valid) => println!("Valid phone: {}", valid),
        Err(e) => println!("Error: {}", e),
    }
    
    match validate_phone("123") {
        Ok(valid) => println!("Valid phone: {}", valid),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Area Code Extraction ===");
    if let Some(area) = extract_area_code("5551234567") {
        println!("Area code: {}", area);
    }
    
    println!("\n=== Message Cache ===");
    let cache = MessageCache::new("msg-001", "delivered");
    println!("Message {} status: {}", cache.message_id, cache.get_status());
}

// BONUS: Add a get_recipient method to ParsedSms
// BONUS: Create a function that returns both the sender and recipient as a tuple

// Exercise 1: Lifetime Basics - Complete Solution

// A parsed SMS message that holds references to the original string.
// The lifetime 'a indicates all string slices live at least as long as 'a.
struct ParsedSms<'a> {
    sender: &'a str,
    recipient: &'a str,
    body: &'a str,
}

impl<'a> ParsedSms<'a> {
    fn new(sender: &'a str, recipient: &'a str, body: &'a str) -> Self {
        ParsedSms {
            sender,
            recipient,
            body,
        }
    }
    
    fn get_sender(&self) -> &'a str {
        self.sender
    }
    
    fn get_recipient(&self) -> &'a str {
        self.recipient
    }
    
    fn get_body(&self) -> &'a str {
        self.body
    }
    
    fn format(&self) -> &'a str {
        self.body
    }
}

// The returned ParsedSms contains slices from raw_message,
// so they share the same lifetime 'a
fn parse_sms<'a>(raw_message: &'a str) -> Option<ParsedSms<'a>> {
    let parts: Vec<&str> = raw_message.splitn(3, '|').collect();
    
    if parts.len() == 3 {
        Some(ParsedSms::new(parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

// Input and output share the same lifetime 'a
fn first_word<'a>(text: &'a str) -> &'a str {
    let bytes = text.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }
    
    &text[..]
}

// Both inputs could be returned, so both need lifetime 'a
// The output lives as long as the shortest of msg1 or msg2
fn longest_message<'a>(msg1: &'a str, msg2: &'a str) -> &'a str {
    if msg1.len() > msg2.len() {
        msg1
    } else {
        msg2
    }
}

// Both references need the same lifetime 'a
struct MessageCache<'a> {
    message_id: &'a str,
    status: &'a str,
}

impl<'a> MessageCache<'a> {
    fn new(id: &'a str, status: &'a str) -> Self {
        MessageCache {
            message_id: id,
            status: status,
        }
    }
    
    fn get_status(&self) -> &'a str {
        self.status
    }
}

// The Ok and Err variants both contain string slices from the input
fn validate_phone<'a>(phone: &'a str) -> Result<&'a str, &'static str> {
    if phone.len() < 10 {
        Err("Phone number too short")
    } else if !phone.chars().all(|c| c.is_numeric()) {
        Err("Phone number must contain only digits")
    } else {
        Ok(phone)
    }
}

// The returned slice is a substring of the input
fn extract_area_code<'a>(phone: &'a str) -> Option<&'a str> {
    if phone.len() >= 3 {
        Some(&phone[0..3])
    } else {
        None
    }
}

fn main() {
    println!("=== Textio SMS Parser ===\n");
    
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

// Exercise 2: Lifetime Elision
// 
// Rust's lifetime elision rules let you omit lifetime annotations in common cases.
// Learn when you can elide and when you must be explicit.
//
// ELISION RULES:
// 1. Each input reference gets its own lifetime
// 2. If one input lifetime (&self or &mut self), it's assigned to all outputs
// 3. If multiple inputs but one is &self/&mut self, self's lifetime goes to outputs

// TODO: For each function, determine if elision applies or if explicit lifetimes are needed

use std::collections::HashMap;

// A message handler for the Textio API
struct MessageHandler {
    messages: HashMap<String, String>,
    default_sender: String,
}

impl MessageHandler {
    fn new(default_sender: &str) -> Self {
        let mut messages = HashMap::new();
        messages.insert("welcome".to_string(), "Welcome to Textio!".to_string());
        
        MessageHandler {
            messages,
            default_sender: default_sender.to_string(),
        }
    }
    
    // TODO: Does elision work here? Fix if needed.
    fn get_default_sender(&self) -> &str {
        &self.default_sender
    }
    
    // TODO: Does elision work here? Fix if needed.
    fn get_message(&self, key: &str) -> Option<&String> {
        self.messages.get(key)
    }
    
    // TODO: Does elision work here? Fix if needed.
    fn store_message(&mut self, key: &str, value: &str) -> &str {
        self.messages.insert(key.to_string(), value.to_string());
        value
    }
}

// A parsed message with optional parts
struct ParsedMessage<'a> {
    sender: &'a str,
    body: &'a str,
}

impl<'a> ParsedMessage<'a> {
    // TODO: Does elision work here?
    fn sender(&self) -> &str {
        self.sender
    }
    
    // TODO: Does elision work here?
    fn body(&self) -> &str {
        self.body
    }
    
    // TODO: Does elision work here? Fix if needed.
    fn as_tuple(&self) -> (&str, &str) {
        (self.sender, self.body)
    }
}

// Independent functions - no &self available

// TODO: This should work with elision. Why?
fn get_first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// TODO: This needs explicit lifetimes. Why? Fix it.
fn get_longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// TODO: Does this need explicit lifetimes?
fn extract_phone_prefix(phone: &str) -> &str {
    if phone.len() >= 3 {
        &phone[..3]
    } else {
        phone
    }
}

// TODO: This has multiple outputs. Does elision work?
fn split_message(msg: &str) -> (&str, &str) {
    match msg.find('|') {
        Some(pos) => (&msg[..pos], &msg[pos + 1..]),
        None => (msg, ""),
    }
}

// TODO: This needs explicit lifetimes. Fix it.
fn combine_sender_body(sender: &str, body: &str) -> String {
    format!("From {}: {}", sender, body)
}

// A message filter
struct MessageFilter<'a> {
    blocked_prefixes: Vec<&'a str>,
}

impl<'a> MessageFilter<'a> {
    fn new() -> Self {
        MessageFilter {
            blocked_prefixes: Vec::new(),
        }
    }
    
    fn add_blocked_prefix(&mut self, prefix: &'a str) {
        self.blocked_prefixes.push(prefix);
    }
    
    // TODO: Does elision work here?
    fn is_blocked(&self, message: &str) -> bool {
        self.blocked_prefixes.iter().any(|p| message.starts_with(p))
    }
    
    // TODO: Does elision work here?
    fn get_blocked_prefixes(&self) -> &Vec<&'a str> {
        &self.blocked_prefixes
    }
}

// Message parsing utilities
fn parse_header(header_line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = header_line.splitn(2, ':').collect();
    if parts.len() == 2 {
        Some((parts[0].trim(), parts[1].trim()))
    } else {
        None
    }
}

// TODO: Fix this function - it needs explicit lifetimes
fn select_message<'a>(use_first: bool, msg1: &'a str, msg2: &'a str) -> &'a str {
    if use_first { msg1 } else { msg2 }
}

// TODO: This should work with elision. Verify.
fn count_messages(messages: &[&str]) -> usize {
    messages.len()
}

// TODO: Fix this - returning a slice needs proper lifetime
fn truncate_message(msg: &str, max_len: usize) -> &str {
    if msg.len() <= max_len {
        msg
    } else {
        &msg[..max_len]
    }
}

fn main() {
    println!("=== Lifetime Elision Demo ===\n");
    
    // Test MessageHandler
    let handler = MessageHandler::new("TextioAPI");
    println!("Default sender: {}", handler.get_default_sender());
    
    if let Some(msg) = handler.get_message("welcome") {
        println!("Welcome message: {}", msg);
    }
    
    // Test standalone functions
    let text = "Hello World from Textio";
    println!("\nFirst word: {}", get_first_word(text));
    
    let a = "Short";
    let b = "Longer message";
    println!("Longer: {}", get_longer(a, b));
    
    let phone = "5551234567";
    println!("Phone prefix: {}", extract_phone_prefix(phone));
    
    let raw = "sender|Hello there";
    let (sender, body) = split_message(raw);
    println!("Split: sender='{}', body='{}'", sender, body);
    
    // Test message filter
    let mut filter = MessageFilter::new();
    filter.add_blocked_prefix("SPAM:");
    filter.add_blocked_prefix("ADS:");
    
    println!("\nBlocked prefixes: {:?}", filter.get_blocked_prefixes());
    println!("Is 'SPAM: Buy now!' blocked? {}", filter.is_blocked("SPAM: Buy now!"));
    println!("Is 'Hello there' blocked? {}", filter.is_blocked("Hello there"));
    
    // Test select_message
    let m1 = "First message";
    let m2 = "Second message";
    println!("\nSelected: {}", select_message(true, m1, m2));
    
    // Test truncate
    let long_msg = "This is a very long message that needs truncation";
    println!("Truncated: {}", truncate_message(long_msg, 20));
    
    // Test parse_header
    let header = "Content-Type: text/plain";
    if let Some((key, value)) = parse_header(header) {
        println!("\nHeader: {} = {}", key, value);
    }
    
    // Test count_messages
    let msgs = ["msg1", "msg2", "msg3"];
    println!("Message count: {}", count_messages(&msgs));
}

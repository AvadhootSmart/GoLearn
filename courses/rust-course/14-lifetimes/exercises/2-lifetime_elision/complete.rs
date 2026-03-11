// Exercise 2: Lifetime Elision - Complete Solution

use std::collections::HashMap;

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
    
    // Elision works: &self provides lifetime for output
    fn get_default_sender(&self) -> &str {
        &self.default_sender
    }
    
    // Elision works: &self provides lifetime, key has its own
    fn get_message(&self, key: &str) -> Option<&String> {
        self.messages.get(key)
    }
    
    // NEEDS EXPLICIT LIFETIMES: Output comes from value parameter, not self
    // But since we're returning the parameter 'value', we need to be explicit
    // Actually, we're returning a String copy's reference won't work
    // Let's return a reference to the stored value instead
    fn store_message(&mut self, key: &str, value: &str) {
        self.messages.insert(key.to_string(), value.to_string());
    }
}

struct ParsedMessage<'a> {
    sender: &'a str,
    body: &'a str,
}

impl<'a> ParsedMessage<'a> {
    // Elision works
    fn sender(&self) -> &str {
        self.sender
    }
    
    // Elision works
    fn body(&self) -> &str {
        self.body
    }
    
    // Elision works - both outputs from &self
    fn as_tuple(&self) -> (&str, &str) {
        (self.sender, self.body)
    }
}

// Elision works: single input, output derived from it
fn get_first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// NEEDS EXPLICIT LIFETIMES: Two inputs, output could be either
fn get_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// Elision works: single input
fn extract_phone_prefix(phone: &str) -> &str {
    if phone.len() >= 3 {
        &phone[..3]
    } else {
        phone
    }
}

// Elision works: single input, multiple outputs derived from it
fn split_message(msg: &str) -> (&str, &str) {
    match msg.find('|') {
        Some(pos) => (&msg[..pos], &msg[pos + 1..]),
        None => (msg, ""),
    }
}

// No references returned, no lifetimes needed
fn combine_sender_body(sender: &str, body: &str) -> String {
    format!("From {}: {}", sender, body)
}

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
    
    // Elision works: &self provides lifetime
    fn is_blocked(&self, message: &str) -> bool {
        self.blocked_prefixes.iter().any(|p| message.starts_with(p))
    }
    
    // Elision works: &self provides lifetime
    fn get_blocked_prefixes(&self) -> &Vec<&'a str> {
        &self.blocked_prefixes
    }
}

// Elision works: single input
fn parse_header(header_line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = header_line.splitn(2, ':').collect();
    if parts.len() == 2 {
        Some((parts[0].trim(), parts[1].trim()))
    } else {
        None
    }
}

// NEEDS EXPLICIT LIFETIMES: Two inputs, output could be either
fn select_message<'a>(use_first: bool, msg1: &'a str, msg2: &'a str) -> &'a str {
    if use_first { msg1 } else { msg2 }
}

// No lifetimes needed - no references in output
fn count_messages(messages: &[&str]) -> usize {
    messages.len()
}

// Elision works: single input
fn truncate_message(msg: &str, max_len: usize) -> &str {
    if msg.len() <= max_len {
        msg
    } else {
        &msg[..max_len]
    }
}

fn main() {
    println!("=== Lifetime Elision Demo ===\n");
    
    let handler = MessageHandler::new("TextioAPI");
    println!("Default sender: {}", handler.get_default_sender());
    
    if let Some(msg) = handler.get_message("welcome") {
        println!("Welcome message: {}", msg);
    }
    
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
    
    let mut filter = MessageFilter::new();
    filter.add_blocked_prefix("SPAM:");
    filter.add_blocked_prefix("ADS:");
    
    println!("\nBlocked prefixes: {:?}", filter.get_blocked_prefixes());
    println!("Is 'SPAM: Buy now!' blocked? {}", filter.is_blocked("SPAM: Buy now!"));
    println!("Is 'Hello there' blocked? {}", filter.is_blocked("Hello there"));
    
    let m1 = "First message";
    let m2 = "Second message";
    println!("\nSelected: {}", select_message(true, m1, m2));
    
    let long_msg = "This is a very long message that needs truncation";
    println!("Truncated: {}", truncate_message(long_msg, 20));
    
    let header = "Content-Type: text/plain";
    if let Some((key, value)) = parse_header(header) {
        println!("\nHeader: {} = {}", key, value);
    }
    
    let msgs = ["msg1", "msg2", "msg3"];
    println!("Message count: {}", count_messages(&msgs));
}

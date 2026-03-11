// String Type Exercise - Textio SMS API
//
// Complete the functions below to work with Strings in the context
// of an SMS messaging system.

/// Represents an SMS message with sender, recipient, and content
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}

impl SmsMessage {
    /// Create a new SMS message
    /// TODO: Convert the string slices to owned Strings
    pub fn new(to: &str, from: &str, body: &str) -> Self {
        // Your code here
        todo!()
    }

    /// Get a preview of the message body (first n characters)
    /// TODO: Use the get() method to safely return a slice
    /// If the body is shorter than len, return the entire body
    pub fn preview(&self, len: usize) -> &str {
        // Your code here
        todo!()
    }

    /// Append a signature to the message body
    /// TODO: Use push_str to add the signature
    pub fn add_signature(&mut self, signature: &str) {
        // Your code here
        todo!()
    }

    /// Add a single character to the body
    /// TODO: Use push to add the character
    pub fn append_char(&mut self, c: char) {
        // Your code here
        todo!()
    }

    /// Truncate the message to a maximum number of characters
    /// TODO: Use chars() iterator to take max_chars characters
    /// and collect them back into a String
    pub fn truncate(&mut self, max_chars: usize) {
        // Your code here
        todo!()
    }

    /// Get the byte length of the body
    pub fn body_byte_length(&self) -> usize {
        // Your code here
        todo!()
    }

    /// Get the character count of the body
    /// TODO: Use chars().count()
    pub fn body_char_count(&self) -> usize {
        // Your code here
        todo!()
    }

    /// Replace all occurrences of a substring in the body
    /// TODO: Use the replace method
    pub fn censor(&mut self, word: &str, replacement: &str) {
        // Your code here
        todo!()
    }
}

/// Concatenate multiple phone numbers with a separator
/// TODO: Use the join method on a vector
pub fn format_phone_list(phones: &[&str], separator: &str) -> String {
    // Your code here
    todo!()
}

/// Build a message from parts using format!
/// TODO: Use the format! macro to create the message
pub fn build_message(greeting: &str, name: &str, content: &str) -> String {
    // Your code here
    todo!()
}

/// Concatenate two strings using the + operator
/// TODO: Use the + operator (remember the first string is moved)
pub fn concatenate(s1: String, s2: &str) -> String {
    // Your code here
    todo!()
}

/// Create a String with pre-allocated capacity
/// TODO: Use String::with_capacity
pub fn create_with_capacity(capacity: usize) -> String {
    // Your code here
    todo!()
}

/// Split a message by a delimiter
/// TODO: Use split() and collect into a Vec
pub fn split_message(message: &str, delimiter: char) -> Vec<&str> {
    // Your code here
    todo!()
}

/// Remove leading and trailing whitespace
/// TODO: Use trim()
pub fn clean_message(message: &str) -> &str {
    // Your code here
    todo!()
}

/// Check if a phone number starts with a country code
/// TODO: Use starts_with()
pub fn has_country_code(phone: &str, code: &str) -> bool {
    // Your code here
    todo!()
}

fn main() {
    // Test SmsMessage
    let mut msg = SmsMessage::new("+1234567890", "+0987654321", "Hello, World!");
    
    println!("To: {}", msg.to);
    println!("From: {}", msg.from);
    println!("Body: {}", msg.body);
    println!("Preview (5 chars): {}", msg.preview(5));
    println!("Byte length: {}", msg.body_byte_length());
    println!("Char count: {}", msg.body_char_count());
    
    msg.add_signature(" - Textio");
    println!("With signature: {}", msg.body);
    
    msg.append_char('!');
    println!("With char appended: {}", msg.body);
    
    msg.truncate(15);
    println!("Truncated to 15 chars: {}", msg.body);
    
    let mut censored = SmsMessage::new("+1", "+2", "The secret word is PASSWORD");
    censored.censor("PASSWORD", "*****");
    println!("Censored: {}", censored.body);
    
    // Test phone list formatting
    let phones = ["+1234567890", "+0987654321", "+1122334455"];
    println!("Phone list: {}", format_phone_list(&phones, ", "));
    
    // Test message building
    let built = build_message("Hello", "Alice", "Your package has been delivered!");
    println!("Built message: {}", built);
    
    // Test concatenation
    let s1 = String::from("Hello, ");
    let s2 = "World!";
    let concat = concatenate(s1, s2);
    println!("Concatenated: {}", concat);
    
    // Test capacity
    let mut s = create_with_capacity(100);
    println!("Empty string capacity: {}", s.capacity());
    s.push_str("Hello");
    println!("After push capacity: {}", s.capacity());
    
    // Test splitting
    let parts = split_message("one,two,three,four", ',');
    println!("Split parts: {:?}", parts);
    
    // Test trimming
    let trimmed = clean_message("  hello world  ");
    println!("Trimmed: '{}'", trimmed);
    
    // Test country code check
    let phone = "+1-555-1234";
    println!("Has +1 code: {}", has_country_code(phone, "+1"));
    println!("Has +44 code: {}", has_country_code(phone, "+44"));
    
    // Test Unicode handling
    let unicode_msg = SmsMessage::new("+1", "+2", "Hello 🎉 日");
    println!("Unicode message: {}", unicode_msg.body);
    println!("Unicode byte length: {}", unicode_msg.body_byte_length());
    println!("Unicode char count: {}", unicode_msg.body_char_count());
}

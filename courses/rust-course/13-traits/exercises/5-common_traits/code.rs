// Exercise 5: Common Standard Library Traits
//
// Implement standard library traits for Textio message types.
// Complete the TODO sections to add trait implementations.

use std::fmt;
use std::convert::TryFrom;

// TODO: Add derive macros for: Debug, Clone, PartialEq
pub struct MessageId {
    pub id: String,
    pub timestamp: u64,
}

// TODO: Implement Display for MessageId
// Format: "msg_{id}@{timestamp}"



// TODO: Implement Default for MessageId
// Default: id = "unknown", timestamp = 0



// TODO: Implement From<&str> for MessageId
// Parse format "msg_{id}@{timestamp}" or return default



// Message priority enum
// TODO: Add derive macros for: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

// TODO: Implement Display for Priority
// Format as lowercase: "low", "normal", "high", "urgent"



// SMS Message struct
// TODO: Add derive macros for: Debug, Clone
pub struct SmsMessage {
    pub id: MessageId,
    pub to: String,
    pub from: String,
    pub body: String,
    pub priority: Priority,
}

// TODO: Implement Display for SmsMessage
// Format:
// [{priority}] {id}
// From: {from}
// To: {to}
// {body}



// TODO: Implement PartialEq for SmsMessage
// Two messages are equal if their id, to, and body are equal



// TODO: Implement Default for SmsMessage
// Use defaults for id, empty strings for fields, Normal priority



// TODO: Implement From<&str> for SmsMessage
// Parse format: "{to}|{from}|{body}"



// Phone number wrapper
pub struct PhoneNumber(pub String);

// TODO: Implement TryFrom<&str> for PhoneNumber
// Valid if starts with '+' and has at least 10 characters
// Error type: &'static str



// TODO: Implement Display for PhoneNumber
// Just show the string



// TODO: Implement AsRef<str> for PhoneNumber
// Return the inner string reference



// Configuration struct
// TODO: Add derive macros for: Debug, Clone, PartialEq
pub struct SmsConfig {
    pub max_length: usize,
    pub encoding: String,
    pub require_delivery_report: bool,
}

// TODO: Implement Default for SmsConfig
// max_length: 160, encoding: "GSM-7", require_delivery_report: false



// TODO: Implement a supertrait MessageExt that requires: Debug + Clone
// With method: summary(&self) -> String that returns a basic description
// Note: You'll need to implement this for SmsMessage



// Helper functions
fn print_message<T: fmt::Display>(item: &T) {
    println!("{}", item);
}

fn compare_messages(m1: &SmsMessage, m2: &SmsMessage) {
    if m1 == m2 {
        println!("Messages are equal");
    } else {
        println!("Messages are different");
    }
}

fn main() {
    // Test MessageId
    println!("=== MessageId ===");
    let id1 = MessageId {
        id: String::from("abc123"),
        timestamp: 1699999999,
    };
    println!("Display: {}", id1);
    println!("Debug: {:?}", id1);

    let id2: MessageId = "msg_xyz789@1700000000".into();
    println!("From string: {}", id2);

    let default_id = MessageId::default();
    println!("Default: {}", default_id);

    // Test Priority
    println!("\n=== Priority ===");
    let low = Priority::Low;
    let high = Priority::High;
    println!("Low: {} < High: {} ? {}", low, high, low < high);
    println!("Debug: {:?}", high);

    // Test SmsMessage
    println!("\n=== SmsMessage ===");
    let msg = SmsMessage {
        id: id1,
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: String::from("Hello from Textio!"),
        priority: Priority::Normal,
    };

    println!("{}", msg);
    println!("\nDebug: {:?}", msg);

    // Test From
    let msg_from_str: SmsMessage = "+15551112222|+15553334444|Test message".into();
    println!("\nFrom string:");
    println!("{}", msg_from_str);

    // Test PartialEq
    println!("\n=== Equality ===");
    let msg2 = msg.clone();
    compare_messages(&msg, &msg2);

    let msg3 = SmsMessage {
        id: MessageId::default(),
        ..Default::default()
    };
    compare_messages(&msg, &msg3);

    // Test Default
    println!("\n=== Defaults ===");
    let default_msg = SmsMessage::default();
    println!("Default message: {}", default_msg);

    let default_config = SmsConfig::default();
    println!("Default config: {:?}", default_config);

    // Test PhoneNumber
    println!("\n=== PhoneNumber ===");
    match PhoneNumber::try_from("+15551234567") {
        Ok(phone) => {
            println!("Valid phone: {}", phone);
            println!("As ref: {}", phone.as_ref());
        }
        Err(e) => println!("Error: {}", e),
    }

    match PhoneNumber::try_from("invalid") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {}", e),
    }

    // Test MessageExt
    println!("\n=== MessageExt ===");
    let test_msg = SmsMessage {
        id: MessageId {
            id: String::from("test"),
            timestamp: 100,
        },
        to: String::from("+15550000000"),
        from: String::from("+15551111111"),
        body: String::from("Test"),
        priority: Priority::Urgent,
    };
    println!("Summary: {}", test_msg.summary());
}

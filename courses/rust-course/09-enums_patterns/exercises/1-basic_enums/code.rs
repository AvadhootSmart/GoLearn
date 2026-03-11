// Exercise 1: Basic Enums
// Define and implement enums for the Textio SMS API

// TODO: Create a MessageStatus enum with these variants:
// - Pending (no data)
// - Sent (no data)  
// - Delivered with timestamp: u64
// - Failed with error_code: u16 and error_message: String
//
// Add these derives: Debug, Clone, PartialEq

// TODO: Implement the MessageStatus enum here
// enum MessageStatus { ... }

// TODO: Implement methods for MessageStatus
impl MessageStatus {
    // Create a new Pending status
    pub fn new() -> Self {
        todo!()
    }
    
    // Mark as sent, returns new status
    pub fn mark_sent(self) -> Self {
        todo!()
    }
    
    // Mark as delivered with timestamp
    pub fn mark_delivered(self, timestamp: u64) -> Self {
        todo!()
    }
    
    // Mark as failed with error info
    pub fn mark_failed(self, code: u16, message: &str) -> Self {
        todo!()
    }
    
    // Check if the message is still in progress (Pending or Sent)
    pub fn is_in_progress(&self) -> bool {
        todo!()
    }
    
    // Check if the message reached a terminal state (Delivered or Failed)
    pub fn is_terminal(&self) -> bool {
        todo!()
    }
    
    // Get a human-readable description
    pub fn description(&self) -> String {
        todo!()
    }
    
    // Get the error code if Failed, None otherwise
    pub fn error_code(&self) -> Option<u16> {
        todo!()
    }
}

// TODO: Create a Priority enum for message priority levels
// Variants: Low, Normal, High, Urgent
// Add derives: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord

// TODO: Implement Priority enum here

impl Priority {
    // Returns the numeric priority level (higher = more urgent)
    pub fn level(&self) -> u8 {
        todo!()
    }
    
    // Returns true if this should be processed before other priorities
    pub fn is_urgent(&self) -> bool {
        todo!()
    }
}

// TODO: Create a Recipient enum for different recipient types
// - PhoneNumber(String)
// - GroupId(u64)
// - ContactList { name: String, count: u32 }
// Add derives: Debug, Clone, PartialEq

// TODO: Implement Recipient enum here

impl Recipient {
    // Returns a display string for the recipient
    pub fn display(&self) -> String {
        todo!()
    }
    
    // Check if this is an individual phone number
    pub fn is_individual(&self) -> bool {
        todo!()
    }
    
    // Get the approximate recipient count (1 for phone, from GroupId, count for ContactList)
    pub fn estimated_count(&self) -> u32 {
        todo!()
    }
}

// Helper function to demonstrate enum usage
fn process_message(status: MessageStatus, priority: Priority, recipient: Recipient) -> String {
    format!(
        "Sending to {} with {:?} priority. Status: {}",
        recipient.display(),
        priority,
        status.description()
    )
}

fn main() {
    // Test MessageStatus
    println!("=== MessageStatus ===");
    
    let status = MessageStatus::new();
    println!("Initial: {:?}", status);
    println!("In progress: {}", status.is_in_progress());
    println!("Description: {}", status.description());
    
    let status = status.mark_sent();
    println!("\nAfter sent: {:?}", status);
    println!("In progress: {}", status.is_in_progress());
    
    let status = status.mark_delivered(1700000000);
    println!("\nAfter delivered: {:?}", status);
    println!("Is terminal: {}", status.is_terminal());
    println!("Error code: {:?}", status.error_code());
    
    let failed_status = MessageStatus::new()
        .mark_sent()
        .mark_failed(500, "Network timeout");
    println!("\nFailed status: {:?}", failed_status);
    println!("Error code: {:?}", failed_status.error_code());
    
    // Test Priority
    println!("\n=== Priority ===");
    
    let low = Priority::Low;
    let normal = Priority::Normal;
    let high = Priority::High;
    let urgent = Priority::Urgent;
    
    println!("Low level: {}, urgent: {}", low.level(), low.is_urgent());
    println!("Normal level: {}, urgent: {}", normal.level(), normal.is_urgent());
    println!("High level: {}, urgent: {}", high.level(), high.is_urgent());
    println!("Urgent level: {}, urgent: {}", urgent.level(), urgent.is_urgent());
    
    println!("\nComparison: High < Urgent = {}", high < urgent);
    println!("Comparison: Low < Normal = {}", low < normal);
    
    // Test Recipient
    println!("\n=== Recipient ===");
    
    let phone = Recipient::PhoneNumber(String::from("+1234567890"));
    let group = Recipient::GroupId(42);
    let list = Recipient::ContactList { 
        name: String::from("Marketing Team"), 
        count: 150 
    };
    
    println!("Phone: {}, individual: {}, count: {}", 
             phone.display(), phone.is_individual(), phone.estimated_count());
    println!("Group: {}, individual: {}, count: {}",
             group.display(), group.is_individual(), group.estimated_count());
    println!("List: {}, individual: {}, count: {}",
             list.display(), list.is_individual(), list.estimated_count());
    
    // Test combined usage
    println!("\n=== Combined Usage ===");
    
    let msg_status = MessageStatus::new();
    let msg_priority = Priority::High;
    let msg_recipient = Recipient::PhoneNumber(String::from("+1555123456"));
    
    println!("{}", process_message(msg_status, msg_priority, msg_recipient));
}

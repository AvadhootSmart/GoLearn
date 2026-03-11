// Exercise 1: Basic Enums - Complete Solution

#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered { timestamp: u64 },
    Failed { error_code: u16, error_message: String },
}

impl MessageStatus {
    pub fn new() -> Self {
        MessageStatus::Pending
    }
    
    pub fn mark_sent(self) -> Self {
        match self {
            MessageStatus::Pending => MessageStatus::Sent,
            other => other,
        }
    }
    
    pub fn mark_delivered(self, timestamp: u64) -> Self {
        match self {
            MessageStatus::Sent => MessageStatus::Delivered { timestamp },
            other => other,
        }
    }
    
    pub fn mark_failed(self, code: u16, message: &str) -> Self {
        match self {
            MessageStatus::Pending | MessageStatus::Sent => {
                MessageStatus::Failed {
                    error_code: code,
                    error_message: message.to_string(),
                }
            }
            other => other,
        }
    }
    
    pub fn is_in_progress(&self) -> bool {
        matches!(self, MessageStatus::Pending | MessageStatus::Sent)
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(self, MessageStatus::Delivered { .. } | MessageStatus::Failed { .. })
    }
    
    pub fn description(&self) -> String {
        match self {
            MessageStatus::Pending => String::from("Message is pending"),
            MessageStatus::Sent => String::from("Message was sent"),
            MessageStatus::Delivered { timestamp } => {
                format!("Message delivered at {}", timestamp)
            }
            MessageStatus::Failed { error_code, error_message } => {
                format!("Message failed (code {}): {}", error_code, error_message)
            }
        }
    }
    
    pub fn error_code(&self) -> Option<u16> {
        match self {
            MessageStatus::Failed { error_code, .. } => Some(*error_code),
            _ => None,
        }
    }
}

impl Default for MessageStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Priority {
    pub fn level(&self) -> u8 {
        match self {
            Priority::Low => 0,
            Priority::Normal => 1,
            Priority::High => 2,
            Priority::Urgent => 3,
        }
    }
    
    pub fn is_urgent(&self) -> bool {
        matches!(self, Priority::Urgent)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Recipient {
    PhoneNumber(String),
    GroupId(u64),
    ContactList { name: String, count: u32 },
}

impl Recipient {
    pub fn display(&self) -> String {
        match self {
            Recipient::PhoneNumber(number) => format!("Phone: {}", number),
            Recipient::GroupId(id) => format!("Group #{}", id),
            Recipient::ContactList { name, count } => {
                format!("List '{}' ({} contacts)", name, count)
            }
        }
    }
    
    pub fn is_individual(&self) -> bool {
        matches!(self, Recipient::PhoneNumber(_))
    }
    
    pub fn estimated_count(&self) -> u32 {
        match self {
            Recipient::PhoneNumber(_) => 1,
            Recipient::GroupId(id) => {
                if *id == 0 { 0 } else { 100 }
            }
            Recipient::ContactList { count, .. } => *count,
        }
    }
}

fn process_message(status: MessageStatus, priority: Priority, recipient: Recipient) -> String {
    format!(
        "Sending to {} with {:?} priority. Status: {}",
        recipient.display(),
        priority,
        status.description()
    )
}

fn main() {
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
    
    println!("\n=== Combined Usage ===");
    
    let msg_status = MessageStatus::new();
    let msg_priority = Priority::High;
    let msg_recipient = Recipient::PhoneNumber(String::from("+1555123456"));
    
    println!("{}", process_message(msg_status, msg_priority, msg_recipient));
}

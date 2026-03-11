// Exercise 5: Common Standard Library Traits - Complete Solution

use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct MessageId {
    pub id: String,
    pub timestamp: u64,
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "msg_{}@{}", self.id, self.timestamp)
    }
}

impl Default for MessageId {
    fn default() -> Self {
        MessageId {
            id: String::from("unknown"),
            timestamp: 0,
        }
    }
}

impl From<&str> for MessageId {
    fn from(s: &str) -> Self {
        if let Some(stripped) = s.strip_prefix("msg_") {
            if let Some(at_pos) = stripped.find('@') {
                let id = &stripped[..at_pos];
                let timestamp_str = &stripped[at_pos + 1..];
                let timestamp = timestamp_str.parse().unwrap_or(0);
                return MessageId {
                    id: id.to_string(),
                    timestamp,
                };
            }
        }
        MessageId::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Normal => write!(f, "normal"),
            Priority::High => write!(f, "high"),
            Priority::Urgent => write!(f, "urgent"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SmsMessage {
    pub id: MessageId,
    pub to: String,
    pub from: String,
    pub body: String,
    pub priority: Priority,
}

impl fmt::Display for SmsMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}] {}", self.priority, self.id)?;
        writeln!(f, "From: {}", self.from)?;
        writeln!(f, "To: {}", self.to)?;
        write!(f, "{}", self.body)
    }
}

impl PartialEq for SmsMessage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.to == other.to && self.body == other.body
    }
}

impl Default for SmsMessage {
    fn default() -> Self {
        SmsMessage {
            id: MessageId::default(),
            to: String::new(),
            from: String::new(),
            body: String::new(),
            priority: Priority::Normal,
        }
    }
}

impl From<&str> for SmsMessage {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('|').collect();
        match parts.as_slice() {
            [to, from, body] => SmsMessage {
                id: MessageId::default(),
                to: to.to_string(),
                from: from.to_string(),
                body: body.to_string(),
                priority: Priority::Normal,
            },
            _ => SmsMessage::default(),
        }
    }
}

pub struct PhoneNumber(pub String);

impl TryFrom<&str> for PhoneNumber {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.starts_with('+') && s.len() >= 10 {
            Ok(PhoneNumber(s.to_string()))
        } else {
            Err("Invalid phone number format")
        }
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmsConfig {
    pub max_length: usize,
    pub encoding: String,
    pub require_delivery_report: bool,
}

impl Default for SmsConfig {
    fn default() -> Self {
        SmsConfig {
            max_length: 160,
            encoding: String::from("GSM-7"),
            require_delivery_report: false,
        }
    }
}

trait MessageExt: fmt::Debug + Clone {
    fn summary(&self) -> String;
}

impl MessageExt for SmsMessage {
    fn summary(&self) -> String {
        format!("SMS[{}] to: {} ({} chars)", self.id.id, self.to, self.body.len())
    }
}

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

    println!("\n=== Priority ===");
    let low = Priority::Low;
    let high = Priority::High;
    println!("Low: {} < High: {} ? {}", low, high, low < high);
    println!("Debug: {:?}", high);

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

    let msg_from_str: SmsMessage = "+15551112222|+15553334444|Test message".into();
    println!("\nFrom string:");
    println!("{}", msg_from_str);

    println!("\n=== Equality ===");
    let msg2 = msg.clone();
    compare_messages(&msg, &msg2);

    let msg3 = SmsMessage {
        id: MessageId::default(),
        ..Default::default()
    };
    compare_messages(&msg, &msg3);

    println!("\n=== Defaults ===");
    let default_msg = SmsMessage::default();
    println!("Default message: {}", default_msg);

    let default_config = SmsConfig::default();
    println!("Default config: {:?}", default_config);

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

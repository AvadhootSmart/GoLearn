// Exercise 3: Default Implementations - Complete Solution

pub trait Serializable {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Option<Self> where Self: Sized;
    
    fn serialize(&self) -> String {
        format!("SERIAL:{}", self.to_string())
    }
    
    fn deserialize(s: &str) -> Option<Self> where Self: Sized {
        s.strip_prefix("SERIAL:").and_then(Self::from_string)
    }
}

pub trait Formattable {
    fn raw_content(&self) -> &str;
    
    fn formatted(&self) -> String {
        format!("[{}]", self.raw_content())
    }
    
    fn length(&self) -> usize {
        self.raw_content().len()
    }
    
    fn is_long(&self) -> bool {
        self.length() > 100
    }
}

pub trait Validatable {
    fn validate_basic(&self) -> Result<(), String>;
    
    fn validate_strict(&self) -> Result<(), String> {
        self.validate_basic()
    }
    
    fn is_valid(&self) -> bool {
        self.validate_basic().is_ok()
    }
}

pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}

impl Serializable for SmsMessage {
    fn to_string(&self) -> String {
        format!("SMS|{}|{}|{}", self.to, self.from, self.body)
    }
    
    fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 4 || parts[0] != "SMS" {
            return None;
        }
        Some(SmsMessage {
            to: parts[1].to_string(),
            from: parts[2].to_string(),
            body: parts[3].to_string(),
        })
    }
}

impl Formattable for SmsMessage {
    fn raw_content(&self) -> &str {
        &self.body
    }
}

impl Validatable for SmsMessage {
    fn validate_basic(&self) -> Result<(), String> {
        if self.to.is_empty() {
            return Err("Recipient is empty".to_string());
        }
        if self.body.is_empty() {
            return Err("Body is empty".to_string());
        }
        Ok(())
    }
}

pub struct EmailMessage {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl Serializable for EmailMessage {
    fn to_string(&self) -> String {
        format!("EMAIL|{}|{}|{}|{}", self.to, self.from, self.subject, self.body)
    }
    
    fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 5 || parts[0] != "EMAIL" {
            return None;
        }
        Some(EmailMessage {
            to: parts[1].to_string(),
            from: parts[2].to_string(),
            subject: parts[3].to_string(),
            body: parts[4].to_string(),
        })
    }
}

impl Formattable for EmailMessage {
    fn raw_content(&self) -> &str {
        &self.body
    }
    
    // Override default formatted() for EmailMessage
    fn formatted(&self) -> String {
        format!("[{}] {}", self.subject, self.body)
    }
}

impl Validatable for EmailMessage {
    fn validate_basic(&self) -> Result<(), String> {
        if self.to.is_empty() {
            return Err("Recipient is empty".to_string());
        }
        if self.subject.is_empty() {
            return Err("Subject is empty".to_string());
        }
        if self.body.is_empty() {
            return Err("Body is empty".to_string());
        }
        Ok(())
    }
}

fn test_serialization<T: Serializable + std::fmt::Debug>(item: &T, name: &str) {
    println!("=== {} Serialization ===", name);
    let serialized = item.serialize();
    println!("Serialized: {}", serialized);
}

fn test_validation<T: Validatable>(item: &T, name: &str) {
    println!("=== {} Validation ===", name);
    println!("Is valid: {}", item.is_valid());
    match item.validate_strict() {
        Ok(_) => println!("Strict validation passed"),
        Err(e) => println!("Strict validation failed: {}", e),
    }
}

fn main() {
    let sms = SmsMessage {
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: String::from("Hello from Textio!"),
    };

    let email = EmailMessage {
        to: String::from("user@example.com"),
        from: String::from("noreply@textio.com"),
        subject: String::from("Welcome"),
        body: String::from("Thank you for joining Textio!"),
    };

    test_serialization(&sms, "SMS");
    test_serialization(&email, "Email");

    println!("\n=== Deserialization ===");
    let sms_data = "SMS|+15551112222|+15553334444|Test message";
    match SmsMessage::from_string(sms_data) {
        Some(msg) => println!("Deserialized SMS to: {}", msg.to),
        None => println!("Failed to deserialize SMS"),
    }

    println!("\n=== Formatting ===");
    println!("SMS formatted: {}", sms.formatted());
    println!("SMS length: {}", sms.length());
    println!("Email formatted: {}", email.formatted());
    println!("Email length: {}", email.length());

    println!("\n=== Validation ===");
    test_validation(&sms, "SMS");
    test_validation(&email, "Email");

    let invalid_sms = SmsMessage {
        to: String::from(""),
        from: String::from("+15559876543"),
        body: String::from(""),
    };
    println!("\n=== Invalid SMS ===");
    println!("Is valid: {}", invalid_sms.is_valid());
    match invalid_sms.validate_strict() {
        Ok(_) => println!("Validation passed"),
        Err(e) => println!("Validation failed: {}", e),
    }

    let long_body = "x".repeat(150);
    let long_sms = SmsMessage {
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: long_body,
    };
    println!("\n=== Long Message Test ===");
    println!("Is long: {}", long_sms.is_long());
    println!("Length: {}", long_sms.length());
}

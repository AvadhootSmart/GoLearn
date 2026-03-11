// Exercise 3: Default Implementations
//
// Learn to create and override default trait implementations.
// Complete the TODO sections to implement the serialization system.

// TODO: Define a Serializable trait with:
// Required methods:
//   - to_string(&self) -> String: Convert to string representation
//   - from_string(s: &str) -> Option<Self> where Self: Sized
// Default methods:
//   - serialize(&self) -> String: Returns "SERIAL:" + to_string()
//   - deserialize(s: &str) -> Option<Self> where Self: Sized:
//     Strips "SERIAL:" prefix and calls from_string



// TODO: Define a Formattable trait with:
// Required methods:
//   - raw_content(&self) -> &str
// Default methods:
//   - formatted(&self) -> String: Returns raw_content wrapped in brackets
//   - length(&self) -> usize: Returns length of raw_content
//   - is_long(&self) -> bool: Returns true if length > 100



// TODO: Define a Validatable trait with:
// Required methods:
//   - validate_basic(&self) -> Result<(), String>
// Default methods:
//   - validate_strict(&self) -> Result<(), String>: Calls validate_basic
//   - is_valid(&self) -> bool: Returns true if validate_basic succeeds



// SMS Message struct
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}

// TODO: Implement Serializable for SmsMessage
// to_string format: "SMS|{to}|{from}|{body}"
// from_string: parse the format above



// TODO: Implement Formattable for SmsMessage
// raw_content returns the body



// TODO: Implement Validatable for SmsMessage
// validate_basic: check that to and body are not empty



// Email Message struct
pub struct EmailMessage {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

// TODO: Implement Serializable for EmailMessage
// to_string format: "EMAIL|{to}|{from}|{subject}|{body}"



// TODO: Implement Formattable for EmailMessage
// raw_content returns the body



// TODO: Implement Validatable for EmailMessage
// validate_basic: check that to, subject, and body are not empty



// TODO: Override the formatted() default for EmailMessage
// Return "[{subject}] {body}" format



// Helper function to test serialization
fn test_serialization<T: Serializable + std::fmt::Debug>(item: &T, name: &str) {
    println!("=== {} Serialization ===", name);
    let serialized = item.serialize();
    println!("Serialized: {}", serialized);
}

// Helper function to test validation
fn test_validation<T: Validatable>(item: &T, name: &str) {
    println!("=== {} Validation ===", name);
    println!("Is valid: {}", item.is_valid());
    match item.validate_strict() {
        Ok(_) => println!("Strict validation passed"),
        Err(e) => println!("Strict validation failed: {}", e),
    }
}

fn main() {
    // Create test messages
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

    // Test serialization
    test_serialization(&sms, "SMS");
    test_serialization(&email, "Email");

    // Test deserialization
    println!("\n=== Deserialization ===");
    let sms_data = "SMS|+15551112222|+15553334444|Test message";
    match SmsMessage::from_string(sms_data) {
        Some(msg) => println!("Deserialized SMS to: {}", msg.to),
        None => println!("Failed to deserialize SMS"),
    }

    // Test formatting
    println!("\n=== Formatting ===");
    println!("SMS formatted: {}", sms.formatted());
    println!("SMS length: {}", sms.length());
    println!("Email formatted: {}", email.formatted());
    println!("Email length: {}", email.length());

    // Test validation
    println!("\n=== Validation ===");
    test_validation(&sms, "SMS");
    test_validation(&email, "Email");

    // Test invalid message
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

    // Test long message
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

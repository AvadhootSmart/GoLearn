// Textio SMS API - Result Type Exercise
// Practice working with Result<T, E> for error handling

use std::fmt;

#[derive(Debug)]
pub enum SmsError {
    InvalidPhoneFormat,
    InvalidPhoneLength,
    MessageTooLong,
    InvalidLengthString,
}

impl fmt::Display for SmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmsError::InvalidPhoneFormat => write!(f, "Phone number contains invalid characters"),
            SmsError::InvalidPhoneLength => write!(f, "Phone number must be 10 digits"),
            SmsError::MessageTooLong => write!(f, "Message exceeds 160 characters"),
            SmsError::InvalidLengthString => write!(f, "Could not parse length value"),
        }
    }
}

#[derive(Debug)]
pub struct MessageId(pub u64);

// TODO: Implement parse_phone_number
// Requirements:
// - Phone must be exactly 10 characters
// - All characters must be digits (0-9)
// - Return Ok(cleaned_phone) on success
// - Return Err(SmsError::InvalidPhoneLength) if wrong length
// - Return Err(SmsError::InvalidPhoneFormat) if contains non-digits
pub fn parse_phone_number(phone: &str) -> Result<String, SmsError> {
    // Your code here
    todo!()
}

// TODO: Implement parse_message_length
// Requirements:
// - Parse the string as a usize
// - Return Ok(length) on success
// - Return Err(SmsError::InvalidLengthString) on failure
// Use the parse() method on str
pub fn parse_message_length(length_str: &str) -> Result<usize, SmsError> {
    // Your code here
    todo!()
}

// TODO: Implement validate_message
// Requirements:
// - Message must be 160 characters or less
// - Return Ok(message.len()) on success
// - Return Err(SmsError::MessageTooLong) if too long
pub fn validate_message(message: &str) -> Result<usize, SmsError> {
    // Your code here
    todo!()
}

// TODO: Implement send_sms
// Requirements:
// - Validate phone using parse_phone_number
// - Validate message using validate_message
// - Return Ok(MessageId(12345)) on success
// - Propagate any errors from validation
pub fn send_sms(phone: &str, message: &str) -> Result<MessageId, SmsError> {
    // Your code here - use pattern matching to handle both validations
    todo!()
}

// Helper function to demonstrate is_ok() and is_err()
pub fn check_result(result: &Result<String, SmsError>) -> String {
    if result.is_ok() {
        format!("Valid: {}", result.as_ref().unwrap())
    } else {
        format!("Invalid: {}", result.as_ref().unwrap_err())
    }
}

// Helper function to demonstrate unwrap_or()
pub fn get_phone_with_default(result: Result<String, SmsError>) -> String {
    result.unwrap_or(String::from("0000000000"))
}

fn main() {
    println!("=== Textio SMS API - Result Type Demo ===\n");

    // Test parse_phone_number
    println!("--- Phone Number Parsing ---");
    
    let valid_phone = parse_phone_number("5551234567");
    println!("Valid phone '5551234567': {:?}", valid_phone);
    
    let short_phone = parse_phone_number("123");
    println!("Short phone '123': {:?}", short_phone);
    
    let invalid_phone = parse_phone_number("555abc4567");
    println!("Invalid phone '555abc4567': {:?}", invalid_phone);
    
    // Test is_ok() and is_err()
    println!("\n--- Using is_ok() and is_err() ---");
    println!("Is valid_phone ok? {}", valid_phone.is_ok());
    println!("Is short_phone err? {}", short_phone.is_err());
    
    // Test check_result helper
    println!("\n--- Using check_result ---");
    println!("{}", check_result(&valid_phone));
    println!("{}", check_result(&short_phone));
    
    // Test parse_message_length
    println!("\n--- Message Length Parsing ---");
    
    let valid_length = parse_message_length("50");
    println!("Valid length '50': {:?}", valid_length);
    
    let invalid_length = parse_message_length("abc");
    println!("Invalid length 'abc': {:?}", invalid_length);
    
    // Test unwrap_or()
    println!("\n--- Using unwrap_or() ---");
    let phone = get_phone_with_default(valid_phone.clone());
    println!("Valid phone with default: {}", phone);
    
    let default_phone = get_phone_with_default(short_phone);
    println!("Invalid phone with default: {}", default_phone);
    
    // Test validate_message
    println!("\n--- Message Validation ---");
    
    let short_msg = validate_message("Hello, World!");
    println!("Short message: {:?}", short_msg);
    
    let long_message = "x".repeat(200);
    let long_msg = validate_message(&long_message);
    println!("Long message (200 chars): {:?}", long_msg);
    
    // Test send_sms
    println!("\n--- Send SMS ---");
    
    let result1 = send_sms("5551234567", "Hello!");
    println!("Valid SMS: {:?}", result1);
    
    let result2 = send_sms("123", "Hello!");
    println!("Invalid phone: {:?}", result2);
    
    let result3 = send_sms("5551234567", &"x".repeat(200));
    println!("Message too long: {:?}", result3);
    
    // Demonstrate unwrap() and expect()
    println!("\n--- unwrap() and expect() ---");
    
    let phone_result = parse_phone_number("5559876543");
    let phone_value = phone_result.unwrap();
    println!("Unwrapped phone: {}", phone_value);
    
    let length_result = parse_message_length("100");
    let length_value = length_result.expect("Length should be valid");
    println!("Expected length: {}", length_value);
    
    println!("\n=== Exercise Complete ===");
}

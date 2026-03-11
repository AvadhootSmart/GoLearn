// Textio SMS API - Result Type Exercise (Complete Solution)
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

pub fn parse_phone_number(phone: &str) -> Result<String, SmsError> {
    if phone.len() != 10 {
        return Err(SmsError::InvalidPhoneLength);
    }
    
    if !phone.chars().all(|c| c.is_ascii_digit()) {
        return Err(SmsError::InvalidPhoneFormat);
    }
    
    Ok(phone.to_string())
}

pub fn parse_message_length(length_str: &str) -> Result<usize, SmsError> {
    length_str
        .parse::<usize>()
        .map_err(|_| SmsError::InvalidLengthString)
}

pub fn validate_message(message: &str) -> Result<usize, SmsError> {
    let len = message.len();
    if len > 160 {
        return Err(SmsError::MessageTooLong);
    }
    Ok(len)
}

pub fn send_sms(phone: &str, message: &str) -> Result<MessageId, SmsError> {
    match parse_phone_number(phone) {
        Ok(_) => {
            match validate_message(message) {
                Ok(_) => Ok(MessageId(12345)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn check_result(result: &Result<String, SmsError>) -> String {
    if result.is_ok() {
        format!("Valid: {}", result.as_ref().unwrap())
    } else {
        format!("Invalid: {}", result.as_ref().unwrap_err())
    }
}

pub fn get_phone_with_default(result: Result<String, SmsError>) -> String {
    result.unwrap_or(String::from("0000000000"))
}

fn main() {
    println!("=== Textio SMS API - Result Type Demo ===\n");

    println!("--- Phone Number Parsing ---");
    
    let valid_phone = parse_phone_number("5551234567");
    println!("Valid phone '5551234567': {:?}", valid_phone);
    
    let short_phone = parse_phone_number("123");
    println!("Short phone '123': {:?}", short_phone);
    
    let invalid_phone = parse_phone_number("555abc4567");
    println!("Invalid phone '555abc4567': {:?}", invalid_phone);
    
    println!("\n--- Using is_ok() and is_err() ---");
    println!("Is valid_phone ok? {}", valid_phone.is_ok());
    println!("Is short_phone err? {}", short_phone.is_err());
    
    println!("\n--- Using check_result ---");
    println!("{}", check_result(&valid_phone));
    println!("{}", check_result(&short_phone));
    
    println!("\n--- Message Length Parsing ---");
    
    let valid_length = parse_message_length("50");
    println!("Valid length '50': {:?}", valid_length);
    
    let invalid_length = parse_message_length("abc");
    println!("Invalid length 'abc': {:?}", invalid_length);
    
    println!("\n--- Using unwrap_or() ---");
    let phone = get_phone_with_default(valid_phone.clone());
    println!("Valid phone with default: {}", phone);
    
    let default_phone = get_phone_with_default(short_phone);
    println!("Invalid phone with default: {}", default_phone);
    
    println!("\n--- Message Validation ---");
    
    let short_msg = validate_message("Hello, World!");
    println!("Short message: {:?}", short_msg);
    
    let long_message = "x".repeat(200);
    let long_msg = validate_message(&long_message);
    println!("Long message (200 chars): {:?}", long_msg);
    
    println!("\n--- Send SMS ---");
    
    let result1 = send_sms("5551234567", "Hello!");
    println!("Valid SMS: {:?}", result1);
    
    let result2 = send_sms("123", "Hello!");
    println!("Invalid phone: {:?}", result2);
    
    let result3 = send_sms("5551234567", &"x".repeat(200));
    println!("Message too long: {:?}", result3);
    
    println!("\n--- unwrap() and expect() ---");
    
    let phone_result = parse_phone_number("5559876543");
    let phone_value = phone_result.unwrap();
    println!("Unwrapped phone: {}", phone_value);
    
    let length_result = parse_message_length("100");
    let length_value = length_result.expect("Length should be valid");
    println!("Expected length: {}", length_value);
    
    println!("\n=== Exercise Complete ===");
}

// Textio SMS API - Custom Errors Exercise
// Practice creating custom error types with std::error::Error

use std::fmt;
use std::num::ParseIntError;

// TODO: Define ValidationError enum
// Requirements:
// - InvalidPhone: contains the invalid phone string and a reason
// - MessageTooLong: contains length and max (both usize)
// - EmptyField: contains the field name
// - InvalidFormat: contains field name and expected format
#[derive(Debug)]
pub enum ValidationError {
    // Your variants here
}

// TODO: Implement Display for ValidationError
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Your code here
        todo!()
    }
}

// TODO: Implement std::error::Error for ValidationError
// (Use the default implementation - just an empty impl block)


// TODO: Define ApiError enum
// Requirements:
// - NotFound: resource name and id
// - Unauthorized: optional reason string
// - RateLimited: retry_after seconds
// - Internal: error message
#[derive(Debug)]
pub enum ApiError {
    // Your variants here
}

// TODO: Implement Display for ApiError
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Your code here
        todo!()
    }
}

// TODO: Implement std::error::Error for ApiError


// TODO: Define TextioError enum
// Requirements:
// - Validation: wraps ValidationError
// - Api: wraps ApiError
// - Parse: wraps ParseIntError
// - Io: contains error message
#[derive(Debug)]
pub enum TextioError {
    // Your variants here
}

// TODO: Implement Display for TextioError
impl fmt::Display for TextioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Your code here
        todo!()
    }
}

// TODO: Implement std::error::Error for TextioError
// Requirements:
// - Return source() for wrapped errors
impl std::error::Error for TextioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Your code here
        todo!()
    }
}

// TODO: Implement From<ValidationError> for TextioError
impl From<ValidationError> for TextioError {
    fn from(e: ValidationError) -> Self {
        // Your code here
        todo!()
    }
}

// TODO: Implement From<ApiError> for TextioError
impl From<ApiError> for TextioError {
    fn from(e: ApiError) -> Self {
        // Your code here
        todo!()
    }
}

// TODO: Implement From<ParseIntError> for TextioError
impl From<ParseIntError> for TextioError {
    fn from(e: ParseIntError) -> Self {
        // Your code here
        todo!()
    }
}

// Functions that return custom errors

// TODO: Implement validate_phone
// Requirements:
// - Phone must be 10 digits
// - Return Ok(phone) if valid
// - Return Err(TextioError::Validation(ValidationError::InvalidPhone)) if invalid
pub fn validate_phone(phone: &str) -> Result<String, TextioError> {
    // Your code here
    todo!()
}

// TODO: Implement validate_message
// Requirements:
// - Message must not be empty
// - Message must be <= 160 characters
// - Return appropriate ValidationError variants
pub fn validate_message(message: &str) -> Result<String, TextioError> {
    // Your code here
    todo!()
}

// TODO: Implement parse_message_id
// Requirements:
// - Parse string to u64
// - Use ? to automatically convert ParseIntError via From trait
pub fn parse_message_id(s: &str) -> Result<u64, TextioError> {
    // Your code here
    todo!()
}

// TODO: Implement get_message
// Requirements:
// - Simulate looking up a message
// - Return Ok(format!("Message {}", id)) if id > 0
// - Return Err(ApiError::NotFound) if id == 0
pub fn get_message(id: u64) -> Result<String, TextioError> {
    // Your code here
    todo!()
}

// TODO: Implement send_sms
// Requirements:
// - Validate phone and message using ?
// - Call get_message(1) to simulate getting a template using ?
// - Return Ok(format!("Sent '{}' to {}", template, phone))
pub fn send_sms(phone: &str, message: &str) -> Result<String, TextioError> {
    // Your code here
    todo!()
}

fn main() {
    println!("=== Textio SMS API - Custom Errors Demo ===\n");

    // Test ValidationError
    println!("--- Validation Errors ---");
    
    let err1 = ValidationError::InvalidPhone {
        phone: "123".to_string(),
        reason: "must be 10 digits".to_string(),
    };
    println!("InvalidPhone: {}", err1);
    
    let err2 = ValidationError::MessageTooLong {
        length: 200,
        max: 160,
    };
    println!("MessageTooLong: {}", err2);
    
    let err3 = ValidationError::EmptyField {
        field: "recipient".to_string(),
    };
    println!("EmptyField: {}", err3);
    
    let err4 = ValidationError::InvalidFormat {
        field: "date".to_string(),
        expected: "YYYY-MM-DD".to_string(),
    };
    println!("InvalidFormat: {}", err4);

    // Test ApiError
    println!("\n--- API Errors ---");
    
    let api1 = ApiError::NotFound {
        resource: "Message".to_string(),
        id: 42,
    };
    println!("NotFound: {}", api1);
    
    let api2 = ApiError::Unauthorized(Some("Token expired".to_string()));
    println!("Unauthorized: {}", api2);
    
    let api3 = ApiError::RateLimited { retry_after: 60 };
    println!("RateLimited: {}", api3);
    
    let api4 = ApiError::Internal("Database connection failed".to_string());
    println!("Internal: {}", api4);

    // Test TextioError
    println!("\n--- Textio Errors ---");
    
    let textio1 = TextioError::Validation(ValidationError::EmptyField {
        field: "body".to_string(),
    });
    println!("TextioError Validation: {}", textio1);
    
    let textio2 = TextioError::Api(ApiError::NotFound {
        resource: "Contact".to_string(),
        id: 99,
    });
    println!("TextioError Api: {}", textio2);

    // Test validate_phone
    println!("\n--- Phone Validation ---");
    
    let valid = validate_phone("5551234567");
    println!("Valid phone: {:?}", valid);
    
    let invalid = validate_phone("abc");
    println!("Invalid phone: {:?}", invalid);

    // Test validate_message
    println!("\n--- Message Validation ---");
    
    let valid_msg = validate_message("Hello, World!");
    println!("Valid message: {:?}", valid_msg);
    
    let empty_msg = validate_message("");
    println!("Empty message: {:?}", empty_msg);
    
    let long_msg = validate_message(&"x".repeat(200));
    println!("Long message: {:?}", long_msg);

    // Test parse_message_id with From trait
    println!("\n--- Parse Message ID (From trait) ---");
    
    let valid_id = parse_message_id("12345");
    println!("Valid ID: {:?}", valid_id);
    
    let invalid_id = parse_message_id("abc");
    println!("Invalid ID: {:?}", invalid_id);

    // Test get_message
    println!("\n--- Get Message ---");
    
    let found = get_message(1);
    println!("Found message: {:?}", found);
    
    let not_found = get_message(0);
    println!("Not found: {:?}", not_found);

    // Test send_sms (full chain)
    println!("\n--- Send SMS (Full Chain) ---");
    
    let success = send_sms("5551234567", "Hello!");
    println!("Success: {:?}", success);
    
    let bad_phone = send_sms("123", "Hello!");
    println!("Bad phone: {:?}", bad_phone);
    
    let empty_msg = send_sms("5551234567", "");
    println!("Empty message: {:?}", empty_msg);

    // Test error source()
    println!("\n--- Error Source ---");
    
    let wrapped = TextioError::Validation(ValidationError::EmptyField {
        field: "test".to_string(),
    });
    println!("Has source: {}", wrapped.source().is_some());

    println!("\n=== Exercise Complete ===");
}

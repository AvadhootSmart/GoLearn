// Textio SMS API - Custom Errors Exercise (Complete Solution)
// Practice creating custom error types with std::error::Error

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ValidationError {
    InvalidPhone { phone: String, reason: String },
    MessageTooLong { length: usize, max: usize },
    EmptyField { field: String },
    InvalidFormat { field: String, expected: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidPhone { phone, reason } => {
                write!(f, "Invalid phone '{}': {}", phone, reason)
            }
            ValidationError::MessageTooLong { length, max } => {
                write!(f, "Message too long: {} characters (max: {})", length, max)
            }
            ValidationError::EmptyField { field } => {
                write!(f, "Field '{}' cannot be empty", field)
            }
            ValidationError::InvalidFormat { field, expected } => {
                write!(f, "Invalid format for '{}', expected: {}", field, expected)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug)]
pub enum ApiError {
    NotFound { resource: String, id: u64 },
    Unauthorized(Option<String>),
    RateLimited { retry_after: u64 },
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NotFound { resource, id } => {
                write!(f, "{} with id {} not found", resource, id)
            }
            ApiError::Unauthorized(reason) => {
                match reason {
                    Some(r) => write!(f, "Unauthorized: {}", r),
                    None => write!(f, "Unauthorized"),
                }
            }
            ApiError::RateLimited { retry_after } => {
                write!(f, "Rate limited, retry after {} seconds", retry_after)
            }
            ApiError::Internal(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug)]
pub enum TextioError {
    Validation(ValidationError),
    Api(ApiError),
    Parse(ParseIntError),
    Io(String),
}

impl fmt::Display for TextioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextioError::Validation(e) => write!(f, "Validation error: {}", e),
            TextioError::Api(e) => write!(f, "API error: {}", e),
            TextioError::Parse(e) => write!(f, "Parse error: {}", e),
            TextioError::Io(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TextioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TextioError::Validation(e) => Some(e),
            TextioError::Api(e) => Some(e),
            TextioError::Parse(e) => Some(e),
            TextioError::Io(_) => None,
        }
    }
}

impl From<ValidationError> for TextioError {
    fn from(e: ValidationError) -> Self {
        TextioError::Validation(e)
    }
}

impl From<ApiError> for TextioError {
    fn from(e: ApiError) -> Self {
        TextioError::Api(e)
    }
}

impl From<ParseIntError> for TextioError {
    fn from(e: ParseIntError) -> Self {
        TextioError::Parse(e)
    }
}

pub fn validate_phone(phone: &str) -> Result<String, TextioError> {
    if phone.len() == 10 && phone.chars().all(|c| c.is_ascii_digit()) {
        Ok(phone.to_string())
    } else {
        Err(ValidationError::InvalidPhone {
            phone: phone.to_string(),
            reason: "must be 10 digits".to_string(),
        }.into())
    }
}

pub fn validate_message(message: &str) -> Result<String, TextioError> {
    if message.is_empty() {
        return Err(ValidationError::EmptyField {
            field: "message".to_string(),
        }.into());
    }
    if message.len() > 160 {
        return Err(ValidationError::MessageTooLong {
            length: message.len(),
            max: 160,
        }.into());
    }
    Ok(message.to_string())
}

pub fn parse_message_id(s: &str) -> Result<u64, TextioError> {
    Ok(s.parse::<u64>()?)
}

pub fn get_message(id: u64) -> Result<String, TextioError> {
    if id > 0 {
        Ok(format!("Message {}", id))
    } else {
        Err(ApiError::NotFound {
            resource: "Message".to_string(),
            id,
        }.into())
    }
}

pub fn send_sms(phone: &str, message: &str) -> Result<String, TextioError> {
    let phone = validate_phone(phone)?;
    let _ = validate_message(message)?;
    let template = get_message(1)?;
    Ok(format!("Sent '{}' to {}", template, phone))
}

fn main() {
    println!("=== Textio SMS API - Custom Errors Demo ===\n");

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

    println!("\n--- Phone Validation ---");
    
    let valid = validate_phone("5551234567");
    println!("Valid phone: {:?}", valid);
    
    let invalid = validate_phone("abc");
    println!("Invalid phone: {:?}", invalid);

    println!("\n--- Message Validation ---");
    
    let valid_msg = validate_message("Hello, World!");
    println!("Valid message: {:?}", valid_msg);
    
    let empty_msg = validate_message("");
    println!("Empty message: {:?}", empty_msg);
    
    let long_msg = validate_message(&"x".repeat(200));
    println!("Long message: {:?}", long_msg);

    println!("\n--- Parse Message ID (From trait) ---");
    
    let valid_id = parse_message_id("12345");
    println!("Valid ID: {:?}", valid_id);
    
    let invalid_id = parse_message_id("abc");
    println!("Invalid ID: {:?}", invalid_id);

    println!("\n--- Get Message ---");
    
    let found = get_message(1);
    println!("Found message: {:?}", found);
    
    let not_found = get_message(0);
    println!("Not found: {:?}", not_found);

    println!("\n--- Send SMS (Full Chain) ---");
    
    let success = send_sms("5551234567", "Hello!");
    println!("Success: {:?}", success);
    
    let bad_phone = send_sms("123", "Hello!");
    println!("Bad phone: {:?}", bad_phone);
    
    let empty_msg = send_sms("5551234567", "");
    println!("Empty message: {:?}", empty_msg);

    println!("\n--- Error Source ---");
    
    let wrapped = TextioError::Validation(ValidationError::EmptyField {
        field: "test".to_string(),
    });
    println!("Has source: {}", wrapped.source().is_some());

    println!("\n=== Exercise Complete ===");
}

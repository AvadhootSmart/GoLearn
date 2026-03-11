// Textio Generic Enums Exercise
// Implement generic enums for Textio's SMS API

use std::fmt::Debug;

// TODO: Create a custom Option<T> enum with variants Some(T) and None
// This mimics Rust's built-in Option type

// enum Option<T> { ... }

// TODO: Implement methods for Option<T>
// impl<T> Option<T> {
//     fn is_some(&self) -> bool { ... }
//     fn is_none(&self) -> bool { ... }
//     fn unwrap(self) -> T { ... panic if None ... }
//     fn unwrap_or(self, default: T) -> T { ... }
//     fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> { ... }
// }

// TODO: Create a custom Result<T, E> enum with variants Ok(T) and Err(E)
// This mimics Rust's built-in Result type

// enum Result<T, E> { ... }

// TODO: Implement methods for Result<T, E>
// impl<T, E> Result<T, E> {
//     fn is_ok(&self) -> bool { ... }
//     fn is_err(&self) -> bool { ... }
//     fn ok(self) -> Option<T> { ... }
//     fn err(self) -> Option<E> { ... }
//     fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Result<U, E> { ... }
//     fn map_err<F, F2: FnOnce(E) -> F>(self, f: F2) -> Result<T, F> { ... }
//     fn unwrap_or(self, default: T) -> T { ... }
// }

// TODO: Create a MessageStatus<T> enum for tracking message states
// Variants:
// - Pending (no data)
// - Processing(T) - T represents progress info
// - Delivered { at: u64, confirmation: T }
// - Failed(String) - error message

// enum MessageStatus<T> { ... }

// TODO: Implement methods for MessageStatus<T>
// impl<T: Debug> MessageStatus<T> {
//     fn is_delivered(&self) -> bool { ... }
//     fn is_failed(&self) -> bool { ... }
//     fn progress(&self) -> Option<&T> { ... }
//     fn error_message(&self) -> Option<&String> { ... }
// }

// TODO: Create a WebResponse<T, E> enum for API responses
// Variants:
// - Success { data: T, status: u16 }
// - ClientError(E) - 4xx errors
// - ServerError { code: u16, message: String } - 5xx errors

// enum WebResponse<T, E> { ... }

// TODO: Implement methods for WebResponse<T, E>
// impl<T, E> WebResponse<T, E> {
//     fn is_success(&self) -> bool { ... }
//     fn status_code(&self) -> u16 { ... }
//     fn data(self) -> Option<T> { ... }
// }

// Sample types for Textio
#[derive(Debug, Clone, PartialEq)]
struct SmsMessage {
    id: u32,
    content: String,
    recipient: String,
}

#[derive(Debug, Clone)]
struct DeliveryProgress {
    step: u32,
    total_steps: u32,
    message: String,
}

#[derive(Debug, Clone)]
struct ApiError {
    code: u16,
    message: String,
}

fn main() {
    // Test Option<T>
    println!("=== Option Tests ===");
    
    let some_number: Option<i32> = Option::Some(42);
    let none_number: Option<i32> = Option::None;
    
    println!("Some is_some: {}", some_number.is_some());
    println!("None is_none: {}", none_number.is_none());
    println!("unwrap_or default: {}", none_number.unwrap_or(0));
    
    let doubled = some_number.map(|n| n * 2);
    println!("Mapped doubled: {:?}", doubled);
    
    let message_option: Option<SmsMessage> = Option::Some(SmsMessage {
        id: 1,
        content: "Hello".to_string(),
        recipient: "+1234567890".to_string(),
    });
    let content = message_option.map(|m| m.content);
    println!("Extracted content: {:?}", content);

    // Test Result<T, E>
    println!("\n=== Result Tests ===");
    
    let ok_result: Result<i32, String> = Result::Ok(100);
    let err_result: Result<i32, String> = Result::Err("Something went wrong".to_string());
    
    println!("Ok is_ok: {}", ok_result.is_ok());
    println!("Err is_err: {}", err_result.is_err());
    println!("ok() gives: {:?}", ok_result.clone().ok());
    println!("err() gives: {:?}", err_result.clone().err());
    
    let mapped_ok = ok_result.map(|n| n * 2);
    println!("Mapped Ok: {:?}", mapped_ok);
    
    let mapped_err = err_result.map_err(|e| format!("ERROR: {}", e));
    println!("Mapped Err: {:?}", mapped_err);
    
    let with_default = err_result.unwrap_or(0);
    println!("With default: {}", with_default);

    // Test MessageStatus<T>
    println!("\n=== MessageStatus Tests ===");
    
    let pending: MessageStatus<DeliveryProgress> = MessageStatus::Pending;
    let processing = MessageStatus::Processing(DeliveryProgress {
        step: 2,
        total_steps: 5,
        message: "Validating".to_string(),
    });
    let delivered = MessageStatus::Delivered {
        at: 1630000000,
        confirmation: DeliveryProgress {
            step: 5,
            total_steps: 5,
            message: "Complete".to_string(),
        },
    };
    let failed: MessageStatus<DeliveryProgress> = MessageStatus::Failed("Network timeout".to_string());
    
    println!("Pending is_delivered: {}", pending.is_delivered());
    println!("Delivered is_delivered: {}", delivered.is_delivered());
    println!("Failed is_failed: {}", failed.is_failed());
    println!("Processing progress: {:?}", processing.progress());
    println!("Failed error_message: {:?}", failed.error_message());

    // Test WebResponse<T, E>
    println!("\n=== WebResponse Tests ===");
    
    let success: WebResponse<SmsMessage, ApiError> = WebResponse::Success {
        data: SmsMessage {
            id: 1,
            content: "Test".to_string(),
            recipient: "+1234567890".to_string(),
        },
        status: 200,
    };
    
    let client_error: WebResponse<SmsMessage, ApiError> = WebResponse::ClientError(ApiError {
        code: 400,
        message: "Invalid phone number".to_string(),
    });
    
    let server_error: WebResponse<SmsMessage, ApiError> = WebResponse::ServerError {
        code: 503,
        message: "Service unavailable".to_string(),
    };
    
    println!("Success is_success: {}", success.is_success());
    println!("ClientError is_success: {}", client_error.is_success());
    println!("Success status: {}", success.clone().status_code());
    println!("ClientError status: {}", client_error.status_code());
    println!("ServerError status: {}", server_error.status_code());
    println!("Success data: {:?}", success.data());
    println!("ClientError data: {:?}", client_error.data());

    // Demonstrate chaining operations
    println!("\n=== Chaining Operations ===");
    
    fn find_message(id: u32) -> Option<SmsMessage> {
        if id == 1 {
            Option::Some(SmsMessage {
                id: 1,
                content: "Found".to_string(),
                recipient: "+1111111111".to_string(),
            })
        } else {
            Option::None
        }
    }
    
    fn validate_message(msg: SmsMessage) -> Result<SmsMessage, String> {
        if msg.recipient.starts_with('+') {
            Result::Ok(msg)
        } else {
            Result::Err("Invalid recipient format".to_string())
        }
    }
    
    let found = find_message(1);
    println!("Found message: {:?}", found);
    
    let validated = found.and_then(|m| match validate_message(m) {
        Result::Ok(msg) => Option::Some(msg),
        Result::Err(_) => Option::None,
    });
    println!("Validated message: {:?}", validated);
    
    let not_found = find_message(99);
    println!("Not found: {:?}", not_found);
}

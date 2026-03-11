// Exercise 3: Match Expressions
// Pattern matching for Textio message handling

#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered { at: u64 },
    Failed { code: u16, reason: String },
}

#[derive(Debug, Clone)]
pub enum Message {
    Sms { to: String, body: String },
    Mms { to: String, body: String, media_url: String },
    Scheduled { to: String, body: String, send_at: u64 },
    Template { to: String, template_id: u32, params: Vec<String> },
}

#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    InvalidNumber(String),
    RateLimited { retry_after: u64 },
    Unauthorized,
    ServerError { code: u16, message: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorClass {
    Temporary,    // Network issues, rate limits
    Permanent,     // Invalid data
    Auth,          // Authentication issues
    Unknown,
}

// TODO: Implement a function to describe message status
// Use match with pattern binding for the status variants
pub fn describe_status(status: &MessageStatus) -> String {
    todo!()
}

// TODO: Implement a function to check if status is terminal
// Use match with multiple patterns (|) for Delivered and Failed
pub fn is_terminal_status(status: &MessageStatus) -> bool {
    todo!()
}

// TODO: Implement a function to get the recipient from any message type
// Use match with struct destructuring
pub fn get_recipient(message: &Message) -> &str {
    todo!()
}

// TODO: Implement a function to estimate message cost
// SMS: 1 credit per 160 chars
// MMS: 3 credits
// Scheduled: same as SMS
// Template: 1 credit + 0.1 per param (round up)
pub fn estimate_cost(message: &Message) -> u32 {
    todo!()
}

// TODO: Implement a function to classify errors
// Use match with guards for detailed classification:
// - NetworkError: Temporary
// - InvalidNumber: Permanent
// - RateLimited: Temporary
// - Unauthorized: Auth
// - ServerError with code >= 500: Temporary
// - ServerError with code < 500: Permanent
pub fn classify_error(error: &ApiError) -> ErrorClass {
    todo!()
}

// TODO: Implement a function to get error message
// Use match with pattern binding
pub fn error_message(error: &ApiError) -> String {
    todo!()
}

// TODO: Implement a function to check if error is retryable
// Use match with guards
pub fn is_retryable(error: &ApiError) -> bool {
    todo!()
}

// TODO: Implement a function to validate phone number format
// Use match with range patterns for character validation
// Valid: starts with +, followed by 10-15 digits
pub fn validate_phone(phone: &str) -> Result<String, String> {
    todo!()
}

// TODO: Implement a function to categorize HTTP status codes
// Use match with range patterns:
// - 200-299: "success"
// - 300-399: "redirect"
// - 400-499: "client error"
// - 500-599: "server error"
// - Other: "unknown"
pub fn categorize_status_code(code: u16) -> &'static str {
    todo!()
}

// TODO: Implement a function to handle nested Option
// Returns the inner value if both are Some, None otherwise
pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    todo!()
}

// TODO: Implement a function to match on a tuple of options
// Returns appropriate message for each combination
pub fn describe_options(a: Option<i32>, b: Option<i32>) -> String {
    todo!()
}

// TODO: Implement a function to extract retry_after from error
// Use match with pattern binding, return None for non-rate-limit errors
pub fn get_retry_after(error: &ApiError) -> Option<u64> {
    todo!()
}

// TODO: Implement a function to match on message length
// Use @ binding with ranges:
// - 0: "empty"
// - 1-10: "short"
// - 11-160: "normal"
// - 161-1000: "long"
// - _ : "very long"
pub fn describe_length(len: usize) -> &'static str {
    todo!()
}

// TODO: Implement a function to match on slice patterns
// Handle different array sizes
pub fn describe_slice(slice: &[i32]) -> String {
    todo!()
}

fn main() {
    // Test describe_status
    println!("=== describe_status ===");
    println!("{}", describe_status(&MessageStatus::Pending));
    println!("{}", describe_status(&MessageStatus::Sent));
    println!("{}", describe_status(&MessageStatus::Delivered { at: 1700000000 }));
    println!("{}", describe_status(&MessageStatus::Failed { 
        code: 500, 
        reason: String::from("Timeout") 
    }));

    // Test is_terminal_status
    println!("\n=== is_terminal_status ===");
    println!("Pending: {}", is_terminal_status(&MessageStatus::Pending));
    println!("Delivered: {}", is_terminal_status(&MessageStatus::Delivered { at: 0 }));
    println!("Failed: {}", is_terminal_status(&MessageStatus::Failed { 
        code: 0, 
        reason: String::new() 
    }));

    // Test get_recipient
    println!("\n=== get_recipient ===");
    let sms = Message::Sms { 
        to: String::from("+1555123456"), 
        body: String::from("Hello") 
    };
    let mms = Message::Mms { 
        to: String::from("+1555987654"), 
        body: String::from("Check this"), 
        media_url: String::from("https://example.com/img.jpg") 
    };
    println!("SMS recipient: {}", get_recipient(&sms));
    println!("MMS recipient: {}", get_recipient(&mms));

    // Test estimate_cost
    println!("\n=== estimate_cost ===");
    println!("SMS (5 chars): {}", estimate_cost(&sms));
    let long_sms = Message::Sms { 
        to: String::from("+1555123456"), 
        body: String::from("x".repeat(200)) 
    };
    println!("SMS (200 chars): {}", estimate_cost(&long_sms));
    println!("MMS: {}", estimate_cost(&mms));
    let template = Message::Template { 
        to: String::from("+1555111111"), 
        template_id: 1, 
        params: vec![String::from("a"), String::from("b")] 
    };
    println!("Template (2 params): {}", estimate_cost(&template));

    // Test classify_error
    println!("\n=== classify_error ===");
    let network = ApiError::NetworkError(String::from("Timeout"));
    let invalid = ApiError::InvalidNumber(String::from("abc"));
    let rate_limited = ApiError::RateLimited { retry_after: 60 };
    let server_500 = ApiError::ServerError { 
        code: 500, 
        message: String::from("Internal Error") 
    };
    let server_400 = ApiError::ServerError { 
        code: 400, 
        message: String::from("Bad Request") 
    };
    
    println!("Network: {:?}", classify_error(&network));
    println!("InvalidNumber: {:?}", classify_error(&invalid));
    println!("RateLimited: {:?}", classify_error(&rate_limited));
    println!("ServerError 500: {:?}", classify_error(&server_500));
    println!("ServerError 400: {:?}", classify_error(&server_400));
    println!("Unauthorized: {:?}", classify_error(&ApiError::Unauthorized));

    // Test error_message
    println!("\n=== error_message ===");
    println!("{}", error_message(&network));
    println!("{}", error_message(&rate_limited));

    // Test is_retryable
    println!("\n=== is_retryable ===");
    println!("Network: {}", is_retryable(&network));
    println!("InvalidNumber: {}", is_retryable(&invalid));
    println!("RateLimited: {}", is_retryable(&rate_limited));

    // Test categorize_status_code
    println!("\n=== categorize_status_code ===");
    println!("200: {}", categorize_status_code(200));
    println!("301: {}", categorize_status_code(301));
    println!("404: {}", categorize_status_code(404));
    println!("500: {}", categorize_status_code(500));
    println!("99: {}", categorize_status_code(99));

    // Test describe_options
    println!("\n=== describe_options ===");
    println!("{}", describe_options(Some(1), Some(2)));
    println!("{}", describe_options(Some(1), None));
    println!("{}", describe_options(None, Some(2)));
    println!("{}", describe_options(None, None));

    // Test get_retry_after
    println!("\n=== get_retry_after ===");
    println!("RateLimited: {:?}", get_retry_after(&rate_limited));
    println!("Network: {:?}", get_retry_after(&network));

    // Test describe_length
    println!("\n=== describe_length ===");
    println!("0: {}", describe_length(0));
    println!("5: {}", describe_length(5));
    println!("100: {}", describe_length(100));
    println!("200: {}", describe_length(200));
    println!("5000: {}", describe_length(5000));

    // Test describe_slice
    println!("\n=== describe_slice ===");
    println!("{}", describe_slice(&[]));
    println!("{}", describe_slice(&[1]));
    println!("{}", describe_slice(&[1, 2]));
    println!("{}", describe_slice(&[1, 2, 3, 4, 5]));
}

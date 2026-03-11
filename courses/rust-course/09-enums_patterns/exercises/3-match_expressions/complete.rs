// Exercise 3: Match Expressions - Complete Solution

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
    Temporary,
    Permanent,
    Auth,
    Unknown,
}

pub fn describe_status(status: &MessageStatus) -> String {
    match status {
        MessageStatus::Pending => String::from("Message is pending"),
        MessageStatus::Sent => String::from("Message was sent"),
        MessageStatus::Delivered { at } => format!("Delivered at timestamp {}", at),
        MessageStatus::Failed { code, reason } => {
            format!("Failed with code {}: {}", code, reason)
        }
    }
}

pub fn is_terminal_status(status: &MessageStatus) -> bool {
    match status {
        MessageStatus::Delivered { .. } | MessageStatus::Failed { .. } => true,
        _ => false,
    }
}

pub fn get_recipient(message: &Message) -> &str {
    match message {
        Message::Sms { to, .. } => to,
        Message::Mms { to, .. } => to,
        Message::Scheduled { to, .. } => to,
        Message::Template { to, .. } => to,
    }
}

pub fn estimate_cost(message: &Message) -> u32 {
    match message {
        Message::Sms { body, .. } => {
            let segments = (body.len() as f64 / 160.0).ceil() as u32;
            segments.max(1)
        }
        Message::Mms { .. } => 3,
        Message::Scheduled { body, .. } => {
            let segments = (body.len() as f64 / 160.0).ceil() as u32;
            segments.max(1)
        }
        Message::Template { params, .. } => {
            1 + ((params.len() as f64 * 0.1).ceil() as u32)
        }
    }
}

pub fn classify_error(error: &ApiError) -> ErrorClass {
    match error {
        ApiError::NetworkError(_) => ErrorClass::Temporary,
        ApiError::InvalidNumber(_) => ErrorClass::Permanent,
        ApiError::RateLimited { .. } => ErrorClass::Temporary,
        ApiError::Unauthorized => ErrorClass::Auth,
        ApiError::ServerError { code, .. } if *code >= 500 => ErrorClass::Temporary,
        ApiError::ServerError { .. } => ErrorClass::Permanent,
    }
}

pub fn error_message(error: &ApiError) -> String {
    match error {
        ApiError::NetworkError(msg) => format!("Network error: {}", msg),
        ApiError::InvalidNumber(num) => format!("Invalid number: {}", num),
        ApiError::RateLimited { retry_after } => {
            format!("Rate limited, retry after {} seconds", retry_after)
        }
        ApiError::Unauthorized => String::from("Authentication required"),
        ApiError::ServerError { code, message } => {
            format!("Server error {}: {}", code, message)
        }
    }
}

pub fn is_retryable(error: &ApiError) -> bool {
    match error {
        ApiError::NetworkError(_) => true,
        ApiError::RateLimited { .. } => true,
        ApiError::ServerError { code, .. } if *code >= 500 => true,
        _ => false,
    }
}

pub fn validate_phone(phone: &str) -> Result<String, String> {
    let chars: Vec<char> = phone.chars().collect();
    
    match chars.as_slice() {
        [] => Err(String::from("Empty phone number")),
        ['+', rest @ ..] if rest.len() >= 10 && rest.len() <= 15 => {
            if rest.iter().all(|c| c.is_ascii_digit()) {
                Ok(phone.to_string())
            } else {
                Err(String::from("Phone must contain only digits after +"))
            }
        }
        ['+', ..] => Err(String::from("Phone must have 10-15 digits after +")),
        _ => Err(String::from("Phone must start with +")),
    }
}

pub fn categorize_status_code(code: u16) -> &'static str {
    match code {
        200..=299 => "success",
        300..=399 => "redirect",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}

pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    match opt {
        Some(inner) => inner,
        None => None,
    }
}

pub fn describe_options(a: Option<i32>, b: Option<i32>) -> String {
    match (a, b) {
        (Some(x), Some(y)) => format!("Both present: {} and {}", x, y),
        (Some(x), None) => format!("Only first: {}", x),
        (None, Some(y)) => format!("Only second: {}", y),
        (None, None) => String::from("Both absent"),
    }
}

pub fn get_retry_after(error: &ApiError) -> Option<u64> {
    match error {
        ApiError::RateLimited { retry_after } => Some(*retry_after),
        _ => None,
    }
}

pub fn describe_length(len: usize) -> &'static str {
    match len {
        0 => "empty",
        n @ 1..=10 => "short",
        n @ 11..=160 => "normal",
        n @ 161..=1000 => "long",
        _ => "very long",
    }
}

pub fn describe_slice(slice: &[i32]) -> String {
    match slice {
        [] => String::from("Empty slice"),
        [single] => format!("Single element: {}", single),
        [first, second] => format!("Pair: {} and {}", first, second),
        [first, .., last] => format!("Multiple: first={}, last={}", first, last),
    }
}

fn main() {
    println!("=== describe_status ===");
    println!("{}", describe_status(&MessageStatus::Pending));
    println!("{}", describe_status(&MessageStatus::Sent));
    println!("{}", describe_status(&MessageStatus::Delivered { at: 1700000000 }));
    println!("{}", describe_status(&MessageStatus::Failed { 
        code: 500, 
        reason: String::from("Timeout") 
    }));

    println!("\n=== is_terminal_status ===");
    println!("Pending: {}", is_terminal_status(&MessageStatus::Pending));
    println!("Delivered: {}", is_terminal_status(&MessageStatus::Delivered { at: 0 }));
    println!("Failed: {}", is_terminal_status(&MessageStatus::Failed { 
        code: 0, 
        reason: String::new() 
    }));

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

    println!("\n=== error_message ===");
    println!("{}", error_message(&network));
    println!("{}", error_message(&rate_limited));

    println!("\n=== is_retryable ===");
    println!("Network: {}", is_retryable(&network));
    println!("InvalidNumber: {}", is_retryable(&invalid));
    println!("RateLimited: {}", is_retryable(&rate_limited));

    println!("\n=== categorize_status_code ===");
    println!("200: {}", categorize_status_code(200));
    println!("301: {}", categorize_status_code(301));
    println!("404: {}", categorize_status_code(404));
    println!("500: {}", categorize_status_code(500));
    println!("99: {}", categorize_status_code(99));

    println!("\n=== describe_options ===");
    println!("{}", describe_options(Some(1), Some(2)));
    println!("{}", describe_options(Some(1), None));
    println!("{}", describe_options(None, Some(2)));
    println!("{}", describe_options(None, None));

    println!("\n=== get_retry_after ===");
    println!("RateLimited: {:?}", get_retry_after(&rate_limited));
    println!("Network: {:?}", get_retry_after(&network));

    println!("\n=== describe_length ===");
    println!("0: {}", describe_length(0));
    println!("5: {}", describe_length(5));
    println!("100: {}", describe_length(100));
    println!("200: {}", describe_length(200));
    println!("5000: {}", describe_length(5000));

    println!("\n=== describe_slice ===");
    println!("{}", describe_slice(&[]));
    println!("{}", describe_slice(&[1]));
    println!("{}", describe_slice(&[1, 2]));
    println!("{}", describe_slice(&[1, 2, 3, 4, 5]));
}

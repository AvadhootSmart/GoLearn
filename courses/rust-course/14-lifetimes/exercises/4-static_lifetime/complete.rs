// Exercise 4: The 'static Lifetime - Complete Solution

use std::sync::atomic::{AtomicUsize, Ordering};

// Static constants for the Textio API
const API_VERSION: &str = "2.0.0";
static API_BASE_URL: &str = "https://api.textio.com/v2";
const MAX_SMS_LENGTH: usize = 160;
const ERR_INVALID_PHONE: &str = "Invalid phone number format";
const ERR_EMPTY_MESSAGE: &str = "Message cannot be empty";
const ERR_TOO_LONG: &str = "Message exceeds maximum length";

static MESSAGE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn get_error_message(code: u32) -> &'static str {
    match code {
        100 => ERR_INVALID_PHONE,
        101 => ERR_EMPTY_MESSAGE,
        102 => ERR_TOO_LONG,
        _ => "Unknown error",
    }
}

pub fn get_api_version() -> &'static str {
    API_VERSION
}

pub fn get_api_base_url() -> &'static str {
    API_BASE_URL
}

pub fn get_max_sms_length() -> usize {
    MAX_SMS_LENGTH
}

pub fn generate_message_id() -> String {
    let count = MESSAGE_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("msg-{:06}", count)
}

pub fn store_permanently<T: 'static + std::fmt::Debug>(value: T) {
    println!("Storing permanently: {:?}", value);
}

pub fn http_status_message(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown Status",
    }
}

pub struct ApiConfig {
    pub version: &'static str,
    pub base_url: &'static str,
    pub max_length: usize,
}

impl ApiConfig {
    pub fn default() -> Self {
        ApiConfig {
            version: API_VERSION,
            base_url: API_BASE_URL,
            max_length: MAX_SMS_LENGTH,
        }
    }
    
    pub fn description(&self) -> String {
        format!("Textio API v{} at {}", self.version, self.base_url)
    }
}

pub struct StaticMessage {
    pub id: String,
    pub body: String,
    pub status: &'static str,
}

impl StaticMessage {
    pub fn new(body: String) -> Self {
        StaticMessage {
            id: generate_message_id(),
            body,
            status: "pending",
        }
    }
    
    pub fn with_status(mut self, status: &'static str) -> Self {
        self.status = status;
        self
    }
    
    pub fn get_status(&self) -> &'static str {
        self.status
    }
}

type MessageHandler = fn(&str) -> Result<String, &'static str>;

pub struct HandlerRegistry {
    handlers: Vec<(&'static str, MessageHandler)>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        HandlerRegistry {
            handlers: Vec::new(),
        }
    }
    
    pub fn register(&mut self, name: &'static str, handler: MessageHandler) {
        self.handlers.push((name, handler));
    }
    
    pub fn get(&self, name: &str) -> Option<MessageHandler> {
        for (n, h) in &self.handlers {
            if *n == name {
                return Some(*h);
            }
        }
        None
    }
    
    pub fn list_handlers(&self) -> Vec<&'static str> {
        self.handlers.iter().map(|(name, _)| *name).collect()
    }
}

fn uppercase_handler(msg: &str) -> Result<String, &'static str> {
    if msg.is_empty() {
        Err(ERR_EMPTY_MESSAGE)
    } else {
        Ok(msg.to_uppercase())
    }
}

fn reverse_handler(msg: &str) -> Result<String, &'static str> {
    if msg.is_empty() {
        Err(ERR_EMPTY_MESSAGE)
    } else {
        Ok(msg.chars().rev().collect())
    }
}

pub fn process_with_static_bound<T: 'static + std::fmt::Display>(value: T) -> String {
    format!("Processed: {}", value)
}

fn main() {
    println!("=== 'static Lifetime Demo ===\n");
    
    println!("API Version: {}", get_api_version());
    println!("API Base URL: {}", get_api_base_url());
    println!("Max SMS Length: {}", get_max_sms_length());
    
    println!("\n=== Error Messages ===");
    println!("Error 100: {}", get_error_message(100));
    println!("Error 101: {}", get_error_message(101));
    println!("Error 102: {}", get_error_message(102));
    println!("Error 999: {}", get_error_message(999));
    
    println!("\n=== HTTP Status ===");
    println!("200: {}", http_status_message(200));
    println!("404: {}", http_status_message(404));
    println!("500: {}", http_status_message(500));
    
    println!("\n=== Message IDs ===");
    println!("ID 1: {}", generate_message_id());
    println!("ID 2: {}", generate_message_id());
    println!("ID 3: {}", generate_message_id());
    
    println!("\n=== API Config ===");
    let config = ApiConfig::default();
    println!("Description: {}", config.description());
    
    println!("\n=== Static Message ===");
    let msg = StaticMessage::new("Hello, World!".to_string())
        .with_status("sent");
    println!("ID: {}, Status: {}, Body: {}", msg.id, msg.get_status(), msg.body);
    
    println!("\n=== Handler Registry ===");
    let mut registry = HandlerRegistry::new();
    registry.register("uppercase", uppercase_handler);
    registry.register("reverse", reverse_handler);
    
    println!("Registered handlers: {:?}", registry.list_handlers());
    
    if let Some(handler) = registry.get("uppercase") {
        match handler("hello") {
            Ok(result) => println!("Uppercase result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    if let Some(handler) = registry.get("reverse") {
        match handler("hello") {
            Ok(result) => println!("Reverse result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    println!("\n=== 'static Bounds ===");
    println!("{}", process_with_static_bound(42));
    println!("{}", process_with_static_bound("literal is static"));
    println!("{}", process_with_static_bound(String::from("owned is ok")));
    
    store_permanently("This string literal is 'static");
    store_permanently(String::from("Owned String is also 'static"));
}

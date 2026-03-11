// Exercise 4: The 'static Lifetime
// 
// 'static represents data that lives for the entire program duration.
// String literals are 'static, and you can create static items.
//
// Key concepts:
// - &'static str for string literals
// - T: 'static means T cannot contain non-'static references
// - static items for global data
// - When to use 'static vs owned data

use std::sync::atomic::{AtomicUsize, Ordering};

// TODO: Define static constants for the Textio API
// These should be &'static str

// The API version
const API_VERSION: &str = "2.0.0";

// Base URL for the API
static API_BASE_URL: &str = "https://api.textio.com/v2";

// Maximum message length (SMS limit)
const MAX_SMS_LENGTH: usize = 160;

// Error messages
const ERR_INVALID_PHONE: &str = "Invalid phone number format";
const ERR_EMPTY_MESSAGE: &str = "Message cannot be empty";
const ERR_TOO_LONG: &str = "Message exceeds maximum length";

// A global message counter using atomic operations
static MESSAGE_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Returns a static error message based on error code
// TODO: Mark the return type as 'static
pub fn get_error_message(code: u32) -> &str {
    match code {
        100 => ERR_INVALID_PHONE,
        101 => ERR_EMPTY_MESSAGE,
        102 => ERR_TOO_LONG,
        _ => "Unknown error",
    }
}

// Returns the API version
// TODO: Mark the return type as 'static
pub fn get_api_version() -> &str {
    API_VERSION
}

// Returns the API base URL
// TODO: Mark the return type as 'static  
pub fn get_api_base_url() -> &'static str {
    API_BASE_URL
}

// Returns the maximum SMS length
pub fn get_max_sms_length() -> usize {
    MAX_SMS_LENGTH
}

// Generates a unique message ID using the global counter
pub fn generate_message_id() -> String {
    let count = MESSAGE_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("msg-{:06}", count)
}

// A function that requires T: 'static
// This is useful when storing data that must outlive any particular scope
// TODO: Add the 'static bound to T
pub fn store_permanently<T: std::fmt::Debug>(value: T) {
    // In a real implementation, this might store in a global collection
    println!("Storing permanently: {:?}", value);
}

// Status code to message mapping
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

// Configuration that uses 'static strings
pub struct ApiConfig {
    pub version: &'static str,
    pub base_url: &'static str,
    pub max_length: usize,
}

impl ApiConfig {
    // TODO: Return a static reference to a config
    pub fn default() -> Self {
        ApiConfig {
            version: API_VERSION,
            base_url: API_BASE_URL,
            max_length: MAX_SMS_LENGTH,
        }
    }
    
    // Returns a description string (owned, not static)
    pub fn description(&self) -> String {
        format!("Textio API v{} at {}", self.version, self.base_url)
    }
}

// Message type that can only hold 'static references or owned data
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
    
    // TODO: Return type should be 'static
    pub fn get_status(&self) -> &str {
        self.status
    }
}

// A registry that stores 'static function pointers
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
    
    // Register a handler with a 'static name
    pub fn register(&mut self, name: &'static str, handler: MessageHandler) {
        self.handlers.push((name, handler));
    }
    
    // Get a handler by name
    pub fn get(&self, name: &str) -> Option<MessageHandler> {
        for (n, h) in &self.handlers {
            if *n == name {
                return Some(*h);
            }
        }
        None
    }
    
    // List all registered handlers
    pub fn list_handlers(&self) -> Vec<&'static str> {
        self.handlers.iter().map(|(name, _)| *name).collect()
    }
}

// Example handler functions
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

// A function demonstrating 'static bounds
pub fn process_with_static_bound<T: 'static + std::fmt::Display>(value: T) -> String {
    format!("Processed: {}", value)
}

fn main() {
    println!("=== 'static Lifetime Demo ===\n");
    
    // Static constants
    println!("API Version: {}", get_api_version());
    println!("API Base URL: {}", get_api_base_url());
    println!("Max SMS Length: {}", get_max_sms_length());
    
    // Error messages
    println!("\n=== Error Messages ===");
    println!("Error 100: {}", get_error_message(100));
    println!("Error 101: {}", get_error_message(101));
    println!("Error 102: {}", get_error_message(102));
    println!("Error 999: {}", get_error_message(999));
    
    // HTTP status messages
    println!("\n=== HTTP Status ===");
    println!("200: {}", http_status_message(200));
    println!("404: {}", http_status_message(404));
    println!("500: {}", http_status_message(500));
    
    // Message IDs
    println!("\n=== Message IDs ===");
    println!("ID 1: {}", generate_message_id());
    println!("ID 2: {}", generate_message_id());
    println!("ID 3: {}", generate_message_id());
    
    // API Config
    println!("\n=== API Config ===");
    let config = ApiConfig::default();
    println!("Description: {}", config.description());
    
    // Static Message
    println!("\n=== Static Message ===");
    let msg = StaticMessage::new("Hello, World!".to_string())
        .with_status("sent");
    println!("ID: {}, Status: {}, Body: {}", msg.id, msg.get_status(), msg.body);
    
    // Handler Registry
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
    
    // 'static bounds
    println!("\n=== 'static Bounds ===");
    println!("{}", process_with_static_bound(42));
    println!("{}", process_with_static_bound("literal is static"));
    println!("{}", process_with_static_bound(String::from("owned is ok")));
    
    // Storing permanently
    store_permanently("This string literal is 'static");
    store_permanently(String::from("Owned String is also 'static"));
}

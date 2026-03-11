// Textio SMS API - Error Chains Exercise
// Demonstrating error handling patterns (without external crates)

use std::fmt;
use std::error::Error;

// TODO: Define a chainable error type
// Requirements:
// - AppError wraps an inner error and adds context
// - Include a message string and optional source
#[derive(Debug)]
pub struct AppError {
    // Your fields here: message, source
}

// TODO: Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Your code here
        todo!()
    }
}

// TODO: Implement Error for AppError
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Your code here
        todo!()
    }
}

// TODO: Implement AppError::new()
impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        // Your code here
        todo!()
    }
    
    // TODO: Implement AppError::with_source()
    pub fn with_source(message: impl Into<String>, source: Box<dyn Error + Send + Sync>) -> Self {
        // Your code here
        todo!()
    }
    
    // TODO: Implement AppError::context() - adds context to an existing error
    pub fn context(self, context: impl Into<String>) -> Self {
        // Your code here
        todo!()
    }
}

// TODO: Define DomainError enum
// Requirements:
// - DatabaseError: contains message
// - NetworkError: contains message
// - ValidationError: contains message
// - NotFoundError: contains resource and id
#[derive(Debug)]
pub enum DomainError {
    // Your variants here
}

// TODO: Implement Display for DomainError
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Your code here
        todo!()
    }
}

// TODO: Implement Error for DomainError
impl Error for DomainError {}

// TODO: Implement From<DomainError> for AppError
impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        // Your code here
        todo!()
    }
}

// Helper function to print error chain
pub fn print_error_chain(error: &dyn Error) {
    println!("Error: {}", error);
    let mut source = error.source();
    let mut level = 1;
    while let Some(err) = source {
        println!("  Caused by ({}): {}", level, err);
        source = err.source();
        level += 1;
    }
}

// Functions that return Box<dyn Error>

// TODO: Implement parse_config
// Requirements:
// - Parse a string as config
// - Return Ok(value) if valid
// - Return Box<dyn Error> if invalid (use DomainError)
pub fn parse_config(input: &str) -> Result<u32, Box<dyn Error>> {
    // Your code here
    todo!()
}

// TODO: Implement connect_database
// Requirements:
// - Simulate database connection
// - Return Ok("connected") if host is not "fail"
// - Return Err with DomainError::DatabaseError if "fail"
pub fn connect_database(host: &str) -> Result<String, Box<dyn Error>> {
    // Your code here
    todo!()
}

// TODO: Implement fetch_user
// Requirements:
// - Simulate fetching user from DB
// - Return Ok(format!("User {}", id)) if id > 0
// - Return Err with DomainError::NotFoundError if id == 0
pub fn fetch_user(id: u64) -> Result<String, Box<dyn Error>> {
    // Your code here
    todo!()
}

// TODO: Implement send_request
// Requirements:
// - Simulate sending network request
// - Return Ok("success") if url starts with "https"
// - Return Err with DomainError::NetworkError otherwise
pub fn send_request(url: &str) -> Result<String, Box<dyn Error>> {
    // Your code here
    todo!()
}

// TODO: Implement process_with_chain
// Requirements:
// - Use AppError to create error chains
// - Call connect_database and wrap errors with context
// - Call fetch_user and wrap errors with context
pub fn process_with_chain(host: &str, user_id: u64) -> Result<String, AppError> {
    // Your code here - use AppError::with_source for chaining
    todo!()
}

fn main() {
    println!("=== Textio SMS API - Error Chains Demo ===\n");

    // Test AppError
    println!("--- AppError Basics ---");
    
    let err = AppError::new("Something went wrong");
    println!("Simple error: {}", err);
    
    let inner: Box<dyn Error + Send + Sync> = Box::new(DomainError::DatabaseError("Connection failed".to_string()));
    let chained = AppError::with_source("Operation failed", inner);
    println!("Chained error: {}", chained);
    if let Some(src) = chained.source() {
        println!("  Source: {}", src);
    }

    // Test DomainError
    println!("\n--- Domain Errors ---");
    
    let db_err = DomainError::DatabaseError("Connection timeout".to_string());
    println!("Database error: {}", db_err);
    
    let net_err = DomainError::NetworkError("Timeout".to_string());
    println!("Network error: {}", net_err);
    
    let val_err = DomainError::ValidationError("Phone is invalid".to_string());
    println!("Validation error: {}", val_err);
    
    let nf_err = DomainError::NotFoundError { resource: "User".to_string(), id: 42 };
    println!("Not found error: {}", nf_err);

    // Test Box<dyn Error> functions
    println!("\n--- Box<dyn Error> Functions ---");
    
    let config_ok = parse_config("42");
    println!("Parse '42': {:?}", config_ok);
    
    let config_err = parse_config("invalid");
    println!("Parse 'invalid': {:?}", config_err);

    let db_ok = connect_database("localhost");
    println!("Connect 'localhost': {:?}", db_ok);
    
    let db_err = connect_database("fail");
    println!("Connect 'fail': {:?}", db_err);

    let user_ok = fetch_user(1);
    println!("Fetch user 1: {:?}", user_ok);
    
    let user_err = fetch_user(0);
    println!("Fetch user 0: {:?}", user_err);

    let req_ok = send_request("https://api.textio.io");
    println!("Request https: {:?}", req_ok);
    
    let req_err = send_request("http://insecure.io");
    println!("Request http: {:?}", req_err);

    // Test error chains
    println!("\n--- Error Chains ---");
    
    let success = process_with_chain("localhost", 1);
    println!("Success case: {:?}", success);
    
    let db_fail = process_with_chain("fail", 1);
    match db_fail {
        Err(e) => {
            println!("DB failure chain:");
            print_error_chain(&e);
        }
        _ => {}
    }
    
    let user_fail = process_with_chain("localhost", 0);
    match user_fail {
        Err(e) => {
            println!("\nUser failure chain:");
            print_error_chain(&e);
        }
        _ => {}
    }

    // Demonstrate From trait conversion
    println!("\n--- From Trait Conversion ---");
    
    let domain: DomainError = DomainError::ValidationError("test".to_string());
    let app: AppError = domain.into();
    println!("Converted DomainError to AppError: {}", app);

    println!("\n=== Exercise Complete ===");
}

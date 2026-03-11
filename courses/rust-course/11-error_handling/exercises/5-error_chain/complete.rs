// Textio SMS API - Error Chains Exercise (Complete Solution)
// Demonstrating error handling patterns (without external crates)

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct AppError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref() as &dyn Error)
    }
}

impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        AppError {
            message: message.into(),
            source: None,
        }
    }
    
    pub fn with_source(message: impl Into<String>, source: Box<dyn Error + Send + Sync>) -> Self {
        AppError {
            message: message.into(),
            source: Some(source),
        }
    }
    
    pub fn context(self, context: impl Into<String>) -> Self {
        AppError {
            message: format!("{}: {}", context.into(), self.message),
            source: self.source,
        }
    }
}

#[derive(Debug)]
pub enum DomainError {
    DatabaseError(String),
    NetworkError(String),
    ValidationError(String),
    NotFoundError { resource: String, id: u64 },
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            DomainError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DomainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DomainError::NotFoundError { resource, id } => {
                write!(f, "{} with id {} not found", resource, id)
            }
        }
    }
}

impl Error for DomainError {}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        AppError::with_source(e.to_string(), Box::new(e))
    }
}

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

pub fn parse_config(input: &str) -> Result<u32, Box<dyn Error>> {
    input.parse::<u32>()
        .map_err(|e| Box::new(DomainError::ValidationError(format!("Invalid config: {}", e))) as Box<dyn Error>)
}

pub fn connect_database(host: &str) -> Result<String, Box<dyn Error>> {
    if host == "fail" {
        Err(Box::new(DomainError::DatabaseError("Connection refused".to_string())))
    } else {
        Ok(format!("Connected to {}", host))
    }
}

pub fn fetch_user(id: u64) -> Result<String, Box<dyn Error>> {
    if id == 0 {
        Err(Box::new(DomainError::NotFoundError {
            resource: "User".to_string(),
            id,
        }))
    } else {
        Ok(format!("User {}", id))
    }
}

pub fn send_request(url: &str) -> Result<String, Box<dyn Error>> {
    if url.starts_with("https") {
        Ok("success".to_string())
    } else {
        Err(Box::new(DomainError::NetworkError("Insecure URL".to_string())))
    }
}

pub fn process_with_chain(host: &str, user_id: u64) -> Result<String, AppError> {
    let _conn = connect_database(host)
        .map_err(|e| AppError::with_source("Failed to connect to database".to_string(), e))?;
    
    let user = fetch_user(user_id)
        .map_err(|e| AppError::with_source("Failed to fetch user".to_string(), e))?;
    
    Ok(user)
}

fn main() {
    println!("=== Textio SMS API - Error Chains Demo ===\n");

    println!("--- AppError Basics ---");
    
    let err = AppError::new("Something went wrong");
    println!("Simple error: {}", err);
    
    let inner: Box<dyn Error + Send + Sync> = Box::new(DomainError::DatabaseError("Connection failed".to_string()));
    let chained = AppError::with_source("Operation failed", inner);
    println!("Chained error: {}", chained);
    if let Some(src) = chained.source() {
        println!("  Source: {}", src);
    }

    println!("\n--- Domain Errors ---");
    
    let db_err = DomainError::DatabaseError("Connection timeout".to_string());
    println!("Database error: {}", db_err);
    
    let net_err = DomainError::NetworkError("Timeout".to_string());
    println!("Network error: {}", net_err);
    
    let val_err = DomainError::ValidationError("Phone is invalid".to_string());
    println!("Validation error: {}", val_err);
    
    let nf_err = DomainError::NotFoundError { resource: "User".to_string(), id: 42 };
    println!("Not found error: {}", nf_err);

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

    println!("\n--- From Trait Conversion ---");
    
    let domain: DomainError = DomainError::ValidationError("test".to_string());
    let app: AppError = domain.into();
    println!("Converted DomainError to AppError: {}", app);

    println!("\n=== Exercise Complete ===");
}

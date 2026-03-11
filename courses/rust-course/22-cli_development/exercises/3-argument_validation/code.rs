use clap::Parser;
use std::fmt;

/// TODO: Create a PhoneNumber wrapper type for validation
// Hint: Create a struct that wraps a String
// #[derive(Debug, Clone)]
// pub struct PhoneNumber(String);

/// TODO: Implement Display for PhoneNumber
// Hint: impl fmt::Display for PhoneNumber { ... }

/// TODO: Create a Message wrapper type for validation
// Hint: Similar to PhoneNumber, but validates length <= 160

/// TODO: Implement parse_phone_number function
// Requirements:
// - Must start with '+'
// - Must have 10-15 digits (excluding '+')
// - Should clean input (remove spaces, dashes)
// - Return helpful error messages
pub fn parse_phone_number(_s: &str) -> Result<String, String> {
    // TODO: Implement phone number validation
    // 1. Clean the input (filter for digits and '+')
    // 2. Check it starts with '+'
    // 3. Count digits (should be 10-15)
    // 4. Return Ok(cleaned) or Err(helpful_message)
    Err("Not implemented".to_string())
}

/// TODO: Implement parse_message function
// Requirements:
// - Must not be empty
// - Must be <= 160 characters
// - Return helpful error with character count
pub fn parse_message(_s: &str) -> Result<String, String> {
    // TODO: Implement message validation
    Err("Not implemented".to_string())
}

/// TODO: Implement parse_api_key function
// Requirements:
// - Must start with "tx_"
// - Must be at least 16 characters
// - Provide guidance on getting an API key
pub fn parse_api_key(_s: &str) -> Result<String, String> {
    // TODO: Implement API key validation
    Err("Not implemented".to_string())
}

/// Textio SMS CLI with robust validation
#[derive(Parser)]
#[command(name = "textio")]
struct Cli {
    /// TODO: Add phone number argument with custom parser
    // Hint: #[arg(short, long, value_parser = parse_phone_number)]
    // to: String,

    /// TODO: Add message argument with custom parser
    // Hint: #[arg(short, long, value_parser = parse_message)]
    // message: String,

    /// TODO: Add API key argument with custom parser
    // Hint: #[arg(long, value_parser = parse_api_key, env = "TEXTIO_API_KEY")]
    // api_key: String,

    /// TODO: Add optional scheduled time
    // Hint: Use Option<String> and validate ISO 8601 format
    // #[arg(short, long)]
    // schedule: Option<String>,

    /// TODO: Add verbose flag
    // #[arg(short, long)]
    // verbose: bool,
}

fn main() {
    // TODO: Parse and handle arguments
    // let cli = Cli::parse();
    
    // TODO: Implement the following:
    // 1. Print sending status
    // 2. If scheduled, show the scheduled time
    // 3. If verbose, show character count and phone format
    // 4. Print success message
    
    println!("Validation not implemented yet!");
}

// HINTS:
//
// 1. Phone number validation:
//    pub fn parse_phone_number(s: &str) -> Result<String, String> {
//        let cleaned: String = s.chars()
//            .filter(|c| c.is_ascii_digit() || *c == '+')
//            .collect();
//        
//        if !cleaned.starts_with('+') {
//            return Err("Phone number must start with '+' and country code.\n\
//                       Example: +1 for US, +44 for UK".to_string());
//        }
//        
//        let digits: String = cleaned[1..].chars()
//            .filter(|c| c.is_ascii_digit())
//            .collect();
//        
//        if digits.len() < 10 || digits.len() > 15 {
//            return Err(format!(
//                "Phone number must have 10-15 digits. Found: {}",
//                digits.len()
//            ));
//        }
//        
//        Ok(cleaned)
//    }
//
// 2. Message validation:
//    pub fn parse_message(s: &str) -> Result<String, String> {
//        let len = s.chars().count();
//        
//        if len == 0 {
//            return Err("Message cannot be empty".to_string());
//        }
//        
//        if len > 160 {
//            return Err(format!(
//                "Message too long: {} characters (max 160)",
//                len
//            ));
//        }
//        
//        Ok(s.to_string())
//    }
//
// 3. API key validation:
//    pub fn parse_api_key(s: &str) -> Result<String, String> {
//        if s.len() < 16 {
//            return Err("API key must be at least 16 characters.\n\
//                       Get your key at: https://textio.example.com/api-keys".to_string());
//        }
//        
//        if !s.starts_with("tx_") {
//            return Err("Invalid API key format. Keys start with 'tx_'".to_string());
//        }
//        
//        Ok(s.to_string())
//    }

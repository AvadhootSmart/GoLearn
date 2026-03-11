// Exercise 3: CLI Arguments
// 
// Implement a CLI parser for the Textio messaging system.
// Complete the functions marked with TODO.

use std::env;

/// Represents a parsed CLI command
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Send {
        to: String,
        message: String,
        from: Option<String>,
    },
    Status {
        message_id: String,
    },
    Contacts {
        action: ContactAction,
    },
    Config {
        key: Option<String>,
        value: Option<String>,
    },
    Help,
    Version,
}

/// Contact management actions
#[derive(Debug, Clone, PartialEq)]
pub enum ContactAction {
    List,
    Add { name: String, phone: String },
    Remove { phone: String },
}

/// Parse command-line arguments into a Command
pub fn parse_args(args: &[String]) -> Result<Command, String> {
    // TODO: Implement this function
    // 1. Skip the program name (args[0])
    // 2. Check for help/version flags
    // 3. Parse the command (send, status, contacts, config)
    // 4. Validate arguments for each command
    // 5. Return appropriate Command variant or error
    
    todo!()
}

/// Parse the "send" command
/// 
/// Format: send <to> <message> [--from <sender_id>]
fn parse_send_command(args: &[String]) -> Result<Command, String> {
    // TODO: Implement this function
    // 1. Extract 'to' (first positional arg, should be a phone number)
    // 2. Extract 'message' (second positional arg)
    // 3. Look for --from flag and extract its value
    // 4. Validate phone number format
    
    todo!()
}

/// Parse the "status" command
/// 
/// Format: status <message_id>
fn parse_status_command(args: &[String]) -> Result<Command, String> {
    // TODO: Implement this function
    
    todo!()
}

/// Parse the "contacts" command
/// 
/// Format: contacts <list|add|remove> [args...]
fn parse_contacts_command(args: &[String]) -> Result<Command, String> {
    // TODO: Implement this function
    // 1. Determine the subcommand (list, add, remove)
    // 2. Parse arguments for each subcommand
    
    todo!()
}

/// Parse the "config" command
/// 
/// Format: config [key] [value]
fn parse_config_command(args: &[String]) -> Result<Command, String> {
    // TODO: Implement this function
    
    todo!()
}

/// Validate a phone number format
/// 
/// Valid formats:
/// - + followed by 10-15 digits
/// - Pure digits (10-15 digits)
pub fn validate_phone(phone: &str) -> Result<String, String> {
    // TODO: Implement this function
    // 1. Check if phone starts with '+'
    // 2. Extract digits only
    // 3. Verify length is between 10-15
    // 4. Return formatted phone number
    
    todo!()
}

/// Extract flags and their values from arguments
/// 
/// Returns a tuple of (flags_with_values, positional_args)
pub fn extract_flags(args: &[String]) -> (Vec<(String, Option<String>)>, Vec<String>) {
    // TODO: Implement this function
    // 1. Iterate through arguments
    // 2. Identify flags (start with - or --)
    // 3. For flags that take values, extract the next argument
    // 4. Collect non-flag arguments as positional
    
    todo!()
}

/// Generate help text
pub fn build_help() -> String {
    // TODO: Implement this function
    // Return a formatted help string
    
    todo!()
}

/// Generate version info
pub fn build_version() -> String {
    // TODO: Implement this function
    
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_help_flag() {
        let args = make_args(&["textio", "-h"]);
        let cmd = parse_args(&args).unwrap();
        assert_eq!(cmd, Command::Help);
        
        let args = make_args(&["textio", "--help"]);
        let cmd = parse_args(&args).unwrap();
        assert_eq!(cmd, Command::Help);
    }

    #[test]
    fn test_version_flag() {
        let args = make_args(&["textio", "-v"]);
        let cmd = parse_args(&args).unwrap();
        assert_eq!(cmd, Command::Version);
        
        let args = make_args(&["textio", "--version"]);
        let cmd = parse_args(&args).unwrap();
        assert_eq!(cmd, Command::Version);
    }

    #[test]
    fn test_send_command() {
        let args = make_args(&["textio", "send", "+1234567890", "Hello"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Send { to, message, from } => {
                assert_eq!(to, "+1234567890");
                assert_eq!(message, "Hello");
                assert_eq!(from, None);
            }
            _ => panic!("Expected Send command"),
        }
    }

    #[test]
    fn test_send_command_with_from() {
        let args = make_args(&["textio", "send", "+1234567890", "Hello", "--from", "Textio"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Send { to, message, from } => {
                assert_eq!(to, "+1234567890");
                assert_eq!(message, "Hello");
                assert_eq!(from, Some("Textio".to_string()));
            }
            _ => panic!("Expected Send command"),
        }
    }

    #[test]
    fn test_status_command() {
        let args = make_args(&["textio", "status", "msg_123"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Status { message_id } => {
                assert_eq!(message_id, "msg_123");
            }
            _ => panic!("Expected Status command"),
        }
    }

    #[test]
    fn test_contacts_list() {
        let args = make_args(&["textio", "contacts", "list"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Contacts { action } => {
                assert_eq!(action, ContactAction::List);
            }
            _ => panic!("Expected Contacts command"),
        }
    }

    #[test]
    fn test_contacts_add() {
        let args = make_args(&["textio", "contacts", "add", "John", "+1234567890"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Contacts { action } => {
                match action {
                    ContactAction::Add { name, phone } => {
                        assert_eq!(name, "John");
                        assert_eq!(phone, "+1234567890");
                    }
                    _ => panic!("Expected Add action"),
                }
            }
            _ => panic!("Expected Contacts command"),
        }
    }

    #[test]
    fn test_config_view() {
        let args = make_args(&["textio", "config"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Config { key, value } => {
                assert_eq!(key, None);
                assert_eq!(value, None);
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_get() {
        let args = make_args(&["textio", "config", "api_key"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Config { key, value } => {
                assert_eq!(key, Some("api_key".to_string()));
                assert_eq!(value, None);
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_set() {
        let args = make_args(&["textio", "config", "api_key", "secret123"]);
        let cmd = parse_args(&args).unwrap();
        
        match cmd {
            Command::Config { key, value } => {
                assert_eq!(key, Some("api_key".to_string()));
                assert_eq!(value, Some("secret123".to_string()));
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_validate_phone_valid() {
        assert!(validate_phone("+1234567890").is_ok());
        assert!(validate_phone("1234567890").is_ok());
        assert!(validate_phone("+12345678901234").is_ok());
    }

    #[test]
    fn test_validate_phone_invalid() {
        assert!(validate_phone("123").is_err());
        assert!(validate_phone("abc").is_err());
        assert!(validate_phone("+123").is_err());
    }

    #[test]
    fn test_extract_flags() {
        let args = make_args(&["send", "+123", "Hello", "--from", "Textio", "-v"]);
        let (flags, positional) = extract_flags(&args);
        
        assert_eq!(positional, make_args(&["send", "+123", "Hello"]));
        assert!(flags.contains(&("--from".to_string(), Some("Textio".to_string()))));
    }

    #[test]
    fn test_unknown_command() {
        let args = make_args(&["textio", "unknown"]);
        assert!(parse_args(&args).is_err());
    }
}

fn main() {
    println!("Run tests with: cargo test");
}

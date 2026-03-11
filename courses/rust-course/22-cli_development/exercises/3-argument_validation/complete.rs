use clap::Parser;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PhoneNumber(String);

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PhoneNumber {
    pub fn formatted(&self) -> String {
        let digits: String = self.0.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() >= 10 {
            format!(
                "+{} ({}) {}-{}",
                &digits[..digits.len() - 10],
                &digits[digits.len() - 10..digits.len() - 7],
                &digits[digits.len() - 7..digits.len() - 4],
                &digits[digits.len() - 4..]
            )
        } else {
            self.0.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message(String);

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Message {
    pub fn len(&self) -> usize {
        self.0.chars().count()
    }

    pub fn remaining(&self) -> usize {
        160 - self.len()
    }
}

pub fn parse_phone_number(s: &str) -> Result<PhoneNumber, String> {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();

    if !cleaned.starts_with('+') {
        return Err(
            "Phone number must start with '+' and include country code.\n\
             Examples: +1 for US/Canada, +44 for UK, +91 for India"
                .to_string(),
        );
    }

    let digits: String = cleaned[1..]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    if digits.len() < 10 {
        return Err(format!(
            "Phone number too short: {} digits found.\n\
             Phone numbers must have 10-15 digits (excluding '+').",
            digits.len()
        ));
    }

    if digits.len() > 15 {
        return Err(format!(
            "Phone number too long: {} digits found.\n\
             Phone numbers must have 10-15 digits (excluding '+').",
            digits.len()
        ));
    }

    Ok(PhoneNumber(cleaned))
}

pub fn parse_message(s: &str) -> Result<Message, String> {
    let len = s.chars().count();

    if len == 0 {
        return Err("Message cannot be empty.".to_string());
    }

    if len > 160 {
        return Err(format!(
            "Message too long: {} characters (maximum is 160).\n\
             Consider splitting into multiple messages or using a link shortener.",
            len
        ));
    }

    Ok(Message(s.to_string()))
}

pub fn parse_api_key(s: &str) -> Result<String, String> {
    if s.len() < 16 {
        return Err(format!(
            "API key too short: {} characters (minimum is 16).\n\
             Get your API key at: https://textio.example.com/dashboard/api-keys",
            s.len()
        ));
    }

    if !s.starts_with("tx_") {
        return Err(
            "Invalid API key format. All Textio API keys start with 'tx_'.\n\
             Example: tx_live_sk_abc123...\n\
             Get your API key at: https://textio.example.com/dashboard/api-keys"
                .to_string(),
        );
    }

    Ok(s.to_string())
}

pub fn parse_schedule(s: &str) -> Result<String, String> {
    let formats = ["%Y-%m-%dT%H:%M:%S", "%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"];

    for fmt in formats {
        if chrono::NaiveDateTime::parse_from_str(s, fmt).is_ok() {
            return Ok(s.to_string());
        }
    }

    if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(format!("{}T00:00:00", date));
    }

    Err(format!(
        "Invalid date/time format: '{}'\n\
         Supported formats:\n\
         - 2024-12-25T14:30:00 (ISO 8601)\n\
         - 2024-12-25 14:30:00\n\
         - 2024-12-25 (defaults to midnight)",
        s
    ))
}

/// Textio SMS CLI with robust validation
#[derive(Parser)]
#[command(name = "textio")]
#[command(version = "1.0")]
#[command(about = "Send SMS messages with validation")]
struct Cli {
    /// Recipient phone number (e.g., +1234567890)
    #[arg(short, long, value_parser = parse_phone_number)]
    to: PhoneNumber,

    /// Message content (max 160 characters)
    #[arg(short, long, value_parser = parse_message)]
    message: Message,

    /// API key for authentication (starts with tx_)
    #[arg(long, value_parser = parse_api_key, env = "TEXTIO_API_KEY")]
    api_key: String,

    /// Schedule message for later (ISO 8601 format)
    #[arg(short, long, value_parser = parse_schedule)]
    schedule: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("=== Textio SMS ===");
    println!();

    if cli.verbose {
        println!("[Verbose Mode]");
        println!("Phone: {} (formatted: {})", cli.to, cli.to.formatted());
        println!("Message length: {}/160 characters ({} remaining)", 
            cli.message.len(), 
            cli.message.remaining()
        );
        println!("API Key: {}...", &cli.api_key[..12.min(cli.api_key.len())]);
        println!();
    }

    if let Some(ref schedule) = cli.schedule {
        println!("Scheduling message for: {}", schedule);
    } else {
        println!("Sending message immediately...");
    }

    println!();
    println!("To: {}", cli.to);
    println!("Message: {}", cli.message);
    println!();
    println!("---");

    if cli.schedule.is_some() {
        println!("Message scheduled successfully!");
    } else {
        println!("Message sent successfully!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_number() {
        assert!(parse_phone_number("+12345678901").is_ok());
        assert!(parse_phone_number("+441234567890").is_ok());
    }

    #[test]
    fn test_invalid_phone_number_no_plus() {
        let result = parse_phone_number("12345678901");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("'+'"));
    }

    #[test]
    fn test_invalid_phone_number_too_short() {
        let result = parse_phone_number("+123456");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }

    #[test]
    fn test_valid_message() {
        assert!(parse_message("Hello").is_ok());
        let msg = parse_message("Test message").unwrap();
        assert_eq!(msg.len(), 12);
    }

    #[test]
    fn test_invalid_message_empty() {
        let result = parse_message("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_invalid_message_too_long() {
        let long_msg = "x".repeat(161);
        let result = parse_message(&long_msg);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too long"));
    }

    #[test]
    fn test_valid_api_key() {
        assert!(parse_api_key("tx_live_12345678").is_ok());
    }

    #[test]
    fn test_invalid_api_key_no_prefix() {
        let result = parse_api_key("live_12345678");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tx_"));
    }

    #[test]
    fn test_invalid_api_key_too_short() {
        let result = parse_api_key("tx_short");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }
}

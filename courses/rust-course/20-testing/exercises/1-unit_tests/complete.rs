// Textio Unit Testing Exercise - Complete Solution

pub struct Message {
    pub to: String,
    pub body: String,
    pub status: MessageStatus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageStatus {
    Pending,
    Sent,
    Failed,
    Delivered,
}

impl Message {
    pub fn new(to: &str, body: &str) -> Self {
        Message {
            to: to.to_string(),
            body: body.to_string(),
            status: MessageStatus::Pending,
        }
    }

    pub fn char_count(&self) -> usize {
        self.body.chars().count()
    }

    pub fn is_valid(&self) -> bool {
        !self.to.is_empty() && !self.body.is_empty() && self.to.starts_with('+')
    }
}

pub fn calculate_cost(message_count: u32) -> u32 {
    if message_count == 0 {
        return 0;
    }
    
    let base_cost = 5;
    let free_tier = 100;
    
    if message_count <= free_tier {
        return 0;
    }
    
    let billable = message_count - free_tier;
    let cost = (billable / 100) * base_cost;
    
    if billable % 100 != 0 {
        cost + base_cost
    } else {
        cost
    }
}

pub fn validate_phone_number(phone: &str) -> Result<String, String> {
    let trimmed = phone.trim();
    
    if trimmed.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }
    
    if !trimmed.starts_with('+') {
        return Err("Phone number must start with '+'".to_string());
    }
    
    let digits: String = trimmed.chars().skip(1).filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() < 10 || digits.len() > 15 {
        return Err(format!("Invalid phone number length: {}", digits.len()));
    }
    
    Ok(trimmed.to_string())
}

pub fn format_message_for_api(message: &Message) -> String {
    format!("TO:{}|BODY:{}|STATUS:{}", message.to, message.body, 
            match message.status {
                MessageStatus::Pending => "PENDING",
                MessageStatus::Sent => "SENT",
                MessageStatus::Failed => "FAILED",
                MessageStatus::Delivered => "DELIVERED",
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message_has_pending_status() {
        let msg = Message::new("+1234567890", "Hello");
        assert_eq!(msg.status, MessageStatus::Pending);
    }

    #[test]
    fn test_char_count() {
        let msg = Message::new("+1234567890", "Hello World");
        assert_eq!(msg.char_count(), 11);
    }

    #[test]
    fn test_char_count_unicode() {
        let msg = Message::new("+1234567890", "Hello 🌍");
        assert_eq!(msg.char_count(), 8);
    }

    #[test]
    fn test_is_valid_true() {
        let msg = Message::new("+1234567890", "Hello");
        assert!(msg.is_valid());
    }

    #[test]
    fn test_is_valid_empty_to() {
        let msg = Message::new("", "Hello");
        assert!(!msg.is_valid());
    }

    #[test]
    fn test_is_valid_empty_body() {
        let msg = Message::new("+1234567890", "");
        assert!(!msg.is_valid());
    }

    #[test]
    fn test_is_valid_no_plus_prefix() {
        let msg = Message::new("1234567890", "Hello");
        assert!(!msg.is_valid());
    }

    #[test]
    fn test_calculate_cost_zero() {
        assert_eq!(calculate_cost(0), 0);
    }

    #[test]
    fn test_calculate_cost_free_tier() {
        assert_eq!(calculate_cost(50), 0);
    }

    #[test]
    fn test_calculate_cost_at_free_limit() {
        assert_eq!(calculate_cost(100), 0);
    }

    #[test]
    fn test_calculate_cost_above_free_tier() {
        assert_eq!(calculate_cost(150), 5);
    }

    #[test]
    fn test_calculate_cost_multiple_tiers() {
        assert_eq!(calculate_cost(250), 10);
        assert_eq!(calculate_cost(350), 15);
    }

    #[test]
    fn test_validate_phone_valid() -> Result<(), String> {
        let result = validate_phone_number("+1234567890")?;
        assert_eq!(result, "+1234567890");
        
        let result = validate_phone_number("  +1234567890  ")?;
        assert_eq!(result, "+1234567890");
        
        Ok(())
    }

    #[test]
    fn test_validate_phone_empty() {
        let result = validate_phone_number("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_validate_phone_no_plus() {
        let result = validate_phone_number("1234567890");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("'+'"));
    }

    #[test]
    fn test_validate_phone_too_short() {
        let result = validate_phone_number("+123");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("length"));
    }

    #[test]
    fn test_validate_phone_too_long() {
        let result = validate_phone_number("+12345678901234567890");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("length"));
    }

    #[test]
    fn test_format_message_for_api() {
        let mut msg = Message::new("+1234567890", "Hello World");
        msg.status = MessageStatus::Sent;
        
        let formatted = format_message_for_api(&msg);
        assert_eq!(formatted, "TO:+1234567890|BODY:Hello World|STATUS:SENT");
    }

    #[test]
    fn test_format_message_for_api_pending() {
        let msg = Message::new("+1234567890", "Test");
        let formatted = format_message_for_api(&msg);
        assert_eq!(formatted, "TO:+1234567890|BODY:Test|STATUS:PENDING");
    }

    #[test]
    fn test_format_message_for_api_delivered() {
        let mut msg = Message::new("+1234567890", "Delivered msg");
        msg.status = MessageStatus::Delivered;
        
        let formatted = format_message_for_api(&msg);
        assert!(formatted.contains("DELIVERED"));
    }
}

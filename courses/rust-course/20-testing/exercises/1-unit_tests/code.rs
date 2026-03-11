// Textio Unit Testing Exercise
// 
// In this exercise, you'll write unit tests for Textio's message validation
// and cost calculation functions.
//
// TODO: Complete all the tests marked with TODO comments

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
    
    let base_cost = 5; // $5 per 100 messages
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

    // TODO: Write a test that verifies Message::new creates a message with Pending status
    // Hint: Use assert_eq! to check the status
    
    #[test]
    fn test_new_message_has_pending_status() {
        // Your code here
    }

    // TODO: Write a test that verifies char_count returns correct character count
    // Test with "Hello World" which should return 11
    
    #[test]
    fn test_char_count() {
        // Your code here
    }

    // TODO: Write a test that verifies is_valid returns true for valid messages
    // Create a message with to: "+1234567890" and body: "Hello"
    
    #[test]
    fn test_is_valid_true() {
        // Your code here
    }

    // TODO: Write tests that verify is_valid returns false for invalid messages
    // Test cases: empty to, empty body, to without '+' prefix
    
    #[test]
    fn test_is_valid_empty_to() {
        // Your code here
    }

    #[test]
    fn test_is_valid_empty_body() {
        // Your code here
    }

    #[test]
    fn test_is_valid_no_plus_prefix() {
        // Your code here
    }

    // TODO: Write tests for calculate_cost
    // - 0 messages should cost 0
    // - 50 messages (under free tier) should cost 0
    // - 100 messages (at free tier limit) should cost 0
    // - 150 messages should cost 5
    // - 250 messages should cost 10
    
    #[test]
    fn test_calculate_cost_zero() {
        // Your code here
    }

    #[test]
    fn test_calculate_cost_free_tier() {
        // Your code here
    }

    #[test]
    fn test_calculate_cost_at_free_limit() {
        // Your code here
    }

    #[test]
    fn test_calculate_cost_above_free_tier() {
        // Your code here
    }

    #[test]
    fn test_calculate_cost_multiple_tiers() {
        // Your code here
    }

    // TODO: Write tests for validate_phone_number
    // Test valid numbers, empty string, missing '+', too short, too long
    
    #[test]
    fn test_validate_phone_valid() {
        // Your code here - use Result return type with ?
    }

    #[test]
    fn test_validate_phone_empty() {
        // Your code here
    }

    #[test]
    fn test_validate_phone_no_plus() {
        // Your code here
    }

    #[test]
    fn test_validate_phone_too_short() {
        // Your code here
    }

    // TODO: Write a test for format_message_for_api
    // Create a message and verify the formatted output matches expected format
    
    #[test]
    fn test_format_message_for_api() {
        // Your code here
    }
}

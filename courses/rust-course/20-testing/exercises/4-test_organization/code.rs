// Textio Test Organization Exercise
//
// In this exercise, you'll organize tests using modules, fixtures,
// and special test attributes.
//
// TODO: Complete all the tests marked with TODO comments

use std::cell::RefCell;

pub struct Message {
    pub id: String,
    pub to: String,
    pub body: String,
}

impl Message {
    pub fn new(id: &str, to: &str, body: &str) -> Self {
        Message {
            id: id.to_string(),
            to: to.to_string(),
            body: body.to_string(),
        }
    }
}

pub struct Client {
    api_key: String,
    messages: RefCell<Vec<Message>>,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn send(&self, to: &str, body: &str) -> Result<Message, String> {
        if !to.starts_with('+') {
            return Err("Phone number must start with '+'".to_string());
        }
        if body.is_empty() {
            return Err("Message body cannot be empty".to_string());
        }
        if body.len() > 160 {
            return Err("Message too long (max 160 characters)".to_string());
        }

        let message = Message {
            id: format!("msg_{}", self.messages.borrow().len() + 1),
            to: to.to_string(),
            body: body.to_string(),
        };
        
        self.messages.borrow_mut().push(message.clone());
        Ok(message)
    }

    pub fn send_batch(&self, messages: Vec<(&str, &str)>) -> Vec<Result<Message, String>> {
        messages.iter().map(|(to, body)| self.send(to, body)).collect()
    }

    pub fn message_count(&self) -> usize {
        self.messages.borrow().len()
    }

    pub fn clear(&self) {
        self.messages.borrow_mut().clear();
    }
}

pub fn calculate_cost(message_count: u32) -> u32 {
    if message_count <= 100 {
        return 0;
    }
    let billable = message_count - 100;
    let units = (billable + 99) / 100;
    units * 5
}

pub fn validate_phone(phone: &str) -> Result<String, String> {
    if !phone.starts_with('+') {
        return Err("Phone number must start with '+'".to_string());
    }
    
    let digits: String = phone[1..].chars().filter(|c| c.is_ascii_digit()).collect();
    
    match digits.len() {
        0..=9 => Err(format!("Phone too short: {} digits", digits.len())),
        10..=15 => Ok(phone.to_string()),
        _ => Err(format!("Phone too long: {} digits", digits.len())),
    }
}

/// Panics if phone number is invalid
pub fn validate_and_panic(phone: &str) {
    if let Err(e) = validate_phone(phone) {
        panic!("{}", e);
    }
}

pub fn parse_message_batch(input: &str) -> Vec<Message> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some(Message::new(
                    &format!("batch_{}", parts[0]),
                    parts[0],
                    parts[1],
                ))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Create a test fixture module with:
    // - valid_phone() -> returns a valid phone number string
    // - invalid_phone() -> returns an invalid phone number string
    // - test_client() -> returns a Client with test API key
    // - sample_messages() -> returns a Vec of test message tuples

    mod fixtures {
        // Your fixtures here
    }

    // TODO: Create a validation test module with tests for:
    // - valid phone numbers (use fixture)
    // - invalid phone numbers (no +, too short, too long)
    // - A #[should_panic] test for validate_and_panic

    mod validation {
        // Your validation tests here
    }

    // TODO: Create a cost calculation test module with:
    // - Tests for all edge cases using a table-driven approach
    // - Organize by: free tier, boundary, paid tier

    mod cost_calculation {
        // Your cost tests here
    }

    // TODO: Create a client test module with:
    // - Tests for single message sending
    // - Tests for batch sending
    // - A slow test marked with #[ignore]

    mod client_tests {
        // Your client tests here
    }

    // TODO: Create a message parsing test module with:
    // - Tests for parse_message_batch
    // - Tests using fixtures

    mod message_parsing {
        // Your parsing tests here
    }

    // TODO: Add a comprehensive integration test that:
    // - Uses multiple fixtures
    // - Tests a complete workflow
    // - Is marked with #[ignore] as it's "slow"

    #[test]
    #[ignore]
    fn complete_workflow() {
        // Your integration test here
    }
}

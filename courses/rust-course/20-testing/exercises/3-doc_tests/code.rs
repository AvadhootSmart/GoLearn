// Textio Documentation Tests Exercise
//
// In this exercise, you'll add documentation with working code examples
// that serve as tests. Add doc comments to all the public items.
//
// TODO: Add documentation with code examples to all items marked with TODO

/// Textio - A simple SMS API library.
///
/// This library provides functionality for sending SMS messages,
/// managing contacts, and tracking message delivery.
///
/// # Getting Started
///
/// TODO: Add a getting started example that shows:
/// - Creating a client
/// - Sending a message
// Insert your module-level documentation here

pub struct Message {
    pub id: String,
    pub to: String,
    pub body: String,
    pub status: String,
}

/// TODO: Add documentation for Message::new
/// - Describe what it does
/// - Document parameters
/// - Add example usage
impl Message {
    pub fn new(id: &str, to: &str, body: &str) -> Self {
        Message {
            id: id.to_string(),
            to: to.to_string(),
            body: body.to_string(),
            status: "pending".to_string(),
        }
    }

    /// TODO: Document the is_valid method
    /// - Show it returns true for valid messages
    /// - Show it returns false for invalid messages
    pub fn is_valid(&self) -> bool {
        !self.to.is_empty() && !self.body.is_empty() && self.to.starts_with('+')
    }

    /// TODO: Document the char_count method
    /// - Show basic usage
    /// - Show it counts Unicode characters correctly
    pub fn char_count(&self) -> usize {
        self.body.chars().count()
    }
}

/// TODO: Document the Client struct
/// - Explain its purpose
/// - Show how to create one
pub struct Client {
    api_key: String,
    messages: Vec<Message>,
}

impl Client {
    /// TODO: Document Client::new
    /// - Explain the api_key parameter
    /// - Show example usage
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
            messages: Vec::new(),
        }
    }

    /// TODO: Document send method with:
    /// - Description of what it does
    /// - # Arguments section
    /// - # Returns section
    /// - # Errors section
    /// - # Examples showing success and error cases
    pub fn send(&mut self, to: &str, body: &str) -> Result<Message, String> {
        if !to.starts_with('+') {
            return Err("Phone number must start with '+'".to_string());
        }
        
        if body.is_empty() {
            return Err("Message body cannot be empty".to_string());
        }

        let message = Message {
            id: format!("msg_{}", self.messages.len() + 1),
            to: to.to_string(),
            body: body.to_string(),
            status: "sent".to_string(),
        };
        
        self.messages.push(message.clone());
        Ok(message)
    }

    /// TODO: Document get_message method
    pub fn get_message(&self, id: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }

    /// TODO: Document message_count method
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

/// TODO: Document calculate_cost function
/// - Explain the pricing model (first 100 free, then $5 per 100)
/// - Show multiple examples:
///   - 0 messages (free)
///   - 50 messages (free)
///   - 100 messages (free)
///   - 150 messages ($5)
///   - 250 messages ($10)
pub fn calculate_cost(message_count: u32) -> u32 {
    if message_count <= 100 {
        return 0;
    }
    
    let billable = message_count - 100;
    let full_units = billable / 100;
    let partial = if billable % 100 > 0 { 1 } else { 0 };
    
    (full_units + partial) * 5
}

/// TODO: Document validate_phone function
/// - Explain validation rules:
///   - Must start with '+'
///   - Must have 10-15 digits after '+'
/// - Show valid and invalid examples
pub fn validate_phone(phone: &str) -> Result<String, String> {
    if !phone.starts_with('+') {
        return Err("Phone must start with '+'".to_string());
    }
    
    let digits: String = phone[1..].chars().filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() < 10 {
        return Err(format!("Phone too short: {} digits (need 10-15)", digits.len()));
    }
    
    if digits.len() > 15 {
        return Err(format!("Phone too long: {} digits (need 10-15)", digits.len()));
    }
    
    Ok(phone.to_string())
}

/// TODO: Document format_message function
/// - Show the output format
/// - Add example
pub fn format_message(to: &str, body: &str) -> String {
    format!("TO:{}|BODY:{}", to, body)
}

// When you're done, run: cargo test --doc
// Then view docs: cargo doc --open

//! Textio - A simple SMS API library.
//!
//! This library provides functionality for sending SMS messages,
//! managing contacts, and tracking message delivery.
//!
//! # Getting Started
//!
//! ```
//! use textio::{Client, calculate_cost, validate_phone};
//!
//! // Create a client with your API key
//! let mut client = Client::new("your_api_key");
//!
//! // Send a message
//! let message = client.send("+1234567890", "Hello, World!")?;
//! println!("Sent message with ID: {}", message.id);
//!
//! // Calculate costs
//! assert_eq!(calculate_cost(150), 5);
//!
//! // Validate phone numbers
//! let valid = validate_phone("+1234567890")?;
//! # Ok::<(), String>(())
//! ```

/// An SMS message with delivery tracking.
///
/// Messages are created when sent and track their delivery status.
///
/// # Examples
///
/// ```
/// use textio::Message;
///
/// let msg = Message::new("msg_1", "+1234567890", "Hello!");
/// assert!(msg.is_valid());
/// assert_eq!(msg.char_count(), 6);
/// ```
pub struct Message {
    pub id: String,
    pub to: String,
    pub body: String,
    pub status: String,
}

impl Message {
    /// Creates a new message with the given parameters.
    ///
    /// The message starts with "pending" status.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the message
    /// * `to` - Recipient phone number in E.164 format (+countrycode...)
    /// * `body` - The message content
    ///
    /// # Examples
    ///
    /// ```
    /// use textio::Message;
    ///
    /// let msg = Message::new("msg_123", "+15551234567", "Hello from Textio!");
    /// assert_eq!(msg.id, "msg_123");
    /// assert_eq!(msg.to, "+15551234567");
    /// assert_eq!(msg.body, "Hello from Textio!");
    /// assert_eq!(msg.status, "pending");
    /// ```
    pub fn new(id: &str, to: &str, body: &str) -> Self {
        Message {
            id: id.to_string(),
            to: to.to_string(),
            body: body.to_string(),
            status: "pending".to_string(),
        }
    }

    /// Checks if the message has valid recipient and body.
    ///
    /// A message is valid if:
    /// - The recipient phone number starts with '+'
    /// - Neither the recipient nor body is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use textio::Message;
    ///
    /// // Valid message
    /// let valid = Message::new("msg_1", "+1234567890", "Hello");
    /// assert!(valid.is_valid());
    ///
    /// // Invalid: no '+' prefix
    /// let no_plus = Message::new("msg_2", "1234567890", "Hello");
    /// assert!(!no_plus.is_valid());
    ///
    /// // Invalid: empty body
    /// let empty_body = Message::new("msg_3", "+1234567890", "");
    /// assert!(!empty_body.is_valid());
    ///
    /// // Invalid: empty recipient
    /// let empty_to = Message::new("msg_4", "", "Hello");
    /// assert!(!empty_to.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.to.is_empty() && !self.body.is_empty() && self.to.starts_with('+')
    }

    /// Returns the character count of the message body.
    ///
    /// This correctly counts Unicode characters, not bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use textio::Message;
    ///
    /// let msg = Message::new("msg_1", "+1234567890", "Hello");
    /// assert_eq!(msg.char_count(), 5);
    ///
    /// // Unicode characters count correctly
    /// let unicode_msg = Message::new("msg_2", "+1234567890", "Hello 🌍");
    /// assert_eq!(unicode_msg.char_count(), 8); // 7 chars + 1 emoji
    ///
    /// let emoji_only = Message::new("msg_3", "+1234567890", "🎉🎊");
    /// assert_eq!(emoji_only.char_count(), 2);
    /// ```
    pub fn char_count(&self) -> usize {
        self.body.chars().count()
    }
}

/// A client for sending SMS messages through the Textio API.
///
/// The client manages message sending and tracks sent messages.
///
/// # Examples
///
/// ```
/// use textio::Client;
///
/// let mut client = Client::new("your_api_key");
/// let msg = client.send("+1234567890", "Hello!")?;
/// assert_eq!(client.message_count(), 1);
/// # Ok::<(), String>(())
/// ```
pub struct Client {
    api_key: String,
    messages: Vec<Message>,
}

impl Client {
    /// Creates a new Textio client with the given API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Textio API key for authentication
    ///
    /// # Examples
    ///
    /// ```
    /// use textio::Client;
    ///
    /// let client = Client::new("sk_test_12345");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
            messages: Vec::new(),
        }
    }

    /// Sends an SMS message to the specified recipient.
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient phone number in E.164 format (must start with '+')
    /// * `body` - The message content (cannot be empty)
    ///
    /// # Returns
    ///
    /// Returns `Ok(Message)` with the created message on success.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if:
    /// - Phone number doesn't start with '+'
    /// - Message body is empty
    ///
    /// # Examples
    ///
    /// Successfully sending a message:
    ///
    /// ```
    /// use textio::Client;
    ///
    /// let mut client = Client::new("test_key");
    /// let msg = client.send("+15551234567", "Hello from Textio!")?;
    ///
    /// assert!(msg.id.starts_with("msg_"));
    /// assert_eq!(msg.to, "+15551234567");
    /// assert_eq!(msg.body, "Hello from Textio!");
    /// assert_eq!(msg.status, "sent");
    /// # Ok::<(), String>(())
    /// ```
    ///
    /// Handling errors:
    ///
    /// ```
    /// use textio::Client;
    ///
    /// let mut client = Client::new("test_key");
    ///
    /// // Invalid phone number (no '+')
    /// let err = client.send("5551234567", "Hello").unwrap_err();
    /// assert!(err.contains("'+'"));
    ///
    /// // Empty message body
    /// let err = client.send("+15551234567", "").unwrap_err();
    /// assert!(err.contains("empty"));
    /// ```
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

    /// Retrieves a message by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the message
    ///
    /// # Returns
    ///
    /// Returns `Some(&Message)` if found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use textio::Client;
    ///
    /// let mut client = Client::new("test_key");
    /// let sent = client.send("+1234567890", "Test")?;
    ///
    /// let found = client.get_message(&sent.id);
    /// assert!(found.is_some());
    ///
    /// let not_found = client.get_message("nonexistent");
    /// assert!(not_found.is_none());
    /// # Ok::<(), String>(())
    /// ```
    pub fn get_message(&self, id: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }

    /// Returns the total number of messages sent through this client.
    ///
    /// # Examples
    ///
    /// ```
    /// use textio::Client;
    ///
    /// let mut client = Client::new("test_key");
    /// assert_eq!(client.message_count(), 0);
    ///
    /// client.send("+1234567890", "First")?;
    /// assert_eq!(client.message_count(), 1);
    ///
    /// client.send("+1234567890", "Second")?;
    /// assert_eq!(client.message_count(), 2);
    /// # Ok::<(), String>(())
    /// ```
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

/// Calculates the cost for a given number of messages.
///
/// Pricing model:
/// - First 100 messages are free
/// - After that: $5 per 100 messages (or partial 100)
///
/// # Examples
///
/// Free tier:
///
/// ```
/// use textio::calculate_cost;
///
/// assert_eq!(calculate_cost(0), 0);
/// assert_eq!(calculate_cost(50), 0);
/// assert_eq!(calculate_cost(100), 0);
/// ```
///
/// Paid tier:
///
/// ```
/// use textio::calculate_cost;
///
/// assert_eq!(calculate_cost(101), 5);
/// assert_eq!(calculate_cost(150), 5);
/// assert_eq!(calculate_cost(200), 5);
/// assert_eq!(calculate_cost(201), 10);
/// assert_eq!(calculate_cost(250), 10);
/// assert_eq!(calculate_cost(1000), 45);
/// ```
pub fn calculate_cost(message_count: u32) -> u32 {
    if message_count <= 100 {
        return 0;
    }
    
    let billable = message_count - 100;
    let full_units = billable / 100;
    let partial = if billable % 100 > 0 { 1 } else { 0 };
    
    (full_units + partial) * 5
}

/// Validates a phone number in E.164 format.
///
/// Validation rules:
/// - Must start with '+'
/// - Must contain 10-15 digits after the '+'
///
/// # Arguments
///
/// * `phone` - The phone number to validate
///
/// # Returns
///
/// Returns `Ok(String)` with the validated phone number on success.
///
/// # Errors
///
/// Returns `Err(String)` if validation fails, with a descriptive message.
///
/// # Examples
///
/// Valid phone numbers:
///
/// ```
/// use textio::validate_phone;
///
/// let phone = validate_phone("+1234567890")?;
/// assert_eq!(phone, "+1234567890");
///
/// let phone = validate_phone("+123456789012345")?; // 15 digits
/// assert_eq!(phone, "+123456789012345");
/// # Ok::<(), String>(())
/// ```
///
/// Invalid phone numbers:
///
/// ```
/// use textio::validate_phone;
///
/// // Missing '+'
/// let err = validate_phone("1234567890").unwrap_err();
/// assert!(err.contains("'+'"));
///
/// // Too short (only 9 digits)
/// let err = validate_phone("+123456789").unwrap_err();
/// assert!(err.contains("short"));
///
/// // Too long (16 digits)
/// let err = validate_phone("+1234567890123456").unwrap_err();
/// assert!(err.contains("long"));
/// ```
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

/// Formats a message for API transmission.
///
/// Creates a pipe-delimited string with TO and BODY fields.
///
/// # Arguments
///
/// * `to` - The recipient phone number
/// * `body` - The message content
///
/// # Returns
///
/// A formatted string in the format: `TO:<phone>|BODY:<message>`
///
/// # Examples
///
/// ```
/// use textio::format_message;
///
/// let formatted = format_message("+1234567890", "Hello!");
/// assert_eq!(formatted, "TO:+1234567890|BODY:Hello!");
///
/// let empty_body = format_message("+1234567890", "");
/// assert_eq!(empty_body, "TO:+1234567890|BODY:");
/// ```
pub fn format_message(to: &str, body: &str) -> String {
    format!("TO:{}|BODY:{}", to, body)
}

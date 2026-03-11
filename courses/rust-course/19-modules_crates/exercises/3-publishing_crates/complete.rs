// Textio SDK - Publishing Exercise - Complete Solution

//! # Textio SDK
//!
//! A Rust SDK for the Textio SMS messaging API.
//!
//! ## Features
//!
//! - Send SMS messages to any phone number
//! - Track message delivery status
//! - Type-safe API with comprehensive error handling
//! - Support for synchronous operations
//!
//! ## Quick Start
//!
//! ```rust
//! use textio_sdk::{Client, Message};
//!
//! // Create a client with your API key
//! let client = Client::new("your-api-key");
//!
//! // Create a message
//! let msg = Message::new("+15551234567", "+15557654321", "Hello!");
//!
//! // Send the message
//! let result = client.send(&msg).unwrap();
//! println!("Message ID: {}", result.message_id);
//! ```
//!
//! ## Features
//!
//! - `default` - Includes sync functionality
//! - `sync` - Synchronous API operations
//! - `async` - Async API operations (requires tokio)
//!
//! ## Installation
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! textio-sdk = "0.1"
//! ```

use std::fmt;

/// Represents an SMS message to be sent via the Textio API.
///
/// # Example
///
/// ```
/// use textio_sdk::Message;
///
/// let msg = Message::new("+15551234567", "+15557654321", "Hello!");
/// ```
#[derive(Debug, Clone)]
pub struct Message {
    /// Recipient phone number in E.164 format (e.g., "+15551234567")
    pub to: String,
    
    /// Sender phone number in E.164 format
    pub from: String,
    
    /// Message body content (max 1600 characters for concatenated SMS)
    pub body: String,
}

impl Message {
    /// Creates a new Message instance.
    ///
    /// # Arguments
    ///
    /// * `to` - Recipient phone number in E.164 format
    /// * `from` - Sender phone number in E.164 format
    /// * `body` - Message content
    ///
    /// # Example
    ///
    /// ```
    /// use textio_sdk::Message;
    ///
    /// let msg = Message::new("+15551234567", "+15557654321", "Hello!");
    /// assert_eq!(msg.to, "+15551234567");
    /// ```
    pub fn new(to: &str, from: &str, body: &str) -> Self {
        Self {
            to: to.to_string(),
            from: from.to_string(),
            body: body.to_string(),
        }
    }
}

/// The delivery status of a message.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    /// Message is queued for sending
    Pending,
    /// Message has been sent to the carrier
    Sent,
    /// Message has been delivered to the recipient
    Delivered,
    /// Message delivery failed
    Failed,
}

/// Result returned after successfully sending a message.
#[derive(Debug, Clone)]
pub struct SendResult {
    /// Unique identifier for the sent message
    pub message_id: String,
    
    /// Current delivery status of the message
    pub status: MessageStatus,
}

/// Error types that can occur when using the SDK.
#[derive(Debug, Clone)]
pub enum SdkError {
    /// The recipient phone number is invalid or empty
    InvalidRecipient,
    
    /// The message body is empty
    EmptyBody,
    
    /// The message ID is invalid or empty
    InvalidMessageId,
    
    /// An error occurred while communicating with the API
    ApiError(String),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdkError::InvalidRecipient => write!(f, "Invalid recipient phone number"),
            SdkError::EmptyBody => write!(f, "Message body cannot be empty"),
            SdkError::InvalidMessageId => write!(f, "Invalid message ID"),
            SdkError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for SdkError {}

/// Client for interacting with the Textio SMS API.
///
/// The client handles authentication and communication with the Textio API.
///
/// # Example
///
/// ```
/// use textio_sdk::{Client, Message};
///
/// let client = Client::new("your-api-key");
/// let msg = Message::new("+15551234567", "+15557654321", "Hello!");
/// let result = client.send(&msg);
/// ```
pub struct Client {
    /// API key for authentication.
    /// 
    /// Keep this secret! Do not expose in logs or error messages.
    pub api_key: String,
}

impl Client {
    /// Creates a new Client instance with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Textio API key (starts with "sk_" for secret keys)
    ///
    /// # Example
    ///
    /// ```
    /// use textio_sdk::Client;
    ///
    /// let client = Client::new("sk_test_12345");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
    
    /// Sends an SMS message via the Textio API.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    ///
    /// # Returns
    ///
    /// A `SendResult` containing the message ID and initial status,
    /// or an `SdkError` if the send failed.
    ///
    /// # Errors
    ///
    /// Returns `SdkError::InvalidRecipient` if the recipient is empty.
    /// Returns `SdkError::EmptyBody` if the message body is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use textio_sdk::{Client, Message};
    ///
    /// let client = Client::new("sk_test_12345");
    /// let msg = Message::new("+15551234567", "+15557654321", "Hello!");
    ///
    /// match client.send(&msg) {
    ///     Ok(result) => println!("Sent! ID: {}", result.message_id),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn send(&self, message: &Message) -> Result<SendResult, SdkError> {
        if message.to.is_empty() {
            return Err(SdkError::InvalidRecipient);
        }
        if message.body.is_empty() {
            return Err(SdkError::EmptyBody);
        }
        
        Ok(SendResult {
            message_id: format!("msg_{}", uuid()),
            status: MessageStatus::Sent,
        })
    }
    
    /// Gets the current delivery status of a message.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The ID of the message to check
    ///
    /// # Returns
    ///
    /// The current `MessageStatus`, or an `SdkError` if the check failed.
    ///
    /// # Example
    ///
    /// ```
    /// use textio_sdk::Client;
    ///
    /// let client = Client::new("sk_test_12345");
    /// let status = client.get_status("msg_123").unwrap();
    /// println!("Status: {:?}", status);
    /// ```
    pub fn get_status(&self, message_id: &str) -> Result<MessageStatus, SdkError> {
        if message_id.is_empty() {
            return Err(SdkError::InvalidMessageId);
        }
        
        Ok(MessageStatus::Delivered)
    }
}

fn uuid() -> String {
    format!("{:x}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos())
}

fn main() {
    println!("=== Textio SDK Demo ===\n");
    
    let client = Client::new("sk_test_12345");
    println!("Client created with API key\n");
    
    let msg = Message::new(
        "+15551234567",
        "+15557654321",
        "Hello from Textio SDK!"
    );
    println!("Message created:");
    println!("  To: {}", msg.to);
    println!("  From: {}", msg.from);
    println!("  Body: {}\n", msg.body);
    
    match client.send(&msg) {
        Ok(result) => {
            println!("Message sent successfully!");
            println!("  Message ID: {}", result.message_id);
            println!("  Status: {:?}\n", result.status);
            
            match client.get_status(&result.message_id) {
                Ok(status) => println!("Current status: {:?}", status),
                Err(e) => println!("Error checking status: {:?}", e),
            }
        }
        Err(e) => println!("Error sending message: {:?}", e),
    }
    
    println!("\n=== Publication Checklist ===");
    println!("✓ Cargo.toml metadata complete");
    println!("✓ README.md with installation and usage");
    println!("✓ LICENSE file (MIT OR Apache-2.0)");
    println!("✓ Crate-level documentation");
    println!("✓ Item-level documentation with examples");
    println!("✓ Semantic versioning (0.1.0)");
    println!("✓ Keywords and categories");
    println!("\nReady to publish: cargo publish");
}

// ============================================================================
// Cargo.toml (Complete)
// ============================================================================
// [package]
// name = "textio-sdk"
// version = "0.1.0"
// edition = "2021"
// authors = ["Textio Team <team@textio.dev>"]
// description = "A Rust SDK for the Textio SMS messaging API"
// license = "MIT OR Apache-2.0"
// repository = "https://github.com/textio/textio-sdk"
// homepage = "https://textio.dev"
// documentation = "https://docs.rs/textio-sdk"
// readme = "README.md"
// keywords = ["sms", "messaging", "api", "textio", "communication"]
// categories = ["api-bindings", "web-programming"]
//
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
//
// [features]
// default = ["sync"]
// sync = []
// async = ["tokio"]

// ============================================================================
// README.md (Complete)
// ============================================================================
// # textio-sdk
//
// A Rust SDK for the Textio SMS messaging API.
//
// ## Installation
//
// ```toml
// [dependencies]
// textio-sdk = "0.1"
// ```
//
// ## Usage
//
// ```rust
// use textio_sdk::{Client, Message};
//
// let client = Client::new("your-api-key");
// let msg = Message::new("+15551234567", "+15557654321", "Hello!");
// let result = client.send(&msg).unwrap();
// ```
//
// ## Features
//
// - Send SMS messages
// - Track delivery status
// - Type-safe error handling
//
// ## License
//
// MIT OR Apache-2.0

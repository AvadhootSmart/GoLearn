// Textio SDK - Publishing Exercise
// 
// This exercise focuses on preparing a crate for publication to crates.io
// You'll add proper metadata, documentation, and follow best practices

// ============================================================================
// TODO 1: Complete the Cargo.toml
// ============================================================================
// File: Cargo.toml
//
// [package]
// name = "textio-sdk"
// version = "0.1.0"
// edition = "2021"
// 
// TODO: Add the following required and recommended fields:
// - authors (your name and email)
// - description (brief description of the SDK)
// - license (use MIT OR Apache-2.0 for dual licensing)
// - repository (GitHub URL)
// - homepage (project website)
// - documentation (docs.rs URL pattern)
// - readme (path to README.md)
// - keywords (up to 5: sms, messaging, api, textio, communication)
// - categories (from crates.io categories)
//
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

// ============================================================================
// TODO 2: Create README.md
// ============================================================================
// File: README.md
//
// Include:
// - Project title and description
// - Installation instructions
// - Quick start / usage example
// - Features list
// - License information
// - Contributing guidelines

// ============================================================================
// TODO 3: Add crate-level documentation
// ============================================================================
// Add comprehensive crate documentation including:
// - Overview of what the crate does
// - Features list
// - Quick example
// - Links to detailed documentation

// --- Your crate-level docs here ---


// ============================================================================
// TODO 4: Complete the Message struct with documentation
// ============================================================================

// Add documentation for:
// - The struct itself
// - Each field
// - An example in the doc comment

pub struct Message {
    pub to: String,
    pub from: String,
    pub body: String,
}

impl Message {
    // TODO: Add documentation with example
    pub fn new(to: &str, from: &str, body: &str) -> Self {
        Self {
            to: to.to_string(),
            from: from.to_string(),
            body: body.to_string(),
        }
    }
}

// ============================================================================
// TODO 5: Complete the MessageStatus enum with documentation
// ============================================================================

// Add documentation for each variant

pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

// ============================================================================
// TODO 6: Complete the Client struct with documentation
// ============================================================================

// Document:
// - The struct
// - The api_key field (why it's needed, security considerations)
// - Methods

pub struct Client {
    pub api_key: String,
}

impl Client {
    // TODO: Document with example
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
    
    // TODO: Document with example and errors
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
    
    // TODO: Document with example
    pub fn get_status(&self, message_id: &str) -> Result<MessageStatus, SdkError> {
        if message_id.is_empty() {
            return Err(SdkError::InvalidMessageId);
        }
        
        Ok(MessageStatus::Delivered)
    }
}

// ============================================================================
// TODO 7: Document the SendResult struct
// ============================================================================

pub struct SendResult {
    pub message_id: String,
    pub status: MessageStatus,
}

// ============================================================================
// TODO 8: Document the error types
// ============================================================================

pub enum SdkError {
    InvalidRecipient,
    EmptyBody,
    InvalidMessageId,
    ApiError(String),
}

// ============================================================================
// TODO 9: Add feature flags documentation
// ============================================================================
// In Cargo.toml, add:
// [features]
// default = ["sync"]
// sync = []
// async = ["tokio"]
// 
// Document these features in crate-level docs

// Simple uuid generator for demo
fn uuid() -> String {
    format!("{:x}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos())
}

// ============================================================================
// Main function demonstrating the SDK
// ============================================================================

fn main() {
    println!("=== Textio SDK Demo ===\n");
    
    // Create client
    let client = Client::new("sk_test_12345");
    println!("Client created with API key\n");
    
    // Create message
    let msg = Message::new(
        "+15551234567",
        "+15557654321",
        "Hello from Textio SDK!"
    );
    println!("Message created:");
    println!("  To: {}", msg.to);
    println!("  From: {}", msg.from);
    println!("  Body: {}\n", msg.body);
    
    // Send message
    match client.send(&msg) {
        Ok(result) => {
            println!("Message sent successfully!");
            println!("  Message ID: {}", result.message_id);
            println!("  Status: {:?}\n", result.status);
            
            // Check status
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

// HINTS:
// 1. Use /// for item documentation (shows in docs)
// 2. Use //! for crate/module documentation
// 3. Include code examples in doc comments with ```rust
// 4. Run `cargo doc --open` to preview documentation
// 5. Use `cargo publish --dry-run` to test before publishing
// 6. Ensure all public items have documentation

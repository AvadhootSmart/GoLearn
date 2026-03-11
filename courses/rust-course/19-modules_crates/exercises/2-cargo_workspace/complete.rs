// Textio Cargo Workspace Exercise - Complete Solution

// ============================================================================
// shared/textio-types/src/lib.rs
// ============================================================================
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub to: String,
    pub body: String,
    pub status: MessageStatus,
}

// ============================================================================
// crates/textio-core/src/lib.rs
// ============================================================================
pub use textio_types::{Message, MessageStatus};

pub fn create_message(id: u64, to: String, body: String) -> Message {
    Message {
        id,
        to,
        body,
        status: MessageStatus::Pending,
    }
}

pub fn send_message(message: &mut Message) -> Result<(), String> {
    if message.to.is_empty() {
        return Err("Recipient cannot be empty".to_string());
    }
    if message.body.is_empty() {
        return Err("Message body cannot be empty".to_string());
    }
    message.status = MessageStatus::Sent;
    Ok(())
}

pub fn get_status(message: &Message) -> MessageStatus {
    message.status.clone()
}

pub fn format_for_api(message: &Message) -> String {
    serde_json::to_string(message).unwrap_or_else(|_| "{}".to_string())
}

// ============================================================================
// crates/textio-cli/src/main.rs
// ============================================================================
use textio_core::{create_message, format_for_api, send_message};
use textio_types::MessageStatus;

fn cli_main() {
    println!("=== Textio CLI ===\n");
    
    let mut msg = create_message(
        1,
        "+15551234567".to_string(),
        "Hello from Textio CLI!".to_string(),
    );
    
    println!("Created message with ID: {}", msg.id);
    println!("Status: {:?}", msg.status);
    
    match send_message(&mut msg) {
        Ok(()) => {
            println!("Message sent successfully!");
            println!("API format: {}", format_for_api(&msg));
        }
        Err(e) => println!("Failed to send: {}", e),
    }
}

// ============================================================================
// crates/textio-server/src/main.rs
// ============================================================================
use textio_core::{create_message, format_for_api, send_message, get_status};
use textio_types::MessageStatus;

fn server_main() {
    println!("=== Textio Server ===\n");
    
    let messages = vec![
        ("+15551111111", "Welcome to Textio!"),
        ("+15552222222", "Your code is 123456"),
        ("+15553333333", "Thanks for using Textio!"),
    ];
    
    println!("Processing {} messages...\n", messages.len());
    
    for (i, (to, body)) in messages.iter().enumerate() {
        let mut msg = create_message(i as u64 + 1, to.to_string(), body.to_string());
        
        match send_message(&mut msg) {
            Ok(()) => {
                let status = get_status(&msg);
                println!("Message {} sent: {:?}", i + 1, status);
            }
            Err(e) => println!("Message {} failed: {}", i + 1, e),
        }
    }
}

// ============================================================================
// Main function demonstrating workspace concepts
// ============================================================================
fn main() {
    println!("=== Textio Workspace Demo ===\n");
    
    println!("Workspace Structure:");
    println!("  textio-workspace/");
    println!("  ├── Cargo.toml (workspace)");
    println!("  ├── crates/");
    println!("  │   ├── textio-core/ (library crate)");
    println!("  │   ├── textio-cli/ (binary crate)");
    println!("  │   └── textio-server/ (binary crate)");
    println!("  └── shared/");
    println!("      └── textio-types/ (shared library)\n");
    
    println!("Shared Dependencies (workspace.dependencies):");
    println!("  - serde with derive feature");
    println!("  - serde_json\n");
    
    println!("Inter-crate Dependencies:");
    println!("  textio-cli depends on: textio-core, textio-types");
    println!("  textio-server depends on: textio-core, textio-types");
    println!("  textio-core depends on: textio-types\n");
    
    println!("To build the workspace:");
    println!("  cargo build --workspace");
    println!("\nTo run the CLI:");
    println!("  cargo run -p textio-cli");
    println!("\nTo test all crates:");
    println!("  cargo test --workspace\n");
    
    // Demonstrate the functionality (simulated)
    println!("=== CLI Demo ===");
    cli_main();
    
    println!("\n=== Server Demo ===");
    server_main();
    
    println!("\n=== Workspace Benefits ===");
    println!("1. Shared Cargo.lock ensures version consistency");
    println!("2. Shared target/ directory reduces disk usage");
    println!("3. Shared dependencies reduce duplication");
    println!("4. Easier to manage related crates together");
    
    println!("\nWorkspace configuration complete!");
}

// ============================================================================
// Cargo.toml files (as comments for reference)
// ============================================================================
// ROOT Cargo.toml:
// [workspace]
// members = [
//     "crates/textio-core",
//     "crates/textio-cli",
//     "crates/textio-server",
//     "shared/textio-types",
// ]
// resolver = "2"
//
// [workspace.dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
//
// shared/textio-types/Cargo.toml:
// [package]
// name = "textio-types"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// serde = { workspace = true }
//
// crates/textio-core/Cargo.toml:
// [package]
// name = "textio-core"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// textio-types = { path = "../../shared/textio-types" }
// serde = { workspace = true }
// serde_json = { workspace = true }
//
// crates/textio-cli/Cargo.toml:
// [package]
// name = "textio-cli"
// version = "0.1.0"
// edition = "2021"
//
// [[bin]]
// name = "textio"
// path = "src/main.rs"
//
// [dependencies]
// textio-core = { path = "../textio-core" }
// textio-types = { path = "../../shared/textio-types" }
//
// crates/textio-server/Cargo.toml:
// [package]
// name = "textio-server"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// textio-core = { path = "../textio-core" }
// textio-types = { path = "../../shared/textio-types" }
// serde_json = { workspace = true }

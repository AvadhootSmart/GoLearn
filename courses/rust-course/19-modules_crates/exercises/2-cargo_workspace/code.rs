// Textio Cargo Workspace Exercise
// 
// This exercise simulates a multi-crate workspace structure
// In a real workspace, these would be separate files in different directories
//
// Workspace structure to create:
// textio-workspace/
// ├── Cargo.toml (workspace root)
// ├── crates/
// │   ├── textio-core/ (library)
// │   │   ├── Cargo.toml
// │   │   └── src/lib.rs
// │   ├── textio-cli/ (binary)
// │   │   ├── Cargo.toml
// │   │   └── src/main.rs
// │   └── textio-server/ (binary)
// │       ├── Cargo.toml
// │       └── src/main.rs
// └── shared/
//     └── textio-types/ (library)
//         ├── Cargo.toml
//         └── src/lib.rs

// ============================================================================
// TODO 1: Create the workspace root Cargo.toml
// ============================================================================
// File: Cargo.toml
//
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

// ============================================================================
// TODO 2: Create shared/textio-types/Cargo.toml
// ============================================================================
// File: shared/textio-types/Cargo.toml
//
// [package]
// name = "textio-types"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// serde = { workspace = true }

// ============================================================================
// TODO 3: Implement shared/textio-types/src/lib.rs
// ============================================================================

// Define the shared types that will be used across all crates
// Message struct with fields: id (u64), to (String), body (String), status (MessageStatus)
// MessageStatus enum with variants: Pending, Sent, Delivered, Failed
// Both should derive Debug, Clone, and appropriate Serde traits

// --- Your code here ---


// ============================================================================
// TODO 4: Create crates/textio-core/Cargo.toml
// ============================================================================
// File: crates/textio-core/Cargo.toml
//
// [package]
// name = "textio-core"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// textio-types = { path = "../../shared/textio-types" }
// serde = { workspace = true }
// serde_json = { workspace = true }

// ============================================================================
// TODO 5: Implement crates/textio-core/src/lib.rs
// ============================================================================

// Create the core SMS functionality
// Include:
// - use textio-types::{Message, MessageStatus}
// - pub fn create_message(id: u64, to: String, body: String) -> Message
// - pub fn send_message(message: &mut Message) -> Result<(), String>
// - pub fn get_status(message: &Message) -> MessageStatus
// - pub fn format_for_api(message: &Message) -> String (use serde_json)

// --- Your code here ---


// ============================================================================
// TODO 6: Create crates/textio-cli/Cargo.toml
// ============================================================================
// File: crates/textio-cli/Cargo.toml
//
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

// ============================================================================
// TODO 7: Implement crates/textio-cli/src/main.rs
// ============================================================================

// Create a simple CLI that demonstrates workspace usage
// - Import from textio_core and textio_types
// - Create a message using textio_core::create_message
// - Send the message using textio_core::send_message
// - Print the formatted API representation

// --- Your code here ---


// ============================================================================
// TODO 8: Create crates/textio-server/Cargo.toml
// ============================================================================
// File: crates/textio-server/Cargo.toml
//
// [package]
// name = "textio-server"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// textio-core = { path = "../textio-core" }
// textio-types = { path = "../../shared/textio-types" }
// serde_json = { workspace = true }

// ============================================================================
// TODO 9: Implement crates/textio-server/src/main.rs
// ============================================================================

// Create a mock server that uses shared types and core functionality
// - Import from textio_core and textio_types
// - Simulate handling a batch of messages
// - Use the core library functions

// --- Your code here ---


// ============================================================================
// Main function to demonstrate workspace concepts (simulated)
// ============================================================================

fn main() {
    // In a real workspace, each crate would be in its own directory
    // This main function simulates the CLI behavior
    
    println!("=== Textio Workspace Demo ===\n");
    
    // Simulate creating the workspace structure
    println!("Workspace Structure:");
    println!("  textio-workspace/");
    println!("  ├── Cargo.toml (workspace)");
    println!("  ├── crates/");
    println!("  │   ├── textio-core/ (library crate)");
    println!("  │   ├── textio-cli/ (binary crate)");
    println!("  │   └── textio-server/ (binary crate)");
    println!("  └── shared/");
    println!("      └── textio-types/ (shared library)\n");
    
    // Demonstrate shared dependencies
    println!("Shared Dependencies (workspace.dependencies):");
    println!("  - serde with derive feature");
    println!("  - serde_json\n");
    
    // Show inter-crate dependencies
    println!("Inter-crate Dependencies:");
    println!("  textio-cli depends on: textio-core, textio-types");
    println!("  textio-server depends on: textio-core, textio-types");
    println!("  textio-core depends on: textio-types\n");
    
    // Note: In a real implementation, you would run:
    // cargo build --workspace    - Build all crates
    // cargo test --workspace     - Test all crates
    // cargo run -p textio-cli    - Run the CLI
    
    println!("To build the workspace:");
    println!("  cargo build --workspace");
    println!("\nTo run the CLI:");
    println!("  cargo run -p textio-cli");
    println!("\nTo test all crates:");
    println!("  cargo test --workspace\n");
    
    println!("=== Workspace Benefits ===");
    println!("1. Shared Cargo.lock ensures version consistency");
    println!("2. Shared target/ directory reduces disk usage");
    println!("3. Shared dependencies reduce duplication");
    println!("4. Easier to manage related crates together");
    
    println!("\nWorkspace configuration complete!");
}

// HINTS:
// 1. Workspace members can be globs: members = ["crates/*", "shared/*"]
// 2. Use path dependencies for local crates: path = "../other-crate"
// 3. Use workspace = true to inherit workspace dependencies
// 4. Each crate needs its own Cargo.toml with [package] section
// 5. Use cargo build -p <name> to build specific crates

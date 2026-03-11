// Textio SMS API - Module Organization Exercise
// 
// TODO: Reorganize this code into a proper module structure
// 
// Required module structure:
// textio (crate root)
// ├── config (public)
// │   └── Settings struct (public)
// ├── models (public)
// │   ├── Message struct (public)
// │   └── User struct (public)
// ├── api (public)
// │   ├── send_message (public)
// │   ├── validate_phone (pub(crate) - internal use only)
// │   └── format_message (pub(super) - for parent module)
// └── utils (pub(crate) - crate internal only)
//     └── format_phone (public within crate)

// TODO: Create the config module with Settings struct
// Settings should have:
// - api_key: String (public)
// - base_url: String (public)
// - timeout_secs: u32 (public)

// --- Your code here ---


// TODO: Create the models module with Message and User structs
// Message should have:
// - to: String (public)
// - body: String (public)
// - from: String (public)
//
// User should have:
// - id: u64 (public)
// - phone: String (public)
// - name: String (public)

// --- Your code here ---


// TODO: Create the api module with messaging functionality
// Include:
// - send_message(msg: Message, settings: &Settings) -> Result<String, String> (public)
// - validate_phone(phone: &str) -> bool (pub(crate))
// - format_message(msg: &Message) -> String (pub(super))
//
// send_message should:
// - Validate the phone number using validate_phone
// - Format the message using format_message
// - Return Ok with formatted message or Err with error message

// --- Your code here ---


// TODO: Create the utils module (crate-internal only)
// Include:
// - format_phone(phone: &str) -> String (pub(crate))
//   This should format phone numbers consistently

// --- Your code here ---


// Main function to test the module structure
fn main() {
    // TODO: Use proper imports with `use` keyword
    
    // Create settings
    let settings = config::Settings {
        api_key: String::from("sk_test_12345"),
        base_url: String::from("https://api.textio.com/v1"),
        timeout_secs: 30,
    };
    
    // Create a message
    let msg = models::Message {
        to: String::from("+15551234567"),
        body: String::from("Hello from Textio!"),
        from: String::from("+15557654321"),
    };
    
    // Create a user
    let user = models::User {
        id: 1,
        phone: String::from("+15559998888"),
        name: String::from("Alice"),
    };
    
    // Send the message
    match api::send_message(msg, &settings) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Demonstrate crate-internal function (this should work)
    if api::validate_phone(&user.phone) {
        println!("User phone is valid");
    }
    
    // Demonstrate utils (this should work within the crate)
    let formatted = utils::format_phone("5551234567");
    println!("Formatted phone: {}", formatted);
    
    println!("\nModule structure verified!");
}

// HINTS:
// 1. Use 'mod name { }' for inline modules
// 2. Use 'pub mod name' to make modules public
// 3. Use 'pub(crate) mod name' for crate-internal modules
// 4. Inside modules, use 'use crate::path::Item' to import from crate root
// 5. Use 'use super::Item' to import from parent module
// 6. Use 'pub(super)' for items visible only to parent module

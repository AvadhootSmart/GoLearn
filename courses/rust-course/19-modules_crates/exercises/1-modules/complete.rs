// Textio SMS API - Module Organization Exercise - Complete Solution

// Config module - public, contains API settings
pub mod config {
    pub struct Settings {
        pub api_key: String,
        pub base_url: String,
        pub timeout_secs: u32,
    }
}

// Models module - public, contains data structures
pub mod models {
    pub struct Message {
        pub to: String,
        pub body: String,
        pub from: String,
    }
    
    pub struct User {
        pub id: u64,
        pub phone: String,
        pub name: String,
    }
}

// API module - public, contains main functionality
pub mod api {
    use crate::config::Settings;
    use crate::models::Message;
    
    pub fn send_message(msg: Message, settings: &Settings) -> Result<String, String> {
        if !validate_phone(&msg.to) {
            return Err(String::from("Invalid recipient phone number"));
        }
        
        if !validate_phone(&msg.from) {
            return Err(String::from("Invalid sender phone number"));
        }
        
        let formatted = format_message(&msg);
        Ok(format!(
            "Sent via {} (timeout: {}s): {}",
            settings.base_url, settings.timeout_secs, formatted
        ))
    }
    
    pub(crate) fn validate_phone(phone: &str) -> bool {
        phone.starts_with('+') && phone.len() >= 10 && phone[1..].chars().all(|c| c.is_numeric())
    }
    
    pub(super) fn format_message(msg: &Message) -> String {
        format!("To: {} | From: {} | Body: {}", msg.to, msg.from, msg.body)
    }
}

// Utils module - crate internal only
pub(crate) mod utils {
    pub(crate) fn format_phone(phone: &str) -> String {
        if phone.starts_with('+') {
            phone.to_string()
        } else {
            format!("+1{}", phone)
        }
    }
}

fn main() {
    use crate::config::Settings;
    use crate::models::{Message, User};
    
    let settings = Settings {
        api_key: String::from("sk_test_12345"),
        base_url: String::from("https://api.textio.com/v1"),
        timeout_secs: 30,
    };
    
    let msg = Message {
        to: String::from("+15551234567"),
        body: String::from("Hello from Textio!"),
        from: String::from("+15557654321"),
    };
    
    let user = User {
        id: 1,
        phone: String::from("+15559998888"),
        name: String::from("Alice"),
    };
    
    match api::send_message(msg, &settings) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    if api::validate_phone(&user.phone) {
        println!("User phone is valid");
    }
    
    let formatted = utils::format_phone("5551234567");
    println!("Formatted phone: {}", formatted);
    
    println!("\nModule structure verified!");
    
    // Demonstrate super usage within a nested context
    mod test_module {
        pub fn test_super_access() {
            // This demonstrates accessing parent module items
            // In a real test module, we'd use super:: to access parent
            println!("Super access demonstrated");
        }
    }
    
    test_module::test_super_access();
    
    // Demonstrate self usage
    mod self_demo {
        pub fn inner() {
            println!("Inner function");
        }
        
        pub fn outer() {
            self::inner(); // Using self to call sibling
        }
    }
    
    self_demo::outer();
}

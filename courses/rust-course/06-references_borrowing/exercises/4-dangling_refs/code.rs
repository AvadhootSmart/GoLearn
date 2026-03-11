// Exercise 4: Why Dangling References Are Impossible
// 
// In this exercise, you'll see how Rust prevents dangling references
// and learn to restructure code to work with the borrow checker.
//
// Key concepts:
// - What dangling references are
// - How Rust prevents them at compile time
// - Common patterns that would create dangling references
// - Fixes for dangling reference errors

fn main() {
    println!("=== Textio: Dangling References Prevention ===\n");
    
    // Part 1: The Local Variable Problem
    // ===================================
    
    println!("Part 1: Returning References to Locals");
    
    // PROBLEM: This function tries to return a reference to a local variable
    // TODO: Fix create_greeting_bad to return an owned String instead
    
    // let greeting = create_greeting_bad();  // Would be a dangling reference!
    // println!("{}", greeting);
    
    // TODO: Call the fixed version
    // let greeting = ???;
    // println!("{}", greeting);
    
    println!();
    
    // Part 2: Scope and Lifetime Issues
    // =================================
    
    println!("Part 2: Scope Problems");
    
    // PROBLEM: Reference to data in smaller scope
    // TODO: Fix this code by moving the variable declaration
    
    // let message_ref;
    // {
    //     let message = String::from("Scoped message");
    //     message_ref = &message;  // Error: message doesn't live long enough
    // }
    // println!("Reference: {}", message_ref);  // Would be dangling
    
    // Your fix here:
    // let message = ???;
    // let message_ref = ???;
    // println!("Reference: {}", message_ref);
    
    println!();
    
    // Part 3: Factory Pattern - Ownership vs Borrowing
    // ================================================
    
    println!("Part 3: Message Factory");
    
    let factory = MessageFactory::new();
    
    // TODO: Use the factory to create a message
    // Note: The factory returns owned values, not references
    // let msg = factory.???;
    // println!("Created: {} -> {}", msg.recipient, msg.body);
    
    println!();
    
    // Part 4: Cache Pattern - Proper Lifetime Management
    // ==================================================
    
    println!("Part 4: Message Cache");
    
    let mut cache = MessageCache::new();
    
    // TODO: Get a message from cache (or compute it)
    // This demonstrates the pattern of borrowing cached data
    // let cached = cache.get_or_create("welcome");
    // println!("Cached message: {}", cached);
    
    // TODO: Get the same message again (should return cached version)
    // let cached_again = cache.???;
    // println!("Cached again: {}", cached_again);
    
    println!();
    
    // Part 5: The 'static Lifetime
    // ============================
    
    println!("Part 5: Static Lifetimes");
    
    // TODO: Get a default error message (static lifetime)
    // let error = get_default_error();
    // println!("Error: {}", error);
    
    // String literals have static lifetime
    let static_str: &'static str = "This lives forever";
    println!("Static: {}", static_str);
    
    println!();
    
    // Part 6: Real-World Textio - Message Validation
    // ==============================================
    
    println!("Part 6: Message Validation");
    
    let validator = MessageValidator::new();
    
    let test_message = "Your verification code is 123456";
    
    // TODO: Validate the message
    // let is_valid = validator.is_valid(???);
    // println!("Message valid: {}", is_valid);
    
    // TODO: Extract the verification code
    // Note: This returns an owned String because the code is extracted
    // let code = validator.extract_code(???);
    // println!("Verification code: {}", code);
    
    println!();
    
    // Part 7: Fixing a Dangling Reference Scenario
    // ============================================
    
    println!("Part 7: Fixing Dangling References");
    
    // PROBLEM: This code has a dangling reference issue
    // fn get_first_word(s: String) -> &str { ... }
    
    // TODO: Implement and use get_first_word_correct
    // The function should take a reference and return a reference
    // with the same lifetime
    
    let text = String::from("Hello, Textio World!");
    
    // TODO: Call your fixed function
    // let first = ???;
    // println!("First word: {}", first);
    
    // The original string is still valid
    println!("Original: {}", text);
}

// Part 1: Factory Functions

// BAD: Returns reference to local (won't compile)
// fn create_greeting_bad() -> &String {
//     let s = String::from("Hello, Textio User!");
//     &s
// }

// TODO: Fix this function to return an owned String
fn create_greeting() -> String {
    String::from("Hello, Textio User!")
}

// Part 3: Message Factory

struct TextMessage {
    recipient: String,
    body: String,
}

struct MessageFactory {
    default_body: String,
}

impl MessageFactory {
    fn new() -> Self {
        MessageFactory {
            default_body: String::from("Welcome to Textio!"),
        }
    }
    
    // TODO: Implement create_message that returns an owned TextMessage
    fn create_message(&self, recipient: &str) -> TextMessage {
        TextMessage {
            recipient: recipient.to_string(),
            body: self.default_body.clone(),
        }
    }
}

// Part 4: Message Cache

struct MessageCache {
    messages: Vec<(String, String)>,  // (key, message)
}

impl MessageCache {
    fn new() -> Self {
        MessageCache {
            messages: Vec::new(),
        }
    }
    
    // Returns an owned String to avoid lifetime issues
    fn get_or_create(&mut self, key: &str) -> String {
        // Check if already cached
        for (k, msg) in &self.messages {
            if k == key {
                return msg.clone();
            }
        }
        
        // Create new message
        let message = format!("Message for key: {}", key);
        self.messages.push((key.to_string(), message.clone()));
        message
    }
}

// Part 5: Static Lifetime

fn get_default_error() -> &'static str {
    "An error occurred. Please try again."
}

// Part 6: Message Validator

struct MessageValidator {
    max_length: usize,
}

impl MessageValidator {
    fn new() -> Self {
        MessageValidator {
            max_length: 160,  // SMS limit
        }
    }
    
    // Takes a reference, returns a value (bool is Copy)
    fn is_valid(&self, message: &str) -> bool {
        message.len() <= self.max_length && !message.is_empty()
    }
    
    // Returns owned String because we're creating new data
    fn extract_code(&self, message: &str) -> String {
        // Find the verification code (last 6 digits)
        let digits: String = message.chars()
            .filter(|c| c.is_numeric())
            .collect();
        
        if digits.len() >= 6 {
            digits[digits.len()-6..].to_string()
        } else {
            String::from("000000")
        }
    }
}

// Part 7: First Word Function

// TODO: Implement this function correctly
// It should take a string slice and return a string slice
// The returned slice should be the first word
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

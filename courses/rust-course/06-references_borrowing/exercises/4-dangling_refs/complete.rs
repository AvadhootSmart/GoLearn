// Exercise 4: Why Dangling References Are Impossible - Complete Solution

fn main() {
    println!("=== Textio: Dangling References Prevention ===\n");
    
    // Part 1: The Local Variable Problem
    // ===================================
    
    println!("Part 1: Returning References to Locals");
    
    let greeting = create_greeting();
    println!("{}", greeting);
    
    println!();
    
    // Part 2: Scope and Lifetime Issues
    // =================================
    
    println!("Part 2: Scope Problems");
    
    let message = String::from("Scoped message");
    let message_ref = &message;
    println!("Reference: {}", message_ref);
    
    println!();
    
    // Part 3: Factory Pattern - Ownership vs Borrowing
    // ================================================
    
    println!("Part 3: Message Factory");
    
    let factory = MessageFactory::new();
    
    let msg = factory.create_message("+15551234567");
    println!("Created: {} -> {}", msg.recipient, msg.body);
    
    println!();
    
    // Part 4: Cache Pattern - Proper Lifetime Management
    // ==================================================
    
    println!("Part 4: Message Cache");
    
    let mut cache = MessageCache::new();
    
    let cached = cache.get_or_create("welcome");
    println!("Cached message: {}", cached);
    
    let cached_again = cache.get_or_create("welcome");
    println!("Cached again: {}", cached_again);
    
    println!();
    
    // Part 5: The 'static Lifetime
    // ============================
    
    println!("Part 5: Static Lifetimes");
    
    let error = get_default_error();
    println!("Error: {}", error);
    
    let static_str: &'static str = "This lives forever";
    println!("Static: {}", static_str);
    
    println!();
    
    // Part 6: Real-World Textio - Message Validation
    // ==============================================
    
    println!("Part 6: Message Validation");
    
    let validator = MessageValidator::new();
    
    let test_message = "Your verification code is 123456";
    
    let is_valid = validator.is_valid(test_message);
    println!("Message valid: {}", is_valid);
    
    let code = validator.extract_code(test_message);
    println!("Verification code: {}", code);
    
    println!();
    
    // Part 7: Fixing a Dangling Reference Scenario
    // ============================================
    
    println!("Part 7: Fixing Dangling References");
    
    let text = String::from("Hello, Textio World!");
    
    let first = get_first_word(&text);
    println!("First word: {}", first);
    
    println!("Original: {}", text);
}

fn create_greeting() -> String {
    String::from("Hello, Textio User!")
}

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
    
    fn create_message(&self, recipient: &str) -> TextMessage {
        TextMessage {
            recipient: recipient.to_string(),
            body: self.default_body.clone(),
        }
    }
}

struct MessageCache {
    messages: Vec<(String, String)>,
}

impl MessageCache {
    fn new() -> Self {
        MessageCache {
            messages: Vec::new(),
        }
    }
    
    fn get_or_create(&mut self, key: &str) -> String {
        for (k, msg) in &self.messages {
            if k == key {
                return msg.clone();
            }
        }
        
        let message = format!("Message for key: {}", key);
        self.messages.push((key.to_string(), message.clone()));
        message
    }
}

fn get_default_error() -> &'static str {
    "An error occurred. Please try again."
}

struct MessageValidator {
    max_length: usize,
}

impl MessageValidator {
    fn new() -> Self {
        MessageValidator {
            max_length: 160,
        }
    }
    
    fn is_valid(&self, message: &str) -> bool {
        message.len() <= self.max_length && !message.is_empty()
    }
    
    fn extract_code(&self, message: &str) -> String {
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

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

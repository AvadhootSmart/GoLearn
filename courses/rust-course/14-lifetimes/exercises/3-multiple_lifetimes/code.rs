// Exercise 3: Multiple Lifetimes
// 
// When references have different valid scopes, use multiple lifetime parameters.
// This gives more flexibility and precision in your APIs.
//
// Key concepts:
// - Independent lifetimes: 'a and 'b are unrelated
// - Output lifetime should match the source of the output
// - Lifetime bounds: T: 'a means T can't contain refs shorter than 'a

use std::collections::HashMap;

// Configuration that lives for the entire application
struct Config {
    api_key: String,
    max_message_length: usize,
    default_region: String,
}

// A temporary cache that might be recreated
struct Cache {
    entries: HashMap<String, String>,
}

// A message processor that uses both config and cache
// TODO: Add appropriate lifetime parameters
struct MessageProcessor {
    config: &Config,
    cache: &mut Cache,
}

// TODO: Implement MessageProcessor with multiple lifetimes
impl MessageProcessor {
    fn new(config: &Config, cache: &mut Cache) -> Self {
        MessageProcessor { config, cache }
    }
    
    // Returns a reference to the API key from config
    // TODO: What lifetime should the return have?
    fn get_api_key(&self) -> &str {
        &self.config.api_key
    }
    
    // Returns the max message length (copy, no lifetime needed)
    fn get_max_length(&self) -> usize {
        self.config.max_message_length
    }
    
    // Process a message and cache the result
    // The input message has its own lifetime, independent of config/cache
    // TODO: Add lifetime parameters
    fn process(&mut self, message_id: &str, message: &str) -> ProcessResult {
        // Check message length
        if message.len() > self.config.max_message_length {
            return ProcessResult {
                success: false,
                error: Some("Message too long"),
                original: message,
            };
        }
        
        // Simulate processing
        let processed = format!("[{}] {}", self.config.default_region, message);
        
        // Cache the result
        self.cache.entries.insert(message_id.to_string(), processed.clone());
        
        ProcessResult {
            success: true,
            error: None,
            original: message,
        }
    }
    
    // Get a cached message
    // TODO: This should work - the cache lives as long as the processor
    fn get_cached(&self, message_id: &str) -> Option<&String> {
        self.cache.entries.get(message_id)
    }
}

// Result of processing a message
// TODO: Add lifetime parameter for the original message reference
struct ProcessResult {
    success: bool,
    error: Option<&str>,
    original: &str,
}

impl ProcessResult {
    // TODO: Add lifetimes
    fn new(success: bool, error: Option<&str>, original: &str) -> Self {
        ProcessResult {
            success,
            error,
            original,
        }
    }
    
    fn is_success(&self) -> bool {
        self.success
    }
    
    // TODO: Return type needs lifetime
    fn get_error(&self) -> Option<&str> {
        self.error
    }
    
    // TODO: Return type needs lifetime
    fn get_original(&self) -> &str {
        self.original
    }
}

// A message with sender and recipient having potentially different lifetimes
// TODO: Add two lifetime parameters
struct AddressedMessage {
    sender: &str,
    recipient: &str,
    body: String,
}

impl AddressedMessage {
    // TODO: Add lifetime parameters
    fn new(sender: &str, recipient: &str, body: String) -> Self {
        AddressedMessage {
            sender,
            recipient,
            body,
        }
    }
    
    // Returns sender reference
    // TODO: What lifetime does this return?
    fn sender(&self) -> &str {
        self.sender
    }
    
    // Returns recipient reference
    // TODO: What lifetime does this return?
    fn recipient(&self) -> &str {
        self.recipient
    }
    
    // Returns owned body
    fn body(&self) -> &str {
        &self.body
    }
}

// A context-aware message that includes environment info
// TODO: Use three lifetime parameters
struct ContextualMessage {
    message: &str,
    user_context: &str,
    app_context: &str,
}

impl ContextualMessage {
    // TODO: Add lifetime parameters
    fn new(message: &str, user_context: &str, app_context: &str) -> Self {
        ContextualMessage {
            message,
            user_context,
            app_context,
        }
    }
    
    // Returns the message
    fn get_message(&self) -> &str {
        self.message
    }
    
    // Returns user context
    fn get_user_context(&self) -> &str {
        self.user_context
    }
    
    // Returns app context
    fn get_app_context(&self) -> &str {
        self.app_context
    }
    
    // Returns a tuple of all references
    // TODO: What are the output lifetimes?
    fn as_tuple(&self) -> (&str, &str, &str) {
        (self.message, self.user_context, self.app_context)
    }
}

// Generic container that stores a reference to T
// TODO: Add lifetime bound to T
struct RefContainer<'a, T> {
    value: &'a T,
}

impl<'a, T> RefContainer<'a, T> {
    fn new(value: &'a T) -> Self {
        RefContainer { value }
    }
    
    fn get(&self) -> &'a T {
        self.value
    }
}

// A function that combines two strings with independent lifetimes
// The output is owned, so no lifetime in return type
fn combine_independent<'a, 'b>(prefix: &'a str, suffix: &'b str) -> String {
    format!("{}{}", prefix, suffix)
}

// A function that returns the first non-empty string
// TODO: Add proper lifetime parameters
fn first_non_empty(a: &str, b: &str, c: &str) -> &str {
    if !a.is_empty() {
        a
    } else if !b.is_empty() {
        b
    } else {
        c
    }
}

// A function where output comes from a specific input
// TODO: 'b should be independent of the output
fn with_prefix<'a>(content: &'a str, _prefix: &str) -> &'a str {
    content
}

fn main() {
    println!("=== Multiple Lifetimes Demo ===\n");
    
    // Set up long-lived config
    let config = Config {
        api_key: "sk-textio-12345".to_string(),
        max_message_length: 160,
        default_region: "US".to_string(),
    };
    
    // Create a cache
    let mut cache = Cache {
        entries: HashMap::new(),
    };
    
    // Create processor
    {
        let mut processor = MessageProcessor::new(&config, &mut cache);
        
        println!("API Key: {}", processor.get_api_key());
        println!("Max Length: {}", processor.get_max_length());
        
        // Process a message
        let msg = "Hello from Textio!";
        let result = processor.process("msg-001", msg);
        println!("\nProcessed: success={}, original='{}'", 
                 result.is_success(), result.get_original());
        
        // Try a too-long message
        let long_msg = "This message is way too long and exceeds the maximum allowed length for SMS messages in this system";
        let result2 = processor.process("msg-002", long_msg);
        if let Some(error) = result2.get_error() {
            println!("Error: {}", error);
        }
        
        // Check cache
        if let Some(cached) = processor.get_cached("msg-001") {
            println!("Cached: {}", cached);
        }
    }
    
    println!("\n=== Addressed Message ===");
    let sender = "5551234567";
    let recipient = "5559876543";
    let addr_msg = AddressedMessage::new(sender, recipient, "Hello!".to_string());
    println!("From: {} To: {}", addr_msg.sender(), addr_msg.recipient());
    println!("Body: {}", addr_msg.body());
    
    println!("\n=== Contextual Message ===");
    let msg = "Test message";
    let user_ctx = "user123";
    let app_ctx = "production";
    let ctx_msg = ContextualMessage::new(msg, user_ctx, app_ctx);
    let (m, u, a) = ctx_msg.as_tuple();
    println!("Message: {}, User: {}, App: {}", m, u, a);
    
    println!("\n=== Independent Lifetimes ===");
    let combined = combine_independent("Hello, ", "World!");
    println!("Combined: {}", combined);
    
    let a = "";
    let b = "second";
    let c = "third";
    println!("First non-empty: {}", first_non_empty(a, b, c));
    
    let content = "main content";
    let prefix = "PREFIX: ";
    println!("Content: {}", with_prefix(content, prefix));
    
    println!("\n=== Generic Container ===");
    let number = 42;
    let container = RefContainer::new(&number);
    println!("Contained value: {}", container.get());
}

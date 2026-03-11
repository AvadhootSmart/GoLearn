// Exercise 3: Multiple Lifetimes - Complete Solution

use std::collections::HashMap;

struct Config {
    api_key: String,
    max_message_length: usize,
    default_region: String,
}

struct Cache {
    entries: HashMap<String, String>,
}

// Two lifetime parameters: 'config for Config, 'cache for Cache
struct MessageProcessor<'config, 'cache> {
    config: &'config Config,
    cache: &'cache mut Cache,
}

impl<'config, 'cache> MessageProcessor<'config, 'cache> {
    fn new(config: &'config Config, cache: &'cache mut Cache) -> Self {
        MessageProcessor { config, cache }
    }
    
    // Returns reference with 'config lifetime (from self.config)
    fn get_api_key(&self) -> &'config str {
        &self.config.api_key
    }
    
    fn get_max_length(&self) -> usize {
        self.config.max_message_length
    }
    
    // 'msg is independent of 'config and 'cache
    fn process<'msg>(&mut self, message_id: &str, message: &'msg str) -> ProcessResult<'msg> {
        if message.len() > self.config.max_message_length {
            return ProcessResult {
                success: false,
                error: Some("Message too long"),
                original: message,
            };
        }
        
        let processed = format!("[{}] {}", self.config.default_region, message);
        self.cache.entries.insert(message_id.to_string(), processed.clone());
        
        ProcessResult {
            success: true,
            error: None,
            original: message,
        }
    }
    
    fn get_cached(&self, message_id: &str) -> Option<&'cache String> {
        self.cache.entries.get(message_id)
    }
}

// The 'a lifetime is for the original message reference
struct ProcessResult<'a> {
    success: bool,
    error: Option<&'a str>,
    original: &'a str,
}

impl<'a> ProcessResult<'a> {
    fn new(success: bool, error: Option<&'a str>, original: &'a str) -> Self {
        ProcessResult {
            success,
            error,
            original,
        }
    }
    
    fn is_success(&self) -> bool {
        self.success
    }
    
    fn get_error(&self) -> Option<&'a str> {
        self.error
    }
    
    fn get_original(&self) -> &'a str {
        self.original
    }
}

// Two independent lifetimes for sender and recipient
struct AddressedMessage<'sender, 'recipient> {
    sender: &'sender str,
    recipient: &'recipient str,
    body: String,
}

impl<'sender, 'recipient> AddressedMessage<'sender, 'recipient> {
    fn new(sender: &'sender str, recipient: &'recipient str, body: String) -> Self {
        AddressedMessage {
            sender,
            recipient,
            body,
        }
    }
    
    fn sender(&self) -> &'sender str {
        self.sender
    }
    
    fn recipient(&self) -> &'recipient str {
        self.recipient
    }
    
    fn body(&self) -> &str {
        &self.body
    }
}

// Three independent lifetimes
struct ContextualMessage<'msg, 'user, 'app> {
    message: &'msg str,
    user_context: &'user str,
    app_context: &'app str,
}

impl<'msg, 'user, 'app> ContextualMessage<'msg, 'user, 'app> {
    fn new(message: &'msg str, user_context: &'user str, app_context: &'app str) -> Self {
        ContextualMessage {
            message,
            user_context,
            app_context,
        }
    }
    
    fn get_message(&self) -> &'msg str {
        self.message
    }
    
    fn get_user_context(&self) -> &'user str {
        self.user_context
    }
    
    fn get_app_context(&self) -> &'app str {
        self.app_context
    }
    
    fn as_tuple(&self) -> (&'msg str, &'user str, &'app str) {
        (self.message, self.user_context, self.app_context)
    }
}

// T: 'a means T cannot contain references shorter than 'a
struct RefContainer<'a, T: 'a> {
    value: &'a T,
}

impl<'a, T: 'a> RefContainer<'a, T> {
    fn new(value: &'a T) -> Self {
        RefContainer { value }
    }
    
    fn get(&self) -> &'a T {
        self.value
    }
}

fn combine_independent<'a, 'b>(prefix: &'a str, suffix: &'b str) -> String {
    format!("{}{}", prefix, suffix)
}

// All three inputs could be returned, so they share lifetime 'a
fn first_non_empty<'a>(a: &'a str, b: &'a str, c: &'a str) -> &'a str {
    if !a.is_empty() {
        a
    } else if !b.is_empty() {
        b
    } else {
        c
    }
}

// Output comes from content, prefix is independent
fn with_prefix<'a, 'b>(content: &'a str, _prefix: &'b str) -> &'a str {
    content
}

fn main() {
    println!("=== Multiple Lifetimes Demo ===\n");
    
    let config = Config {
        api_key: "sk-textio-12345".to_string(),
        max_message_length: 160,
        default_region: "US".to_string(),
    };
    
    let mut cache = Cache {
        entries: HashMap::new(),
    };
    
    {
        let mut processor = MessageProcessor::new(&config, &mut cache);
        
        println!("API Key: {}", processor.get_api_key());
        println!("Max Length: {}", processor.get_max_length());
        
        let msg = "Hello from Textio!";
        let result = processor.process("msg-001", msg);
        println!("\nProcessed: success={}, original='{}'", 
                 result.is_success(), result.get_original());
        
        let long_msg = "This message is way too long and exceeds the maximum allowed length for SMS messages in this system";
        let result2 = processor.process("msg-002", long_msg);
        if let Some(error) = result2.get_error() {
            println!("Error: {}", error);
        }
        
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

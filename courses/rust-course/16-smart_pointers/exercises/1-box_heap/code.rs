// Exercise 1: Box<T> - Heap Allocation and Recursive Types
// 
// TODO: Implement the following structures and functions using Box<T>

use std::cmp::Ordering;

// ============================================================================
// Part 1: Recursive Message Thread
// ============================================================================

/// A message thread tree where each message can have replies
#[derive(Debug)]
pub enum MessageThread {
    Message {
        id: u64,
        content: String,
        sender: String,
        replies: Vec<Box<MessageThread>>,
    },
    Deleted,
}

impl MessageThread {
    /// Create a new message node
    pub fn new(id: u64, content: &str, sender: &str) -> Box<Self> {
        // TODO: Create a boxed MessageThread::Message
        todo!()
    }

    /// Add a reply to this message
    pub fn add_reply(&mut self, reply: Box<MessageThread>) {
        // TODO: If this is a Message, add the reply to replies vector
        // If it's Deleted, do nothing
        todo!()
    }

    /// Count total messages in thread (including replies)
    pub fn count_messages(&self) -> usize {
        // TODO: Recursively count all messages (not Deleted)
        todo!()
    }

    /// Find a message by ID
    pub fn find_by_id(&self, id: u64) -> Option<&Self> {
        // TODO: Search through the tree for a message with the given id
        // Return None if not found or Deleted
        todo!()
    }

    /// Get all messages from a specific sender
    pub fn from_sender(&self, sender: &str) -> Vec<&str> {
        // TODO: Return content of all messages from the given sender
        todo!()
    }
}

// ============================================================================
// Part 2: Binary Search Tree for Contacts
// ============================================================================

/// A contact in the address book
#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub left: Option<Box<Contact>>,
    pub right: Option<Box<Contact>>,
}

impl Contact {
    /// Create a new contact node
    pub fn new(name: &str, phone: &str) -> Box<Self> {
        // TODO: Create a boxed Contact with no children
        todo!()
    }

    /// Insert a new contact into the tree
    pub fn insert(&mut self, name: &str, phone: &str) {
        // TODO: Insert contact in the correct position based on name
        // If name < self.name, go left; otherwise go right
        todo!()
    }

    /// Find a contact by name
    pub fn find(&self, name: &str) -> Option<&String> {
        // TODO: Return phone number if contact found, None otherwise
        todo!()
    }

    /// Count total contacts in tree
    pub fn count(&self) -> usize {
        // TODO: Recursively count all contacts
        todo!()
    }

    /// Get all contacts sorted by name
    pub fn to_sorted_vec(&self) -> Vec<(&str, &str)> {
        // TODO: Return all contacts as (name, phone) tuples, sorted
        // Hint: In-order traversal gives sorted order
        todo!()
    }
}

// ============================================================================
// Part 3: Expression Evaluator
// ============================================================================

/// Arithmetic expression tree
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

impl Expr {
    /// Create a number expression
    pub fn num(n: i64) -> Box<Self> {
        Box::new(Expr::Number(n))
    }

    /// Create an addition expression
    pub fn add(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Add(left, right))
    }

    /// Create a subtraction expression
    pub fn subtract(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Subtract(left, right))
    }

    /// Create a multiplication expression
    pub fn multiply(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Multiply(left, right))
    }

    /// Create a division expression
    pub fn divide(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Divide(left, right))
    }

    /// Create a negation expression
    pub fn negate(expr: Box<Self>) -> Box<Self> {
        Box::new(Expr::Negate(expr))
    }

    /// Evaluate the expression
    pub fn evaluate(&self) -> Result<i64, String> {
        // TODO: Recursively evaluate the expression
        // For division, return Err if dividing by zero
        // For division, use integer division (truncating)
        todo!()
    }

    /// Convert expression to string representation
    pub fn to_string(&self) -> String {
        // TODO: Convert to readable string like "(5 + 3)"
        // Numbers should just be the number
        // Operations should be (left op right)
        // Negate should be (-expr)
        todo!()
    }
}

// ============================================================================
// Part 4: Trait Objects for Message Handlers
// ============================================================================

/// Trait for handling messages
pub trait MessageHandler {
    fn handle(&self, to: &str, content: &str) -> String;
    fn handler_type(&self) -> &str;
}

/// SMS message handler
pub struct SmsHandler {
    pub rate_per_message: f64,
}

impl MessageHandler for SmsHandler {
    fn handle(&self, to: &str, content: &str) -> String {
        format!("SMS to {}: {} (cost: ${:.2})", to, content, self.rate_per_message)
    }

    fn handler_type(&self) -> &str {
        "SMS"
    }
}

/// Email message handler
pub struct EmailHandler {
    pub smtp_server: String,
}

impl MessageHandler for EmailHandler {
    fn handle(&self, to: &str, content: &str) -> String {
        format!("Email to {} via {}: {}", to, self.smtp_server, content)
    }

    fn handler_type(&self) -> &str {
        "Email"
    }
}

/// Push notification handler
pub struct PushHandler {
    pub app_id: String,
}

impl MessageHandler for PushHandler {
    fn handle(&self, to: &str, content: &str) -> String {
        format!("Push to device {} [{}]: {}", to, self.app_id, content)
    }

    fn handler_type(&self) -> &str {
        "Push"
    }
}

/// Message dispatcher using boxed trait objects
pub struct MessageDispatcher {
    handlers: Vec<Box<dyn MessageHandler>>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        MessageDispatcher { handlers: Vec::new() }
    }

    pub fn add_handler(&mut self, handler: Box<dyn MessageHandler>) {
        // TODO: Add handler to the dispatcher
        todo!()
    }

    pub fn broadcast(&self, to: &str, content: &str) -> Vec<String> {
        // TODO: Send message through all handlers, collect results
        todo!()
    }

    pub fn handler_count(&self) -> usize {
        // TODO: Return number of handlers
        todo!()
    }

    pub fn handler_types(&self) -> Vec<&str> {
        // TODO: Return list of handler types
        todo!()
    }
}

// ============================================================================
// Main function for testing
// ============================================================================

fn main() {
    println!("=== Part 1: Message Thread ===\n");
    
    let mut thread = MessageThread::new(1, "Welcome to Textio!", "System");
    thread.add_reply(MessageThread::new(2, "Thanks!", "Alice"));
    thread.add_reply(MessageThread::new(3, "Hello!", "Bob"));
    
    if let MessageThread::Message { replies, .. } = &mut *thread {
        replies[0].add_reply(MessageThread::new(4, "You're welcome!", "System"));
    }
    
    println!("Thread: {:#?}", thread);
    println!("Message count: {}", thread.count_messages());
    
    if let Some(msg) = thread.find_by_id(3) {
        if let MessageThread::Message { content, .. } = msg {
            println!("Found message 3: {}", content);
        }
    }
    
    println!("\nMessages from System: {:?}", thread.from_sender("System"));
    
    println!("\n=== Part 2: Contact BST ===\n");
    
    let mut contacts = Contact::new("Alice", "+1-555-0101");
    contacts.insert("Bob", "+1-555-0102");
    contacts.insert("Charlie", "+1-555-0103");
    contacts.insert("Anna", "+1-555-0104");
    
    println!("Contact count: {}", contacts.count());
    println!("Find Alice: {:?}", contacts.find("Alice"));
    println!("Find David: {:?}", contacts.find("David"));
    println!("Sorted contacts: {:?}", contacts.to_sorted_vec());
    
    println!("\n=== Part 3: Expression Evaluator ===\n");
    
    // (5 + 3) * (10 - 4)
    let expr = Expr::multiply(
        Expr::add(Expr::num(5), Expr::num(3)),
        Expr::subtract(Expr::num(10), Expr::num(4))
    );
    
    println!("Expression: {}", expr.to_string());
    println!("Result: {:?}", expr.evaluate());
    
    // Negate: -(2 + 3)
    let neg = Expr::negate(Expr::add(Expr::num(2), Expr::num(3)));
    println!("Negated: {} = {:?}", neg.to_string(), neg.evaluate());
    
    // Division by zero
    let div_zero = Expr::divide(Expr::num(10), Expr::num(0));
    println!("Division by zero: {:?}", div_zero.evaluate());
    
    println!("\n=== Part 4: Trait Objects ===\n");
    
    let mut dispatcher = MessageDispatcher::new();
    dispatcher.add_handler(Box::new(SmsHandler { rate_per_message: 0.05 }));
    dispatcher.add_handler(Box::new(EmailHandler { 
        smtp_server: "smtp.textio.io".to_string() 
    }));
    dispatcher.add_handler(Box::new(PushHandler { 
        app_id: "com.textio.app".to_string() 
    }));
    
    println!("Handler count: {}", dispatcher.handler_count());
    println!("Handler types: {:?}", dispatcher.handler_types());
    
    let results = dispatcher.broadcast("+1-555-0199", "Hello from Textio!");
    for result in results {
        println!("{}", result);
    }
}

// Exercise 1: Box<T> - Complete Solution

use std::cmp::Ordering;

// ============================================================================
// Part 1: Recursive Message Thread
// ============================================================================

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
    pub fn new(id: u64, content: &str, sender: &str) -> Box<Self> {
        Box::new(MessageThread::Message {
            id,
            content: content.to_string(),
            sender: sender.to_string(),
            replies: Vec::new(),
        })
    }

    pub fn add_reply(&mut self, reply: Box<MessageThread>) {
        if let MessageThread::Message { replies, .. } = self {
            replies.push(reply);
        }
    }

    pub fn count_messages(&self) -> usize {
        match self {
            MessageThread::Message { replies, .. } => {
                1 + replies.iter().map(|r| r.count_messages()).sum::<usize>()
            }
            MessageThread::Deleted => 0,
        }
    }

    pub fn find_by_id(&self, id: u64) -> Option<&Self> {
        match self {
            MessageThread::Message { id: msg_id, replies, .. } => {
                if *msg_id == id {
                    return Some(self);
                }
                for reply in replies {
                    if let Some(found) = reply.find_by_id(id) {
                        return Some(found);
                    }
                }
                None
            }
            MessageThread::Deleted => None,
        }
    }

    pub fn from_sender(&self, sender: &str) -> Vec<&str> {
        let mut results = Vec::new();
        match self {
            MessageThread::Message { content, sender: s, replies, .. } => {
                if s == sender {
                    results.push(content.as_str());
                }
                for reply in replies {
                    results.extend(reply.from_sender(sender));
                }
            }
            MessageThread::Deleted => {}
        }
        results
    }
}

// ============================================================================
// Part 2: Binary Search Tree for Contacts
// ============================================================================

#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub left: Option<Box<Contact>>,
    pub right: Option<Box<Contact>>,
}

impl Contact {
    pub fn new(name: &str, phone: &str) -> Box<Self> {
        Box::new(Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            left: None,
            right: None,
        })
    }

    pub fn insert(&mut self, name: &str, phone: &str) {
        match name.cmp(&self.name) {
            Ordering::Less => {
                match &mut self.left {
                    Some(node) => node.insert(name, phone),
                    None => self.left = Some(Contact::new(name, phone)),
                }
            }
            Ordering::Equal => {
                self.phone = phone.to_string();
            }
            Ordering::Greater => {
                match &mut self.right {
                    Some(node) => node.insert(name, phone),
                    None => self.right = Some(Contact::new(name, phone)),
                }
            }
        }
    }

    pub fn find(&self, name: &str) -> Option<&String> {
        match name.cmp(&self.name) {
            Ordering::Less => self.left.as_ref()?.find(name),
            Ordering::Equal => Some(&self.phone),
            Ordering::Greater => self.right.as_ref()?.find(name),
        }
    }

    pub fn count(&self) -> usize {
        1 + self.left.as_ref().map_or(0, |n| n.count())
            + self.right.as_ref().map_or(0, |n| n.count())
    }

    pub fn to_sorted_vec(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        if let Some(left) = &self.left {
            result.extend(left.to_sorted_vec());
        }
        result.push((&self.name, &self.phone));
        if let Some(right) = &self.right {
            result.extend(right.to_sorted_vec());
        }
        result
    }
}

// ============================================================================
// Part 3: Expression Evaluator
// ============================================================================

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
    pub fn num(n: i64) -> Box<Self> {
        Box::new(Expr::Number(n))
    }

    pub fn add(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Add(left, right))
    }

    pub fn subtract(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Subtract(left, right))
    }

    pub fn multiply(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Multiply(left, right))
    }

    pub fn divide(left: Box<Self>, right: Box<Self>) -> Box<Self> {
        Box::new(Expr::Divide(left, right))
    }

    pub fn negate(expr: Box<Self>) -> Box<Self> {
        Box::new(Expr::Negate(expr))
    }

    pub fn evaluate(&self) -> Result<i64, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Add(l, r) => {
                let left = l.evaluate()?;
                let right = r.evaluate()?;
                Ok(left + right)
            }
            Expr::Subtract(l, r) => {
                let left = l.evaluate()?;
                let right = r.evaluate()?;
                Ok(left - right)
            }
            Expr::Multiply(l, r) => {
                let left = l.evaluate()?;
                let right = r.evaluate()?;
                Ok(left * right)
            }
            Expr::Divide(l, r) => {
                let left = l.evaluate()?;
                let right = r.evaluate()?;
                if right == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
            Expr::Negate(e) => {
                let val = e.evaluate()?;
                Ok(-val)
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Add(l, r) => format!("({} + {})", l.to_string(), r.to_string()),
            Expr::Subtract(l, r) => format!("({} - {})", l.to_string(), r.to_string()),
            Expr::Multiply(l, r) => format!("({} * {})", l.to_string(), r.to_string()),
            Expr::Divide(l, r) => format!("({} / {})", l.to_string(), r.to_string()),
            Expr::Negate(e) => format!("(-{})", e.to_string()),
        }
    }
}

// ============================================================================
// Part 4: Trait Objects for Message Handlers
// ============================================================================

pub trait MessageHandler {
    fn handle(&self, to: &str, content: &str) -> String;
    fn handler_type(&self) -> &str;
}

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

pub struct MessageDispatcher {
    handlers: Vec<Box<dyn MessageHandler>>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        MessageDispatcher { handlers: Vec::new() }
    }

    pub fn add_handler(&mut self, handler: Box<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub fn broadcast(&self, to: &str, content: &str) -> Vec<String> {
        self.handlers.iter()
            .map(|h| h.handle(to, content))
            .collect()
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    pub fn handler_types(&self) -> Vec<&str> {
        self.handlers.iter().map(|h| h.handler_type()).collect()
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
    
    let expr = Expr::multiply(
        Expr::add(Expr::num(5), Expr::num(3)),
        Expr::subtract(Expr::num(10), Expr::num(4))
    );
    
    println!("Expression: {}", expr.to_string());
    println!("Result: {:?}", expr.evaluate());
    
    let neg = Expr::negate(Expr::add(Expr::num(2), Expr::num(3)));
    println!("Negated: {} = {:?}", neg.to_string(), neg.evaluate());
    
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

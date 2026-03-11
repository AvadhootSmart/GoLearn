// Exercise 2: Rc<T> - Reference Counting and Multiple Ownership
//
// TODO: Implement the following using Rc<T> for shared ownership

use std::rc::{Rc, Weak};
use std::cell::RefCell;

// ============================================================================
// Part 1: Shared Configuration
// ============================================================================

/// Global configuration shared across Textio services
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub max_messages_per_day: u32,
    pub default_country_code: String,
    pub rate_limit: u32,
}

impl Config {
    pub fn new(api_key: &str, max_messages: u32, country_code: &str, rate_limit: u32) -> Self {
        Config {
            api_key: api_key.to_string(),
            max_messages_per_day: max_messages,
            default_country_code: country_code.to_string(),
            rate_limit,
        }
    }
}

/// SMS service that shares configuration
pub struct SmsService {
    pub name: String,
    pub config: Rc<Config>,
}

impl SmsService {
    pub fn new(name: &str, config: Rc<Config>) -> Self {
        // TODO: Create service with shared config
        todo!()
    }

    pub fn send(&self, to: &str, message: &str) -> String {
        // TODO: Return formatted send string using config
        // Format: "[{name}] Sending to {to}: {message} (api_key: {api_key})"
        todo!()
    }
}

/// Email service that shares configuration
pub struct EmailService {
    pub name: String,
    pub config: Rc<Config>,
}

impl EmailService {
    pub fn new(name: &str, config: Rc<Config>) -> Self {
        // TODO: Create service with shared config
        todo!()
    }

    pub fn send(&self, to: &str, subject: &str, body: &str) -> String {
        // TODO: Return formatted send string
        // Format: "[{name}] Email to {to}: {subject} - {body}"
        todo!()
    }
}

/// Service manager holding multiple services with shared config
pub struct ServiceManager {
    pub config: Rc<Config>,
    pub services: Vec<String>,
}

impl ServiceManager {
    pub fn new(config: Rc<Config>) -> Self {
        // TODO: Create manager with shared config
        todo!()
    }

    pub fn create_sms_service(&mut self, name: &str) -> SmsService {
        // TODO: Create and return SMS service, add name to services list
        todo!()
    }

    pub fn create_email_service(&mut self, name: &str) -> EmailService {
        // TODO: Create and return Email service, add name to services list
        todo!()
    }

    pub fn config_ref_count(&self) -> usize {
        // TODO: Return the strong reference count of config
        todo!()
    }
}

// ============================================================================
// Part 2: Message Graph
// ============================================================================

/// A message that can be related to other messages
#[derive(Debug)]
pub struct MessageNode {
    pub id: u64,
    pub content: String,
    pub sender: String,
    pub related: Vec<Rc<MessageNode>>,
}

impl MessageNode {
    pub fn new(id: u64, content: &str, sender: &str) -> Rc<Self> {
        // TODO: Create a new Rc<MessageNode>
        todo!()
    }

    pub fn add_related(&mut self, other: Rc<MessageNode>) {
        // TODO: Add a related message
        todo!()
    }

    pub fn count_references(&self) -> usize {
        // This counts how many messages reference this one
        // We can't directly count external Rc references from inside
        // So we just return the number of related messages
        self.related.len()
    }
}

/// Graph of interconnected messages
pub struct MessageGraph {
    pub messages: Vec<Rc<MessageNode>>,
}

impl MessageGraph {
    pub fn new() -> Self {
        // TODO: Create empty graph
        todo!()
    }

    pub fn add_message(&mut self, message: Rc<MessageNode>) {
        // TODO: Add message to the graph
        todo!()
    }

    pub fn find_by_id(&self, id: u64) -> Option<Rc<MessageNode>> {
        // TODO: Find message by ID, return cloned Rc
        todo!()
    }

    pub fn get_all_related(&self, id: u64) -> Vec<Rc<MessageNode>> {
        // TODO: Get all messages related to the message with given id
        // Return empty vec if not found
        todo!()
    }

    pub fn total_reference_count(&self) -> usize {
        // TODO: Sum of all strong reference counts minus self-owned references
        // Each message in self.messages has at least 1 reference (from the vec)
        // We want to count additional references from related messages
        todo!()
    }
}

// ============================================================================
// Part 3: Shared Templates
// ============================================================================

/// Message template that can be reused
#[derive(Debug)]
pub struct MessageTemplate {
    pub name: String,
    pub content: String,
    pub usage_count: RefCell<u32>,
}

impl MessageTemplate {
    pub fn new(name: &str, content: &str) -> Rc<Self> {
        // TODO: Create template with RefCell for tracking usage
        todo!()
    }

    pub fn render(&self, variables: &[(String, String)]) -> String {
        // TODO: Replace {variable} placeholders with values
        // Increment usage_count in RefCell
        // Example: "Hello {name}" with [("name", "Alice")] -> "Hello Alice"
        todo!()
    }

    pub fn get_usage(&self) -> u32 {
        // TODO: Return current usage count
        todo!()
    }
}

/// Campaign using shared templates
pub struct Campaign {
    pub name: String,
    pub template: Rc<MessageTemplate>,
}

impl Campaign {
    pub fn new(name: &str, template: Rc<MessageTemplate>) -> Self {
        // TODO: Create campaign with shared template
        todo!()
    }

    pub fn send_to(&self, recipient: &str) -> String {
        // TODO: Render template with recipient variable and return
        // Use "recipient" as the variable name
        // Format: "[{campaign_name}] {rendered_content}"
        todo!()
    }
}

/// Template manager
pub struct TemplateManager {
    pub templates: Vec<(String, Rc<MessageTemplate>)>,
}

impl TemplateManager {
    pub fn new() -> Self {
        // TODO: Create empty manager
        todo!()
    }

    pub fn register(&mut self, template: Rc<MessageTemplate>) {
        // TODO: Store template by name
        todo!()
    }

    pub fn get(&self, name: &str) -> Option<Rc<MessageTemplate>> {
        // TODO: Get template by name
        todo!()
    }

    pub fn create_campaign(&self, name: &str, template_name: &str) -> Option<Campaign> {
        // TODO: Create campaign using named template
        todo!()
    }
}

// ============================================================================
// Part 4: Workflow DAG
// ============================================================================

/// A node in a workflow DAG
#[derive(Debug)]
pub struct WorkflowNode {
    pub id: String,
    pub action: String,
    pub dependencies: Vec<Rc<WorkflowNode>>,
    pub dependents: Vec<Weak<WorkflowNode>>,
}

impl WorkflowNode {
    pub fn new(id: &str, action: &str) -> Rc<RefCell<Self>> {
        // TODO: Create node with no dependencies
        todo!()
    }

    pub fn add_dependency(&mut self, node: Rc<WorkflowNode>) {
        // TODO: Add dependency and register self as dependent on that node
        // Use Weak reference for dependents to avoid cycles
        todo!()
    }

    pub fn can_execute(&self, completed: &[String]) -> bool {
        // TODO: Check if all dependencies are in completed list
        todo!()
    }
}

/// Workflow executor
pub struct Workflow {
    pub nodes: Vec<Rc<RefCell<WorkflowNode>>>,
    pub completed: Vec<String>,
}

impl Workflow {
    pub fn new() -> Self {
        // TODO: Create empty workflow
        todo!()
    }

    pub fn add_node(&mut self, node: Rc<RefCell<WorkflowNode>>) {
        // TODO: Add node to workflow
        todo!()
    }

    pub fn execute_next(&mut self) -> Option<String> {
        // TODO: Find and "execute" the next executable node
        // Return the action string if a node was executed
        // Add node id to completed list
        // Return None if no node can execute
        todo!()
    }

    pub fn execute_all(&mut self) -> Vec<String> {
        // TODO: Execute all nodes in dependency order
        // Return list of executed actions
        todo!()
    }

    pub fn is_complete(&self) -> bool {
        // TODO: Check if all nodes are completed
        todo!()
    }
}

// ============================================================================
// Main function for testing
// ============================================================================

fn main() {
    println!("=== Part 1: Shared Configuration ===\n");
    
    let config = Rc::new(Config::new("textio_key_123", 1000, "+1", 100));
    println!("Initial ref count: {}", Rc::strong_count(&config));
    
    let mut manager = ServiceManager::new(Rc::clone(&config));
    println!("After ServiceManager: {}", Rc::strong_count(&config));
    
    let sms = manager.create_sms_service("SMS-Gateway");
    println!("After SMS service: {}", manager.config_ref_count());
    
    let email = manager.create_email_service("Email-Gateway");
    println!("After Email service: {}", manager.config_ref_count());
    
    println!("\n{}", sms.send("+15550100", "Hello from Textio!"));
    println!("{}", email.send("user@example.com", "Welcome", "Thanks for joining!"));
    
    println!("\n=== Part 2: Message Graph ===\n");
    
    let msg1 = MessageNode::new(1, "Hello everyone!", "Alice");
    let msg2 = MessageNode::new(2, "Hi Alice!", "Bob");
    let msg3 = MessageNode::new(3, "Hey all!", "Charlie");
    
    // Make msg2 and msg3 related to msg1
    let mut graph = MessageGraph::new();
    graph.add_message(Rc::clone(&msg1));
    
    // Create references
    if let Some(mut m) = graph.find_by_id(1) {
        // We need interior mutability for this pattern
        // For this exercise, we'll use a different approach
    }
    
    let mut msg2_mut = (*msg2).clone();
    msg2_mut.related.push(Rc::clone(&msg1));
    
    println!("Message 1 ref count: {}", Rc::strong_count(&msg1));
    
    println!("\n=== Part 3: Shared Templates ===\n");
    
    let mut template_mgr = TemplateManager::new();
    
    let welcome = MessageTemplate::new("welcome", "Hello {name}, welcome to Textio!");
    let promo = MessageTemplate::new("promo", "Hi {name}! Use code {code} for 20% off!");
    
    template_mgr.register(Rc::clone(&welcome));
    template_mgr.register(Rc::clone(&promo));
    
    println!("Welcome template ref count: {}", Rc::strong_count(&welcome));
    
    let campaign1 = template_mgr.create_campaign("Spring Sale", "promo").unwrap();
    let campaign2 = template_mgr.create_campaign("New User", "welcome").unwrap();
    
    println!("Welcome template ref count after campaigns: {}", Rc::strong_count(&welcome));
    
    println!("\n{}", campaign1.send_to("Alice"));
    println!("{}", campaign2.send_to("Bob"));
    
    println!("Template usage - welcome: {}, promo: {}", welcome.get_usage(), promo.get_usage());
    
    println!("\n=== Part 4: Workflow DAG ===\n");
    
    let start = WorkflowNode::new("start", "Initialize connection");
    let auth = WorkflowNode::new("auth", "Authenticate user");
    let fetch = WorkflowNode::new("fetch", "Fetch messages");
    let send = WorkflowNode::new("send", "Send response");
    
    // auth depends on start
    auth.borrow_mut().add_dependency(Rc::clone(&start));
    // fetch depends on auth
    fetch.borrow_mut().add_dependency(Rc::clone(&auth));
    // send depends on fetch
    send.borrow_mut().add_dependency(Rc::clone(&fetch));
    
    let mut workflow = Workflow::new();
    workflow.add_node(start);
    workflow.add_node(auth);
    workflow.add_node(fetch);
    workflow.add_node(send);
    
    println!("Executing workflow:");
    let executed = workflow.execute_all();
    for (i, action) in executed.iter().enumerate() {
        println!("  {}. {}", i + 1, action);
    }
    
    println!("\nWorkflow complete: {}", workflow.is_complete());
}

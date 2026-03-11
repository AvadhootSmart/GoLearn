// Exercise 2: Rc<T> - Complete Solution

use std::rc::{Rc, Weak};
use std::cell::RefCell;

// ============================================================================
// Part 1: Shared Configuration
// ============================================================================

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

pub struct SmsService {
    pub name: String,
    pub config: Rc<Config>,
}

impl SmsService {
    pub fn new(name: &str, config: Rc<Config>) -> Self {
        SmsService {
            name: name.to_string(),
            config,
        }
    }

    pub fn send(&self, to: &str, message: &str) -> String {
        format!("[{}] Sending to {}: {} (api_key: {})", 
            self.name, to, message, self.config.api_key)
    }
}

pub struct EmailService {
    pub name: String,
    pub config: Rc<Config>,
}

impl EmailService {
    pub fn new(name: &str, config: Rc<Config>) -> Self {
        EmailService {
            name: name.to_string(),
            config,
        }
    }

    pub fn send(&self, to: &str, subject: &str, body: &str) -> String {
        format!("[{}] Email to {}: {} - {}", self.name, to, subject, body)
    }
}

pub struct ServiceManager {
    pub config: Rc<Config>,
    pub services: Vec<String>,
}

impl ServiceManager {
    pub fn new(config: Rc<Config>) -> Self {
        ServiceManager {
            config,
            services: Vec::new(),
        }
    }

    pub fn create_sms_service(&mut self, name: &str) -> SmsService {
        self.services.push(name.to_string());
        SmsService::new(name, Rc::clone(&self.config))
    }

    pub fn create_email_service(&mut self, name: &str) -> EmailService {
        self.services.push(name.to_string());
        EmailService::new(name, Rc::clone(&self.config))
    }

    pub fn config_ref_count(&self) -> usize {
        Rc::strong_count(&self.config)
    }
}

// ============================================================================
// Part 2: Message Graph
// ============================================================================

#[derive(Debug)]
pub struct MessageNode {
    pub id: u64,
    pub content: String,
    pub sender: String,
    pub related: Vec<Rc<MessageNode>>,
}

impl MessageNode {
    pub fn new(id: u64, content: &str, sender: &str) -> Rc<Self> {
        Rc::new(MessageNode {
            id,
            content: content.to_string(),
            sender: sender.to_string(),
            related: Vec::new(),
        })
    }

    pub fn add_related(&mut self, other: Rc<MessageNode>) {
        self.related.push(other);
    }

    pub fn count_references(&self) -> usize {
        self.related.len()
    }
}

pub struct MessageGraph {
    pub messages: Vec<Rc<MessageNode>>,
}

impl MessageGraph {
    pub fn new() -> Self {
        MessageGraph { messages: Vec::new() }
    }

    pub fn add_message(&mut self, message: Rc<MessageNode>) {
        self.messages.push(message);
    }

    pub fn find_by_id(&self, id: u64) -> Option<Rc<MessageNode>> {
        self.messages.iter()
            .find(|m| m.id == id)
            .map(|m| Rc::clone(m))
    }

    pub fn get_all_related(&self, id: u64) -> Vec<Rc<MessageNode>> {
        self.find_by_id(id)
            .map(|m| m.related.clone())
            .unwrap_or_default()
    }

    pub fn total_reference_count(&self) -> usize {
        self.messages.iter()
            .map(|m| Rc::strong_count(m) - 1)
            .sum()
    }
}

// ============================================================================
// Part 3: Shared Templates
// ============================================================================

#[derive(Debug)]
pub struct MessageTemplate {
    pub name: String,
    pub content: String,
    pub usage_count: RefCell<u32>,
}

impl MessageTemplate {
    pub fn new(name: &str, content: &str) -> Rc<Self> {
        Rc::new(MessageTemplate {
            name: name.to_string(),
            content: content.to_string(),
            usage_count: RefCell::new(0),
        })
    }

    pub fn render(&self, variables: &[(String, String)]) -> String {
        *self.usage_count.borrow_mut() += 1;
        let mut result = self.content.clone();
        for (key, value) in variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    pub fn get_usage(&self) -> u32 {
        *self.usage_count.borrow()
    }
}

pub struct Campaign {
    pub name: String,
    pub template: Rc<MessageTemplate>,
}

impl Campaign {
    pub fn new(name: &str, template: Rc<MessageTemplate>) -> Self {
        Campaign {
            name: name.to_string(),
            template,
        }
    }

    pub fn send_to(&self, recipient: &str) -> String {
        let rendered = self.template.render(&[
            ("name".to_string(), recipient.to_string())
        ]);
        format!("[{}] {}", self.name, rendered)
    }
}

pub struct TemplateManager {
    pub templates: Vec<(String, Rc<MessageTemplate>)>,
}

impl TemplateManager {
    pub fn new() -> Self {
        TemplateManager { templates: Vec::new() }
    }

    pub fn register(&mut self, template: Rc<MessageTemplate>) {
        let name = template.name.clone();
        self.templates.push((name, template));
    }

    pub fn get(&self, name: &str) -> Option<Rc<MessageTemplate>> {
        self.templates.iter()
            .find(|(n, _)| n == name)
            .map(|(_, t)| Rc::clone(t))
    }

    pub fn create_campaign(&self, name: &str, template_name: &str) -> Option<Campaign> {
        self.get(template_name)
            .map(|t| Campaign::new(name, t))
    }
}

// ============================================================================
// Part 4: Workflow DAG
// ============================================================================

#[derive(Debug)]
pub struct WorkflowNode {
    pub id: String,
    pub action: String,
    pub dependencies: Vec<Rc<WorkflowNode>>,
    pub dependents: Vec<Weak<WorkflowNode>>,
}

impl WorkflowNode {
    pub fn new(id: &str, action: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WorkflowNode {
            id: id.to_string(),
            action: action.to_string(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
        }))
    }

    pub fn add_dependency(&mut self, node: Rc<WorkflowNode>) {
        node.borrow_mut().dependents.push(Rc::downgrade(&node));
        self.dependencies.push(node);
    }

    pub fn can_execute(&self, completed: &[String]) -> bool {
        self.dependencies.iter().all(|dep| {
            completed.contains(&dep.borrow().id)
        })
    }
}

pub struct Workflow {
    pub nodes: Vec<Rc<RefCell<WorkflowNode>>>,
    pub completed: Vec<String>,
}

impl Workflow {
    pub fn new() -> Self {
        Workflow {
            nodes: Vec::new(),
            completed: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Rc<RefCell<WorkflowNode>>) {
        self.nodes.push(node);
    }

    pub fn execute_next(&mut self) -> Option<String> {
        for node in &self.nodes {
            let node_ref = node.borrow();
            if !self.completed.contains(&node_ref.id) && node_ref.can_execute(&self.completed) {
                let action = node_ref.action.clone();
                let id = node_ref.id.clone();
                drop(node_ref);
                self.completed.push(id);
                return Some(action);
            }
        }
        None
    }

    pub fn execute_all(&mut self) -> Vec<String> {
        let mut executed = Vec::new();
        while let Some(action) = self.execute_next() {
            executed.push(action);
        }
        executed
    }

    pub fn is_complete(&self) -> bool {
        self.nodes.len() == self.completed.len()
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
    
    let mut graph = MessageGraph::new();
    graph.add_message(Rc::clone(&msg1));
    graph.add_message(Rc::clone(&msg2));
    graph.add_message(Rc::clone(&msg3));
    
    println!("Message 1 ref count: {}", Rc::strong_count(&msg1));
    println!("Total messages in graph: {}", graph.messages.len());
    
    if let Some(found) = graph.find_by_id(2) {
        println!("Found message 2: {} - {}", found.sender, found.content);
    }
    
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
    
    auth.borrow_mut().add_dependency(Rc::clone(&start));
    fetch.borrow_mut().add_dependency(Rc::clone(&auth));
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

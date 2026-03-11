// Exercise 4: RefCell<T> - Complete Solution

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque};

// ============================================================================
// Part 1: Message Handler with Mutable Stats
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct HandlerStats {
    pub messages_processed: u32,
    pub messages_failed: u32,
    pub total_bytes: u64,
}

pub struct MessageHandler {
    pub name: String,
    pub stats: RefCell<HandlerStats>,
}

impl MessageHandler {
    pub fn new(name: &str) -> Self {
        MessageHandler {
            name: name.to_string(),
            stats: RefCell::new(HandlerStats::default()),
        }
    }

    pub fn process(&self, message: &str) -> Result<String, String> {
        if message.is_empty() {
            self.stats.borrow_mut().messages_failed += 1;
            return Err("Empty message".to_string());
        }

        self.stats.borrow_mut().messages_processed += 1;
        self.stats.borrow_mut().total_bytes += message.len() as u64;
        Ok(format!("Processed: {}", message))
    }

    pub fn get_stats(&self) -> HandlerStats {
        self.stats.borrow().clone()
    }

    pub fn reset_stats(&self) {
        *self.stats.borrow_mut() = HandlerStats::default();
    }
}

pub trait Processor {
    fn process(&self, input: &str) -> Result<String, String>;
    fn get_processed_count(&self) -> u32;
}

impl Processor for MessageHandler {
    fn process(&self, input: &str) -> Result<String, String> {
        self.process(input)
    }

    fn get_processed_count(&self) -> u32 {
        self.stats.borrow().messages_processed
    }
}

// ============================================================================
// Part 2: Lazy-Loaded Configuration Cache
// ============================================================================

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub endpoint: String,
    pub timeout_ms: u64,
}

pub struct ConfigCache {
    pub config: RefCell<Option<Config>>,
    pub load_count: RefCell<u32>,
}

impl ConfigCache {
    pub fn new() -> Self {
        ConfigCache {
            config: RefCell::new(None),
            load_count: RefCell::new(0),
        }
    }

    pub fn get(&self) -> Config {
        self.get_or_load(|| Config {
            api_key: "default_key".to_string(),
            endpoint: "https://api.textio.io".to_string(),
            timeout_ms: 5000,
        })
    }

    pub fn get_or_load<F>(&self, loader: F) -> Config
    where
        F: FnOnce() -> Config,
    {
        if self.config.borrow().is_none() {
            *self.load_count.borrow_mut() += 1;
            *self.config.borrow_mut() = Some(loader());
        }
        self.config.borrow().clone().unwrap()
    }

    pub fn is_loaded(&self) -> bool {
        self.config.borrow().is_some()
    }

    pub fn invalidate(&self) {
        *self.config.borrow_mut() = None;
    }

    pub fn get_load_count(&self) -> u32 {
        *self.load_count.borrow()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut Config),
    {
        if self.config.borrow().is_none() {
            self.get();
        }
        f(&mut self.config.borrow_mut().as_mut().unwrap());
    }
}

// ============================================================================
// Part 3: Observable Event System
// ============================================================================

#[derive(Debug, Clone)]
pub enum MessageEvent {
    Sent { recipient: String, content: String },
    Delivered { recipient: String },
    Failed { recipient: String, error: String },
}

pub trait EventListener {
    fn on_event(&self, event: &MessageEvent);
}

pub struct LoggingListener {
    pub events: RefCell<Vec<String>>,
}

impl LoggingListener {
    pub fn new() -> Self {
        LoggingListener {
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn get_events(&self) -> Vec<String> {
        self.events.borrow().clone()
    }

    pub fn clear(&self) {
        self.events.borrow_mut().clear();
    }
}

impl EventListener for LoggingListener {
    fn on_event(&self, event: &MessageEvent) {
        let log = match event {
            MessageEvent::Sent { recipient, content } => {
                format!("Sent to {}: {}", recipient, content)
            }
            MessageEvent::Delivered { recipient } => {
                format!("Delivered to {}", recipient)
            }
            MessageEvent::Failed { recipient, error } => {
                format!("Failed for {}: {}", recipient, error)
            }
        };
        self.events.borrow_mut().push(log);
    }
}

pub struct StatsListener {
    pub sent_count: RefCell<u32>,
    pub delivered_count: RefCell<u32>,
    pub failed_count: RefCell<u32>,
}

impl StatsListener {
    pub fn new() -> Self {
        StatsListener {
            sent_count: RefCell::new(0),
            delivered_count: RefCell::new(0),
            failed_count: RefCell::new(0),
        }
    }

    pub fn get_summary(&self) -> (u32, u32, u32) {
        (
            *self.sent_count.borrow(),
            *self.delivered_count.borrow(),
            *self.failed_count.borrow(),
        )
    }
}

impl EventListener for StatsListener {
    fn on_event(&self, event: &MessageEvent) {
        match event {
            MessageEvent::Sent { .. } => *self.sent_count.borrow_mut() += 1,
            MessageEvent::Delivered { .. } => *self.delivered_count.borrow_mut() += 1,
            MessageEvent::Failed { .. } => *self.failed_count.borrow_mut() += 1,
        }
    }
}

pub struct EventEmitter {
    pub listeners: RefCell<Vec<Box<dyn EventListener>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        EventEmitter {
            listeners: RefCell::new(Vec::new()),
        }
    }

    pub fn subscribe<L: EventListener + 'static>(&self, listener: L) {
        self.listeners.borrow_mut().push(Box::new(listener));
    }

    pub fn emit(&self, event: &MessageEvent) {
        for listener in self.listeners.borrow().iter() {
            listener.on_event(event);
        }
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.borrow().len()
    }
}

// ============================================================================
// Part 4: Graph with Mutable Edges
// ============================================================================

pub struct GraphNode {
    pub id: u32,
    pub label: String,
    pub edges: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    pub fn new(id: u32, label: &str) -> Rc<Self> {
        Rc::new(GraphNode {
            id,
            label: label.to_string(),
            edges: RefCell::new(Vec::new()),
        })
    }

    pub fn add_edge(&self, other: Rc<GraphNode>) {
        self.edges.borrow_mut().push(other);
    }

    pub fn add_bidirectional_edge(&self, other: Rc<GraphNode>) {
        self.add_edge(Rc::clone(&other));
        other.add_edge(Rc::new(GraphNode {
            id: self.id,
            label: self.label.clone(),
            edges: RefCell::new(Vec::new()),
        }));
    }

    pub fn edge_count(&self) -> usize {
        self.edges.borrow().len()
    }

    pub fn neighbors(&self) -> Vec<Rc<GraphNode>> {
        self.edges.borrow().clone()
    }

    pub fn has_edge_to(&self, node_id: u32) -> bool {
        self.edges.borrow().iter().any(|n| n.id == node_id)
    }

    pub fn remove_edge_to(&self, node_id: u32) -> bool {
        let mut edges = self.edges.borrow_mut();
        let len_before = edges.len();
        edges.retain(|n| n.id != node_id);
        edges.len() < len_before
    }
}

pub struct Graph {
    pub nodes: RefCell<HashMap<u32, Rc<GraphNode>>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_node(&self, id: u32, label: &str) -> Rc<GraphNode> {
        let mut nodes = self.nodes.borrow_mut();
        if let Some(existing) = nodes.get(&id) {
            return Rc::clone(existing);
        }
        let node = GraphNode::new(id, label);
        nodes.insert(id, Rc::clone(&node));
        node
    }

    pub fn get_node(&self, id: u32) -> Option<Rc<GraphNode>> {
        self.nodes.borrow().get(&id).map(Rc::clone)
    }

    pub fn add_edge(&self, from_id: u32, to_id: u32) -> bool {
        let nodes = self.nodes.borrow();
        let from = match nodes.get(&from_id) {
            Some(n) => Rc::clone(n),
            None => return false,
        };
        let to = match nodes.get(&to_id) {
            Some(n) => Rc::clone(n),
            None => return false,
        };
        drop(nodes);
        from.add_edge(to);
        true
    }

    pub fn node_count(&self) -> usize {
        self.nodes.borrow().len()
    }

    pub fn total_edges(&self) -> usize {
        self.nodes.borrow().values().map(|n| n.edge_count()).sum()
    }

    pub fn find_path(&self, from_id: u32, to_id: u32) -> Option<Vec<u32>> {
        let nodes = self.nodes.borrow();
        
        if !nodes.contains_key(&from_id) || !nodes.contains_key(&to_id) {
            return None;
        }
        
        if from_id == to_id {
            return Some(vec![from_id]);
        }

        let mut visited = std::collections::HashSet::new();
        let mut queue: VecDeque<(u32, Vec<u32>)> = VecDeque::new();
        
        visited.insert(from_id);
        queue.push_back((from_id, vec![from_id]));

        while let Some((current, path)) = queue.pop_front() {
            if let Some(node) = nodes.get(&current) {
                for neighbor in node.edges.borrow().iter() {
                    if neighbor.id == to_id {
                        let mut result = path.clone();
                        result.push(neighbor.id);
                        return Some(result);
                    }
                    if !visited.contains(&neighbor.id) {
                        visited.insert(neighbor.id);
                        let mut new_path = path.clone();
                        new_path.push(neighbor.id);
                        queue.push_back((neighbor.id, new_path));
                    }
                }
            }
        }

        None
    }
}

// ============================================================================
// Main function for testing
// ============================================================================

fn main() {
    println!("=== Part 1: Message Handler with Stats ===\n");
    
    let handler = MessageHandler::new("SMS-Handler");
    
    handler.process("Hello World").unwrap();
    handler.process("Test message").unwrap();
    handler.process("").unwrap_err();
    handler.process("Another message").unwrap();
    
    let stats = handler.get_stats();
    println!("Stats: processed={}, failed={}, bytes={}", 
        stats.messages_processed, stats.messages_failed, stats.total_bytes);
    
    let processor: &dyn Processor = &handler;
    processor.process("Trait message").unwrap();
    println!("Processed through trait: {}", processor.get_processed_count());
    
    println!("\n=== Part 2: Lazy Configuration Cache ===\n");
    
    let cache = ConfigCache::new();
    println!("Loaded: {}", cache.is_loaded());
    
    let config1 = cache.get();
    println!("Config: api_key={}, endpoint={}", config1.api_key, config1.endpoint);
    println!("Loaded: {}, load_count: {}", cache.is_loaded(), cache.get_load_count());
    
    let _config2 = cache.get();
    println!("Load count after second get: {}", cache.get_load_count());
    
    cache.invalidate();
    println!("After invalidate, loaded: {}", cache.is_loaded());
    
    cache.get_or_load(|| Config {
        api_key: "custom_key".to_string(),
        endpoint: "https://custom.api.io".to_string(),
        timeout_ms: 3000,
    });
    println!("Load count: {}", cache.get_load_count());
    
    cache.update(|c| c.timeout_ms = 10000);
    
    println!("\n=== Part 3: Event System ===\n");
    
    let emitter = EventEmitter::new();
    
    emitter.subscribe(LoggingListener::new());
    emitter.subscribe(StatsListener::new());
    
    println!("Listeners: {}", emitter.listener_count());
    
    emitter.emit(&MessageEvent::Sent {
        recipient: "+1234567890".to_string(),
        content: "Hello!".to_string(),
    });
    
    emitter.emit(&MessageEvent::Delivered {
        recipient: "+1234567890".to_string(),
    });
    
    emitter.emit(&MessageEvent::Failed {
        recipient: "+9999999999".to_string(),
        error: "Invalid number".to_string(),
    });
    
    println!("\n=== Part 4: Graph with Mutable Edges ===\n");
    
    let graph = Graph::new();
    
    let a = graph.add_node(1, "A");
    let b = graph.add_node(2, "B");
    let c = graph.add_node(3, "C");
    let d = graph.add_node(4, "D");
    
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);
    graph.add_edge(1, 3);
    
    println!("Nodes: {}, Total edges: {}", graph.node_count(), graph.total_edges());
    println!("A edges: {}", a.edge_count());
    println!("A has edge to B: {}", a.has_edge_to(2));
    println!("A has edge to D: {}", a.has_edge_to(4));
    
    if let Some(path) = graph.find_path(1, 4) {
        println!("Path from A to D: {:?}", path);
    }
    
    a.remove_edge_to(3);
    println!("After removing A->C, A edges: {}", a.edge_count());
}

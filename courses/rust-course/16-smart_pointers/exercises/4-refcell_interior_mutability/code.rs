// Exercise 4: RefCell<T> - Interior Mutability and Runtime Borrowing
//
// TODO: Implement patterns using RefCell for interior mutability

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// ============================================================================
// Part 1: Message Handler with Mutable Stats
// ============================================================================

/// Statistics for a message handler
#[derive(Debug, Clone, Default)]
pub struct HandlerStats {
    pub messages_processed: u32,
    pub messages_failed: u32,
    pub total_bytes: u64,
}

/// Message handler that tracks statistics through interior mutability
pub struct MessageHandler {
    pub name: String,
    pub stats: RefCell<HandlerStats>,
}

impl MessageHandler {
    pub fn new(name: &str) -> Self {
        // TODO: Create handler with empty stats
        todo!()
    }

    pub fn process(&self, message: &str) -> Result<String, String> {
        // TODO: Process message and update stats
        // If message is not empty: increment messages_processed, add length to total_bytes
        // Return Ok with format "Processed: {message}"
        // If message is empty: increment messages_failed, return Err("Empty message")
        todo!()
    }

    pub fn get_stats(&self) -> HandlerStats {
        // TODO: Return cloned stats
        todo!()
    }

    pub fn reset_stats(&self) {
        // TODO: Reset stats to default
        todo!()
    }
}

/// Trait that requires &self but needs internal mutation
pub trait Processor {
    fn process(&self, input: &str) -> Result<String, String>;
    fn get_processed_count(&self) -> u32;
}

impl Processor for MessageHandler {
    fn process(&self, input: &str) -> Result<String, String> {
        // TODO: Call self.process and return result
        todo!()
    }

    fn get_processed_count(&self) -> u32 {
        // TODO: Return messages_processed from stats
        todo!()
    }
}

// ============================================================================
// Part 2: Lazy-Loaded Configuration Cache
// ============================================================================

/// Configuration that is loaded on first access
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub endpoint: String,
    pub timeout_ms: u64,
}

/// Cache for lazy-loaded configuration
pub struct ConfigCache {
    pub config: RefCell<Option<Config>>,
    pub load_count: RefCell<u32>,
}

impl ConfigCache {
    pub fn new() -> Self {
        // TODO: Create empty cache
        todo!()
    }

    pub fn get(&self) -> Config {
        // TODO: Return config, loading if necessary
        // If not loaded: increment load_count, create default config
        // Default: api_key="default_key", endpoint="https://api.textio.io", timeout_ms=5000
        todo!()
    }

    pub fn get_or_load<F>(&self, loader: F) -> Config
    where
        F: FnOnce() -> Config,
    {
        // TODO: Return config, using loader if not yet loaded
        // Increment load_count when loading
        todo!()
    }

    pub fn is_loaded(&self) -> bool {
        // TODO: Check if config has been loaded
        todo!()
    }

    pub fn invalidate(&self) {
        // TODO: Clear the cache
        todo!()
    }

    pub fn get_load_count(&self) -> u32 {
        // TODO: Return number of times config was loaded
        todo!()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut Config),
    {
        // TODO: Update config using provided function
        // If not loaded, load first
        todo!()
    }
}

// ============================================================================
// Part 3: Observable Event System
// ============================================================================

/// Event type for the messaging system
#[derive(Debug, Clone)]
pub enum MessageEvent {
    Sent { recipient: String, content: String },
    Delivered { recipient: String },
    Failed { recipient: String, error: String },
}

/// Event listener trait
pub trait EventListener {
    fn on_event(&self, event: &MessageEvent);
}

/// Logging listener
pub struct LoggingListener {
    pub events: RefCell<Vec<String>>,
}

impl LoggingListener {
    pub fn new() -> Self {
        // TODO: Create listener with empty events
        todo!()
    }

    pub fn get_events(&self) -> Vec<String> {
        // TODO: Return cloned events
        todo!()
    }

    pub fn clear(&self) {
        // TODO: Clear events
        todo!()
    }
}

impl EventListener for LoggingListener {
    fn on_event(&self, event: &MessageEvent) {
        // TODO: Format event and add to events
        // Sent: "Sent to {recipient}: {content}"
        // Delivered: "Delivered to {recipient}"
        // Failed: "Failed for {recipient}: {error}"
        todo!()
    }
}

/// Statistics listener
pub struct StatsListener {
    pub sent_count: RefCell<u32>,
    pub delivered_count: RefCell<u32>,
    pub failed_count: RefCell<u32>,
}

impl StatsListener {
    pub fn new() -> Self {
        // TODO: Create listener with zero counts
        todo!()
    }

    pub fn get_summary(&self) -> (u32, u32, u32) {
        // TODO: Return (sent, delivered, failed)
        todo!()
    }
}

impl EventListener for StatsListener {
    fn on_event(&self, event: &MessageEvent) {
        // TODO: Increment appropriate counter based on event type
        todo!()
    }
}

/// Event emitter that notifies listeners
pub struct EventEmitter {
    pub listeners: RefCell<Vec<Box<dyn EventListener>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        // TODO: Create emitter with no listeners
        todo!()
    }

    pub fn subscribe<L: EventListener + 'static>(&self, listener: L) {
        // TODO: Add boxed listener to listeners
        todo!()
    }

    pub fn emit(&self, event: &MessageEvent) {
        // TODO: Notify all listeners of event
        todo!()
    }

    pub fn listener_count(&self) -> usize {
        // TODO: Return number of listeners
        todo!()
    }
}

// ============================================================================
// Part 4: Graph with Mutable Edges
// ============================================================================

/// Node in a graph with mutable connections
pub struct GraphNode {
    pub id: u32,
    pub label: String,
    pub edges: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    pub fn new(id: u32, label: &str) -> Rc<Self> {
        // TODO: Create node with no edges
        todo!()
    }

    pub fn add_edge(&self, other: Rc<GraphNode>) {
        // TODO: Add directed edge to other node
        todo!()
    }

    pub fn add_bidirectional_edge(&self, other: Rc<GraphNode>) {
        // TODO: Add edges in both directions
        todo!()
    }

    pub fn edge_count(&self) -> usize {
        // TODO: Return number of edges
        todo!()
    }

    pub fn neighbors(&self) -> Vec<Rc<GraphNode>> {
        // TODO: Return clone of edges vector
        todo!()
    }

    pub fn has_edge_to(&self, node_id: u32) -> bool {
        // TODO: Check if there's an edge to node with given id
        todo!()
    }

    pub fn remove_edge_to(&self, node_id: u32) -> bool {
        // TODO: Remove edge to node with given id, return true if removed
        todo!()
    }
}

/// Graph structure
pub struct Graph {
    pub nodes: RefCell<HashMap<u32, Rc<GraphNode>>>,
}

impl Graph {
    pub fn new() -> Self {
        // TODO: Create empty graph
        todo!()
    }

    pub fn add_node(&self, id: u32, label: &str) -> Rc<GraphNode> {
        // TODO: Add node to graph, return it
        // If node already exists, return existing
        todo!()
    }

    pub fn get_node(&self, id: u32) -> Option<Rc<GraphNode>> {
        // TODO: Get node by id
        todo!()
    }

    pub fn add_edge(&self, from_id: u32, to_id: u32) -> bool {
        // TODO: Add edge from from_id to to_id
        // Return true if both nodes exist and edge was added
        todo!()
    }

    pub fn node_count(&self) -> usize {
        // TODO: Return number of nodes
        todo!()
    }

    pub fn total_edges(&self) -> usize {
        // TODO: Return total number of edges in graph
        todo!()
    }

    pub fn find_path(&self, from_id: u32, to_id: u32) -> Option<Vec<u32>> {
        // TODO: Find path using BFS, return node ids
        // Return None if no path exists
        todo!()
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
    
    // Using through trait
    let processor: &dyn Processor = &handler;
    processor.process("Trait message").unwrap();
    println!("Processed through trait: {}", processor.get_processed_count());
    
    println!("\n=== Part 2: Lazy Configuration Cache ===\n");
    
    let cache = ConfigCache::new();
    println!("Loaded: {}", cache.is_loaded());
    
    let config1 = cache.get();
    println!("Config: api_key={}, endpoint={}", config1.api_key, config1.endpoint);
    println!("Loaded: {}, load_count: {}", cache.is_loaded(), cache.get_load_count());
    
    let config2 = cache.get();
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
    
    let logger = LoggingListener::new();
    let stats_listener = StatsListener::new();
    
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

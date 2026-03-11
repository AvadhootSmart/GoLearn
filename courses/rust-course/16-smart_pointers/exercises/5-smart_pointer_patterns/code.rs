// Exercise 5: Smart Pointer Patterns - Combining Types
//
// TODO: Implement advanced patterns combining smart pointers

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

// ============================================================================
// Part 1: Doubly-Linked Message Thread
// ============================================================================

/// A message in a thread with bidirectional links
pub struct ThreadMessage {
    pub id: u64,
    pub content: String,
    pub sender: String,
    pub replies: RefCell<Vec<Rc<ThreadMessage>>>,
    pub parent: RefCell<Weak<ThreadMessage>>,
}

impl ThreadMessage {
    pub fn new(id: u64, content: &str, sender: &str) -> Rc<Self> {
        // TODO: Create a new message with no parent or replies
        todo!()
    }

    pub fn add_reply(self: &Rc<Self>, id: u64, content: &str, sender: &str) -> Rc<ThreadMessage> {
        // TODO: Create a reply, set its parent to self (as Weak)
        // Add reply to self's replies, return the reply
        todo!()
    }

    pub fn get_parent(&self) -> Option<Rc<ThreadMessage>> {
        // TODO: Upgrade parent Weak to Rc, return None if no parent
        todo!()
    }

    pub fn get_replies(&self) -> Vec<Rc<ThreadMessage>> {
        // TODO: Return clone of replies vector
        todo!()
    }

    pub fn reply_count(&self) -> usize {
        // TODO: Return number of replies
        todo!()
    }

    pub fn depth(&self) -> usize {
        // TODO: Return depth in thread (root = 0, immediate reply = 1, etc.)
        todo!()
    }

    pub fn flatten(&self) -> Vec<Rc<ThreadMessage>> {
        // TODO: Return all messages in thread including self and all descendants
        // Order: self first, then recursively flatten replies
        todo!()
    }

    pub fn find_by_id(&self, id: u64) -> Option<Rc<ThreadMessage>> {
        // TODO: Search self and all descendants for message with given id
        todo!()
    }

    pub fn ancestors(&self) -> Vec<Rc<ThreadMessage>> {
        // TODO: Return all ancestors from immediate parent to root
        todo!()
    }
}

/// Thread manager
pub struct ThreadManager {
    pub roots: RefCell<Vec<Rc<ThreadMessage>>>,
    pub next_id: RefCell<u64>,
}

impl ThreadManager {
    pub fn new() -> Self {
        // TODO: Create manager with no roots and next_id = 1
        todo!()
    }

    pub fn create_thread(&self, content: &str, sender: &str) -> Rc<ThreadMessage> {
        // TODO: Create root message, add to roots, return it
        // Use next_id and increment it
        todo!()
    }

    pub fn add_reply(&self, parent_id: u64, content: &str, sender: &str) -> Option<Rc<ThreadMessage>> {
        // TODO: Find parent by id and add reply
        // Return the reply or None if parent not found
        todo!()
    }

    pub fn get_thread_count(&self) -> usize {
        // TODO: Return number of root threads
        todo!()
    }

    pub fn get_total_messages(&self) -> usize {
        // TODO: Count all messages in all threads
        todo!()
    }
}

// ============================================================================
// Part 2: Observable State Manager
// ============================================================================

/// Observer trait for state changes
pub trait StateObserver {
    fn on_change(&self, key: &str, old_value: Option<&str>, new_value: &str);
}

/// Logging observer
pub struct LoggingObserver {
    pub logs: RefCell<Vec<String>>,
}

impl LoggingObserver {
    pub fn new() -> Self {
        // TODO: Create observer with empty logs
        todo!()
    }

    pub fn get_logs(&self) -> Vec<String> {
        // TODO: Return cloned logs
        todo!()
    }
}

impl StateObserver for LoggingObserver {
    fn on_change(&self, key: &str, old_value: Option<&str>, new_value: &str) {
        // TODO: Format and log the change
        // Format: "{key}: {old:?} -> {new}"
        todo!()
    }
}

/// State manager with observable changes
pub struct StateManager {
    pub state: RefCell<HashMap<String, String>>,
    pub observers: RefCell<Vec<Rc<dyn StateObserver>>>,
}

impl StateManager {
    pub fn new() -> Self {
        // TODO: Create manager with empty state and observers
        todo!()
    }

    pub fn subscribe(&self, observer: Rc<dyn StateObserver>) {
        // TODO: Add observer to observers
        todo!()
    }

    pub fn set(&self, key: &str, value: &str) {
        // TODO: Set key to value, notify all observers
        // old_value is previous value or None
        todo!()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        // TODO: Get value for key
        todo!()
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        // TODO: Remove key, notify observers with old value and empty new value
        // Format new value as "<removed>"
        todo!()
    }

    pub fn keys(&self) -> Vec<String> {
        // TODO: Return all keys
        todo!()
    }

    pub fn observer_count(&self) -> usize {
        // TODO: Return number of observers
        todo!()
    }
}

// ============================================================================
// Part 3: Message Pool
// ============================================================================

/// Reusable message buffer
pub struct MessageBuffer {
    pub id: u64,
    pub content: RefCell<String>,
    pub recipient: RefCell<String>,
    pub in_use: RefCell<bool>,
}

impl MessageBuffer {
    pub fn new(id: u64) -> Rc<Self> {
        // TODO: Create buffer with empty content and recipient, not in use
        todo!()
    }

    pub fn reset(&self) {
        // TODO: Clear content and recipient, set in_use to false
        todo!()
    }

    pub fn set(&self, content: &str, recipient: &str) {
        // TODO: Set content and recipient, set in_use to true
        todo!()
    }

    pub fn is_in_use(&self) -> bool {
        // TODO: Return in_use status
        todo!()
    }

    pub fn get_message(&self) -> (String, String) {
        // TODO: Return (content, recipient) tuple
        todo!()
    }
}

/// Pool of reusable message buffers
pub struct MessagePool {
    pub buffers: RefCell<Vec<Rc<MessageBuffer>>>,
    pub pool_size: usize,
    pub next_id: RefCell<u64>,
}

impl MessagePool {
    pub fn new(pool_size: usize) -> Self {
        // TODO: Create pool with pool_size pre-allocated buffers
        // Each buffer has sequential ids starting from 1
        todo!()
    }

    pub fn acquire(&self) -> Option<Rc<MessageBuffer>> {
        // TODO: Find first buffer not in use, return it
        // Return None if all in use
        todo!()
    }

    pub fn release(&self, buffer: &MessageBuffer) {
        // TODO: Reset the buffer
        todo!()
    }

    pub fn available_count(&self) -> usize {
        // TODO: Count buffers not in use
        todo!()
    }

    pub fn in_use_count(&self) -> usize {
        // TODO: Count buffers in use
        todo!()
    }

    pub fn create_message(&self, content: &str, recipient: &str) -> Option<Rc<MessageBuffer>> {
        // TODO: Acquire a buffer, set content and recipient, return it
        // Return None if no buffer available
        todo!()
    }
}

// ============================================================================
// Part 4: Reference-Counted Cache with Eviction
// ============================================================================

/// Cache entry with metadata
#[derive(Debug)]
pub struct CacheEntry {
    pub value: String,
    pub access_count: RefCell<u32>,
    pub created_at: u64,
}

impl CacheEntry {
    pub fn new(value: &str, created_at: u64) -> Rc<Self> {
        // TODO: Create entry with value, access_count = 0
        todo!()
    }

    pub fn access(&self) -> &str {
        // TODO: Increment access_count, return value
        todo!()
    }

    pub fn get_access_count(&self) -> u32 {
        // TODO: Return access_count
        todo!()
    }
}

/// LRU cache with reference-counted entries
pub struct LRUCache {
    pub entries: RefCell<HashMap<String, Rc<CacheEntry>>>,
    pub access_order: RefCell<Vec<String>>,
    pub max_size: usize,
    pub current_time: RefCell<u64>,
    pub hits: RefCell<u64>,
    pub misses: RefCell<u64>,
}

impl LRUCache {
    pub fn new(max_size: usize) -> Self {
        // TODO: Create cache with max_size, empty entries, time starting at 0
        todo!()
    }

    fn advance_time(&self) -> u64 {
        // TODO: Increment current_time and return new value
        todo!()
    }

    pub fn get(&self, key: &str) -> Option<Rc<CacheEntry>> {
        // TODO: Get entry, update access_order (move key to end)
        // Increment hits or misses
        todo!()
    }

    pub fn insert(&self, key: &str, value: &str) {
        // TODO: Insert entry, evict LRU if over max_size
        // Update access_order
        todo!()
    }

    pub fn remove(&self, key: &str) -> Option<Rc<CacheEntry>> {
        // TODO: Remove entry, update access_order
        todo!()
    }

    pub fn evict_lru(&self) -> Option<String> {
        // TODO: Find and remove least recently used (first in access_order)
        // Return the evicted key
        todo!()
    }

    pub fn size(&self) -> usize {
        // TODO: Return current size
        todo!()
    }

    pub fn get_stats(&self) -> (u64, u64, f64) {
        // TODO: Return (hits, misses, hit_rate)
        todo!()
    }

    pub fn clear(&self) {
        // TODO: Clear all entries and access_order
        todo!()
    }
}

// ============================================================================
// Main function for testing
// ============================================================================

fn main() {
    println!("=== Part 1: Doubly-Linked Message Thread ===\n");
    
    let manager = ThreadManager::new();
    
    let root = manager.create_thread("Welcome to Textio!", "System");
    println!("Thread count: {}", manager.get_thread_count());
    
    let reply1 = manager.add_reply(1, "Thanks!", "Alice").unwrap();
    let reply2 = manager.add_reply(1, "Hello!", "Bob").unwrap();
    let reply3 = manager.add_reply(2, "You're welcome!", "System").unwrap();
    
    println!("Total messages: {}", manager.get_total_messages());
    println!("Root reply count: {}", root.reply_count());
    println!("Reply1 depth: {}", reply1.depth());
    println!("Reply3 depth: {}", reply3.depth());
    
    if let Some(parent) = reply3.get_parent() {
        println!("Reply3's parent: {}", parent.content.borrow());
    }
    
    let ancestors: Vec<_> = reply3.ancestors().iter()
        .map(|m| m.content.borrow().clone())
        .collect();
    println!("Reply3's ancestors: {:?}", ancestors);
    
    println!("\n=== Part 2: Observable State Manager ===\n");
    
    let state = StateManager::new();
    let logger = Rc::new(LoggingObserver::new());
    
    state.subscribe(Rc::clone(&logger) as Rc<dyn StateObserver>);
    
    state.set("api_key", "secret123");
    state.set("endpoint", "https://api.textio.io");
    state.set("api_key", "newkey456");
    
    println!("API key: {:?}", state.get("api_key"));
    println!("Observer count: {}", state.observer_count());
    
    for log in logger.get_logs() {
        println!("  {}", log);
    }
    
    println!("\n=== Part 3: Message Pool ===\n");
    
    let pool = MessagePool::new(3);
    
    println!("Pool: available={}, in_use={}", pool.available_count(), pool.in_use_count());
    
    let msg1 = pool.create_message("Hello", "+111").unwrap();
    let msg2 = pool.create_message("World", "+222").unwrap();
    
    println!("After allocations: available={}, in_use={}", pool.available_count(), pool.in_use_count());
    
    let msg3 = pool.create_message("Test", "+333");
    let msg4 = pool.create_message("Fail", "+444");
    
    println!("msg3: {:?}", msg3.as_ref().map(|m| m.get_message()));
    println!("msg4: {:?}", msg4.as_ref().map(|m| m.get_message()));
    
    pool.release(&msg1);
    println!("After release: available={}", pool.available_count());
    
    println!("\n=== Part 4: LRU Cache ===\n");
    
    let cache = LRUCache::new(3);
    
    cache.insert("a", "Alpha");
    cache.insert("b", "Beta");
    cache.insert("c", "Charlie");
    
    println!("Size: {}", cache.size());
    
    cache.get("a");
    cache.get("a");
    cache.get("b");
    
    cache.insert("d", "Delta");
    
    println!("After inserting d, size: {}", cache.size());
    println!("Contains a: {}", cache.get("a").is_some());
    println!("Contains c: {}", cache.get("c").is_some());  // Should be evicted
    
    if let Some(entry) = cache.get("a") {
        println!("Entry 'a' access count: {}", entry.get_access_count());
    }
    
    let (hits, misses, rate) = cache.get_stats();
    println!("Stats: hits={}, misses={}, rate={:.2}%", hits, misses, rate * 100.0);
}

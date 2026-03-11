// Exercise 5: Smart Pointer Patterns - Complete Solution

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

// ============================================================================
// Part 1: Doubly-Linked Message Thread
// ============================================================================

pub struct ThreadMessage {
    pub id: u64,
    pub content: String,
    pub sender: String,
    pub replies: RefCell<Vec<Rc<ThreadMessage>>>,
    pub parent: RefCell<Weak<ThreadMessage>>,
}

impl ThreadMessage {
    pub fn new(id: u64, content: &str, sender: &str) -> Rc<Self> {
        Rc::new(ThreadMessage {
            id,
            content: content.to_string(),
            sender: sender.to_string(),
            replies: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    pub fn add_reply(self: &Rc<Self>, id: u64, content: &str, sender: &str) -> Rc<ThreadMessage> {
        let reply = Rc::new(ThreadMessage {
            id,
            content: content.to_string(),
            sender: sender.to_string(),
            replies: RefCell::new(Vec::new()),
            parent: RefCell::new(Rc::downgrade(self)),
        });
        self.replies.borrow_mut().push(Rc::clone(&reply));
        reply
    }

    pub fn get_parent(&self) -> Option<Rc<ThreadMessage>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_replies(&self) -> Vec<Rc<ThreadMessage>> {
        self.replies.borrow().clone()
    }

    pub fn reply_count(&self) -> usize {
        self.replies.borrow().len()
    }

    pub fn depth(&self) -> usize {
        match self.get_parent() {
            Some(parent) => 1 + parent.depth(),
            None => 0,
        }
    }

    pub fn flatten(&self) -> Vec<Rc<ThreadMessage>> {
        let mut result = vec![Rc::new(ThreadMessage {
            id: self.id,
            content: self.content.clone(),
            sender: self.sender.clone(),
            replies: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })];
        for reply in self.replies.borrow().iter() {
            result.extend(reply.flatten());
        }
        result
    }

    pub fn find_by_id(&self, id: u64) -> Option<Rc<ThreadMessage>> {
        if self.id == id {
            return Some(Rc::new(ThreadMessage {
                id: self.id,
                content: self.content.clone(),
                sender: self.sender.clone(),
                replies: RefCell::new(Vec::new()),
                parent: RefCell::new(Weak::new()),
            }));
        }
        for reply in self.replies.borrow().iter() {
            if let Some(found) = reply.find_by_id(id) {
                return Some(found);
            }
        }
        None
    }

    pub fn ancestors(&self) -> Vec<Rc<ThreadMessage>> {
        let mut result = Vec::new();
        let mut current = self.get_parent();
        while let Some(parent) = current {
            result.push(Rc::new(ThreadMessage {
                id: parent.id,
                content: parent.content.clone(),
                sender: parent.sender.clone(),
                replies: RefCell::new(Vec::new()),
                parent: RefCell::new(Weak::new()),
            }));
            current = parent.get_parent();
        }
        result
    }
}

pub struct ThreadManager {
    pub roots: RefCell<Vec<Rc<ThreadMessage>>>,
    pub next_id: RefCell<u64>,
}

impl ThreadManager {
    pub fn new() -> Self {
        ThreadManager {
            roots: RefCell::new(Vec::new()),
            next_id: RefCell::new(1),
        }
    }

    pub fn create_thread(&self, content: &str, sender: &str) -> Rc<ThreadMessage> {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        let msg = ThreadMessage::new(id, content, sender);
        self.roots.borrow_mut().push(Rc::clone(&msg));
        msg
    }

    pub fn add_reply(&self, parent_id: u64, content: &str, sender: &str) -> Option<Rc<ThreadMessage>> {
        for root in self.roots.borrow().iter() {
            if let Some(parent) = self.find_in_thread(root, parent_id) {
                let id = *self.next_id.borrow();
                *self.next_id.borrow_mut() += 1;
                return Some(parent.add_reply(id, content, sender));
            }
        }
        None
    }

    fn find_in_thread(&self, msg: &Rc<ThreadMessage>, id: u64) -> Option<Rc<ThreadMessage>> {
        if msg.id == id {
            return Some(Rc::clone(msg));
        }
        for reply in msg.replies.borrow().iter() {
            if let Some(found) = self.find_in_thread(reply, id) {
                return Some(found);
            }
        }
        None
    }

    pub fn get_thread_count(&self) -> usize {
        self.roots.borrow().len()
    }

    pub fn get_total_messages(&self) -> usize {
        self.roots.borrow().iter()
            .map(|r| self.count_messages(r))
            .sum()
    }

    fn count_messages(&self, msg: &Rc<ThreadMessage>) -> usize {
        1 + msg.replies.borrow().iter()
            .map(|r| self.count_messages(r))
            .sum::<usize>()
    }
}

// ============================================================================
// Part 2: Observable State Manager
// ============================================================================

pub trait StateObserver {
    fn on_change(&self, key: &str, old_value: Option<&str>, new_value: &str);
}

pub struct LoggingObserver {
    pub logs: RefCell<Vec<String>>,
}

impl LoggingObserver {
    pub fn new() -> Self {
        LoggingObserver {
            logs: RefCell::new(Vec::new()),
        }
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }
}

impl StateObserver for LoggingObserver {
    fn on_change(&self, key: &str, old_value: Option<&str>, new_value: &str) {
        let log = format!("{}: {:?} -> {}", key, old_value, new_value);
        self.logs.borrow_mut().push(log);
    }
}

pub struct StateManager {
    pub state: RefCell<HashMap<String, String>>,
    pub observers: RefCell<Vec<Rc<dyn StateObserver>>>,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            state: RefCell::new(HashMap::new()),
            observers: RefCell::new(Vec::new()),
        }
    }

    pub fn subscribe(&self, observer: Rc<dyn StateObserver>) {
        self.observers.borrow_mut().push(observer);
    }

    pub fn set(&self, key: &str, value: &str) {
        let old_value = self.state.borrow().get(key).cloned();
        self.state.borrow_mut().insert(key.to_string(), value.to_string());
        for observer in self.observers.borrow().iter() {
            observer.on_change(key, old_value.as_deref(), value);
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.state.borrow().get(key).cloned()
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        let old_value = self.state.borrow_mut().remove(key);
        if let Some(ref val) = old_value {
            for observer in self.observers.borrow().iter() {
                observer.on_change(key, Some(val), "<removed>");
            }
        }
        old_value
    }

    pub fn keys(&self) -> Vec<String> {
        self.state.borrow().keys().cloned().collect()
    }

    pub fn observer_count(&self) -> usize {
        self.observers.borrow().len()
    }
}

// ============================================================================
// Part 3: Message Pool
// ============================================================================

pub struct MessageBuffer {
    pub id: u64,
    pub content: RefCell<String>,
    pub recipient: RefCell<String>,
    pub in_use: RefCell<bool>,
}

impl MessageBuffer {
    pub fn new(id: u64) -> Rc<Self> {
        Rc::new(MessageBuffer {
            id,
            content: RefCell::new(String::new()),
            recipient: RefCell::new(String::new()),
            in_use: RefCell::new(false),
        })
    }

    pub fn reset(&self) {
        *self.content.borrow_mut() = String::new();
        *self.recipient.borrow_mut() = String::new();
        *self.in_use.borrow_mut() = false;
    }

    pub fn set(&self, content: &str, recipient: &str) {
        *self.content.borrow_mut() = content.to_string();
        *self.recipient.borrow_mut() = recipient.to_string();
        *self.in_use.borrow_mut() = true;
    }

    pub fn is_in_use(&self) -> bool {
        *self.in_use.borrow()
    }

    pub fn get_message(&self) -> (String, String) {
        (self.content.borrow().clone(), self.recipient.borrow().clone())
    }
}

pub struct MessagePool {
    pub buffers: RefCell<Vec<Rc<MessageBuffer>>>,
    pub pool_size: usize,
    pub next_id: RefCell<u64>,
}

impl MessagePool {
    pub fn new(pool_size: usize) -> Self {
        let mut buffers = Vec::new();
        for i in 1..=pool_size {
            buffers.push(MessageBuffer::new(i as u64));
        }
        MessagePool {
            buffers: RefCell::new(buffers),
            pool_size,
            next_id: RefCell::new(pool_size as u64 + 1),
        }
    }

    pub fn acquire(&self) -> Option<Rc<MessageBuffer>> {
        for buffer in self.buffers.borrow().iter() {
            if !buffer.is_in_use() {
                *buffer.in_use.borrow_mut() = true;
                return Some(Rc::clone(buffer));
            }
        }
        None
    }

    pub fn release(&self, buffer: &MessageBuffer) {
        buffer.reset();
    }

    pub fn available_count(&self) -> usize {
        self.buffers.borrow().iter().filter(|b| !b.is_in_use()).count()
    }

    pub fn in_use_count(&self) -> usize {
        self.buffers.borrow().iter().filter(|b| b.is_in_use()).count()
    }

    pub fn create_message(&self, content: &str, recipient: &str) -> Option<Rc<MessageBuffer>> {
        let buffer = self.acquire()?;
        buffer.set(content, recipient);
        Some(buffer)
    }
}

// ============================================================================
// Part 4: Reference-Counted Cache with Eviction
// ============================================================================

#[derive(Debug)]
pub struct CacheEntry {
    pub value: String,
    pub access_count: RefCell<u32>,
    pub created_at: u64,
}

impl CacheEntry {
    pub fn new(value: &str, created_at: u64) -> Rc<Self> {
        Rc::new(CacheEntry {
            value: value.to_string(),
            access_count: RefCell::new(0),
            created_at,
        })
    }

    pub fn access(&self) -> &str {
        *self.access_count.borrow_mut() += 1;
        &self.value
    }

    pub fn get_access_count(&self) -> u32 {
        *self.access_count.borrow()
    }
}

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
        LRUCache {
            entries: RefCell::new(HashMap::new()),
            access_order: RefCell::new(Vec::new()),
            max_size,
            current_time: RefCell::new(0),
            hits: RefCell::new(0),
            misses: RefCell::new(0),
        }
    }

    fn advance_time(&self) -> u64 {
        *self.current_time.borrow_mut() += 1;
        *self.current_time.borrow()
    }

    pub fn get(&self, key: &str) -> Option<Rc<CacheEntry>> {
        if let Some(entry) = self.entries.borrow().get(key) {
            *self.hits.borrow_mut() += 1;
            entry.access();
            self.access_order.borrow_mut().retain(|k| k != key);
            self.access_order.borrow_mut().push(key.to_string());
            Some(Rc::clone(entry))
        } else {
            *self.misses.borrow_mut() += 1;
            None
        }
    }

    pub fn insert(&self, key: &str, value: &str) {
        if self.entries.borrow().contains_key(key) {
            self.access_order.borrow_mut().retain(|k| k != key);
            self.access_order.borrow_mut().push(key.to_string());
            let entry = CacheEntry::new(value, self.advance_time());
            self.entries.borrow_mut().insert(key.to_string(), entry);
        } else {
            if self.entries.borrow().len() >= self.max_size {
                self.evict_lru();
            }
            self.access_order.borrow_mut().push(key.to_string());
            let entry = CacheEntry::new(value, self.advance_time());
            self.entries.borrow_mut().insert(key.to_string(), entry);
        }
    }

    pub fn remove(&self, key: &str) -> Option<Rc<CacheEntry>> {
        self.access_order.borrow_mut().retain(|k| k != key);
        self.entries.borrow_mut().remove(key)
    }

    pub fn evict_lru(&self) -> Option<String> {
        if let Some(lru_key) = self.access_order.borrow_mut().first().cloned() {
            self.access_order.borrow_mut().remove(0);
            self.entries.borrow_mut().remove(&lru_key);
            Some(lru_key)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.entries.borrow().len()
    }

    pub fn get_stats(&self) -> (u64, u64, f64) {
        let hits = *self.hits.borrow();
        let misses = *self.misses.borrow();
        let total = hits + misses;
        let rate = if total > 0 { hits as f64 / total as f64 } else { 0.0 };
        (hits, misses, rate)
    }

    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
        self.access_order.borrow_mut().clear();
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
        println!("Reply3's parent: {}", parent.content);
    }
    
    let ancestors: Vec<_> = reply3.ancestors().iter()
        .map(|m| m.content.clone())
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
    println!("Contains c: {}", cache.get("c").is_some());
    
    if let Some(entry) = cache.get("a") {
        println!("Entry 'a' access count: {}", entry.get_access_count());
    }
    
    let (hits, misses, rate) = cache.get_stats();
    println!("Stats: hits={}, misses={}, rate={:.2}%", hits, misses, rate * 100.0);
}

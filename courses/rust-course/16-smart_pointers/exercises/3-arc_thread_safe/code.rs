// Exercise 3: Arc<T> - Atomic Reference Counting and Thread Safety
//
// TODO: Implement thread-safe shared ownership using Arc<T>

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::thread;
use std::collections::HashMap;

// ============================================================================
// Part 1: Thread-Safe Configuration
// ============================================================================

/// Global configuration shared across threads
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub endpoint: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
}

impl Config {
    pub fn new(api_key: &str, endpoint: &str, timeout_ms: u64, max_retries: u32) -> Self {
        Config {
            api_key: api_key.to_string(),
            endpoint: endpoint.to_string(),
            timeout_ms,
            max_retries,
        }
    }
}

/// Worker that processes messages using shared config
pub struct Worker {
    pub id: usize,
    pub config: Arc<Config>,
}

impl Worker {
    pub fn new(id: usize, config: Arc<Config>) -> Self {
        // TODO: Create worker with shared config
        todo!()
    }

    pub fn process(&self, message: &str) -> String {
        // TODO: Return formatted processing string
        // Format: "[Worker {id}] Processing '{message}' via {endpoint}"
        todo!()
    }
}

/// Worker pool that spawns threads with shared config
pub struct WorkerPool {
    pub config: Arc<Config>,
    pub worker_count: usize,
}

impl WorkerPool {
    pub fn new(config: Arc<Config>, worker_count: usize) -> Self {
        // TODO: Create worker pool
        todo!()
    }

    pub fn spawn_and_process(&self, messages: Vec<String>) -> Vec<String> {
        // TODO: Spawn worker_count threads
        // Each thread processes messages[thread_id % messages.len()]
        // Collect and return all results
        // Hint: Use thread::spawn and JoinHandle
        todo!()
    }

    pub fn config_ref_count(&self) -> usize {
        // TODO: Return Arc strong count
        todo!()
    }
}

// ============================================================================
// Part 2: Parallel Message Processor with Stats
// ============================================================================

/// Thread-safe statistics tracker
pub struct Stats {
    pub messages_processed: AtomicU64,
    pub messages_failed: AtomicU64,
    pub total_bytes: AtomicU64,
    pub is_running: AtomicBool,
}

impl Stats {
    pub fn new() -> Self {
        // TODO: Create stats with all counters at 0, is_running true
        todo!()
    }

    pub fn record_success(&self, bytes: u64) {
        // TODO: Atomically increment messages_processed and total_bytes
        todo!()
    }

    pub fn record_failure(&self) {
        // TODO: Atomically increment messages_failed
        todo!()
    }

    pub fn get_summary(&self) -> (u64, u64, u64) {
        // TODO: Return (processed, failed, total_bytes)
        todo!()
    }

    pub fn stop(&self) {
        // TODO: Set is_running to false
        todo!()
    }

    pub fn is_running(&self) -> bool {
        // TODO: Return current is_running value
        todo!()
    }
}

/// Message for processing
#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub content: String,
    pub recipient: String,
}

/// Parallel message processor
pub struct MessageProcessor {
    pub config: Arc<Config>,
    pub stats: Arc<Stats>,
    pub pending: Arc<Mutex<Vec<Message>>>,
}

impl MessageProcessor {
    pub fn new(config: Arc<Config>, stats: Arc<Stats>) -> Self {
        // TODO: Create processor with empty pending queue
        todo!()
    }

    pub fn enqueue(&self, message: Message) {
        // TODO: Add message to pending queue
        todo!()
    }

    pub fn enqueue_batch(&self, messages: Vec<Message>) {
        // TODO: Add all messages to pending queue
        todo!()
    }

    pub fn pending_count(&self) -> usize {
        // TODO: Return number of pending messages
        todo!()
    }

    pub fn spawn_worker(&self, worker_id: usize) -> thread::JoinHandle<Vec<String>> {
        // TODO: Spawn a thread that:
        // 1. Loops while stats.is_running() and there are pending messages
        // 2. Locks pending, pops a message, releases lock
        // 3. Processes the message (simulate: format string)
        // 4. Records success with message content length as bytes
        // 5. Returns vector of processed message strings
        // Processing format: "[Worker {id}] Sent to {recipient}: {content}"
        todo!()
    }

    pub fn process_all(&self, worker_count: usize) -> Vec<String> {
        // TODO: Spawn worker_count workers, join all, return combined results
        todo!()
    }
}

// ============================================================================
// Part 3: Thread-Safe Message Cache
// ============================================================================

/// Cached message entry
#[derive(Debug, Clone)]
pub struct CachedMessage {
    pub id: u64,
    pub content: String,
    pub timestamp: u64,
    pub access_count: u64,
}

/// Thread-safe message cache using RwLock
pub struct MessageCache {
    pub cache: Arc<RwLock<HashMap<u64, CachedMessage>>>,
    pub hits: AtomicU64,
    pub misses: AtomicU64,
}

impl MessageCache {
    pub fn new() -> Self {
        // TODO: Create empty cache
        todo!()
    }

    pub fn get(&self, id: u64) -> Option<CachedMessage> {
        // TODO: Read from cache
        // If found: increment hits, increment access_count, return cloned message
        // If not found: increment misses, return None
        todo!()
    }

    pub fn insert(&self, message: CachedMessage) {
        // TODO: Write to cache
        todo!()
    }

    pub fn remove(&self, id: u64) -> bool {
        // TODO: Remove from cache, return true if existed
        todo!()
    }

    pub fn size(&self) -> usize {
        // TODO: Return cache size
        todo!()
    }

    pub fn get_stats(&self) -> (u64, u64, f64) {
        // TODO: Return (hits, misses, hit_rate)
        // hit_rate = hits / (hits + misses), or 0.0 if no requests
        todo!()
    }

    pub fn clear(&self) {
        // TODO: Clear the cache
        todo!()
    }
}

/// Cache worker that reads and writes to shared cache
pub struct CacheWorker {
    pub id: usize,
    pub cache: Arc<MessageCache>,
}

impl CacheWorker {
    pub fn new(id: usize, cache: Arc<MessageCache>) -> Self {
        // TODO: Create cache worker
        todo!()
    }

    pub fn run_reads(&self, ids: Vec<u64>) -> Vec<Option<CachedMessage>> {
        // TODO: For each id, try to get from cache
        // Return vector of results
        todo!()
    }

    pub fn populate(&self, messages: Vec<CachedMessage>) {
        // TODO: Insert all messages into cache
        todo!()
    }
}

// ============================================================================
// Part 4: Circuit Breaker Pattern
// ============================================================================

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed,    // Normal operation
    Open,      // Failing, reject requests
    HalfOpen,  // Testing if recovered
}

/// Thread-safe circuit breaker
pub struct CircuitBreaker {
    pub state: Arc<RwLock<CircuitState>>,
    pub failure_count: AtomicU64,
    pub success_count: AtomicU64,
    pub failure_threshold: u64,
    pub success_threshold: u64,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u64, success_threshold: u64) -> Self {
        // TODO: Create circuit breaker starting in Closed state
        todo!()
    }

    pub fn is_available(&self) -> bool {
        // TODO: Check if requests should be allowed
        // Closed: yes, Open: no, HalfOpen: yes (but limited)
        todo!()
    }

    pub fn record_success(&self) {
        // TODO: Increment success_count
        // If HalfOpen and success_count >= success_threshold: transition to Closed
        todo!()
    }

    pub fn record_failure(&self) {
        // TODO: Increment failure_count
        // If Closed and failure_count >= failure_threshold: transition to Open
        // If HalfOpen: transition to Open
        todo!()
    }

    pub fn get_state(&self) -> CircuitState {
        // TODO: Return current state
        todo!()
    }

    pub fn try_reset(&self) -> bool {
        // TODO: If Open, transition to HalfOpen and return true
        // Otherwise return false
        todo!()
    }

    pub fn reset_counts(&self) {
        // TODO: Reset failure_count and success_count to 0
        todo!()
    }
}

/// Service with circuit breaker protection
pub struct ProtectedService {
    pub name: String,
    pub circuit: Arc<CircuitBreaker>,
}

impl ProtectedService {
    pub fn new(name: &str, circuit: Arc<CircuitBreaker>) -> Self {
        // TODO: Create protected service
        todo!()
    }

    pub fn call(&self, should_succeed: bool) -> Result<String, String> {
        // TODO: If circuit allows, simulate call
        // If should_succeed: record success, return Ok with message
        // If not should_succeed: record failure, return Err with message
        // If circuit doesn't allow: return Err("Circuit open")
        // Success message: "[{name}] Success"
        // Failure message: "[{name}] Failed"
        todo!()
    }
}

// ============================================================================
// Main function for testing
// ============================================================================

fn main() {
    println!("=== Part 1: Thread-Safe Configuration ===\n");
    
    let config = Arc::new(Config::new("key123", "https://api.textio.io", 5000, 3));
    let pool = WorkerPool::new(Arc::clone(&config), 4);
    
    println!("Config ref count: {}", pool.config_ref_count());
    
    let messages = vec![
        "Hello".to_string(),
        "World".to_string(),
        "Test".to_string(),
        "Textio".to_string(),
    ];
    
    let results = pool.spawn_and_process(messages);
    for result in results {
        println!("{}", result);
    }
    
    println!("\n=== Part 2: Parallel Message Processor ===\n");
    
    let config = Arc::new(Config::new("key456", "https://api.textio.io", 3000, 2));
    let stats = Arc::new(Stats::new());
    let processor = MessageProcessor::new(Arc::clone(&config), Arc::clone(&stats));
    
    processor.enqueue_batch(vec![
        Message { id: 1, content: "First".to_string(), recipient: "+111".to_string() },
        Message { id: 2, content: "Second".to_string(), recipient: "+222".to_string() },
        Message { id: 3, content: "Third".to_string(), recipient: "+333".to_string() },
        Message { id: 4, content: "Fourth".to_string(), recipient: "+444".to_string() },
    ]);
    
    println!("Pending messages: {}", processor.pending_count());
    
    let results = processor.process_all(2);
    for result in &results {
        println!("{}", result);
    }
    
    let (processed, failed, bytes) = stats.get_summary();
    println!("\nStats: processed={}, failed={}, bytes={}", processed, failed, bytes);
    
    println!("\n=== Part 3: Thread-Safe Cache ===\n");
    
    let cache = Arc::new(MessageCache::new());
    let worker1 = CacheWorker::new(1, Arc::clone(&cache));
    let worker2 = CacheWorker::new(2, Arc::clone(&cache));
    
    worker1.populate(vec![
        CachedMessage { id: 1, content: "Hello".to_string(), timestamp: 1000, access_count: 0 },
        CachedMessage { id: 2, content: "World".to_string(), timestamp: 1001, access_count: 0 },
    ]);
    
    println!("Cache size: {}", cache.size());
    
    let results1 = worker1.run_reads(vec![1, 2, 3]);
    println!("Worker 1 reads: {:?}", results1.iter().map(|r| r.as_ref().map(|m| &m.content)).collect::<Vec<_>>());
    
    let results2 = worker2.run_reads(vec![1, 3]);
    println!("Worker 2 reads: {:?}", results2.iter().map(|r| r.as_ref().map(|m| &m.content)).collect::<Vec<_>>());
    
    let (hits, misses, rate) = cache.get_stats();
    println!("Cache stats: hits={}, misses={}, rate={:.2}%", hits, misses, rate * 100.0);
    
    println!("\n=== Part 4: Circuit Breaker ===\n");
    
    let circuit = Arc::new(CircuitBreaker::new(3, 2));
    let service = ProtectedService::new("SMS-Gateway", Arc::clone(&circuit));
    
    println!("Initial state: {:?}", circuit.get_state());
    
    // Cause failures
    for _ in 0..3 {
        let _ = service.call(false);
    }
    println!("After failures: {:?}", circuit.get_state());
    println!("Service available: {}", circuit.is_available());
    
    // Reset to half-open
    circuit.try_reset();
    println!("After reset: {:?}", circuit.get_state());
    
    // Succeed to close
    for _ in 0..2 {
        let _ = service.call(true);
    }
    println!("After successes: {:?}", circuit.get_state());
}

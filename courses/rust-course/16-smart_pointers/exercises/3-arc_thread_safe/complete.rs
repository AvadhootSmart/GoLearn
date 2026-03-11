// Exercise 3: Arc<T> - Complete Solution

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::thread;
use std::collections::HashMap;

// ============================================================================
// Part 1: Thread-Safe Configuration
// ============================================================================

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

pub struct Worker {
    pub id: usize,
    pub config: Arc<Config>,
}

impl Worker {
    pub fn new(id: usize, config: Arc<Config>) -> Self {
        Worker { id, config }
    }

    pub fn process(&self, message: &str) -> String {
        format!("[Worker {}] Processing '{}' via {}", self.id, message, self.config.endpoint)
    }
}

pub struct WorkerPool {
    pub config: Arc<Config>,
    pub worker_count: usize,
}

impl WorkerPool {
    pub fn new(config: Arc<Config>, worker_count: usize) -> Self {
        WorkerPool { config, worker_count }
    }

    pub fn spawn_and_process(&self, messages: Vec<String>) -> Vec<String> {
        let handles: Vec<_> = (0..self.worker_count)
            .map(|i| {
                let config = Arc::clone(&self.config);
                let msg = messages[i % messages.len()].clone();
                thread::spawn(move || {
                    let worker = Worker::new(i, config);
                    worker.process(&msg)
                })
            })
            .collect();

        handles.into_iter()
            .map(|h| h.join().unwrap())
            .collect()
    }

    pub fn config_ref_count(&self) -> usize {
        Arc::strong_count(&self.config)
    }
}

// ============================================================================
// Part 2: Parallel Message Processor with Stats
// ============================================================================

pub struct Stats {
    pub messages_processed: AtomicU64,
    pub messages_failed: AtomicU64,
    pub total_bytes: AtomicU64,
    pub is_running: AtomicBool,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            messages_processed: AtomicU64::new(0),
            messages_failed: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
            is_running: AtomicBool::new(true),
        }
    }

    pub fn record_success(&self, bytes: u64) {
        self.messages_processed.fetch_add(1, Ordering::SeqCst);
        self.total_bytes.fetch_add(bytes, Ordering::SeqCst);
    }

    pub fn record_failure(&self) {
        self.messages_failed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_summary(&self) -> (u64, u64, u64) {
        (
            self.messages_processed.load(Ordering::SeqCst),
            self.messages_failed.load(Ordering::SeqCst),
            self.total_bytes.load(Ordering::SeqCst),
        )
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub content: String,
    pub recipient: String,
}

pub struct MessageProcessor {
    pub config: Arc<Config>,
    pub stats: Arc<Stats>,
    pub pending: Arc<Mutex<Vec<Message>>>,
}

impl MessageProcessor {
    pub fn new(config: Arc<Config>, stats: Arc<Stats>) -> Self {
        MessageProcessor {
            config,
            stats,
            pending: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn enqueue(&self, message: Message) {
        self.pending.lock().unwrap().push(message);
    }

    pub fn enqueue_batch(&self, messages: Vec<Message>) {
        self.pending.lock().unwrap().extend(messages);
    }

    pub fn pending_count(&self) -> usize {
        self.pending.lock().unwrap().len()
    }

    pub fn spawn_worker(&self, worker_id: usize) -> thread::JoinHandle<Vec<String>> {
        let stats = Arc::clone(&self.stats);
        let pending = Arc::clone(&self.pending);

        thread::spawn(move || {
            let mut results = Vec::new();
            while stats.is_running() {
                let msg = pending.lock().unwrap().pop();
                match msg {
                    Some(m) => {
                        let result = format!("[Worker {}] Sent to {}: {}", worker_id, m.recipient, m.content);
                        stats.record_success(m.content.len() as u64);
                        results.push(result);
                    }
                    None => break,
                }
            }
            results
        })
    }

    pub fn process_all(&self, worker_count: usize) -> Vec<String> {
        let handles: Vec<_> = (0..worker_count)
            .map(|id| self.spawn_worker(id))
            .collect();

        handles.into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    }
}

// ============================================================================
// Part 3: Thread-Safe Message Cache
// ============================================================================

#[derive(Debug, Clone)]
pub struct CachedMessage {
    pub id: u64,
    pub content: String,
    pub timestamp: u64,
    pub access_count: u64,
}

pub struct MessageCache {
    pub cache: Arc<RwLock<HashMap<u64, CachedMessage>>>,
    pub hits: AtomicU64,
    pub misses: AtomicU64,
}

impl MessageCache {
    pub fn new() -> Self {
        MessageCache {
            cache: Arc::new(RwLock::new(HashMap::new())),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub fn get(&self, id: u64) -> Option<CachedMessage> {
        let mut cache = self.cache.write().unwrap();
        if let Some(msg) = cache.get_mut(&id) {
            self.hits.fetch_add(1, Ordering::SeqCst);
            msg.access_count += 1;
            Some(msg.clone())
        } else {
            self.misses.fetch_add(1, Ordering::SeqCst);
            None
        }
    }

    pub fn insert(&self, message: CachedMessage) {
        self.cache.write().unwrap().insert(message.id, message);
    }

    pub fn remove(&self, id: u64) -> bool {
        self.cache.write().unwrap().remove(&id).is_some()
    }

    pub fn size(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    pub fn get_stats(&self) -> (u64, u64, f64) {
        let hits = self.hits.load(Ordering::SeqCst);
        let misses = self.misses.load(Ordering::SeqCst);
        let total = hits + misses;
        let rate = if total > 0 { hits as f64 / total as f64 } else { 0.0 };
        (hits, misses, rate)
    }

    pub fn clear(&self) {
        self.cache.write().unwrap().clear();
    }
}

pub struct CacheWorker {
    pub id: usize,
    pub cache: Arc<MessageCache>,
}

impl CacheWorker {
    pub fn new(id: usize, cache: Arc<MessageCache>) -> Self {
        CacheWorker { id, cache }
    }

    pub fn run_reads(&self, ids: Vec<u64>) -> Vec<Option<CachedMessage>> {
        ids.into_iter().map(|id| self.cache.get(id)).collect()
    }

    pub fn populate(&self, messages: Vec<CachedMessage>) {
        for msg in messages {
            self.cache.insert(msg);
        }
    }
}

// ============================================================================
// Part 4: Circuit Breaker Pattern
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

pub struct CircuitBreaker {
    pub state: Arc<RwLock<CircuitState>>,
    pub failure_count: AtomicU64,
    pub success_count: AtomicU64,
    pub failure_threshold: u64,
    pub success_threshold: u64,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u64, success_threshold: u64) -> Self {
        CircuitBreaker {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
            failure_threshold,
            success_threshold,
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(*self.state.read().unwrap(), CircuitState::Closed | CircuitState::HalfOpen)
    }

    pub fn record_success(&self) {
        let count = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;
        let mut state = self.state.write().unwrap();
        if *state == CircuitState::HalfOpen && count >= self.success_threshold {
            *state = CircuitState::Closed;
            self.failure_count.store(0, Ordering::SeqCst);
            self.success_count.store(0, Ordering::SeqCst);
        }
    }

    pub fn record_failure(&self) {
        let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
        let mut state = self.state.write().unwrap();
        match *state {
            CircuitState::Closed if count >= self.failure_threshold => {
                *state = CircuitState::Open;
            }
            CircuitState::HalfOpen => {
                *state = CircuitState::Open;
                self.success_count.store(0, Ordering::SeqCst);
            }
            _ => {}
        }
    }

    pub fn get_state(&self) -> CircuitState {
        *self.state.read().unwrap()
    }

    pub fn try_reset(&self) -> bool {
        let mut state = self.state.write().unwrap();
        if *state == CircuitState::Open {
            *state = CircuitState::HalfOpen;
            self.failure_count.store(0, Ordering::SeqCst);
            self.success_count.store(0, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn reset_counts(&self) {
        self.failure_count.store(0, Ordering::SeqCst);
        self.success_count.store(0, Ordering::SeqCst);
    }
}

pub struct ProtectedService {
    pub name: String,
    pub circuit: Arc<CircuitBreaker>,
}

impl ProtectedService {
    pub fn new(name: &str, circuit: Arc<CircuitBreaker>) -> Self {
        ProtectedService {
            name: name.to_string(),
            circuit,
        }
    }

    pub fn call(&self, should_succeed: bool) -> Result<String, String> {
        if !self.circuit.is_available() {
            return Err("Circuit open".to_string());
        }

        if should_succeed {
            self.circuit.record_success();
            Ok(format!("[{}] Success", self.name))
        } else {
            self.circuit.record_failure();
            Err(format!("[{}] Failed", self.name))
        }
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
    
    for _ in 0..3 {
        let _ = service.call(false);
    }
    println!("After failures: {:?}", circuit.get_state());
    println!("Service available: {}", circuit.is_available());
    
    circuit.try_reset();
    println!("After reset: {:?}", circuit.get_state());
    
    for _ in 0..2 {
        let _ = service.call(true);
    }
    println!("After successes: {:?}", circuit.get_state());
}

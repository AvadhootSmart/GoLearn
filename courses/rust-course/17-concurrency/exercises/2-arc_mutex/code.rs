// Exercise 2: Arc<Mutex<T>> - Shared State Concurrency
//
// This exercise demonstrates shared mutable state using Arc and Mutex
// in the context of Textio's message tracking system.
//
// TODO: Complete the tasks marked with FIXME

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Textio Shared State Demo ===\n");
    
    // Task 1: Basic Arc usage
    basic_arc();
    
    // Task 2: Mutex for mutual exclusion
    basic_mutex();
    
    // Task 3: Arc<Mutex<T>> for shared mutable state
    shared_counter();
    
    // Task 4: Message status aggregator
    status_aggregator();
    
    // Task 5: RwLock for read-heavy workloads
    rwlock_demo();
}

fn basic_arc() {
    println!("--- Basic Arc ---");
    
    // FIXME: Create an Arc containing a vector of message IDs: [1, 2, 3, 4, 5]
    // let message_ids = Arc::new(/* ... */);
    
    // FIXME: Clone the Arc for use in a thread
    // let ids_for_thread = Arc::clone(/* ... */);
    
    // FIXME: Spawn a thread that prints the count of message IDs
    // The thread should print: "Thread sees {} messages"
    
    // FIXME: Join the thread
    
    // FIXME: Print the count from the main thread
    // println!("Main thread sees {} messages", /* ... */);
    
    println!();
}

fn basic_mutex() {
    println!("--- Basic Mutex ---");
    
    // FIXME: Create a Mutex containing a counter starting at 0
    // let counter = Mutex::new(0);
    
    // FIXME: Lock the mutex and increment the counter
    // {
    //     let mut num = counter.lock().unwrap();
    //     *num += 1;
    // }
    
    // FIXME: Print the counter value
    // println!("Counter: {}", /* ... */);
    
    println!();
}

fn shared_counter() {
    println!("--- Shared Counter ---");
    
    // FIXME: Create an Arc<Mutex<i32>> counter starting at 0
    
    // FIXME: Spawn 5 threads, each incrementing the counter 10 times
    // Each thread should:
    // 1. Clone the Arc
    // 2. In a loop, lock the mutex and increment
    // 3. Sleep 1ms between increments (simulate work)
    
    // FIXME: Join all threads
    
    // FIXME: Print the final counter value (should be 50)
    // println!("Final counter: {}", /* ... */);
    
    println!();
}

#[derive(Debug, Default)]
struct MessageStats {
    sent: usize,
    delivered: usize,
    failed: usize,
}

fn status_aggregator() {
    println!("--- Status Aggregator ---");
    
    let stats = Arc::new(Mutex::new(MessageStats::default()));
    let mut handles = vec![];
    
    // Simulate multiple workers reporting status
    let statuses: Vec<&str> = vec!["sent", "delivered", "failed", "sent", "delivered"];
    
    // FIXME: For each status, spawn a thread that updates the stats
    // - Clone the Arc for each thread
    // - Lock the mutex and update the appropriate field
    // - Print "Updated {status} count"
    
    // FIXME: Join all threads
    
    // FIXME: Print the final stats
    // println!("Final stats: {:?}", /* ... */);
    
    println!();
}

fn rwlock_demo() {
    println!("--- RwLock Demo ---");
    
    // FIXME: Create an Arc<RwLock<Vec<String>>> containing an empty vector
    // This simulates a message log that is read frequently but written rarely
    
    // FIXME: Spawn 3 reader threads that read the log
    // Each should print "Reader {id}: {} messages in log"
    
    // FIXME: Spawn 1 writer thread that adds a message
    // It should add "New message" to the log
    
    // FIXME: Join all threads
    
    // FIXME: Print the final log contents
    
    println!();
}

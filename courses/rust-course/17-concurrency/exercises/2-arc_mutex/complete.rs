// Exercise 2: Arc<Mutex<T>> - Complete Solution

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Textio Shared State Demo ===\n");
    
    basic_arc();
    basic_mutex();
    shared_counter();
    status_aggregator();
    rwlock_demo();
}

fn basic_arc() {
    println!("--- Basic Arc ---");
    
    let message_ids = Arc::new(vec![1, 2, 3, 4, 5]);
    let ids_for_thread = Arc::clone(&message_ids);
    
    let handle = thread::spawn(move || {
        println!("Thread sees {} messages", ids_for_thread.len());
    });
    
    handle.join().unwrap();
    
    println!("Main thread sees {} messages", message_ids.len());
    println!();
}

fn basic_mutex() {
    println!("--- Basic Mutex ---");
    
    let counter = Mutex::new(0);
    
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }
    
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }
    
    println!("Counter: {}", *counter.lock().unwrap());
    println!();
}

fn shared_counter() {
    println!("--- Shared Counter ---");
    
    let counter = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                drop(num);
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter: {}", *counter.lock().unwrap());
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
    
    let statuses: Vec<&str> = vec!["sent", "delivered", "failed", "sent", "delivered"];
    
    for status in statuses {
        let stats = Arc::clone(&stats);
        let handle = thread::spawn(move || {
            let mut s = stats.lock().unwrap();
            match status {
                "sent" => s.sent += 1,
                "delivered" => s.delivered += 1,
                "failed" => s.failed += 1,
                _ => {}
            }
            println!("Updated {} count", status);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_stats = stats.lock().unwrap();
    println!("Final stats: {:?}", *final_stats);
    println!();
}

fn rwlock_demo() {
    println!("--- RwLock Demo ---");
    
    let log = Arc::new(RwLock::new(Vec::<String>::new()));
    let mut handles = vec![];
    
    for id in 0..3 {
        let log = Arc::clone(&log);
        let handle = thread::spawn(move || {
            let messages = log.read().unwrap();
            println!("Reader {}: {} messages in log", id, messages.len());
        });
        handles.push(handle);
    }
    
    {
        let log = Arc::clone(&log);
        let handle = thread::spawn(move || {
            let mut messages = log.write().unwrap();
            messages.push("New message".to_string());
            println!("Writer: Added message");
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_log = log.read().unwrap();
    println!("Final log: {:?}", *final_log);
    println!();
}

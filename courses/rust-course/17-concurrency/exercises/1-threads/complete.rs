// Exercise 1: Threads in Rust - Complete Solution

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Textio Thread Demo ===\n");
    
    basic_threads();
    move_closures();
    scoped_threads();
    parallel_processing();
}

fn basic_threads() {
    println!("--- Basic Threads ---");
    
    let handle = thread::spawn(|| {
        println!("Worker thread started");
    });
    
    match handle.join() {
        Ok(_) => println!("Thread completed successfully"),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
    
    println!("Main thread continues after join\n");
}

fn move_closures() {
    println!("--- Move Closures ---");
    
    let message_id = 42;
    let content = String::from("Hello from Textio!");
    
    let handle = thread::spawn(move || {
        println!("Sending message {}: {}", message_id, content);
    });
    
    handle.join().unwrap();
    
    println!();
}

fn scoped_threads() {
    println!("--- Scoped Threads ---");
    
    let messages = vec![
        "Welcome to Textio!",
        "Your verification code is 123456",
        "Thank you for using our service",
    ];
    
    thread::scope(|s| {
        for message in &messages {
            s.spawn(|| {
                println!("Processing: {}", message);
            });
        }
    });
    
    println!("All scoped threads completed\n");
}

fn parallel_processing() {
    println!("--- Parallel Message Processing ---");
    
    let messages: Vec<String> = (0..5)
        .map(|i| format!("Message content {}", i))
        .collect();
    
    let handles: Vec<thread::JoinHandle<String>> = messages
        .into_iter()
        .map(|msg| {
            thread::spawn(move || simulate_send(&msg))
        })
        .collect();
    
    let results: Vec<String> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("Results:");
    for result in results {
        println!("  {}", result);
    }
}

fn simulate_send(message: &str) -> String {
    thread::sleep(Duration::from_millis(50));
    format!("Sent: {}", message)
}

// Exercise 1: Threads in Rust
// 
// This exercise demonstrates thread creation, join handles, and scoped threads
// in the context of Textio's parallel message processing.
//
// TODO: Complete the tasks marked with FIXME

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Textio Thread Demo ===\n");
    
    // Task 1: Basic thread spawn and join
    basic_threads();
    
    // Task 2: Using move closures
    move_closures();
    
    // Task 3: Scoped threads for borrowing
    scoped_threads();
    
    // Task 4: Parallel message processing
    parallel_processing();
}

fn basic_threads() {
    println!("--- Basic Threads ---");
    
    // FIXME: Spawn a thread that prints "Worker thread started"
    // Store the JoinHandle in a variable called `handle`
    
    // FIXME: Call join() on the handle and handle the result
    
    println!("Main thread continues after join\n");
}

fn move_closures() {
    println!("--- Move Closures ---");
    
    let message_id = 42;
    let content = String::from("Hello from Textio!");
    
    // FIXME: Spawn a thread that uses both message_id and content
    // Remember to use 'move' to transfer ownership
    // The thread should print: "Sending message {message_id}: {content}"
    
    // FIXME: Join the thread
    
    // Note: The following line would cause a compile error because content was moved:
    // println!("{}", content);
    
    println!();
}

fn scoped_threads() {
    println!("--- Scoped Threads ---");
    
    let messages = vec![
        "Welcome to Textio!",
        "Your verification code is 123456",
        "Thank you for using our service",
    ];
    
    // FIXME: Use thread::scope to process each message in parallel
    // Each thread should print "Processing: {message}"
    // No need for 'move' - scoped threads can borrow!
    
    println!("All scoped threads completed\n");
}

fn parallel_processing() {
    println!("--- Parallel Message Processing ---");
    
    let messages: Vec<String> = (0..5)
        .map(|i| format!("Message content {}", i))
        .collect();
    
    // FIXME: Process messages in parallel using threads
    // Each thread should:
    // 1. Sleep for 50ms (simulating network latency)
    // 2. Return a string: "Sent: {message}"
    // Collect all results into a Vec<String>
    
    // Hint: Create a Vec<JoinHandle<String>> to store handles
    // Then iterate over handles and collect the results
    
    let results: Vec<String> = vec![]; // FIXME: Replace with actual implementation
    
    println!("Results:");
    for result in results {
        println!("  {}", result);
    }
}

// Helper function to simulate sending an SMS
fn simulate_send(message: &str) -> String {
    thread::sleep(Duration::from_millis(50));
    format!("Sent: {}", message)
}

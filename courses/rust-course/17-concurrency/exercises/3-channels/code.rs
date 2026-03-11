// Exercise 3: Channels - Message Passing Concurrency
//
// This exercise demonstrates mpsc channels for thread communication
// in the context of Textio's message distribution system.
//
// TODO: Complete the tasks marked with FIXME

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Textio Channels Demo ===\n");
    
    // Task 1: Basic channel communication
    basic_channel();
    
    // Task 2: Multiple producers
    multiple_producers();
    
    // Task 3: Message pipeline
    message_pipeline();
    
    // Task 4: Worker pool pattern
    worker_pool();
}

fn basic_channel() {
    println!("--- Basic Channel ---");
    
    // FIXME: Create a channel
    // let (tx, rx) = mpsc::channel();
    
    // FIXME: Spawn a thread that sends "Hello from Textio!" through the channel
    
    // FIXME: Receive and print the message
    // let received = rx.recv().unwrap();
    // println!("Received: {}", received);
    
    println!();
}

fn multiple_producers() {
    println!("--- Multiple Producers ---");
    
    // FIXME: Create a channel
    // let (tx, rx) = mpsc::channel();
    
    // FIXME: Spawn 3 threads, each sending a unique message
    // Thread 0: "Message from worker 0"
    // Thread 1: "Message from worker 1"
    // Thread 2: "Message from worker 2"
    // Remember to clone tx for each thread!
    
    // FIXME: Drop the original sender so the receiver iterator terminates
    // drop(tx);
    
    // FIXME: Iterate over received messages and print each
    // for msg in rx {
    //     println!("Got: {}", msg);
    // }
    
    println!();
}

fn message_pipeline() {
    println!("--- Message Pipeline ---");
    
    // Create a two-stage pipeline:
    // Stage 1: Add prefix to messages
    // Stage 2: Add suffix to messages
    
    // FIXME: Create two channels
    // let (tx1, rx1) = mpsc::channel();
    // let (tx2, rx2) = mpsc::channel();
    
    // FIXME: Spawn stage 1 thread
    // - Receives from rx1
    // - Adds prefix "[Textio] " to each message
    // - Sends to tx2
    
    // FIXME: Spawn stage 2 thread
    // - Receives from rx2
    // - Adds suffix " [SENT]" to each message
    // - Prints the final message
    
    // FIXME: Send messages through the pipeline
    // Send: "Hello", "World", "Test"
    
    // FIXME: Drop tx1 to signal end of input
    
    // FIXME: Wait for pipeline to complete (join threads)
    
    println!();
}

#[derive(Debug)]
struct SmsJob {
    id: u32,
    recipient: String,
    message: String,
}

#[derive(Debug)]
struct JobResult {
    id: u32,
    status: String,
}

fn worker_pool() {
    println!("--- Worker Pool ---");
    
    // FIXME: Create a channel for sending jobs
    // let (job_tx, job_rx) = mpsc::channel();
    
    // FIXME: Create a channel for receiving results
    // let (result_tx, result_rx) = mpsc::channel();
    
    // FIXME: Wrap job_rx in Arc<Mutex<>> for sharing among workers
    // use std::sync::{Arc, Mutex};
    // let job_rx = Arc::new(Mutex::new(job_rx));
    
    // FIXME: Spawn 3 worker threads
    // Each worker:
    // 1. Locks job_rx and receives a job
    // 2. Simulates processing (sleep 50ms)
    // 3. Sends JobResult { id, status: "delivered" } through result_tx
    
    // FIXME: Send 5 jobs
    // Jobs: id 1-5, recipient "user@textio.io", message "Test message {id}"
    
    // FIXME: Drop job_tx to signal no more jobs
    
    // FIXME: Collect and print results
    // for result in result_rx {
    //     println!("Job {} status: {}", result.id, result.status);
    // }
    
    println!();
}

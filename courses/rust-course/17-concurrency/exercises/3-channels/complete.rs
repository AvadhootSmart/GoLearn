// Exercise 3: Channels - Complete Solution

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Textio Channels Demo ===\n");
    
    basic_channel();
    multiple_producers();
    message_pipeline();
    worker_pool();
}

fn basic_channel() {
    println!("--- Basic Channel ---");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from Textio!").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
    
    println!();
}

fn multiple_producers() {
    println!("--- Multiple Producers ---");
    
    let (tx, rx) = mpsc::channel();
    
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("Message from worker {}", i)).unwrap();
        });
    }
    
    drop(tx);
    
    for msg in rx {
        println!("Got: {}", msg);
    }
    
    println!();
}

fn message_pipeline() {
    println!("--- Message Pipeline ---");
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    let stage1 = thread::spawn(move || {
        for msg in rx1 {
            let processed = format!("[Textio] {}", msg);
            tx2.send(processed).unwrap();
        }
    });
    
    let stage2 = thread::spawn(move || {
        for msg in rx2 {
            let final_msg = format!("{} [SENT]", msg);
            println!("Final: {}", final_msg);
        }
    });
    
    let messages = vec!["Hello", "World", "Test"];
    for msg in messages {
        tx1.send(msg.to_string()).unwrap();
    }
    
    drop(tx1);
    
    stage1.join().unwrap();
    stage2.join().unwrap();
    
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
    
    let (job_tx, job_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    
    let job_rx = Arc::new(Mutex::new(job_rx));
    
    let mut workers = vec![];
    
    for worker_id in 0..3 {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();
        
        let worker = thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                
                match job {
                    Ok(job) => {
                        let job: SmsJob = job;
                        thread::sleep(Duration::from_millis(50));
                        let result = JobResult {
                            id: job.id,
                            status: "delivered".to_string(),
                        };
                        result_tx.send(result).unwrap();
                        println!("Worker {} processed job {}", worker_id, job.id);
                    }
                    Err(_) => break,
                }
            }
        });
        workers.push(worker);
    }
    
    drop(result_tx);
    
    for id in 1..=5 {
        let job = SmsJob {
            id,
            recipient: "user@textio.io".to_string(),
            message: format!("Test message {}", id),
        };
        job_tx.send(job).unwrap();
    }
    
    drop(job_tx);
    
    for result in result_rx {
        println!("Job {} status: {}", result.id, result.status);
    }
    
    for worker in workers {
        worker.join().unwrap();
    }
    
    println!();
}

// Exercise 4: Send and Sync Traits - Complete Solution

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Textio Send/Sync Demo ===\n");
    
    send_trait_demo();
    sync_trait_demo();
    atomic_counter();
    custom_thread_safe();
    compile_errors_demo();
}

fn send_trait_demo() {
    println!("--- Send Trait ---");
    
    let message = String::from("Hello from main");
    let handle = thread::spawn(move || {
        println!("Thread received: {}", message);
    });
    handle.join().unwrap();
    
    let shared = Arc::new(String::from("Shared data"));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        println!("Thread sees: {}", shared_clone);
    });
    handle.join().unwrap();
    println!("Main still has: {}", shared);
    
    println!();
}

fn sync_trait_demo() {
    println!("--- Sync Trait ---");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
    
    println!();
}

fn atomic_counter() {
    println!("--- Atomic Counter ---");
    
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("Atomic counter: {}", counter.load(Ordering::SeqCst));
    
    println!();
}

struct MessageTracker {
    sent: AtomicU32,
    failed: AtomicU32,
}

impl MessageTracker {
    fn new() -> Self {
        Self {
            sent: AtomicU32::new(0),
            failed: AtomicU32::new(0),
        }
    }
    
    fn record_sent(&self) {
        self.sent.fetch_add(1, Ordering::Relaxed);
    }
    
    fn record_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }
    
    fn get_stats(&self) -> (u32, u32) {
        (
            self.sent.load(Ordering::Relaxed),
            self.failed.load(Ordering::Relaxed),
        )
    }
}

fn custom_thread_safe() {
    println!("--- Custom Thread-Safe Type ---");
    
    let tracker = Arc::new(MessageTracker::new());
    let mut handles = vec![];
    
    for _ in 0..3 {
        let tracker = Arc::clone(&tracker);
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                tracker.record_sent();
            }
        }));
    }
    
    for _ in 0..2 {
        let tracker = Arc::clone(&tracker);
        handles.push(thread::spawn(move || {
            for _ in 0..5 {
                tracker.record_failed();
            }
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    let (sent, failed) = tracker.get_stats();
    println!("Sent: {}, Failed: {}", sent, failed);
    
    println!();
}

fn compile_errors_demo() {
    println!("--- Compile Errors Demo ---");
    
    // Uncomment each section to see the compile error
    
    // Error 1: Rc is not Send
    // let rc = Rc::new(5);
    // thread::spawn(move || {
    //     println!("{}", rc);
    // });
    // Error: `Rc<i32>` cannot be sent between threads safely
    
    // Error 2: RefCell is not Sync
    // let refcell = Arc::new(RefCell::new(5));
    // let refcell2 = Arc::clone(&refcell);
    // thread::spawn(move || {
    //     *refcell2.borrow_mut() = 10;
    // });
    // Error: `RefCell<i32>` cannot be shared between threads safely
    
    // Error 3: Cell is not Sync
    // let cell = Arc::new(Cell::new(5));
    // let cell2 = Arc::clone(&cell);
    // thread::spawn(move || {
    //     cell2.set(10);
    // });
    // Error: `Cell<i32>` cannot be shared between threads safely
    
    println!("Uncomment code in compile_errors_demo() to see errors");
    println!("Use Arc<Mutex<T>> or Arc<Atomic*> instead of Rc/RefCell/Cell");
    println!();
}

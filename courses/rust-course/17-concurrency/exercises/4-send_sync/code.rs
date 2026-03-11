// Exercise 4: Send and Sync Traits
//
// This exercise demonstrates thread safety traits and their implications
// in the context of Textio's concurrent message handling.
//
// TODO: Complete the tasks marked with FIXME
// Some code is intentionally commented out to show compile errors

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Textio Send/Sync Demo ===\n");
    
    // Task 1: Understanding Send
    send_trait_demo();
    
    // Task 2: Understanding Sync
    sync_trait_demo();
    
    // Task 3: Thread-safe counter with atomics
    atomic_counter();
    
    // Task 4: Custom thread-safe type
    custom_thread_safe();
    
    // Task 5: Demonstrating compile errors (uncomment to see)
    compile_errors_demo();
}

fn send_trait_demo() {
    println!("--- Send Trait ---");
    
    // Types that ARE Send can be moved to threads
    
    // FIXME: Create a String and send it to a thread
    // let message = String::from("Hello from main");
    // let handle = thread::spawn(move || {
    //     println!("Thread received: {}", message);
    // });
    // handle.join().unwrap();
    
    // Arc is Send when T: Send
    // FIXME: Create Arc<String> and send to thread
    // let shared = Arc::new(String::from("Shared data"));
    // let shared_clone = Arc::clone(&shared);
    // let handle = thread::spawn(move || {
    //     println!("Thread sees: {}", shared_clone);
    // });
    // handle.join().unwrap();
    // println!("Main still has: {}", shared);
    
    println!();
}

fn sync_trait_demo() {
    println!("--- Sync Trait ---");
    
    // Types that ARE Sync can have references shared across threads
    
    // Mutex<T> is Sync, so &Mutex<T> can be sent to threads
    // FIXME: Create Arc<Mutex<i32>> and share between threads
    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];
    // 
    // for _ in 0..3 {
    //     let counter = Arc::clone(&counter);
    //     handles.push(thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     }));
    // }
    // 
    // for h in handles {
    //     h.join().unwrap();
    // }
    // println!("Final count: {}", *counter.lock().unwrap());
    
    println!();
}

fn atomic_counter() {
    println!("--- Atomic Counter ---");
    
    // FIXME: Create an AtomicU32 counter starting at 0
    // let counter = Arc::new(AtomicU32::new(0));
    
    // FIXME: Spawn 5 threads, each incrementing 100 times
    // Use fetch_add with Ordering::SeqCst
    
    // FIXME: Join all threads
    
    // FIXME: Print final count (should be 500)
    // println!("Atomic counter: {}", counter.load(Ordering::SeqCst));
    
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
    
    // MessageTracker uses atomics, so it's automatically Send + Sync
    
    // FIXME: Create Arc<MessageTracker>
    // let tracker = Arc::new(MessageTracker::new());
    
    // FIXME: Spawn threads that record sent/failed messages
    // Thread 0-2: record 10 sent each
    // Thread 3-4: record 5 failed each
    
    // FIXME: Join all threads
    
    // FIXME: Print final stats
    // let (sent, failed) = tracker.get_stats();
    // println!("Sent: {}, Failed: {}", sent, failed);
    
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

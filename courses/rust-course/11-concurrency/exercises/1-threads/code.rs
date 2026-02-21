// 1. Import thread and Duration
use std::thread;
use std::time::Duration;

fn main() {
    // 2. Spawn a thread
    let handle = thread::spawn(|| {
        for i in 1..4 {
            // print from spawned thread
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 3. Loop in main thread
    for i in 1..4 {
        // print from main thread
        
        // sleep 1 ms
    }

    // 4. Join the spawned thread
    
}

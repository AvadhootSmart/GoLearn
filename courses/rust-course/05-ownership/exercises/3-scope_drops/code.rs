use std::mem;

struct SmsMessage {
    id: u32,
    to: String,
    body: String,
}

impl Drop for SmsMessage {
    fn drop(&mut self) {
        println!("  [DROP] Message {} to '{}': '{}'", self.id, self.to, self.body);
    }
}

struct Connection {
    id: u32,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("  [DROP] Connection {} closed", self.id);
    }
}

struct ConnectionPool {
    name: String,
    connections: Vec<Connection>,
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        println!("  [DROP] Connection pool '{}' shutting down", self.name);
    }
}

struct MessageQueue {
    name: String,
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        println!("  [DROP] Message queue '{}' cleared", self.name);
    }
}

fn main() {
    println!("=== Exercise 1: Basic Drop ===\n");
    
    // TASK 1: Create a simple SmsMessage and observe when it drops
    // Create a message with id=1, to="Alice", body="Hello"
    // Print "Message created"
    // The message should drop automatically at end of scope
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 2: Drop Order ===\n");
    
    // TASK 2: Create multiple messages and observe drop order
    // Create message A (id=1)
    // Create message B (id=2)
    // Create message C (id=3)
    // They should drop in reverse order: C, B, A
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 3: Nested Scopes ===\n");
    
    // TASK 3: Demonstrate nested scope drops
    // Create outer message (id=10)
    // Inside a new block:
    //   Create inner message (id=20)
    //   Print "Inner scope ending"
    // Print "Outer scope continuing"
    // Observe that inner drops before outer
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 4: Explicit Drop ===\n");
    
    // TASK 4: Use std::mem::drop() to drop early
    // Create a message (id=100)
    // Print "Before explicit drop"
    // Call drop(message)
    // Print "After explicit drop"
    // Note: Trying to use message after would be a compile error!
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 5: Vec Drop Order ===\n");
    
    // TASK 5: Observe how Vec elements drop
    // Create a Vec<SmsMessage> with 3 messages (ids 1, 2, 3)
    // Print "Messages in vec"
    // Vec and all elements drop at end of scope
    // Note: Elements drop in order 3, 2, 1 (reverse)
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 6: Complex Nested Structure ===\n");
    
    // TASK 6: Multiple types with Drop
    // Create a ConnectionPool with name "TextioPool"
    //   Add 2 connections (ids 1, 2)
    // Create a MessageQueue with name "OutboundQueue"
    // Observe drop order: Queue, Pool, Connection 2, Connection 1
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 7: Shadowing and Drop ===\n");
    
    // TASK 7: Show that shadowing creates new values
    // Create msg with id=1
    // Shadow it with a new msg with id=2
    // Shadow it with a new msg with id=3
    // At scope end, all three drop in reverse order!
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 8: Early Return Drops ===\n");
    
    // TASK 8: Simulate early return
    // Call process_batch(true) and process_batch(false)
    // The function should drop all values even on early return
    
    process_batch(true);
    process_batch(false);
    
    println!("\n=== Exercise 9: Reassignment Drops Old Value ===\n");
    
    // TASK 9: Show that reassignment drops the old value
    // Create a mutable message (id=1)
    // Print "Created message 1"
    // Reassign it to a new message (id=2)
    // Print "Reassigned to message 2"
    // Old message 1 should drop immediately on reassignment!
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 10: Real Textio Scenario ===\n");
    
    // TASK 10: Full Textio message processing
    // Create a ConnectionPool named "ProductionPool" with 3 connections
    // Create a Vec of 3 SmsMessages (ids 1-3)
    // Print "Processing 3 messages with pool"
    // Everything drops in correct order at end
    
    // YOUR CODE HERE
    
    println!("\n=== End of main ===\n");
}

fn process_batch(should_continue: bool) {
    println!("  Entering process_batch({})", should_continue);
    
    let msg1 = SmsMessage {
        id: 1,
        to: String::from("Alice"),
        body: String::from("Hello"),
    };
    
    let msg2 = SmsMessage {
        id: 2,
        to: String::from("Bob"),
        body: String::from("World"),
    };
    
    if !should_continue {
        println!("  Early return!");
        return;  // Both messages drop here!
    }
    
    println!("  Processing messages normally");
    // Messages drop here on normal path
}

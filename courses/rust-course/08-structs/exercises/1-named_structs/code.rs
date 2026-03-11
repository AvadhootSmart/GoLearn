// Named Structs - Textio SMS Message System
// 
// In this exercise, you'll work with named structs to represent
// SMS messages in the Textio system.

// TODO: Add the Debug derive attribute to this struct
struct Message {
    to: String,
    from: String,
    body: String,
    priority: u8,
    delivered: bool,
}

// TODO: Create a struct called `MessageStats` with these fields:
// - total_sent: u64
// - total_delivered: u64
// - total_failed: u64
// Don't forget to derive Debug!



fn create_message(to: String, from: String, body: String) -> Message {
    // TODO: Use field init shorthand for to, from, and body
    // Set priority to 1 and delivered to false
    Message {
        // Your code here
        priority: 1,
        delivered: false,
    }
}

fn main() {
    // TODO: Create a message using the create_message function
    // Use variables named exactly the same as the struct fields
    let to = String::from("+15550001");
    let from = String::from("+15550002");
    let body = String::from("Hello from Textio!");
    
    // Create msg1 by calling create_message
    

    // TODO: Print msg1 using debug format {:?}
    

    // TODO: Create msg2 using struct update syntax from msg1
    // Change only the body to "Priority message!"
    // Remember: ..msg1 must come last
    

    // TODO: Print msg2 using pretty debug format {:#?}
    

    // TODO: Create msg3 using struct update syntax from msg1
    // Change priority to 5 and delivered to true
    

    // TODO: Print msg3
    

    // TODO: Try to print msg1.to - this should work because we didn't 
    // move the 'to' field when creating msg2
    // Uncomment and verify:
    // println!("msg1.to is still valid: {}", msg1.to);
    

    // TODO: Create an instance of MessageStats
    // Set total_sent to 1000, total_delivered to 950, total_failed to 50
    

    // TODO: Print the stats using debug format
    
}

use std::mem;

struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

fn main() {
    println!("=== Exercise 1: Basic Move Semantics ===\n");
    
    // TASK 1: Demonstrate basic move
    // Create a String called `original` with value "Hello, Textio!"
    // Then move it to a variable called `moved`
    // Print only `moved`
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 2: Move vs Copy ===\n");
    
    // TASK 2a: Show that integers are copied, not moved
    // Create an i32 called `number` with value 42
    // Assign it to another variable called `copy_number`
    // Print BOTH to show they're both valid
    
    // YOUR CODE HERE
    
    // TASK 2b: Show that String is moved
    // Create a String called `text` with value "Textio SMS"
    // Assign it to another variable called `moved_text`
    // Print only `moved_text` (trying to print `text` would error)
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 3: Using Clone ===\n");
    
    // TASK 3: Clone a String to keep both copies
    // Create a String called `original` with value "Original message"
    // Clone it to a variable called `clone`
    // Print BOTH to show they're both valid and independent
    
    // YOUR CODE HERE
    
    // Modify the original and show they're independent
    // YOUR CODE HERE (push " - modified" to original, print both)
    
    println!("\n=== Exercise 4: Struct Ownership ===\n");
    
    // TASK 4: Work with struct ownership
    // Create an SmsMessage struct with:
    //   to: "+1234567890"
    //   from: "+0987654321"  
    //   body: "Hello from Textio!"
    // Move it to another variable called `sent`
    // Print sent.to, sent.from, sent.body
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 5: Memory Analysis ===\n");
    
    // TASK 5: Analyze memory sizes
    // Create a String with a long message (at least 100 chars)
    // Print the size of the String struct (not the data it points to)
    // using std::mem::size_of_val()
    // This shows String is always 24 bytes regardless of content!
    
    // YOUR CODE HERE
    
    // Now create a small String and show it's also 24 bytes
    // YOUR CODE HERE
    
    println!("\n=== Exercise 6: Tuple Move Semantics ===\n");
    
    // TASK 6: Demonstrate partial moves with tuples
    // Create a tuple: (String::from("hello"), 42, String::from("world"))
    // Move just the first element to a new variable
    // Show that you can still access the second (Copy) element
    // Show that you cannot access the first element anymore
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 7: Ownership Transfer Chain ===\n");
    
    // TASK 7: Chain of ownership transfers
    // Create a String called `step1` with value "Step 1"
    // Move it to `step2`
    // Move it to `step3`
    // Move it to `step4`
    // Print only `step4`
    // This shows ownership can be transferred multiple times
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 8: Array of Owned Values ===\n");
    
    // TASK 8: Working with arrays of Strings
    // Create a Vec<String> with three messages
    // Move one message out (using .remove() or indexing with clone)
    // Print the removed message
    // Note: We'll use .remove() which actually removes and returns
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 9: String vs &str ===\n");
    
    // TASK 9: Compare owned vs borrowed strings
    // Create an owned String called `owned` 
    // Create a &str called `borrowed` that references `owned`
    // Print both
    // Show that `owned` is still valid (the borrow is just a reference)
    
    // YOUR CODE HERE
    
    println!("\n=== Exercise 10: Real Textio Scenario ===\n");
    
    // TASK 10: Simulate sending an SMS
    // Create a function above main called `send_sms` that:
    //   - Takes ownership of an SmsMessage
    //   - Prints "Sending SMS from {from} to {to}: {body}"
    //   - Returns a bool indicating success (always true)
    // Call this function with a message you create
    // Show that you CANNOT use the message after sending
    
    // YOUR CODE HERE (function is already defined above)
}

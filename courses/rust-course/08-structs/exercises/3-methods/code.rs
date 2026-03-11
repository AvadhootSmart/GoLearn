// Methods - Textio Message Implementation
// 
// Implement methods for the Message struct using
// &self, &mut self, and self parameters.

#[derive(Debug, Clone)]
struct Message {
    to: String,
    from: String,
    body: String,
    delivered: bool,
    attempts: u8,
}

// TODO: Create an impl block for Message
// Add these methods:
// - is_long(&self) -> bool - returns true if body > 160 chars
// - recipient(&self) -> &str - returns a reference to 'to'
// - sender(&self) -> &str - returns a reference to 'from'
// - char_count(&self) -> usize - returns body length



// TODO: Create another impl block for Message (demonstrating multiple impl blocks)
// Add these methods:
// - mark_delivered(&mut self) - sets delivered to true
// - increment_attempts(&mut self) - increases attempts by 1
// - append(&mut self, text: &str) - appends text to body
// - reset_attempts(&mut self) - sets attempts to 0



fn main() {
    let mut msg = Message {
        to: String::from("+15550001"),
        from: String::from("+15550002"),
        body: String::from("Hello, this is a test message from Textio!"),
        delivered: false,
        attempts: 0,
    };

    // TODO: Call is_long() and print the result
    

    // TODO: Get the recipient using recipient()
    

    // TODO: Get the character count
    

    // TODO: Increment attempts twice
    

    // TODO: Print attempts
    

    // TODO: Append " More text." to the body
    

    // TODO: Check if message is now long
    

    // TODO: Mark as delivered
    

    // TODO: Print the delivered status
    

    // TODO: Print the full message using debug format
    
}

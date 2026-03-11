// Exercise: Basic Functions
// Complete the functions below as described in the comments

fn main() {
    // Call the welcome function
    
    // Call validate_length with a 50-character message and 160-char limit
    // Hint: Create a string with 50 characters
    
    // Call format_message and print the result
    // From: "+15551234567", To: "+15559876543", Body: "Hello from Textio!"
    
    // Call calculate_cost for 1000 messages at $0.0075 each
    // Print the result with 2 decimal places
}

// Create a function called `welcome` that prints:
// "Welcome to Textio SMS API!"
// This function takes no parameters and returns nothing


// Create a function called `validate_length` that takes:
// - message: a string slice
// - max_chars: a usize
// It should print: "Message length: X chars (limit: Y) - Valid!"
// or "Message length: X chars (limit: Y) - Too long!"
// depending on whether the message fits within the limit


// Create a function called `format_message` that takes:
// - sender: a string slice
// - recipient: a string slice  
// - body: a string slice
// It should return a String formatted as:
// "From: +15551234567
//  To: +15559876543
//  Message: Hello from Textio!"


// Create a function called `calculate_cost` that takes:
// - message_count: a u32
// - rate_per_sms: an f64
// It should return the total cost as f64

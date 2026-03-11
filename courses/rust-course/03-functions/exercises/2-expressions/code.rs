// Exercise: Expressions
// Complete the functions using expression-based programming

fn main() {
    // Test classify_message_length
    println!("Length 50: {}", classify_message_length(50));
    println!("Length 100: {}", classify_message_length(100));
    println!("Length 150: {}", classify_message_length(150));
    
    // Test calculate_segment_cost
    println!("5 segments at $0.05: ${:.2}", calculate_segment_cost(5, 0.05));
    println!("15 segments at $0.05: ${:.2}", calculate_segment_cost(15, 0.05));
    
    // Test format_delivery_report
    println!("{}", format_delivery_report(12345, true, 1));
    println!("{}", format_delivery_report(67890, false, 3));
    
    // Test get_rate_tier
    println!("500 messages: {}", get_rate_tier(500));
    println!("5000 messages: {}", get_rate_tier(5000));
    println!("50000 messages: {}", get_rate_tier(50000));
    println!("500000 messages: {}", get_rate_tier(500000));
}

// Create classify_message_length using a match expression
// Returns "short" for 0-70 chars
// Returns "medium" for 71-134 chars
// Returns "long" for 135+ chars
fn classify_message_length(len: usize) -> &'static str {
    // Use match here - remember it's an expression!
}


// Create calculate_segment_cost using block expressions
// Apply 10% discount for 10+ segments
// Return the final cost WITHOUT using the `return` keyword
fn calculate_segment_cost(segments: u32, rate: f64) -> f64 {
    // Use a block expression for the calculation
    // Remember: no semicolon on the final expression!
}


// Create format_delivery_report using if expressions
// Format: "Message #X: STATUS (Y attempts)"
// STATUS should be "DELIVERED" if success is true, "FAILED" otherwise
fn format_delivery_report(id: u64, success: bool, attempts: u8) -> String {
    // Use if as an expression to get the status string
    // Use a block to build the final string
}


// Create get_rate_tier using a match expression
// "tier1" for 0-1000 messages
// "tier2" for 1001-10000 messages  
// "tier3" for 10001-100000 messages
// "enterprise" for 100001+ messages
fn get_rate_tier(monthly_volume: u32) -> &'static str {
    // Use match with range patterns
}

// Exercise: Function Parameters
// Complete the functions with proper parameter handling

fn main() {
    // Test format_phone_number
    println!("{}", format_phone_number("1", "5551234567"));
    println!("{}", format_phone_number("44", "7911123456"));
    
    // Test calculate_message_cost
    let cost1 = calculate_message_cost(2, 0.05, false, false);
    println!("Domestic regular 2 segments: ${:.2}", cost1);
    
    let cost2 = calculate_message_cost(2, 0.05, true, false);
    println!("International regular 2 segments: ${:.2}", cost2);
    
    let cost3 = calculate_message_cost(2, 0.05, false, true);
    println!("Domestic priority 2 segments: ${:.2}", cost3);
    
    let cost4 = calculate_message_cost(2, 0.05, true, true);
    println!("International priority 2 segments: ${:.2}", cost4);
    
    // Test truncate_message
    let long_msg = "This is a very long message that needs to be truncated";
    println!("Original ({} chars): {}", long_msg.len(), long_msg);
    println!("Truncated to 20: {}", truncate_message(long_msg, 20));
    println!("Truncated to 100: {}", truncate_message(long_msg, 100));
    
    // Test validate_sms_request
    match validate_sms_request("+15551234567", "Textio", "Hello!", 3) {
        Ok(segments) => println!("Valid! Segments needed: {}", segments),
        Err(e) => println!("Error: {}", e),
    }
    
    match validate_sms_request("", "Textio", "Hello!", 3) {
        Ok(segments) => println!("Valid! Segments needed: {}", segments),
        Err(e) => println!("Error: {}", e),
    }
    
    match validate_sms_request("+15551234567", "Textio", "This is a very long message that will require multiple segments to send properly", 1) {
        Ok(segments) => println!("Valid! Segments needed: {}", segments),
        Err(e) => println!("Error: {}", e),
    }
}

// Create format_phone_number
// Parameters: country_code (e.g., "1" for US), number (local number)
// Return format: "+{code}{number}" (ensure exactly one '+' at start)
fn format_phone_number(country_code: &str, number: &str) -> String {
    
}


// Create calculate_message_cost
// Parameters: segments, rate, is_international, is_priority
// International: multiply base by 1.5
// Priority: add $0.02 per segment
fn calculate_message_cost(segments: u32, rate: f64, is_international: bool, is_priority: bool) -> f64 {
    
}


// Create truncate_message
// Parameters: message string slice, max_length
// Return a slice that fits within max_length
// Hint: use .chars() for proper Unicode handling
fn truncate_message(message: &str, max_length: usize) -> &str {
    
}


// Create validate_sms_request
// Parameters: recipient, sender, body, max_segments
// Calculate segments: (body length + 152) / 153
// Validate:
//   - recipient not empty
//   - sender not empty  
//   - segments_needed <= max_segments
// Return Ok(segments_needed) or Err(message)
fn validate_sms_request(recipient: &str, sender: &str, body: &str, max_segments: u8) -> Result<u8, String> {
    
}

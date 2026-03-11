// Textio SMS API - String Slices Exercise
// Complete the functions below to process SMS messages using string slices

fn main() {
    println!("=== Textio String Slices Exercise ===\n");
    
    // Test 1: Extract area code
    let phone1 = "+1-555-123-4567";
    let phone2 = "+44-20-7946-0958";
    println!("Test 1: Extract Area Code");
    println!("Phone: {} -> Area: {}", phone1, extract_area_code(phone1));
    println!("Phone: {} -> Area: {}", phone2, extract_area_code(phone2));
    println!();
    
    // Test 2: Get message preview
    let message = "Hello! This is a long message from Textio SMS API.";
    println!("Test 2: Message Preview");
    println!("Full: {}", message);
    println!("Preview (20 chars): {}", get_message_preview(message, 20));
    println!("Preview (10 chars): {}", get_message_preview(message, 10));
    println!("Preview (100 chars): {}", get_message_preview(message, 100));
    println!();
    
    // Test 3: Find keyword
    let sms = "URGENT: Your package will arrive tomorrow at 3pm";
    println!("Test 3: Find Keyword");
    println!("Message: {}", sms);
    println!("'URGENT' at: {:?}", find_keyword(sms, "URGENT"));
    println!("'package' at: {:?}", find_keyword(sms, "package"));
    println!("'missing' at: {:?}", find_keyword(sms, "missing"));
    println!();
    
    // Test 4: Safe slicing
    let text = "Hello, Textio!";
    println!("Test 4: Safe Slicing");
    println!("Text: {}", text);
    println!("Safe slice [0..5]: {:?}", safe_slice(text, 0, 5));
    println!("Safe slice [7..13]: {:?}", safe_slice(text, 7, 13));
    println!("Safe slice [0..100]: {:?}", safe_slice(text, 0, 100));
    println!("Safe slice [5..3]: {:?}", safe_slice(text, 5, 3));
    println!();
    
    // Test 5: Demonstrate slice flexibility
    demonstrate_slice_flexibility();
}

/// Extract the area code from a phone number
/// Format: +CC-AREA-XXX-XXXX (e.g., "+1-555-123-4567")
/// Returns the AREA portion as a string slice
fn extract_area_code(phone: &str) -> &str {
    // TODO: Extract the area code between the first and second hyphens
    // Hint: Use find() to locate the hyphens, then slice
    
    ""
}

/// Get a preview of the message (first n characters)
/// If the message is shorter than max_len, return the entire message
fn get_message_preview(message: &str, max_len: usize) -> &str {
    // TODO: Return a slice containing at most max_len characters
    // Be careful with the length!
    
    ""
}

/// Find the byte position of a keyword in the message
/// Returns Some(index) if found, None if not found
fn find_keyword(message: &str, keyword: &str) -> Option<usize> {
    // TODO: Find where the keyword starts in the message
    // Hint: Use the find() method on &str
    
    None
}

/// Safely slice a string without panicking
/// Returns Some(slice) if valid, None if the range is invalid
fn safe_slice(text: &str, start: usize, end: usize) -> Option<&str> {
    // TODO: Return a slice from start to end, or None if invalid
    // Hint: Use the get() method for safe slicing
    
    None
}

/// Demonstrates how slices work with different string types
fn demonstrate_slice_flexibility() {
    println!("Test 5: Slice Flexibility");
    
    // String literals are &str
    let literal = "Hello from literal";
    print_slice_type(literal);
    
    // String can be sliced
    let owned = String::from("Hello from String");
    print_slice_type(&owned);
    
    // Using as_str()
    let another = String::from("Using as_str");
    print_slice_type(another.as_str());
    
    // Partial slices
    let full = "Hello, World!";
    let partial = &full[0..5];
    println!("Partial slice: {}", partial);
}

/// A function that accepts any string slice
fn print_slice_type(s: &str) {
    println!("Received slice: {} (length: {})", s, s.len());
}

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

fn extract_area_code(phone: &str) -> &str {
    if let Some(first_hyphen) = phone.find('-') {
        let after_first = &phone[first_hyphen + 1..];
        if let Some(second_hyphen) = after_first.find('-') {
            return &after_first[..second_hyphen];
        }
    }
    ""
}

fn get_message_preview(message: &str, max_len: usize) -> &str {
    if message.len() <= max_len {
        &message[..]
    } else {
        &message[..max_len]
    }
}

fn find_keyword(message: &str, keyword: &str) -> Option<usize> {
    message.find(keyword)
}

fn safe_slice(text: &str, start: usize, end: usize) -> Option<&str> {
    if start > end {
        return None;
    }
    text.get(start..end)
}

fn demonstrate_slice_flexibility() {
    println!("Test 5: Slice Flexibility");
    
    let literal = "Hello from literal";
    print_slice_type(literal);
    
    let owned = String::from("Hello from String");
    print_slice_type(&owned);
    
    let another = String::from("Using as_str");
    print_slice_type(another.as_str());
    
    let full = "Hello, World!";
    let partial = &full[0..5];
    println!("Partial slice: {}", partial);
}

fn print_slice_type(s: &str) {
    println!("Received slice: {} (length: {})", s, s.len());
}

fn main() {
    println!("{}", format_phone_number("1", "5551234567"));
    println!("{}", format_phone_number("44", "7911123456"));
    
    let cost1 = calculate_message_cost(2, 0.05, false, false);
    println!("Domestic regular 2 segments: ${:.2}", cost1);
    
    let cost2 = calculate_message_cost(2, 0.05, true, false);
    println!("International regular 2 segments: ${:.2}", cost2);
    
    let cost3 = calculate_message_cost(2, 0.05, false, true);
    println!("Domestic priority 2 segments: ${:.2}", cost3);
    
    let cost4 = calculate_message_cost(2, 0.05, true, true);
    println!("International priority 2 segments: ${:.2}", cost4);
    
    let long_msg = "This is a very long message that needs to be truncated";
    println!("Original ({} chars): {}", long_msg.len(), long_msg);
    println!("Truncated to 20: {}", truncate_message(long_msg, 20));
    println!("Truncated to 100: {}", truncate_message(long_msg, 100));
    
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

fn format_phone_number(country_code: &str, number: &str) -> String {
    format!("+{}{}", country_code, number)
}

fn calculate_message_cost(segments: u32, rate: f64, is_international: bool, is_priority: bool) -> f64 {
    let base = segments as f64 * rate;
    let with_international = if is_international { base * 1.5 } else { base };
    let priority_fee = if is_priority { segments as f64 * 0.02 } else { 0.0 };
    with_international + priority_fee
}

fn truncate_message(message: &str, max_length: usize) -> &str {
    if message.len() <= max_length {
        message
    } else {
        &message[..max_length]
    }
}

fn validate_sms_request(recipient: &str, sender: &str, body: &str, max_segments: u8) -> Result<u8, String> {
    if recipient.is_empty() {
        return Err("Recipient cannot be empty".to_string());
    }
    if sender.is_empty() {
        return Err("Sender cannot be empty".to_string());
    }
    
    let segments_needed = ((body.len() + 152) / 153) as u8;
    
    if segments_needed > max_segments {
        return Err(format!("Message requires {} segments, but maximum is {}", segments_needed, max_segments));
    }
    
    Ok(segments_needed)
}

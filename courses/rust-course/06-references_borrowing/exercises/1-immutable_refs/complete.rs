// Exercise 1: Immutable References (&T) - Complete Solution

fn main() {
    println!("=== Textio Message Analysis ===\n");
    
    // Part 1: Creating References
    // ===========================
    
    let message = String::from("Welcome to Textio! Your verification code is 789123.");
    
    let message_ref = &message;
    
    println!("Original: {}", message);
    println!("Reference: {}", message_ref);
    
    println!();
    
    // Part 2: Multiple References
    // ===========================
    
    let ref1 = &message;
    let ref2 = &message;
    let ref3 = &message;
    
    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
    println!("ref3: {}", ref3);
    
    println!();
    
    // Part 3: Passing References to Functions
    // =======================================
    
    let length = analyze_message_length(&message);
    println!("Message length: {} characters", length);
    
    let word_count = count_words(&message);
    println!("Word count: {}", word_count);
    
    let code = extract_verification_code(&message);
    println!("Verification code: {}", code);
    
    println!();
    
    // Part 4: Deref Coercion
    // =====================
    
    let phone_number = String::from("+15551234567");
    
    let is_valid = validate_phone_number(&phone_number);
    println!("Phone valid: {}", is_valid);
    
    println!();
    
    // Part 5: References with Structs
    // ==============================
    
    let sms = SmsMessage {
        to: String::from("+15551112222"),
        from: String::from("+15553334444"),
        body: String::from("Your order has shipped!"),
    };
    
    print_message_summary(&sms);
    
    println!("Still accessible: {}", sms.body);
    
    println!();
    
    // Part 6: Iterating with References
    // =================================
    
    let messages = vec![
        String::from("Hello, World!"),
        String::from("Textio is great!"),
        String::from("Rust is awesome!"),
    ];
    
    println!("All messages:");
    for msg in &messages {
        println!("  - {}", msg);
    }
    
    println!("Total messages: {}", messages.len());
    
    println!();
    
    // Part 7: References Keep Owner Valid
    // ===================================
    
    let original = String::from("I still exist!");
    let borrowed = &original;
    
    println!("Original: {}", original);
    println!("Borrowed: {}", borrowed);
}

struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

fn analyze_message_length(msg: &String) -> usize {
    msg.len()
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn extract_verification_code(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for i in 0..words.len() {
        if words[i] == "code" && i + 2 < words.len() {
            return words[i + 2].trim_end_matches('.');
        }
    }
    
    "not found"
}

fn validate_phone_number(phone: &str) -> bool {
    phone.starts_with('+') && phone.len() >= 10 && phone.len() <= 15
}

fn print_message_summary(msg: &SmsMessage) {
    println!("SMS Summary:");
    println!("  From: {}", msg.from);
    println!("  To: {}", msg.to);
    println!("  Body: {}", msg.body);
    println!("  Body length: {} characters", msg.body.len());
}

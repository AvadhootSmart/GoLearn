// Exercise 1: Immutable References (&T)
// 
// In this exercise, you'll learn to create and use immutable references.
// Follow the TODO comments to complete the code.
//
// Key concepts:
// - Creating references with &
// - Borrowing vs ownership
// - Multiple immutable references
// - References in function signatures

fn main() {
    println!("=== Textio Message Analysis ===\n");
    
    // Part 1: Creating References
    // ===========================
    
    let message = String::from("Welcome to Textio! Your verification code is 789123.");
    
    // TODO: Create an immutable reference to `message` called `message_ref`
    // let message_ref = ???;
    
    // TODO: Print both the original and the reference
    // Hint: Both should print the same content
    // println!("Original: {}", ???);
    // println!("Reference: {}", ???);
    
    println!();
    
    // Part 2: Multiple References
    // ===========================
    
    // TODO: Create three different references to `message`
    // let ref1 = ???;
    // let ref2 = ???;
    // let ref3 = ???;
    
    // TODO: Print all three references
    // println!("ref1: {}", ???);
    // println!("ref2: {}", ???);
    // println!("ref3: {}", ???);
    
    println!();
    
    // Part 3: Passing References to Functions
    // =======================================
    
    // TODO: Call analyze_message_length with a reference to message
    // Hint: Don't forget the & when passing
    // let length = ???;
    // println!("Message length: {} characters", length);
    
    // TODO: Call count_words with a reference to message
    // let word_count = ???;
    // println!("Word count: {}", word_count);
    
    // TODO: Call extract_verification_code with a reference to message
    // let code = ???;
    // println!("Verification code: {}", code);
    
    println!();
    
    // Part 4: Deref Coercion
    // =====================
    
    let phone_number = String::from("+15551234567");
    
    // TODO: Call validate_phone_number with the phone_number
    // Note: The function takes &str, but you have a String
    // Hint: Deref coercion handles this automatically!
    // let is_valid = ???;
    // println!("Phone valid: {}", is_valid);
    
    println!();
    
    // Part 5: References with Structs
    // ==============================
    
    let sms = SmsMessage {
        to: String::from("+15551112222"),
        from: String::from("+15553334444"),
        body: String::from("Your order has shipped!"),
    };
    
    // TODO: Call print_message_summary with a reference to sms
    // Hint: Borrow the struct, don't move it
    // ???;
    
    // TODO: After printing the summary, prove sms is still valid
    // by printing its body field
    // println!("Still accessible: {}", ???);
    
    println!();
    
    // Part 6: Iterating with References
    // =================================
    
    let messages = vec![
        String::from("Hello, World!"),
        String::from("Textio is great!"),
        String::from("Rust is awesome!"),
    ];
    
    // TODO: Iterate over messages using a reference
    // Hint: Use &messages in the for loop
    // println!("All messages:");
    // for msg in ??? {
    //     println!("  - {}", msg);
    // }
    
    // TODO: Prove messages is still owned by printing its length
    // println!("Total messages: {}", ???);
    
    println!();
    
    // Part 7: References Keep Owner Valid
    // ===================================
    
    let original = String::from("I still exist!");
    let borrowed = &original;
    
    // TODO: Print both original and borrowed to show the original
    // is still valid even after borrowing
    // println!("Original: {}", ???);
    // println!("Borrowed: {}", ???);
    
    // This is the key difference from moving!
    // If we had done `let moved = original;`, we couldn't use original here.
}

// Struct for Part 5
struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

// Function for Part 3: Takes an immutable reference to String
// Returns the length without taking ownership
fn analyze_message_length(msg: &String) -> usize {
    msg.len()
}

// Function for Part 3: Takes a string slice (more flexible)
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// Function for Part 3: Extracts verification code from message
// Returns "not found" if no code is present
fn extract_verification_code(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for i in 0..words.len() {
        if words[i] == "code" && i + 2 < words.len() {
            return words[i + 2].trim_end_matches('.');
        }
    }
    
    "not found"
}

// Function for Part 4: Takes &str (string slice)
// Works with both &String and &str due to deref coercion
fn validate_phone_number(phone: &str) -> bool {
    phone.starts_with('+') && phone.len() >= 10 && phone.len() <= 15
}

// Function for Part 5: Borrows the SmsMessage struct
fn print_message_summary(msg: &SmsMessage) {
    println!("SMS Summary:");
    println!("  From: {}", msg.from);
    println!("  To: {}", msg.to);
    println!("  Body: {}", msg.body);
    println!("  Body length: {} characters", msg.body.len());
}

use std::mem;

struct SmsMessage {
    to: String,
    from: String,
    body: String,
}

fn send_sms(message: SmsMessage) -> bool {
    println!("Sending SMS from {} to {}: {}", message.from, message.to, message.body);
    true
}

fn main() {
    println!("=== Exercise 1: Basic Move Semantics ===\n");
    
    let original = String::from("Hello, Textio!");
    let moved = original;
    println!("Moved value: {}", moved);
    
    println!("\n=== Exercise 2: Move vs Copy ===\n");
    
    let number = 42;
    let copy_number = number;
    println!("Original number: {}", number);
    println!("Copy number: {}", copy_number);
    
    let text = String::from("Textio SMS");
    let moved_text = text;
    println!("Moved text: {}", moved_text);
    
    println!("\n=== Exercise 3: Using Clone ===\n");
    
    let mut original = String::from("Original message");
    let clone = original.clone();
    println!("Original: {}", original);
    println!("Clone: {}", clone);
    
    original.push_str(" - modified");
    println!("After modification:");
    println!("Original: {}", original);
    println!("Clone: {}", clone);
    
    println!("\n=== Exercise 4: Struct Ownership ===\n");
    
    let message = SmsMessage {
        to: String::from("+1234567890"),
        from: String::from("+0987654321"),
        body: String::from("Hello from Textio!"),
    };
    let sent = message;
    println!("Sent to: {}", sent.to);
    println!("Sent from: {}", sent.from);
    println!("Body: {}", sent.body);
    
    println!("\n=== Exercise 5: Memory Analysis ===\n");
    
    let long_string = String::from("This is a very long message that contains at least one hundred characters to demonstrate that the String struct size remains constant regardless of content length!");
    println!("Long string length: {} characters", long_string.len());
    println!("String struct size: {} bytes", mem::size_of_val(&long_string));
    
    let small_string = String::from("Hi");
    println!("Small string length: {} characters", small_string.len());
    println!("String struct size: {} bytes", mem::size_of_val(&small_string));
    
    println!("\n=== Exercise 6: Tuple Move Semantics ===\n");
    
    let tuple = (String::from("hello"), 42, String::from("world"));
    let first = tuple.0;
    println!("First element (moved): {}", first);
    println!("Second element (copied): {}", tuple.1);
    println!("Third element (still accessible): {}", tuple.2);
    
    println!("\n=== Exercise 7: Ownership Transfer Chain ===\n");
    
    let step1 = String::from("Step 1");
    let step2 = step1;
    let step3 = step2;
    let step4 = step3;
    println!("Final step: {}", step4);
    
    println!("\n=== Exercise 8: Array of Owned Values ===\n");
    
    let mut messages = vec![
        String::from("First message"),
        String::from("Second message"),
        String::from("Third message"),
    ];
    println!("Original messages: {:?}", messages);
    
    let removed = messages.remove(1);
    println!("Removed message: {}", removed);
    println!("Remaining messages: {:?}", messages);
    
    println!("\n=== Exercise 9: String vs &str ===\n");
    
    let owned = String::from("I own this data");
    let borrowed: &str = &owned;
    println!("Owned String: {}", owned);
    println!("Borrowed &str: {}", borrowed);
    println!("Both valid - borrow doesn't take ownership!");
    
    println!("\n=== Exercise 10: Real Textio Scenario ===\n");
    
    let sms = SmsMessage {
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: String::from("Your verification code is 12345"),
    };
    
    let success = send_sms(sms);
    println!("SMS sent successfully: {}", success);
}

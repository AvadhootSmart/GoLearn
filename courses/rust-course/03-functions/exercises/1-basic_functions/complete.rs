fn main() {
    welcome();
    
    let test_message = "This is exactly a fifty character test message!!";
    validate_length(test_message, 160);
    
    let formatted = format_message("+15551234567", "+15559876543", "Hello from Textio!");
    println!("{}", formatted);
    
    let cost = calculate_cost(1000, 0.0075);
    println!("Total cost for 1000 messages: ${:.2}", cost);
}

fn welcome() {
    println!("Welcome to Textio SMS API!");
}

fn validate_length(message: &str, max_chars: usize) {
    let len = message.len();
    if len <= max_chars {
        println!("Message length: {} chars (limit: {}) - Valid!", len, max_chars);
    } else {
        println!("Message length: {} chars (limit: {}) - Too long!", len, max_chars);
    }
}

fn format_message(sender: &str, recipient: &str, body: &str) -> String {
    format!("From: {}\nTo: {}\nMessage: {}", sender, recipient, body)
}

fn calculate_cost(message_count: u32, rate_per_sms: f64) -> f64 {
    message_count as f64 * rate_per_sms
}

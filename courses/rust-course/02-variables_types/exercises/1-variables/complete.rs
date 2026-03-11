// Exercise 1: Variable Declaration - Complete Solution

fn main() {
    // ============================================
    // PART 1: SMS Message Variables
    // ============================================
    
    // Type inference - Rust figures out these are &str
    let sender_phone = "+1-555-0100";
    let recipient_phone = "+1-555-0200";
    let message_content = "Your verification code is 42";
    
    // Explicit type annotation - we specify i32
    let character_count: i32 = 28;
    
    // Type inference - Rust figures out this is bool
    let is_delivered = false;
    
    // ============================================
    // PART 2: API Configuration Variables
    // ============================================
    
    // Type inference for string
    let api_endpoint = "https://api.textio.io/v2/send";
    
    // Explicit types for numeric config values
    let max_length: u16 = 160;
    let timeout_seconds: u32 = 30;
    
    // ============================================
    // PART 3: Print Variables
    // ============================================
    
    // Print SMS message variables
    println!("Sender: {}", sender_phone);
    println!("Recipient: {}", recipient_phone);
    println!("Message: {}", message_content);
    println!("Characters: {}", character_count);
    println!("Delivered: {}", is_delivered);
    
    // Print API config variables
    println!("API Endpoint: {}", api_endpoint);
    println!("Max Length: {}", max_length);
    println!("Timeout: {}s", timeout_seconds);
    
    // Debug printing with dbg! macro
    dbg!(character_count);
    dbg!(max_length);
}

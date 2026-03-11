// Exercise 2: Mutability - Complete Solution

fn main() {
    // ============================================
    // PART 1: Immutable Configuration
    // ============================================
    
    // Immutable - API version never changes
    let api_version = "v2.1";
    
    // ============================================
    // PART 2: Mutable State Variables
    // ============================================
    
    // Mutable - these values need to change
    let mut messages_sent = 0;
    let mut delivery_successes = 0;
    let mut account_balance: f64 = 100.0;
    let mut current_status = "idle";
    
    // ============================================
    // PART 3: Simulate Message Sending
    // ============================================
    
    // Print initial state
    println!("=== Initial State ===");
    println!("API Version: {}", api_version);
    println!("Messages Sent: {}", messages_sent);
    println!("Balance: ${}", account_balance);
    println!("Status: {}", current_status);
    println!();
    
    // Simulate sending Message 1
    println!("=== Message 1 ===");
    current_status = "sending";
    println!("Status: {}", current_status);
    messages_sent = messages_sent + 1;
    delivery_successes = delivery_successes + 1;
    account_balance = account_balance - 0.05;
    current_status = "sent";
    println!("Messages Sent: {}", messages_sent);
    println!("Balance: ${:.2}", account_balance);
    println!("Status: {}", current_status);
    current_status = "idle";
    println!("Status: {}", current_status);
    println!();
    
    // Simulate sending Message 2
    println!("=== Message 2 ===");
    current_status = "sending";
    println!("Status: {}", current_status);
    messages_sent = messages_sent + 1;
    delivery_successes = delivery_successes + 1;
    account_balance = account_balance - 0.05;
    current_status = "sent";
    println!("Messages Sent: {}", messages_sent);
    println!("Balance: ${:.2}", account_balance);
    println!("Status: {}", current_status);
    current_status = "idle";
    println!("Status: {}", current_status);
    println!();
    
    // Simulate sending Message 3
    println!("=== Message 3 ===");
    current_status = "sending";
    println!("Status: {}", current_status);
    messages_sent = messages_sent + 1;
    delivery_successes = delivery_successes + 1;
    account_balance = account_balance - 0.05;
    current_status = "sent";
    println!("Messages Sent: {}", messages_sent);
    println!("Balance: ${:.2}", account_balance);
    println!("Status: {}", current_status);
    current_status = "idle";
    println!("Status: {}", current_status);
    println!();
    
    // ============================================
    // PART 4: Final Summary
    // ============================================
    
    println!("=== Final Summary ===");
    println!("API Version: {} (unchanged)", api_version);
    println!("Total Messages Sent: {}", messages_sent);
    println!("Successful Deliveries: {}", delivery_successes);
    println!("Final Balance: ${:.2}", account_balance);
    println!("Cost per Message: $0.05");
    println!("Total Cost: ${:.2}", messages_sent as f64 * 0.05);
}

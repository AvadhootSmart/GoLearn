// Exercise 2: Mutability
//
// TODO: Create a message tracking system that demonstrates
// the difference between immutable and mutable variables

fn main() {
    // ============================================
    // PART 1: Immutable Configuration
    // ============================================
    
    // TODO: Create an IMMUTABLE variable for API version
    // This should never change throughout the program
    // Value: "v2.1"
    
    
    // ============================================
    // PART 2: Mutable State Variables
    // ============================================
    
    // TODO: Create MUTABLE variable for messages sent
    // Value: 0 (will be incremented)
    
    
    // TODO: Create MUTABLE variable for delivery successes
    // Value: 0 (will be incremented)
    
    
    // TODO: Create MUTABLE variable for account balance
    // Value: 100.0 (f64, will decrease)
    
    
    // TODO: Create MUTABLE variable for current status
    // Value: "idle" (will change through states)
    
    
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
    
    // TODO: Simulate sending Message 1
    // 1. Set status to "sending"
    // 2. Increment messages_sent
    // 3. Increment delivery_successes
    // 4. Subtract 0.05 from balance
    // 5. Set status to "sent"
    // 6. Print the state
    // 7. Set status back to "idle"
    // 8. Print the state
    
    
    // TODO: Simulate sending Message 2
    // Same steps as Message 1
    
    
    // TODO: Simulate sending Message 3
    // Same steps as Message 1
    
    
    // ============================================
    // PART 4: Final Summary
    // ============================================
    
    // TODO: Print final summary showing:
    // - Total messages sent
    // - Total successful deliveries
    // - Final account balance
    // - Cost per message (0.05)
    // - Total cost (messages_sent * 0.05)
    
}

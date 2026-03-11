// Exercise 3: Variable Shadowing - Complete Solution

fn main() {
    // ============================================
    // PART 1: Type Transformation with Shadowing
    // ============================================
    
    // Start with raw message
    let message = "  Hello, Textio User!  ";
    println!("Raw message: '{}'", message);
    
    // Shadow with trimmed version
    let message = message.trim();
    println!("Trimmed: '{}'", message);
    
    // Shadow with uppercase version
    let message = message.to_uppercase();
    println!("Uppercase: '{}'", message);
    
    // Shadow with character count (type changes from String to i32)
    let message: i32 = message.len() as i32;
    println!("Character count: {}", message);
    
    println!();
    
    // ============================================
    // PART 2: Scope-Based Shadowing
    // ============================================
    
    let status = "idle";
    println!("Outer status: {}", status);
    
    {
        // This shadows the outer 'status'
        let status = "processing";
        println!("Inner status: {}", status);
        
        {
            // Nested shadowing
            let status = "sending";
            println!("Nested status: {}", status);
        }
        
        println!("Inner status again: {}", status);
    }
    
    // Original status is restored
    println!("Outer status restored: {}", status);
    
    println!();
    
    // ============================================
    // PART 3: Shadowing vs Mutation
    // ============================================
    
    // SHADOWING - Type change is allowed
    let data = "42";
    println!("Data as string: '{}'", data);
    
    let data: i32 = data.parse().unwrap();
    println!("Data as integer: {}", data);
    
    let data = data * 2;
    println!("Data doubled: {}", data);
    
    println!();
    
    // MUTATION - Same type only
    let mut counter = 0;
    println!("Counter start: {}", counter);
    
    counter = counter + 1;
    println!("Counter +1: {}", counter);
    
    counter = counter + 1;
    println!("Counter +1: {}", counter);
    
    // counter = "text";  // This would ERROR - can't change type with mutation
    
    println!();
    
    // ============================================
    // PART 4: Practical Example - Phone Processing
    // ============================================
    
    println!("=== Phone Number Processing ===");
    
    let phone = "+1-555-123-4567";
    println!("Original: {}", phone);
    
    // Remove country code
    let phone = phone.replace("+1-", "");
    println!("Without country code: {}", phone);
    
    // Remove dashes
    let phone = phone.replace("-", "");
    println!("Without dashes: {}", phone);
    
    // Convert to internal format
    let phone: i64 = phone.parse().unwrap();
    println!("As number: {}", phone);
    
    // Format for display
    let phone = format!("+1 ({}) {}-{}", 
        phone / 10000000 % 1000,
        phone / 10000 % 1000,
        phone % 10000
    );
    println!("Formatted: {}", phone);
}

// Exercise 3: Variable Shadowing
//
// TODO: Demonstrate shadowing through a message processing pipeline

fn main() {
    // ============================================
    // PART 1: Type Transformation with Shadowing
    // ============================================
    
    // TODO: Create a raw message variable
    // Value: "  Hello, Textio User!  " (note the spaces)
    // Print it with: println!("Raw message: '{}'", message);
    
    
    // TODO: Shadow 'message' with trimmed version
    // Use: message.trim()
    // Print: println!("Trimmed: '{}'", message);
    
    
    // TODO: Shadow 'message' with uppercase version
    // Use: message.to_uppercase()
    // Print: println!("Uppercase: '{}'", message);
    
    
    // TODO: Shadow 'message' with character count (i32)
    // Use: message.len() as i32
    // Print: println!("Character count: {}", message);
    
    
    // ============================================
    // PART 2: Scope-Based Shadowing
    // ============================================
    
    // TODO: Create a status variable
    // Value: "idle"
    // Print: println!("Outer status: {}", status);
    
    
    // TODO: Create an inner scope that shadows status
    // Inside the scope:
    //   - Shadow status with "processing"
    //   - Print: println!("Inner status: {}", status);
    // After the scope:
    //   - Print: println!("Outer status restored: {}", status);
    
    
    // ============================================
    // PART 3: Shadowing vs Mutation
    // ============================================
    
    // TODO: Demonstrate shadowing (type change allowed)
    // Create: let data = "42"
    // Shadow: let data: i32 = data.parse().unwrap()
    // Print: println!("String to int: {}", data)
    
    
    // TODO: Demonstrate mutation (same type only)
    // Create: let mut counter = 0
    // Modify: counter = counter + 1
    // Modify: counter = counter + 1
    // Print: println!("Counter after mutations: {}", counter)
    
    
    // ============================================
    // PART 4: Practical Example - Phone Processing
    // ============================================
    
    // TODO: Process a phone number through shadowing
    // Start: let phone = "+1-555-123-4567"
    // Shadow: Remove the +1 prefix (use .replace("+1-", ""))
    // Shadow: Remove dashes (use .replace("-", ""))
    // Print each step
    
}

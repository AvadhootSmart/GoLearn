// Textio SMS API - While/For Exercise
// Implement message processing using while and for loops

fn main() {
    println!("=== Textio Message Iterator System ===\n");

    // Task 1: Message Countdown
    // Use a for loop with reverse range for countdown
    println!("--- Task 1: Message Countdown ---");
    
    // TODO: Create a countdown from 5 to 1 (inclusive)
    // Print each number, then "Sending messages..."
    // Use a reverse inclusive range
    
    // Your for loop here
    
    println!("Sending messages...");

    // Task 2: Batch Processing with Ranges
    // Process messages in batches of 3
    println!("\n--- Task 2: Batch Processing ---");
    
    let all_messages: Vec<&str> = vec![
        "msg1", "msg2", "msg3", "msg4", "msg5",
        "msg6", "msg7", "msg8", "msg9", "msg10",
    ];
    
    let batch_size = 3;
    
    // TODO: Process messages in batches
    // - Use step_by to create batch start indices
    // - For each batch, print the batch number and messages
    // - Handle the last batch which may be smaller
    
    // Your for loop with step_by here

    // Task 3: User Message Statistics
    // Calculate statistics for each user
    println!("\n--- Task 3: User Statistics ---");
    
    let users = vec![
        ("Alice", 45, 50),
        ("Bob", 30, 100),
        ("Charlie", 99, 100),
        ("Diana", 0, 50),
    ];
    // (name, messages_sent, limit)
    
    // TODO: Use enumerate to iterate and display:
    // - User number (starting from 1)
    // - User name
    // - Messages sent
    // - Usage percentage
    // - Status: "Over limit" if >= 100%, "Near limit" if >= 80%, "OK" otherwise
    
    // Your for loop with enumerate here

    // Task 4: Message Transformation
    // Modify messages in place
    println!("\n--- Task 4: Message Transformation ---");
    
    let mut messages = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "textio".to_string(),
    ];
    
    println!("Before: {:?}", messages);
    
    // TODO: Use mutable iteration to:
    // - Convert each message to uppercase
    // - Add a sequence number at the end
    // Track the sequence with a counter outside the loop
    
    // Your mutable for loop here
    
    println!("After:  {:?}", messages);

    // Task 5: Polling with While
    // Simulate polling for message status
    println!("\n--- Task 5: Status Polling ---");
    
    // Simulated status function (don't modify)
    let mut status_checks = 0;
    fn check_status(checks: &mut i32) -> &'static str {
        *checks += 1;
        if *checks >= 4 {
            "delivered"
        } else {
            "pending"
        }
    }
    
    // TODO: Use a while loop to poll until "delivered"
    // - Call check_status(&mut status_checks)
    // - Print each status check
    // - Maximum 10 attempts
    // - Print final result
    
    // Your while loop here

    // Task 6: Range Operations
    // Demonstrate different range types
    println!("\n--- Task 6: Range Operations ---");
    
    // TODO: Create the following outputs using ranges:
    
    // Part A: Print indices 0-4 (exclusive range)
    print!("Exclusive 0..5: ");
    // Your for loop here
    println!();
    
    // Part B: Print indices 1-5 (inclusive range)
    print!("Inclusive 1..=5: ");
    // Your for loop here
    println!();
    
    // Part C: Print even numbers 0-10 (step_by)
    print!("Even 0-10: ");
    // Your for loop here
    println!();
    
    // Part D: Print every 3rd character a-j
    print!("Every 3rd char a-j: ");
    // Your for loop here
    println!();

    // Task 7: Processing Nested Collections
    // Process messages grouped by priority
    println!("\n--- Task 7: Priority Processing ---");
    
    let priority_messages = vec![
        vec!["urgent1", "urgent2"],           // Priority 1
        vec!["normal1", "normal2", "normal3"], // Priority 2
        vec!["low1"],                          // Priority 3
    ];
    
    // TODO: Use nested for loops to process messages by priority
    // - Outer loop: iterate with index (priority level 1-3)
    // - Inner loop: iterate over messages in that priority
    // - Print: "Priority [N]: Processing [message]"
    
    // Your nested for loops here

    // Task 8: Collection Statistics
    // Calculate message statistics
    println!("\n--- Task 8: Collection Statistics ---");
    
    let message_lengths: Vec<usize> = vec![15, 42, 8, 120, 65, 30, 85, 12];
    
    let mut total_chars = 0;
    let mut short_messages = 0;  // < 20 chars
    let mut medium_messages = 0; // 20-60 chars
    let mut long_messages = 0;   // > 60 chars
    
    // TODO: Use a for loop to calculate:
    // - Total characters
    // - Count of short (<20), medium (20-60), and long (>60) messages
    
    // Your for loop here
    
    println!("Total messages: {}", message_lengths.len());
    println!("Total characters: {}", total_chars);
    println!("Average length: {:.1}", total_chars as f64 / message_lengths.len() as f64);
    println!("Short (<20): {}", short_messages);
    println!("Medium (20-60): {}", medium_messages);
    println!("Long (>60): {}", long_messages);

    // Task 9: Slice Processing
    // Process specific portions of data
    println!("\n--- Task 9: Slice Processing ---");
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // TODO: Using range-based indexing:
    // Part A: Print first 3 elements (0..3)
    print!("First 3: ");
    // Your code here
    println!();
    
    // Part B: Print last 3 elements
    print!("Last 3: ");
    // Your code here
    println!();
    
    // Part C: Print middle elements (indices 3-6 inclusive)
    print!("Middle (3-6): ");
    // Your code here
    println!();

    // Task 10: While with Complex Condition
    // Find optimal message batch size
    println!("\n--- Task 10: Batch Size Optimization ---");
    
    let mut batch_size = 1;
    let max_batch = 10;
    let target_throughput = 50;
    
    // Simulated throughput function (don't modify)
    fn calculate_throughput(batch: i32) -> i32 {
        // Throughput increases then decreases
        60 - (batch - 5).abs() * 8
    }
    
    // TODO: Use while loop to find batch size with throughput >= target
    // - Start at batch_size 1
    // - Check throughput for each batch size
    // - Stop when throughput >= target or batch_size > max_batch
    // - Print each attempt
    
    // Your while loop here
    
    println!("Optimal batch size: {}", batch_size);

    println!("\n=== Iterator System Complete ===");
}

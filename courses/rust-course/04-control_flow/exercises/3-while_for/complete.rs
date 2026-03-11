// Textio SMS API - While/For Exercise - Complete Solution

fn main() {
    println!("=== Textio Message Iterator System ===\n");

    // Task 1: Message Countdown
    println!("--- Task 1: Message Countdown ---");
    
    for i in (1..=5).rev() {
        println!("{}", i);
    }
    println!("Sending messages...");

    // Task 2: Batch Processing with Ranges
    println!("\n--- Task 2: Batch Processing ---");
    
    let all_messages: Vec<&str> = vec![
        "msg1", "msg2", "msg3", "msg4", "msg5",
        "msg6", "msg7", "msg8", "msg9", "msg10",
    ];
    
    let batch_size = 3;
    let mut batch_num = 0;
    
    for batch_start in (0..all_messages.len()).step_by(batch_size) {
        batch_num += 1;
        let batch_end = (batch_start + batch_size).min(all_messages.len());
        let batch = &all_messages[batch_start..batch_end];
        
        println!("Batch {} ({} messages):", batch_num, batch.len());
        for msg in batch {
            println!("  - {}", msg);
        }
    }

    // Task 3: User Message Statistics
    println!("\n--- Task 3: User Statistics ---");
    
    let users = vec![
        ("Alice", 45, 50),
        ("Bob", 30, 100),
        ("Charlie", 99, 100),
        ("Diana", 0, 50),
    ];
    
    for (index, (name, sent, limit)) in users.iter().enumerate() {
        let usage_percent = (**sent as f64 / *limit as f64) * 100.0;
        let status = if usage_percent >= 100.0 {
            "Over limit"
        } else if usage_percent >= 80.0 {
            "Near limit"
        } else {
            "OK"
        };
        
        println!(
            "User {}: {} - Sent: {}/{}, Usage: {:.1}%, Status: {}",
            index + 1,
            name,
            sent,
            limit,
            usage_percent,
            status
        );
    }

    // Task 4: Message Transformation
    println!("\n--- Task 4: Message Transformation ---");
    
    let mut messages = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "textio".to_string(),
    ];
    
    println!("Before: {:?}", messages);
    
    let mut seq = 1;
    for msg in &mut messages {
        *msg = format!("{}-{:03}", msg.to_uppercase(), seq);
        seq += 1;
    }
    
    println!("After:  {:?}", messages);

    // Task 5: Polling with While
    println!("\n--- Task 5: Status Polling ---");
    
    let mut status_checks = 0;
    
    fn check_status(checks: &mut i32) -> &'static str {
        *checks += 1;
        if *checks >= 4 {
            "delivered"
        } else {
            "pending"
        }
    }
    
    let mut attempts = 0;
    let max_attempts = 10;
    
    while attempts < max_attempts {
        let status = check_status(&mut status_checks);
        attempts += 1;
        println!("Attempt {}: Status = {}", attempts, status);
        
        if status == "delivered" {
            break;
        }
    }
    
    if attempts < max_attempts {
        println!("Message delivered after {} attempts", attempts);
    } else {
        println!("Max attempts reached");
    }

    // Task 6: Range Operations
    println!("\n--- Task 6: Range Operations ---");
    
    print!("Exclusive 0..5: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
    
    print!("Inclusive 1..=5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    print!("Even 0-10: ");
    for i in (0..=10).step_by(2) {
        print!("{} ", i);
    }
    println!();
    
    print!("Every 3rd char a-j: ");
    for c in ('a'..='j').step_by(3) {
        print!("{} ", c);
    }
    println!();

    // Task 7: Processing Nested Collections
    println!("\n--- Task 7: Priority Processing ---");
    
    let priority_messages = vec![
        vec!["urgent1", "urgent2"],
        vec!["normal1", "normal2", "normal3"],
        vec!["low1"],
    ];
    
    for (priority_idx, messages) in priority_messages.iter().enumerate() {
        let priority_level = priority_idx + 1;
        for msg in messages {
            println!("Priority {}: Processing {}", priority_level, msg);
        }
    }

    // Task 8: Collection Statistics
    println!("\n--- Task 8: Collection Statistics ---");
    
    let message_lengths: Vec<usize> = vec![15, 42, 8, 120, 65, 30, 85, 12];
    
    let mut total_chars = 0;
    let mut short_messages = 0;
    let mut medium_messages = 0;
    let mut long_messages = 0;
    
    for length in &message_lengths {
        total_chars += length;
        
        if *length < 20 {
            short_messages += 1;
        } else if *length <= 60 {
            medium_messages += 1;
        } else {
            long_messages += 1;
        }
    }
    
    println!("Total messages: {}", message_lengths.len());
    println!("Total characters: {}", total_chars);
    println!("Average length: {:.1}", total_chars as f64 / message_lengths.len() as f64);
    println!("Short (<20): {}", short_messages);
    println!("Medium (20-60): {}", medium_messages);
    println!("Long (>60): {}", long_messages);

    // Task 9: Slice Processing
    println!("\n--- Task 9: Slice Processing ---");
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    print!("First 3: ");
    for i in &data[0..3] {
        print!("{} ", i);
    }
    println!();
    
    print!("Last 3: ");
    for i in &data[data.len()-3..] {
        print!("{} ", i);
    }
    println!();
    
    print!("Middle (3-6): ");
    for i in &data[3..=6] {
        print!("{} ", i);
    }
    println!();

    // Task 10: While with Complex Condition
    println!("\n--- Task 10: Batch Size Optimization ---");
    
    let mut batch_size = 1;
    let max_batch = 10;
    let target_throughput = 50;
    
    fn calculate_throughput(batch: i32) -> i32 {
        60 - (batch - 5).abs() * 8
    }
    
    while batch_size <= max_batch {
        let throughput = calculate_throughput(batch_size);
        println!("Batch size {}: throughput = {}", batch_size, throughput);
        
        if throughput >= target_throughput {
            break;
        }
        
        batch_size += 1;
    }
    
    println!("Optimal batch size: {}", batch_size);

    println!("\n=== Iterator System Complete ===");
}

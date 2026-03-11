// Textio SMS API - Loop Exercise - Complete Solution

fn main() {
    println!("=== Textio Message Processing System ===\n");

    // Task 1: Message Queue Processor
    let mut message_queue: Vec<&str> = vec![
        "Hello from Alice",
        "Meeting at 3pm",
        "Don't forget the report",
        "See you tomorrow",
    ];

    println!("--- Task 1: Queue Processing ---");
    println!("Initial queue size: {}", message_queue.len());
    
    let mut processed_count = 0;
    
    loop {
        if message_queue.is_empty() {
            break;
        }
        
        let message = message_queue.remove(0);
        println!("Processing: {}", message);
        processed_count += 1;
    }
    
    println!("Total messages processed: {}", processed_count);

    // Task 2: Retry Mechanism
    println!("\n--- Task 2: Retry Logic ---");
    
    fn try_send(message: &str, attempt: u32) -> Result<&str, &str> {
        if attempt >= 3 {
            Ok("Message sent successfully")
        } else {
            Err("Temporary failure")
        }
    }
    
    let message_to_send = "Important notification";
    let max_retries = 5;
    let mut attempts = 0;
    
    let final_status = loop {
        attempts += 1;
        println!("Attempt {} for: {}", attempts, message_to_send);
        
        match try_send(message_to_send, attempts) {
            Ok(msg) => break msg,
            Err(e) => {
                println!("  Failed: {}", e);
                if attempts >= max_retries {
                    break "Max retries exceeded";
                }
                continue;
            }
        }
    };
    
    println!("Final status: {}", final_status);

    // Task 3: Message Search with Loop Return
    println!("\n--- Task 3: Message Search ---");
    
    let messages = vec![
        (101, "First message"),
        (102, "Second message"),
        (103, "Target message"),
        (104, "Fourth message"),
    ];
    
    let target_id = 103;
    let mut index = 0;
    
    let found_message = loop {
        if index >= messages.len() {
            break "Not found";
        }
        
        let (id, content) = messages[index];
        if id == target_id {
            break content;
        }
        
        index += 1;
    };
    
    println!("Searching for ID: {}", target_id);
    println!("Found: {}", found_message);

    // Task 4: Nested Loop with Labels
    println!("\n--- Task 4: Batch Processing ---");
    
    let batches = vec![
        vec!["msg1", "msg2", "valid"],
        vec!["msg3", "error", "msg4"],
        vec!["msg5", "valid", "msg6"],
        vec!["error", "msg7", "msg8"],
        vec!["msg9", "msg10", "valid"],
    ];
    
    let mut successful_batches = 0;
    let mut total_messages = 0;
    let mut skipped_batches = 0;
    
    'batches: for batch in &batches {
        println!("Processing batch...");
        
        'messages: for msg in batch {
            if *msg == "error" {
                println!("  Error found, skipping batch");
                skipped_batches += 1;
                continue 'batches;
            }
            
            if *msg == "valid" {
                println!("  Batch completed successfully");
                successful_batches += 1;
                continue 'batches;
            }
            
            println!("  Processed: {}", msg);
            total_messages += 1;
        }
    }
    
    println!("Successful batches: {}", successful_batches);
    println!("Total messages processed: {}", total_messages);
    println!("Skipped batches: {}", skipped_batches);

    // Task 5: Rate Limiter with Continue
    println!("\n--- Task 5: Rate Limiter ---");
    
    let requests = vec![
        ("user1", 5),
        ("user2", 3),
        ("user3", 8),
        ("user4", 2),
        ("user5", 12),
    ];
    
    let rate_limit = 5;
    let mut allowed_requests = 0;
    let mut denied_requests = 0;
    let mut total_messages_requested = 0;
    
    let mut idx = 0;
    
    loop {
        if idx >= requests.len() {
            break;
        }
        
        let (user, count) = requests[idx];
        total_messages_requested += count;
        
        if count > rate_limit {
            println!("DENIED: {} requested {} (limit: {})", user, count, rate_limit);
            denied_requests += 1;
            idx += 1;
            continue;
        }
        
        println!("ALLOWED: {} sent {} messages", user, count);
        allowed_requests += 1;
        idx += 1;
    }
    
    println!("Allowed requests: {}", allowed_requests);
    println!("Denied requests: {}", denied_requests);
    println!("Total messages requested: {}", total_messages_requested);

    // Task 6: Connection Retry with Backoff
    println!("\n--- Task 6: Connection Retry ---");
    
    fn try_connect(attempt: u32) -> bool {
        attempt >= 2
    }
    
    let max_connection_attempts = 5;
    let mut connection_attempts = 0;
    
    let connection_status = loop {
        if try_connect(connection_attempts) {
            break "Connected";
        }
        
        connection_attempts += 1;
        println!("Connection attempt {} failed", connection_attempts);
        
        if connection_attempts >= max_connection_attempts {
            break "Connection failed";
        }
    };
    
    println!("Connection status: {}", connection_status);
    println!("Attempts made: {}", connection_attempts);

    // Task 7: Message Aggregator
    println!("\n--- Task 7: Message Aggregator ---");
    
    let incoming_messages = vec![
        "Update: System online",
        "Update: All services running",
        "Update: Database healthy",
        "END_OF_BATCH",
        "Update: This should not be processed",
    ];
    
    let mut batch_messages: Vec<&str> = Vec::new();
    let mut msg_idx = 0;
    
    loop {
        if msg_idx >= incoming_messages.len() {
            break;
        }
        
        let msg = incoming_messages[msg_idx];
        
        if msg == "END_OF_BATCH" {
            break;
        }
        
        batch_messages.push(msg);
        msg_idx += 1;
    }
    
    println!("Messages in batch: {}", batch_messages.len());
    for (i, msg) in batch_messages.iter().enumerate() {
        println!("  {}: {}", i + 1, msg);
    }

    println!("\n=== Processing Complete ===");
}

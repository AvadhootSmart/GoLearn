// Textio SMS API - Loop Exercise
// Implement message processing using loops, break, continue, and labels

fn main() {
    println!("=== Textio Message Processing System ===\n");

    // Task 1: Message Queue Processor
    // Process messages from a queue until it's empty
    let mut message_queue: Vec<&str> = vec![
        "Hello from Alice",
        "Meeting at 3pm",
        "Don't forget the report",
        "See you tomorrow",
    ];

    println!("--- Task 1: Queue Processing ---");
    println!("Initial queue size: {}", message_queue.len());
    
    // TODO: Use a loop to process all messages
    // - Remove each message from the front of the queue
    // - Print "Processing: [message]"
    // - Count total processed
    // - Use loop, break when queue is empty
    
    let mut processed_count = 0;
    
    // Your loop here
    
    println!("Total messages processed: {}", processed_count);

    // Task 2: Retry Mechanism
    // Implement retry logic for failed message sends
    println!("\n--- Task 2: Retry Logic ---");
    
    // Simulated send function (don't modify)
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
    
    // TODO: Implement retry loop
    // - Call try_send(message_to_send, attempts)
    // - On Ok, break with success message
    // - On Err, increment attempts and continue
    // - If attempts exceed max_retries, break with failure
    // - Return the final result from the loop
    
    let final_status = ""; // Replace with your loop
    
    println!("Final status: {}", final_status);

    // Task 3: Message Search with Loop Return
    // Search for a message by ID in a collection
    println!("\n--- Task 3: Message Search ---");
    
    let messages = vec![
        (101, "First message"),
        (102, "Second message"),
        (103, "Target message"),
        (104, "Fourth message"),
    ];
    
    let target_id = 103;
    
    // TODO: Use a loop to find the message with target_id
    // - Return the message content if found
    // - Return "Not found" if not in the list
    // - Use loop return value
    
    let mut index = 0;
    let found_message = ""; // Replace with your loop
    
    println!("Searching for ID: {}", target_id);
    println!("Found: {}", found_message);

    // Task 4: Nested Loop with Labels
    // Process batches of messages, skipping problematic ones
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
    
    // TODO: Process batches using nested loops with labels
    // - If any message in a batch is "error", skip entire batch
    // - If any message is "valid", that's a valid completion marker
    // - Count successful batches and total messages processed
    // - Use 'outer and 'inner labels
    
    // Your nested loops here
    
    println!("Successful batches: {}", successful_batches);
    println!("Total messages processed: {}", total_messages);
    println!("Skipped batches: {}", skipped_batches);

    // Task 5: Rate Limiter with Continue
    // Process requests with rate limiting
    println!("\n--- Task 5: Rate Limiter ---");
    
    let requests = vec![
        ("user1", 5),   // 5 messages
        ("user2", 3),   // 3 messages
        ("user3", 8),   // 8 messages - over limit
        ("user4", 2),   // 2 messages
        ("user5", 12),  // 12 messages - over limit
    ];
    
    let rate_limit = 5;
    let mut allowed_requests = 0;
    let mut denied_requests = 0;
    let mut total_messages_requested = 0;
    
    // TODO: Process requests using loop with continue
    // - If request count <= rate_limit, process it
    // - If over limit, deny and continue to next
    // - Track counts
    
    let mut idx = 0;
    
    // Your loop here with continue
    
    println!("Allowed requests: {}", allowed_requests);
    println!("Denied requests: {}", denied_requests);
    println!("Total messages requested: {}", total_messages_requested);

    // Task 6: Connection Retry with Backoff
    // Simulate connecting to SMS gateway with increasing delays
    println!("\n--- Task 6: Connection Retry ---");
    
    // Simulated connection function (don't modify)
    fn try_connect(attempt: u32) -> bool {
        attempt >= 2  // Succeeds on 3rd attempt (0, 1, 2)
    }
    
    let max_connection_attempts = 5;
    let mut connection_attempts = 0;
    
    // TODO: Try to connect with the following logic:
    // - Attempt connection
    // - If successful, break with "Connected"
    // - If failed and attempts < max, continue
    // - If max attempts reached, break with "Connection failed"
    // - Return the connection status from loop
    
    let connection_status = ""; // Replace with your loop
    
    println!("Connection status: {}", connection_status);
    println!("Attempts made: {}", connection_attempts);

    // Task 7: Message Aggregator
    // Collect messages until a sentinel value is found
    println!("\n--- Task 7: Message Aggregator ---");
    
    let incoming_messages = vec![
        "Update: System online",
        "Update: All services running",
        "Update: Database healthy",
        "END_OF_BATCH",
        "Update: This should not be processed",
    ];
    
    let mut batch_messages: Vec<&str> = Vec::new();
    
    // TODO: Use a loop to collect messages
    // - Add messages to batch_messages
    // - Stop when you encounter "END_OF_BATCH"
    // - Don't include the sentinel in the batch
    
    // Your loop here
    
    println!("Messages in batch: {}", batch_messages.len());
    for (i, msg) in batch_messages.iter().enumerate() {
        println!("  {}: {}", i + 1, msg);
    }

    println!("\n=== Processing Complete ===");
}

// Textio SMS API - Advanced Pattern Matching Exercise
// Implement complex patterns with guards, bindings, and destructuring

fn main() {
    println!("=== Textio Advanced Pattern Matching ===\n");

    // Task 1: Message Validation with Guards
    // Validate messages using multiple conditions
    println!("--- Task 1: Message Validation ---");
    
    let messages = [
        ("Alice", "Bob", "Hello, how are you?"),
        ("Charlie", "Charlie", "Note to self"),
        ("Diana", "Eve", ""),
        ("Frank", "Grace", "A".repeat(200)),
        ("Heidi", "Ivan", "Short"),
    ];
    
    for (sender, recipient, content) in &messages {
        // TODO: Use match with guards to validate:
        // - Self-message (sender == recipient): "Self-message, allowed"
        // - Empty content: "Error: Empty message"
        // - Content > 160 chars: "Error: Too long (X chars)"
        // - Valid: "Valid: X chars from SENDER to RECIPIENT"
        
        let validation = ""; // Replace with match
        
        println!("{} -> {}: {}", sender, recipient, validation);
    }

    // Task 2: HTTP Status with Bindings and Guards
    // Classify status codes with detailed info
    println!("\n--- Task 2: HTTP Status Classification ---");
    
    let responses = [
        (200, None),
        (201, None),
        (400, None),
        (429, Some(60)),
        (429, Some(300)),
        (503, Some(30)),
        (500, None),
    ];
    
    for (code, retry_after) in responses {
        // TODO: Match with bindings and guards:
        // - 200-299: "Success (CODE)"
        // - 400-499 with code bound: "Client error: CODE"
        // - 429 with retry_after 1-60: "Rate limited, retry in Xs (short)"
        // - 429 with retry_after > 60: "Rate limited, retry in Xs (long)"
        // - 500-599 with code bound: "Server error: CODE"
        // - Other: "Unknown: CODE"
        
        let status = ""; // Replace with match
        
        println!("{}", status);
    }

    // Task 3: Struct Destructuring
    // Match on message structs
    println!("\n--- Task 3: Message Processing ---");
    
    struct Message {
        id: u64,
        from: String,
        to: String,
        priority: u8,
        status: String,
    }
    
    let messages = [
        Message {
            id: 1,
            from: String::from("alice@textio.com"),
            to: String::from("bob@example.com"),
            priority: 1,
            status: String::from("delivered"),
        },
        Message {
            id: 2,
            from: String::from("system@textio.com"),
            to: String::from("user@test.com"),
            priority: 3,
            status: String::from("failed"),
        },
        Message {
            id: 3,
            from: String::from("admin@textio.com"),
            to: String::from("all@textio.com"),
            priority: 1,
            status: String::from("pending"),
        },
    ];
    
    for msg in &messages {
        // TODO: Destructure and match:
        // - priority 1 with status "delivered": "URGENT delivered: ID from FROM"
        // - priority 1 with status "pending": "URGENT pending: ID"
        // - priority 1 with status "failed": "URGENT FAILED: ID to TO"
        // - Any status "failed": "Failed: ID to TO"
        // - From system@textio.com: "System message ID"
        // - Other: "Message ID: FROM -> TO"
        
        let processing = ""; // Replace with match
        
        println!("{}", processing);
    }

    // Task 4: Tuple Matching with Guards
    // Match on coordinate pairs
    println!("\n--- Task 4: Coordinate Analysis ---");
    
    let points = [
        (0, 0),
        (0, 5),
        (5, 0),
        (3, 3),
        (-2, -2),
        (10, 20),
    ];
    
    for (x, y) in points {
        // TODO: Match with guards:
        // - Origin (0, 0): "Origin"
        // - On X axis (y == 0, x != 0): "On X axis at X"
        // - On Y axis (x == 0, y != 0): "On Y axis at Y"
        // - Diagonal (|x| == |y|): "On diagonal at (X, Y)"
        // - Far from origin (> 15 units): "Far from origin: (X, Y)"
        // - Near origin: "Near origin: (X, Y)"
        
        let location = ""; // Replace with match
        
        println!("{}", location);
    }

    // Task 5: Enum Pattern Matching
    // Match on complex enums
    println!("\n--- Task 5: Event Handling ---");
    
    enum Event {
        MessageSent { id: u64, recipient: String },
        MessageFailed { id: u64, error_code: u16 },
        UserLogin { user: String, device: String },
        UserLogout { user: String },
        SystemAlert { level: u8, message: String },
    }
    
    let events = [
        Event::MessageSent { 
            id: 1001, 
            recipient: String::from("bob@example.com") 
        },
        Event::MessageFailed { 
            id: 1002, 
            error_code: 404 
        },
        Event::MessageFailed { 
            id: 1003, 
            error_code: 500 
        },
        Event::UserLogin { 
            user: String::from("alice"), 
            device: String::from("mobile") 
        },
        Event::UserLogin { 
            user: String::from("admin"), 
            device: String::from("desktop") 
        },
        Event::UserLogout { 
            user: String::from("alice") 
        },
        Event::SystemAlert { 
            level: 1, 
            message: String::from("High CPU") 
        },
        Event::SystemAlert { 
            level: 3, 
            message: String::from("Disk space low") 
        },
    ];
    
    for event in &events {
        // TODO: Match on Event enum with guards:
        // - MessageSent: "Sent ID to RECIPIENT"
        // - MessageFailed with 4xx error: "Client error CODE for ID"
        // - MessageFailed with 5xx error: "Server error CODE for ID"
        // - UserLogin with "admin": "Admin login from DEVICE"
        // - UserLogin: "USER logged in from DEVICE"
        // - UserLogout: "USER logged out"
        // - SystemAlert level 1-2: "URGENT: MESSAGE"
        // - SystemAlert level 3+: "Warning: MESSAGE"
        
        let event_log = ""; // Replace with match
        
        println!("{}", event_log);
    }

    // Task 6: Range Bindings
    // Use @ operator for range bindings
    println!("\n--- Task 6: Score Classification ---");
    
    let scores = [95, 82, 75, 68, 55, 42, 105];
    
    for score in scores {
        // TODO: Use @ bindings with ranges:
        // - 90-100: "Excellent: SCORE"
        // - 80-89: "Good: SCORE"
        // - 70-79: "Average: SCORE"
        // - 60-69: "Below Average: SCORE"
        // - 0-59: "Failing: SCORE"
        // - > 100: "Invalid score: SCORE"
        // - < 0: "Invalid score: SCORE"
        
        let grade = ""; // Replace with match
        
        println!("{}", grade);
    }

    // Task 7: Nested Struct Matching
    // Match on nested data structures
    println!("\n--- Task 7: Nested Matching ---");
    
    struct Location {
        city: String,
        country: String,
    }
    
    struct User {
        name: String,
        location: Location,
        tier: String,
    }
    
    let users = [
        User {
            name: String::from("Alice"),
            location: Location {
                city: String::from("New York"),
                country: String::from("US"),
            },
            tier: String::from("gold"),
        },
        User {
            name: String::from("Bob"),
            location: Location {
                city: String::from("London"),
                country: String::from("UK"),
            },
            tier: String::from("silver"),
        },
        User {
            name: String::from("Charlie"),
            location: Location {
                city: String::from("San Francisco"),
                country: String::from("US"),
            },
            tier: String::from("free"),
        },
    ];
    
    for user in &users {
        // TODO: Match on nested structure:
        // - US gold user: "Premium US user: NAME in CITY"
        // - US silver/bronze user: "Standard US user: NAME in CITY"
        // - US free user: "Free US user: NAME"
        // - UK user: "UK user: NAME in CITY"
        // - Other: "International user: NAME"
        
        let user_info = ""; // Replace with match
        
        println!("{}", user_info);
    }

    // Task 8: Complex Conditions
    // Multiple guards and bindings
    println!("\n--- Task 8: Complex Validation ---");
    
    let accounts = [
        ("alice", 5000, true, 12),
        ("bob", 500, true, 6),
        ("charlie", 100, false, 24),
        ("diana", 10000, true, 3),
        ("eve", 0, true, 1),
    ];
    // (username, messages_sent, verified, account_age_months)
    
    for (username, sent, verified, age) in accounts {
        // TODO: Match with complex guards:
        // - Verified, > 1000 messages, age > 6 months: 
        //   "Power user USERNAME: SENT messages"
        // - Verified, > 100 messages, any age: 
        //   "Active user USERNAME: SENT messages"
        // - Not verified, any messages: 
        //   "Unverified user USERNAME"
        // - Verified, <= 100 messages, age <= 3 months: 
        //   "New user USERNAME"
        // - Verified, <= 100 messages: 
        //   "Inactive user USERNAME"
        
        let status = ""; // Replace with match
        
        println!("{}", status);
    }

    // Task 9: Slice Patterns
    // Match on array patterns
    println!("\n--- Task 9: Array Patterns ---");
    
    let sequences = [
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4, 5],
    ];
    
    for seq in &sequences {
        // TODO: Match on slice patterns:
        // - Empty: "Empty sequence"
        // - Single element: "Single: X"
        // - Two elements: "Pair: X, Y"
        // - First and last of longer: "Starts with X, ends with Y"
        
        // Note: Use seq.as_slice() or &seq[..]
        
        let description = ""; // Replace with match
        
        println!("{}", description);
    }

    // Task 10: Result/Option Patterns
    // Match on Option and Result types
    println!("\n--- Task 10: Result Handling ---");
    
    let results: Vec<Result<(u32, String), String>> = vec![
        Ok((200, String::from("Success"))),
        Ok((404, String::from("Not found"))),
        Err(String::from("Network timeout")),
        Ok((500, String::from("Server error"))),
        Err(String::from("Invalid credentials")),
    ];
    
    for result in &results {
        // TODO: Match on Result with complex patterns:
        // - Ok((200, msg)): "OK: MSG"
        // - Ok((code @ 400..=499, msg)): "Client error CODE: MSG"
        // - Ok((code @ 500..=599, msg)): "Server error CODE: MSG"
        // - Ok((code, msg)): "Response CODE: MSG"
        // - Err(e) if e contains "timeout": "Timeout error: E"
        // - Err(e): "Error: E"
        
        let output = ""; // Replace with match
        
        println!("{}", output);
    }

    println!("\n=== Pattern Matching Complete ===");
}

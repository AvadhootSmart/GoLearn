// Textio SMS API - Advanced Pattern Matching Exercise - Complete Solution

fn main() {
    println!("=== Textio Advanced Pattern Matching ===\n");

    // Task 1: Message Validation with Guards
    println!("--- Task 1: Message Validation ---");
    
    let messages = [
        ("Alice", "Bob", "Hello, how are you?"),
        ("Charlie", "Charlie", "Note to self"),
        ("Diana", "Eve", ""),
        ("Frank", "Grace", "A".repeat(200)),
        ("Heidi", "Ivan", "Short"),
    ];
    
    for (sender, recipient, content) in &messages {
        let validation = match (sender, recipient, content.as_str()) {
            (s, r, _) if s == r => "Self-message, allowed".to_string(),
            (_, _, c) if c.is_empty() => "Error: Empty message".to_string(),
            (_, _, c) if c.len() > 160 => 
                format!("Error: Too long ({} chars)", c.len()),
            (s, r, c) => 
                format!("Valid: {} chars from {} to {}", c.len(), s, r),
        };
        
        println!("{} -> {}: {}", sender, recipient, validation);
    }

    // Task 2: HTTP Status with Bindings and Guards
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
        let status = match (code, retry_after) {
            (c @ 200..=299, _) => format!("Success ({})", c),
            (c @ 400..=499, _) => format!("Client error: {}", c),
            (429, Some(seconds @ 1..=60)) => 
                format!("Rate limited, retry in {}s (short)", seconds),
            (429, Some(seconds)) => 
                format!("Rate limited, retry in {}s (long)", seconds),
            (c @ 500..=599, _) => format!("Server error: {}", c),
            (c, _) => format!("Unknown: {}", c),
        };
        
        println!("{}", status);
    }

    // Task 3: Struct Destructuring
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
        let processing = match msg {
            Message { id, from, priority: 1, status: s, .. } 
                if s == "delivered" => 
                format!("URGENT delivered: {} from {}", id, from),
            Message { id, priority: 1, status: s, .. } 
                if s == "pending" => 
                format!("URGENT pending: {}", id),
            Message { id, to, priority: 1, status: s, .. } 
                if s == "failed" => 
                format!("URGENT FAILED: {} to {}", id, to),
            Message { id, to, status: s, .. } if s == "failed" => 
                format!("Failed: {} to {}", id, to),
            Message { id, from, .. } if from == "system@textio.com" => 
                format!("System message {}", id),
            Message { id, from, to, .. } => 
                format!("Message {}: {} -> {}", id, from, to),
        };
        
        println!("{}", processing);
    }

    // Task 4: Tuple Matching with Guards
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
        let location = match (x, y) {
            (0, 0) => String::from("Origin"),
            (x, 0) if x != 0 => format!("On X axis at {}", x),
            (0, y) if y != 0 => format!("On Y axis at {}", y),
            (x, y) if x.abs() == y.abs() => 
                format!("On diagonal at ({}, {})", x, y),
            (x, y) if (x*x + y*y) > 225 => 
                format!("Far from origin: ({}, {})", x, y),
            (x, y) => format!("Near origin: ({}, {})", x, y),
        };
        
        println!("{}", location);
    }

    // Task 5: Enum Pattern Matching
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
        let event_log = match event {
            Event::MessageSent { id, recipient } => 
                format!("Sent {} to {}", id, recipient),
            Event::MessageFailed { id, error_code: c @ 400..=499 } => 
                format!("Client error {} for {}", c, id),
            Event::MessageFailed { id, error_code: c @ 500..=599 } => 
                format!("Server error {} for {}", c, id),
            Event::UserLogin { user, device } if user == "admin" => 
                format!("Admin login from {}", device),
            Event::UserLogin { user, device } => 
                format!("{} logged in from {}", user, device),
            Event::UserLogout { user } => 
                format!("{} logged out", user),
            Event::SystemAlert { level: l @ 1..=2, message } => 
                format!("URGENT: {}", message),
            Event::SystemAlert { level, message } => 
                format!("Warning: {}", message),
        };
        
        println!("{}", event_log);
    }

    // Task 6: Range Bindings
    println!("\n--- Task 6: Score Classification ---");
    
    let scores = [95, 82, 75, 68, 55, 42, 105];
    
    for score in scores {
        let grade = match score {
            s @ 90..=100 => format!("Excellent: {}", s),
            s @ 80..=89 => format!("Good: {}", s),
            s @ 70..=79 => format!("Average: {}", s),
            s @ 60..=69 => format!("Below Average: {}", s),
            s @ 0..=59 => format!("Failing: {}", s),
            s => format!("Invalid score: {}", s),
        };
        
        println!("{}", grade);
    }

    // Task 7: Nested Struct Matching
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
        let user_info = match user {
            User { 
                name, 
                location: Location { city, country: "US" }, 
                tier: "gold" 
            } => format!("Premium US user: {} in {}", name, city),
            User { 
                name, 
                location: Location { city, country: "US" }, 
                tier: "silver" | "bronze" 
            } => format!("Standard US user: {} in {}", name, city),
            User { 
                name, 
                location: Location { country: "US", .. }, 
                tier: "free" 
            } => format!("Free US user: {}", name),
            User { 
                name, 
                location: Location { city, country: "UK" }, 
                .. 
            } => format!("UK user: {} in {}", name, city),
            User { name, .. } => format!("International user: {}", name),
        };
        
        println!("{}", user_info);
    }

    // Task 8: Complex Conditions
    println!("\n--- Task 8: Complex Validation ---");
    
    let accounts = [
        ("alice", 5000, true, 12),
        ("bob", 500, true, 6),
        ("charlie", 100, false, 24),
        ("diana", 10000, true, 3),
        ("eve", 0, true, 1),
    ];
    
    for (username, sent, verified, age) in accounts {
        let status = match (username, sent, verified, age) {
            (u, s, true, a) if s > &1000 && a > &6 => 
                format!("Power user {}: {} messages", u, s),
            (u, s, true, _) if s > &100 => 
                format!("Active user {}: {} messages", u, s),
            (u, _, false, _) => 
                format!("Unverified user {}", u),
            (u, s, true, a) if s <= &100 && a <= &3 => 
                format!("New user {}", u),
            (u, s, true, _) if s <= &100 => 
                format!("Inactive user {}", u),
            (u, _, _, _) => format!("User {}", u),
        };
        
        println!("{}", status);
    }

    // Task 9: Slice Patterns
    println!("\n--- Task 9: Array Patterns ---");
    
    let sequences = [
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4, 5],
    ];
    
    for seq in &sequences {
        let description = match seq.as_slice() {
            [] => String::from("Empty sequence"),
            [x] => format!("Single: {}", x),
            [x, y] => format!("Pair: {}, {}", x, y),
            [first, .., last] => 
                format!("Starts with {}, ends with {}", first, last),
        };
        
        println!("{}", description);
    }

    // Task 10: Result/Option Patterns
    println!("\n--- Task 10: Result Handling ---");
    
    let results: Vec<Result<(u32, String), String>> = vec![
        Ok((200, String::from("Success"))),
        Ok((404, String::from("Not found"))),
        Err(String::from("Network timeout")),
        Ok((500, String::from("Server error"))),
        Err(String::from("Invalid credentials")),
    ];
    
    for result in &results {
        let output = match result {
            Ok((200, msg)) => format!("OK: {}", msg),
            Ok((code @ 400..=499, msg)) => 
                format!("Client error {}: {}", code, msg),
            Ok((code @ 500..=599, msg)) => 
                format!("Server error {}: {}", code, msg),
            Ok((code, msg)) => format!("Response {}: {}", code, msg),
            Err(e) if e.contains("timeout") => 
                format!("Timeout error: {}", e),
            Err(e) => format!("Error: {}", e),
        };
        
        println!("{}", output);
    }

    println!("\n=== Pattern Matching Complete ===");
}

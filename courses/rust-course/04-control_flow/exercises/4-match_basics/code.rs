// Textio SMS API - Match Basics Exercise
// Implement pattern matching for message routing and status handling

fn main() {
    println!("=== Textio Match Expression System ===\n");

    // Task 1: HTTP Status Handler
    // Categorize HTTP status codes
    println!("--- Task 1: HTTP Status Handler ---");
    
    let status_codes = [200, 201, 400, 401, 404, 429, 500, 503, 502];
    
    for code in status_codes {
        // TODO: Match on status code and categorize:
        // - 200, 201, 204: "Success"
        // - 400, 401, 403, 404: "Client Error"
        // - 429: "Rate Limited"
        // - 500, 502, 503, 504: "Server Error"
        // - Anything else: "Unknown"
        // Also determine if request should be retried:
        // - Retry for 429 and 5xx errors
        
        let category = ""; // Replace with match
        let should_retry = false; // Replace with match
        
        println!("HTTP {}: {} (retry: {})", code, category, should_retry);
    }

    // Task 2: Message Router
    // Route messages to appropriate gateways
    println!("\n--- Task 2: Message Router ---");
    
    let destinations = [
        ("+1-555-0100", "US"),
        ("+44-20-1234", "UK"),
        ("+49-30-5678", "DE"),
        ("+33-1-9999", "FR"),
        ("+81-3-1111", "JP"),
        ("+86-10-2222", "CN"),
        ("+55-11-3333", "BR"),
    ];
    
    for (phone, country) in destinations {
        // TODO: Match on country code to determine gateway:
        // - "US" or "CA": "Americas Gateway"
        // - "UK": "UK Gateway"
        // - "DE", "FR", "IT", "ES": "EU Gateway"
        // - "JP" or "KR": "Asia-Pacific Gateway"
        // - "CN" or "IN": "Asia Gateway"
        // - Anything else: "International Gateway"
        
        let gateway = ""; // Replace with match
        
        println!("{} ({}) -> {}", phone, country, gateway);
    }

    // Task 3: Priority Handler
    // Match priority levels to timeouts and queue names
    println!("\n--- Task 3: Priority Handler ---");
    
    let priorities = [1, 2, 3, 4, 5, 0];
    
    for priority in priorities {
        // TODO: Match priority (1-5, with 0 as default):
        // Priority | Queue Name      | Timeout (ms) | Max Retries
        //     1    | "critical"      | 100          | 10
        //     2    | "high"          | 500          | 5
        //     3    | "normal"        | 2000         | 3
        //     4    | "low"           | 5000         | 2
        //     5    | "bulk"          | 10000        | 1
        //   other  | "default"       | 1000         | 3
        
        let (queue, timeout, retries) = ("", 0, 0); // Replace with match
        
        println!("Priority {}: Queue={}, Timeout={}ms, Retries={}", 
                 priority, queue, timeout, retries);
    }

    // Task 4: Subscription Tier Manager
    // Match subscription tiers to features and limits
    println!("\n--- Task 4: Subscription Manager ---");
    
    let tiers = ["free", "bronze", "silver", "gold", "platinum", "enterprise"];
    
    for tier in tiers {
        // TODO: Match tier to (monthly_limit, daily_limit, features_count)
        // Tier      | Monthly | Daily | Features
        // free      | 100     | 10    | 1
        // bronze    | 500     | 50    | 3
        // silver    | 2000    | 200   | 5
        // gold      | 10000   | 1000  | 10
        // platinum  | 50000   | 5000  | 20
        // other     | 0       | 0     | 0 (invalid tier)
        
        let (monthly, daily, features) = (0, 0, 0); // Replace with match
        
        println!("{}: {}/month, {}/day, {} features", 
                 tier, monthly, daily, features);
    }

    // Task 5: Message Length Classifier
    // Classify messages by their length
    println!("\n--- Task 5: Message Classifier ---");
    
    let messages = [
        ("msg1", 0),
        ("msg2", 15),
        ("msg3", 80),
        ("msg4", 160),
        ("msg5", 200),
        ("msg6", 500),
    ];
    
    for (name, length) in messages {
        // TODO: Match on length to classify:
        // - 0: "Empty"
        // - 1-50: "Short"
        // - 51-100: "Medium"
        // - 101-160: "Standard"
        // - 161-320: "Extended" (multi-part)
        // - >320: "Long" (multi-part)
        
        let classification = ""; // Replace with match
        
        // TODO: Calculate number of parts (each part max 160 chars)
        let parts = 1; // Replace with match
        
        println!("{} ({} chars): {}, {} part(s)", 
                 name, length, classification, parts);
    }

    // Task 6: Error Classifier
    // Categorize error codes
    println!("\n--- Task 6: Error Classifier ---");
    
    let error_codes = [1001, 1002, 2001, 2002, 3001, 4001, 9999];
    
    for code in error_codes {
        // TODO: Match error codes by range:
        // 1000-1999: "Network Error" (recoverable)
        // 2000-2999: "Validation Error" (not recoverable)
        // 3000-3999: "Rate Limit Error" (recoverable)
        // 4000-4999: "Authentication Error" (not recoverable)
        // Other: "Unknown Error" (not recoverable)
        
        let (error_type, recoverable) = ("", false); // Replace with match
        
        println!("Error {}: {} (recoverable: {})", 
                 code, error_type, recoverable);
    }

    // Task 7: Day of Week Handler
    // Match days to business rules
    println!("\n--- Task 7: Business Day Handler ---");
    
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", 
                "Friday", "Saturday", "Sunday", "Holiday"];
    
    for day in days {
        // TODO: Match day to determine:
        // - is_business_day: bool
        // - delivery_estimate: "same day", "next business day", "2 business days"
        // - surcharge: 0.0, 0.5, or 1.0
        
        // Business days: Mon-Fri, same day delivery, no surcharge
        // Saturday: not business day, 2 business days, $0.50 surcharge
        // Sunday/Holiday: not business day, 2 business days, $1.00 surcharge
        
        let (is_business, delivery, surcharge) = (false, "", 0.0); // Replace with match
        
        println!("{}: business={}, delivery={}, surcharge=${:.2}", 
                 day, is_business, delivery, surcharge);
    }

    // Task 8: Response Handler
    // Match on response status and handle accordingly
    println!("\n--- Task 8: Response Handler ---");
    
    enum ApiResponse {
        Success(String),
        NotFound,
        Unauthorized,
        RateLimited { retry_after: u32 },
        ServerError(u16),
    }
    
    let responses = [
        ApiResponse::Success(String::from("Message sent")),
        ApiResponse::NotFound,
        ApiResponse::Unauthorized,
        ApiResponse::RateLimited { retry_after: 60 },
        ApiResponse::ServerError(503),
    ];
    
    for response in &responses {
        // TODO: Match on ApiResponse enum:
        // - Success(msg): print the message
        // - NotFound: print "Resource not found"
        // - Unauthorized: print "Authentication required"
        // - RateLimited { retry_after }: print "Rate limited, retry in X seconds"
        // - ServerError(code): print "Server error: CODE"
        
        let message = ""; // Replace with match
        
        println!("Response: {}", message);
    }

    // Task 9: Character Classifier
    // Classify ASCII characters
    println!("\n--- Task 9: Character Classifier ---");
    
    let chars = ['A', 'z', '5', '@', ' ', '\n', 'M', '0'];
    
    for c in chars {
        // TODO: Match character to type:
        // - 'A'..='Z': "Uppercase letter"
        // - 'a'..='z': "Lowercase letter"
        // - '0'..='9': "Digit"
        // - ' ': "Space"
        // - '\n' | '\t': "Whitespace"
        // - _: "Special character"
        
        let char_type = ""; // Replace with match
        
        println!("'{}' -> {}", 
                 if c == '\n' { "\\n" } else if c == '\t' { "\\t" } 
                 else if c == ' ' { "space" } else { 
                     c.to_string().as_str() 
                 }, 
                 char_type);
    }

    // Task 10: Plan Selector
    // Return plan details using match as expression
    println!("\n--- Task 10: Plan Selector ---");
    
    let selected_plan = "gold";
    
    // TODO: Create a struct Plan with name, price, and messages
    struct Plan {
        name: String,
        price: f64,
        messages: u32,
    }
    
    // TODO: Use match to create the appropriate Plan
    // - "free": $0, 100 messages
    // - "basic": $9.99, 1000 messages
    // - "pro": $29.99, 5000 messages
    // - "gold": $79.99, 20000 messages
    // - _: $0, 0 messages (invalid)
    
    let plan = Plan {
        name: String::new(),
        price: 0.0,
        messages: 0,
    }; // Replace with match expression
    
    println!("Plan: {}", plan.name);
    println!("Price: ${:.2}/month", plan.price);
    println!("Messages: {}/month", plan.messages);

    println!("\n=== Match System Complete ===");
}

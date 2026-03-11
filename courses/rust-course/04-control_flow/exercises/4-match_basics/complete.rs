// Textio SMS API - Match Basics Exercise - Complete Solution

fn main() {
    println!("=== Textio Match Expression System ===\n");

    // Task 1: HTTP Status Handler
    println!("--- Task 1: HTTP Status Handler ---");
    
    let status_codes = [200, 201, 400, 401, 404, 429, 500, 503, 502];
    
    for code in status_codes {
        let (category, should_retry) = match code {
            200 | 201 | 204 => ("Success", false),
            400 | 401 | 403 | 404 => ("Client Error", false),
            429 => ("Rate Limited", true),
            500 | 502 | 503 | 504 => ("Server Error", true),
            _ => ("Unknown", false),
        };
        
        println!("HTTP {}: {} (retry: {})", code, category, should_retry);
    }

    // Task 2: Message Router
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
        let gateway = match country {
            "US" | "CA" => "Americas Gateway",
            "UK" => "UK Gateway",
            "DE" | "FR" | "IT" | "ES" => "EU Gateway",
            "JP" | "KR" => "Asia-Pacific Gateway",
            "CN" | "IN" => "Asia Gateway",
            _ => "International Gateway",
        };
        
        println!("{} ({}) -> {}", phone, country, gateway);
    }

    // Task 3: Priority Handler
    println!("\n--- Task 3: Priority Handler ---");
    
    let priorities = [1, 2, 3, 4, 5, 0];
    
    for priority in priorities {
        let (queue, timeout, retries) = match priority {
            1 => ("critical", 100, 10),
            2 => ("high", 500, 5),
            3 => ("normal", 2000, 3),
            4 => ("low", 5000, 2),
            5 => ("bulk", 10000, 1),
            _ => ("default", 1000, 3),
        };
        
        println!("Priority {}: Queue={}, Timeout={}ms, Retries={}", 
                 priority, queue, timeout, retries);
    }

    // Task 4: Subscription Tier Manager
    println!("\n--- Task 4: Subscription Manager ---");
    
    let tiers = ["free", "bronze", "silver", "gold", "platinum", "enterprise"];
    
    for tier in tiers {
        let (monthly, daily, features) = match tier {
            "free" => (100, 10, 1),
            "bronze" => (500, 50, 3),
            "silver" => (2000, 200, 5),
            "gold" => (10000, 1000, 10),
            "platinum" => (50000, 5000, 20),
            _ => (0, 0, 0),
        };
        
        println!("{}: {}/month, {}/day, {} features", 
                 tier, monthly, daily, features);
    }

    // Task 5: Message Length Classifier
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
        let classification = match length {
            0 => "Empty",
            1..=50 => "Short",
            51..=100 => "Medium",
            101..=160 => "Standard",
            161..=320 => "Extended",
            _ => "Long",
        };
        
        let parts = match length {
            0 => 0,
            n => (n + 159) / 160,
        };
        
        println!("{} ({} chars): {}, {} part(s)", 
                 name, length, classification, parts);
    }

    // Task 6: Error Classifier
    println!("\n--- Task 6: Error Classifier ---");
    
    let error_codes = [1001, 1002, 2001, 2002, 3001, 4001, 9999];
    
    for code in error_codes {
        let (error_type, recoverable) = match code {
            1000..=1999 => ("Network Error", true),
            2000..=2999 => ("Validation Error", false),
            3000..=3999 => ("Rate Limit Error", true),
            4000..=4999 => ("Authentication Error", false),
            _ => ("Unknown Error", false),
        };
        
        println!("Error {}: {} (recoverable: {})", 
                 code, error_type, recoverable);
    }

    // Task 7: Day of Week Handler
    println!("\n--- Task 7: Business Day Handler ---");
    
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", 
                "Friday", "Saturday", "Sunday", "Holiday"];
    
    for day in days {
        let (is_business, delivery, surcharge) = match day {
            "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => 
                (true, "same day", 0.0),
            "Saturday" => (false, "2 business days", 0.5),
            "Sunday" | "Holiday" => (false, "2 business days", 1.0),
            _ => (false, "unknown", 0.0),
        };
        
        println!("{}: business={}, delivery={}, surcharge=${:.2}", 
                 day, is_business, delivery, surcharge);
    }

    // Task 8: Response Handler
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
        let message = match response {
            ApiResponse::Success(msg) => format!("Success: {}", msg),
            ApiResponse::NotFound => String::from("Resource not found"),
            ApiResponse::Unauthorized => String::from("Authentication required"),
            ApiResponse::RateLimited { retry_after } => 
                format!("Rate limited, retry in {} seconds", retry_after),
            ApiResponse::ServerError(code) => 
                format!("Server error: {}", code),
        };
        
        println!("Response: {}", message);
    }

    // Task 9: Character Classifier
    println!("\n--- Task 9: Character Classifier ---");
    
    let chars = ['A', 'z', '5', '@', ' ', '\n', 'M', '0'];
    
    for c in chars {
        let char_type = match c {
            'A'..='Z' => "Uppercase letter",
            'a'..='z' => "Lowercase letter",
            '0'..='9' => "Digit",
            ' ' => "Space",
            '\n' | '\t' => "Whitespace",
            _ => "Special character",
        };
        
        let display = match c {
            '\n' => "\\n",
            '\t' => "\\t",
            ' ' => "space",
            _ => c.to_string().as_str(),
        };
        
        println!("'{}' -> {}", display, char_type);
    }

    // Task 10: Plan Selector
    println!("\n--- Task 10: Plan Selector ---");
    
    let selected_plan = "gold";
    
    struct Plan {
        name: String,
        price: f64,
        messages: u32,
    }
    
    let plan = match selected_plan {
        "free" => Plan {
            name: String::from("Free"),
            price: 0.0,
            messages: 100,
        },
        "basic" => Plan {
            name: String::from("Basic"),
            price: 9.99,
            messages: 1000,
        },
        "pro" => Plan {
            name: String::from("Professional"),
            price: 29.99,
            messages: 5000,
        },
        "gold" => Plan {
            name: String::from("Gold"),
            price: 79.99,
            messages: 20000,
        },
        _ => Plan {
            name: String::from("Invalid"),
            price: 0.0,
            messages: 0,
        },
    };
    
    println!("Plan: {}", plan.name);
    println!("Price: ${:.2}/month", plan.price);
    println!("Messages: {}/month", plan.messages);

    println!("\n=== Match System Complete ===");
}

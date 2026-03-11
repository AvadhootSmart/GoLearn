// Textio SMS API - If/Else Exercise - Complete Solution

fn main() {
    println!("=== Textio SMS Message Router ===\n");

    // Task 1: Message Routing
    let phone_number = "+1-555-0123";
    let country_code = &phone_number[0..2];
    
    let gateway = if country_code == "+1" {
        "US_GATEWAY"
    } else if country_code == "+44" {
        "UK_GATEWAY"
    } else if country_code == "+33" || country_code == "+49" || country_code == "+39" {
        "EU_GATEWAY"
    } else if country_code == "+81" || country_code == "+86" {
        "ASIA_GATEWAY"
    } else {
        "INTERNATIONAL"
    };
    
    println!("Phone: {}", phone_number);
    println!("Country Code: {}", country_code);
    println!("Gateway: {}", gateway);
    
    // Task 2: Message Pricing
    let message_length = 165;
    let is_premium_customer = true;
    let base_rate = 0.01;
    
    let length_multiplier = if message_length > 160 {
        1.5
    } else {
        1.0
    };
    
    let customer_discount = if is_premium_customer {
        0.8
    } else {
        1.0
    };
    
    let total_cost = message_length as f64 * base_rate * length_multiplier * customer_discount;
    
    println!("\nMessage Length: {} chars", message_length);
    println!("Premium Customer: {}", is_premium_customer);
    println!("Total Cost: ${:.4}", total_cost);
    
    // Task 3: Message Validation
    let message = "Hello from Textio!";
    let max_length = 160;
    let min_length = 1;
    
    let validation_status = if message.len() > max_length {
        "Too long"
    } else if message.len() < min_length {
        "Too short"
    } else {
        "Valid"
    };
    
    println!("\nMessage: \"{}\"", message);
    println!("Length: {}", message.len());
    println!("Status: {}", validation_status);
    
    // Task 4: Customer Tier Assignment
    let monthly_messages = 5500;
    let account_age_months = 6;
    
    let (base_tier, base_discount) = if monthly_messages > 10000 {
        ("Platinum", 25)
    } else if monthly_messages > 5000 {
        ("Gold", 15)
    } else if monthly_messages > 1000 {
        ("Silver", 10)
    } else if monthly_messages > 100 {
        ("Bronze", 5)
    } else {
        ("Free", 0)
    };
    
    let loyalty_bonus = if account_age_months > 12 {
        5
    } else {
        0
    };
    
    let (tier, discount) = (base_tier, base_discount + loyalty_bonus);
    
    println!("\nMonthly Messages: {}", monthly_messages);
    println!("Account Age: {} months", account_age_months);
    println!("Tier: {}", tier);
    println!("Discount: {}%", discount);
    
    // Task 5: Delivery Priority
    let is_urgent = true;
    let is_business_hours = true;
    let has_priority_subscription = false;
    
    let priority_status = if is_urgent || has_priority_subscription {
        "Immediate"
    } else if is_business_hours && !is_urgent {
        "Same Day"
    } else {
        "Next Day"
    };
    
    println!("\nUrgent: {}", is_urgent);
    println!("Business Hours: {}", is_business_hours);
    println!("Priority Sub: {}", has_priority_subscription);
    println!("Priority: {}", priority_status);
    
    // Task 6: Conditional Message Formatting
    let messages_sent = 950;
    let messages_limit = 1000;
    let remaining = messages_limit - messages_sent;
    let usage_percent = (messages_sent as f64 / messages_limit as f64) * 100.0;
    
    let warning_level = if usage_percent >= 95.0 {
        "CRITICAL"
    } else if usage_percent >= 80.0 {
        "WARNING"
    } else {
        "OK"
    };
    
    let status_message = if remaining > 0 {
        format!("{} - {} messages remaining", warning_level, remaining)
    } else {
        format!("{} - Limit reached!", warning_level)
    };
    
    println!("\nMessages: {}/{}", messages_sent, messages_limit);
    println!("Usage: {:.1}%", usage_percent);
    println!("Status: {}", warning_level);
    println!("Message: {}", status_message);
    
    println!("\n=== Routing Complete ===");
}

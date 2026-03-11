// Textio SMS API - If/Else Exercise
// Complete the conditional logic to route messages and calculate pricing

fn main() {
    println!("=== Textio SMS Message Router ===\n");

    // Task 1: Message Routing
    // Route messages to different gateways based on country code
    let phone_number = "+1-555-0123";
    
    // Extract country code (first 2 characters)
    let country_code = &phone_number[0..2];
    
    // TODO: Create a variable `gateway` that contains:
    // - "US_GATEWAY" for "+1"
    // - "UK_GATEWAY" for "+44"
    // - "EU_GATEWAY" for "+33", "+49", or "+39"
    // - "ASIA_GATEWAY" for "+81" or "+86"
    // - "INTERNATIONAL" for anything else
    let gateway = ""; // Replace with your if/else expression
    
    println!("Phone: {}", phone_number);
    println!("Country Code: {}", country_code);
    println!("Gateway: {}", gateway);
    
    // Task 2: Message Pricing
    // Calculate the cost of sending a message
    let message_length = 165;
    let is_premium_customer = true;
    let base_rate = 0.01;
    
    // TODO: Calculate total cost based on:
    // - Messages over 160 characters cost 1.5x
    // - Premium customers get 20% discount
    // - Use an if expression to calculate the cost
    let length_multiplier = 0.0; // Replace with your logic
    let customer_discount = 0.0; // Replace with your logic
    
    let total_cost = message_length as f64 * base_rate * length_multiplier * customer_discount;
    
    println!("\nMessage Length: {} chars", message_length);
    println!("Premium Customer: {}", is_premium_customer);
    println!("Total Cost: ${:.4}", total_cost);
    
    // Task 3: Message Validation
    // Validate messages and return a status
    let message = "Hello from Textio!";
    let max_length = 160;
    let min_length = 1;
    
    // TODO: Create validation_status using if/else if/else
    // - "Too long" if over max_length
    // - "Too short" if under min_length
    // - "Valid" otherwise
    let validation_status = ""; // Replace with your logic
    
    println!("\nMessage: \"{}\"", message);
    println!("Length: {}", message.len());
    println!("Status: {}", validation_status);
    
    // Task 4: Customer Tier Assignment
    // Assign customer tier based on monthly message volume
    let monthly_messages = 5500;
    let account_age_months = 6;
    
    // TODO: Create a tuple (tier_name, discount_percent) using if expressions
    // Tiers based on monthly messages:
    // - "Platinum": > 10000 messages, discount 25%
    // - "Gold": > 5000 messages, discount 15%
    // - "Silver": > 1000 messages, discount 10%
    // - "Bronze": > 100 messages, discount 5%
    // - "Free": anything else, discount 0%
    // BONUS: Add 5% extra discount if account_age_months > 12
    let (tier, discount) = ("", 0); // Replace with your logic
    
    println!("\nMonthly Messages: {}", monthly_messages);
    println!("Account Age: {} months", account_age_months);
    println!("Tier: {}", tier);
    println!("Discount: {}%", discount);
    
    // Task 5: Delivery Priority
    // Determine message delivery priority
    let is_urgent = true;
    let is_business_hours = true;
    let has_priority_subscription = false;
    
    // TODO: Create priority_status using compound conditions (&&, ||)
    // - "Immediate" if is_urgent OR has_priority_subscription
    // - "Same Day" if is_business_hours AND NOT urgent
    // - "Next Day" for anything else
    let priority_status = ""; // Replace with your logic
    
    println!("\nUrgent: {}", is_urgent);
    println!("Business Hours: {}", is_business_hours);
    println!("Priority Sub: {}", has_priority_subscription);
    println!("Priority: {}", priority_status);
    
    // Task 6: Conditional Message Formatting
    // Use if as an expression to build a status message
    let messages_sent = 950;
    let messages_limit = 1000;
    let remaining = messages_limit - messages_sent;
    let usage_percent = (messages_sent as f64 / messages_limit as f64) * 100.0;
    
    // TODO: Create a warning_level string using if expression
    // - "CRITICAL" if usage >= 95%
    // - "WARNING" if usage >= 80%
    // - "OK" otherwise
    let warning_level = ""; // Replace with your logic
    
    // TODO: Create a status_message that includes:
    // - The warning level
    // - Number of remaining messages (if > 0)
    // - "Limit reached!" (if remaining == 0)
    // Use if expressions embedded in the string
    let status_message = ""; // Replace with your logic
    
    println!("\nMessages: {}/{}", messages_sent, messages_limit);
    println!("Usage: {:.1}%", usage_percent);
    println!("Status: {}", warning_level);
    println!("Message: {}", status_message);
    
    println!("\n=== Routing Complete ===");
}

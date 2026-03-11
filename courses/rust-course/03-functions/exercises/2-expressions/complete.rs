fn main() {
    println!("Length 50: {}", classify_message_length(50));
    println!("Length 100: {}", classify_message_length(100));
    println!("Length 150: {}", classify_message_length(150));
    
    println!("5 segments at $0.05: ${:.2}", calculate_segment_cost(5, 0.05));
    println!("15 segments at $0.05: ${:.2}", calculate_segment_cost(15, 0.05));
    
    println!("{}", format_delivery_report(12345, true, 1));
    println!("{}", format_delivery_report(67890, false, 3));
    
    println!("500 messages: {}", get_rate_tier(500));
    println!("5000 messages: {}", get_rate_tier(5000));
    println!("50000 messages: {}", get_rate_tier(50000));
    println!("500000 messages: {}", get_rate_tier(500000));
}

fn classify_message_length(len: usize) -> &'static str {
    match len {
        0..=70 => "short",
        71..=134 => "medium",
        _ => "long",
    }
}

fn calculate_segment_cost(segments: u32, rate: f64) -> f64 {
    let base_cost = {
        let subtotal = segments as f64 * rate;
        if segments >= 10 {
            subtotal * 0.9
        } else {
            subtotal
        }
    };
    base_cost
}

fn format_delivery_report(id: u64, success: bool, attempts: u8) -> String {
    let status = if success { "DELIVERED" } else { "FAILED" };
    format!("Message #{}: {} ({} attempts)", id, status, attempts)
}

fn get_rate_tier(monthly_volume: u32) -> &'static str {
    match monthly_volume {
        0..=1000 => "tier1",
        1001..=10000 => "tier2",
        10001..=100000 => "tier3",
        _ => "enterprise",
    }
}

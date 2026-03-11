fn main() {
    println!("Validating 'Textio': {:?}", validate_sender_id("Textio"));
    println!("Validating '': {:?}", validate_sender_id(""));
    println!("Validating 'ThisIsTooLong123': {:?}", validate_sender_id("ThisIsTooLong123"));
    println!("Validating 'Bad-Chars!': {:?}", validate_sender_id("Bad-Chars!"));
    
    let delivery_results = [true, true, false, true, false, false, true];
    let (delivered, failed, rate) = calculate_delivery_stats(&delivery_results);
    println!("Delivery stats: {} delivered, {} failed, {:.1}% success rate", 
             delivered, failed, rate);
    
    let empty: [bool; 0] = [];
    let (d, f, r) = calculate_delivery_stats(&empty);
    println!("Empty stats: {} delivered, {} failed, {:.1}% success rate", d, f, r);
    
    let indices = [0, 1, 2, 3, 4];
    let delivered = [true, false, true, false, true];
    let undelivered = find_undelivered(&indices, &delivered);
    println!("Undelivered message indices: {:?}", undelivered);
    
    let mismatch_indices = [0, 1, 2];
    let mismatch_delivered = [true, false];
    let mismatch_result = find_undelivered(&mismatch_indices, &mismatch_delivered);
    println!("Mismatch result: {:?}", mismatch_result);
    
    let rates = [(100.0, 0.05), (250.0, 0.04), (500.0, 0.03)];
    println!("{}", format_cost_table(&rates));
    
    let empty_rates: [(f64, f64); 0] = [];
    println!("Empty table:\n{}", format_cost_table(&empty_rates));
}

fn validate_sender_id(id: &str) -> Result<&'static str, &'static str> {
    if id.is_empty() {
        return Err("Sender ID cannot be empty");
    }
    if id.len() > 11 {
        return Err("Sender ID cannot exceed 11 characters");
    }
    if !id.chars().all(|c| c.is_alphanumeric()) {
        return Err("Sender ID must contain only alphanumeric characters");
    }
    Ok("valid")
}

fn calculate_delivery_stats(messages: &[bool]) -> (u32, u32, f64) {
    if messages.is_empty() {
        return (0, 0, 0.0);
    }
    
    let delivered = messages.iter().filter(|&&b| b).count() as u32;
    let failed = messages.len() as u32 - delivered;
    let rate = (delivered as f64 / messages.len() as f64) * 100.0;
    
    (delivered, failed, rate)
}

fn find_undelivered(indices: &[usize], delivered: &[bool]) -> Vec<usize> {
    if indices.len() != delivered.len() {
        return Vec::new();
    }
    
    indices
        .iter()
        .zip(delivered.iter())
        .filter_map(|(&idx, &del)| if !del { Some(idx) } else { None })
        .collect()
}

fn format_cost_table(rates: &[(f64, f64)]) -> String {
    if rates.is_empty() {
        return String::from("No data\n");
    }
    
    let mut result = String::new();
    result.push_str("Qty     Rate    Cost\n");
    result.push_str("-----   -----   ------\n");
    
    let mut total = 0.0;
    for &(qty, rate) in rates {
        let cost = qty * rate;
        total += cost;
        result.push_str(&format!("{:<5.0}   {:.2}   {:.2}\n", qty, rate, cost));
    }
    
    result.push_str("-----           ------\n");
    result.push_str(&format!("Total:          {:.2}\n", total));
    result
}

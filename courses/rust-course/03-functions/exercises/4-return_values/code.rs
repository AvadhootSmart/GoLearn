// Exercise: Return Values
// Complete the functions demonstrating various return patterns

fn main() {
    // Test validate_sender_id
    println!("Validating 'Textio': {:?}", validate_sender_id("Textio"));
    println!("Validating '': {:?}", validate_sender_id(""));
    println!("Validating 'ThisIsTooLong123': {:?}", validate_sender_id("ThisIsTooLong123"));
    println!("Validating 'Bad-Chars!': {:?}", validate_sender_id("Bad-Chars!"));
    
    // Test calculate_delivery_stats
    let delivery_results = [true, true, false, true, false, false, true];
    let (delivered, failed, rate) = calculate_delivery_stats(&delivery_results);
    println!("Delivery stats: {} delivered, {} failed, {:.1}% success rate", 
             delivered, failed, rate);
    
    let empty: [bool; 0] = [];
    let (d, f, r) = calculate_delivery_stats(&empty);
    println!("Empty stats: {} delivered, {} failed, {:.1}% success rate", d, f, r);
    
    // Test find_undelivered
    let indices = [0, 1, 2, 3, 4];
    let delivered = [true, false, true, false, true];
    let undelivered = find_undelivered(&indices, &delivered);
    println!("Undelivered message indices: {:?}", undelivered);
    
    let mismatch_indices = [0, 1, 2];
    let mismatch_delivered = [true, false];
    let mismatch_result = find_undelivered(&mismatch_indices, &mismatch_delivered);
    println!("Mismatch result: {:?}", mismatch_result);
    
    // Test format_cost_table
    let rates = [(100.0, 0.05), (250.0, 0.04), (500.0, 0.03)];
    println!("{}", format_cost_table(&rates));
    
    let empty_rates: [(f64, f64); 0] = [];
    println!("Empty table:\n{}", format_cost_table(&empty_rates));
}

// Create validate_sender_id
// Must be 1-11 alphanumeric characters
// Return Ok("valid") or Err with reason string
// Use early returns for validation
fn validate_sender_id(id: &str) -> Result<&'static str, &'static str> {
    
}


// Create calculate_delivery_stats
// Input: slice of delivery success booleans
// Return: (delivered_count, failed_count, success_rate)
// Handle empty slice: return (0, 0, 0.0)
fn calculate_delivery_stats(messages: &[bool]) -> (u32, u32, f64) {
    
}


// Create find_undelivered
// Return indices where delivered is false
// Early return empty vec if slice lengths don't match
fn find_undelivered(indices: &[usize], delivered: &[bool]) -> Vec<usize> {
    
}


// Create format_cost_table
// Input: slice of (quantity, rate) tuples
// Return formatted table:
// Qty     Rate    Cost
// -----   -----   -----
// 100     0.05    5.00
// 250     0.04    10.00
// -----          ------
// Total:          15.00
//
// Use early return for empty input: return "No data\n"
fn format_cost_table(rates: &[(f64, f64)]) -> String {
    
}

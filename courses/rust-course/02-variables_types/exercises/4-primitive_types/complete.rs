// Exercise 4: Primitive Types - Complete Solution

fn main() {
    // ============================================
    // PART 1: Signed Integers
    // ============================================
    
    let tiny_signed: i8 = -10;
    let small_signed: i16 = -1000;
    let standard_signed: i32 = -100000;
    let large_signed: i64 = -10000000000;
    let huge_signed: i128 = -100000000000000000000;
    let pointer_signed: isize = -100;
    
    println!("=== Signed Integers ===");
    println!("i8: {}", tiny_signed);
    println!("i16: {}", small_signed);
    println!("i32: {}", standard_signed);
    println!("i64: {}", large_signed);
    println!("i128: {}", huge_signed);
    println!("isize: {}", pointer_signed);
    println!();
    
    // ============================================
    // PART 2: Unsigned Integers
    // ============================================
    
    let byte_val: u8 = 255;
    let word_val: u16 = 60000;
    let dword_val: u32 = 4000000000;
    let qword_val: u64 = 18000000000000000000;
    let huge_val: u128 = 340282366920938463463374607431768211455;
    let pointer_unsigned: usize = 100;
    
    println!("=== Unsigned Integers ===");
    println!("u8: {}", byte_val);
    println!("u16: {}", word_val);
    println!("u32: {}", dword_val);
    println!("u64: {}", qword_val);
    println!("u128: {}", huge_val);
    println!("usize: {}", pointer_unsigned);
    println!();
    
    // ============================================
    // PART 3: Floating-Point Types
    // ============================================
    
    let single_precision: f32 = 3.14159;
    let double_precision: f64 = 3.141592653589793;
    let cost_per_message: f64 = 0.05;
    let monthly_revenue: f64 = 15000.50;
    
    println!("=== Floating-Point Types ===");
    println!("f32: {}", single_precision);
    println!("f64: {}", double_precision);
    println!("Cost per SMS: ${:.2}", cost_per_message);
    println!("Monthly revenue: ${:.2}", monthly_revenue);
    println!();
    
    // ============================================
    // PART 4: Boolean and Character Types
    // ============================================
    
    let is_delivered: bool = true;
    let is_premium: bool = false;
    let has_attachment: bool = false;
    
    let message_type: char = 'S';
    let priority: char = 'H';
    let emoji: char = '✓';
    
    println!("=== Boolean and Character Types ===");
    println!("is_delivered: {}", is_delivered);
    println!("is_premium: {}", is_premium);
    println!("has_attachment: {}", has_attachment);
    println!("message_type: {}", message_type);
    println!("priority: {}", priority);
    println!("emoji: {}", emoji);
    println!();
    
    // ============================================
    // PART 5: String Slice Type
    // ============================================
    
    let api_endpoint: &str = "https://api.textio.io/v2";
    let default_message: &str = "Your code is: 123456";
    let status_text: &str = "Message delivered successfully";
    
    println!("=== String Slice Type ===");
    println!("API: {}", api_endpoint);
    println!("Default: {}", default_message);
    println!("Status: {}", status_text);
    println!();
    
    // ============================================
    // PART 6: Type Casting with 'as'
    // ============================================
    
    println!("=== Type Casting ===");
    
    let small: u8 = 255;
    let larger: u32 = small as u32;
    println!("u8 {} -> u32 {}", small, larger);
    
    let standard: i32 = -100000;
    let bigger: i64 = standard as i64;
    println!("i32 {} -> i64 {}", standard, bigger);
    
    let float_val: f64 = 3.99;
    let int_val: i32 = float_val as i32;
    println!("f64 {} -> i32 {} (truncated)", float_val, int_val);
    
    let letter: char = 'A';
    let code: u32 = letter as u32;
    println!("char '{}' -> u32 {} (Unicode)", letter, code);
    
    let code_point: u32 = 128512;
    let emoji_char: char = code_point as char;
    println!("u32 {} -> char '{}' (Unicode)", code_point, emoji_char);
    println!();
    
    // ============================================
    // PART 7: Overflow Handling
    // ============================================
    
    println!("=== Overflow Handling ===");
    
    let max_byte: u8 = 255;
    
    // Wrapping: wraps around to 0
    let wrapped = max_byte.wrapping_add(1);
    println!("255 wrapping_add(1) = {}", wrapped);
    
    // Saturating: stays at max
    let saturated = max_byte.saturating_add(1);
    println!("255 saturating_add(1) = {}", saturated);
    
    // Checked: returns None on overflow
    let checked = max_byte.checked_add(1);
    println!("255 checked_add(1) = {:?}", checked);
    
    // Overflowing: returns (result, did_overflow)
    let (result, overflowed) = max_byte.overflowing_add(1);
    println!("255 overflowing_add(1) = ({}, {})", result, overflowed);
    println!();
    
    // ============================================
    // PART 8: Debug Printing
    // ============================================
    
    println!("=== Debug Printing ===");
    
    let debug_number: i32 = 42;
    let debug_text: &str = "Hello, Textio!";
    let debug_flag: bool = true;
    
    // Using {:?} format
    println!("Debug number: {:?}", debug_number);
    println!("Debug text: {:?}", debug_text);
    println!("Debug flag: {:?}", debug_flag);
    
    // Using dbg! macro
    dbg!(debug_number);
    dbg!(debug_text);
    dbg!(debug_flag);
    
    // dbg! in expression
    let computed = dbg!(debug_number * 2);
    println!("Computed: {}", computed);
}

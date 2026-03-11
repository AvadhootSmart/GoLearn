// Exercise 4: if let and let else
// Concise pattern matching for Textio

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Sms { to: String, body: String },
    Mms { to: String, body: String, media_url: String },
    Scheduled { to: String, body: String, send_at: u64 },
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub credits: u32,
}

#[derive(Debug)]
pub enum SendResult {
    Success { id: String, segments: u8 },
    Failed { code: u16, reason: String },
}

// TODO: Implement extract_sms_body using if let
// Returns Some(body) if message is Sms, None otherwise
pub fn extract_sms_body(message: &Message) -> Option<&str> {
    todo!()
}

// TODO: Implement extract_mms_url using if let with else
// Returns the URL if MMS, or "Not an MMS" if not
pub fn extract_mms_url(message: &Message) -> &str {
    todo!()
}

// TODO: Implement get_scheduled_time using let else
// Returns the send_at time or 0 if not a scheduled message
pub fn get_scheduled_time(message: Message) -> u64 {
    todo!()
}

// TODO: Implement validate_scheduled using let else with early return
// Returns Ok(()) if valid scheduled message, Err if not
pub fn validate_scheduled(message: &Message, min_delay: u64, current_time: u64) -> Result<(), String> {
    todo!()
}

// TODO: Implement get_user_phone using let else
// Returns the phone number or an error message
pub fn get_user_phone(user: &User) -> Result<&str, String> {
    todo!()
}

// TODO: Implement process_send_result using if let
// Prints success message if Success variant, does nothing otherwise
pub fn process_send_result(result: &SendResult) {
    todo!()
}

// TODO: Implement count_successful_messages using if let
// Count how many results are Success variants
pub fn count_successful_messages(results: &[SendResult]) -> usize {
    todo!()
}

// TODO: Implement find_first_success using while let
// Iterate through results and return the first Success id
pub fn find_first_success(results: &[SendResult]) -> Option<&str> {
    todo!()
}

// TODO: Implement extract_all_bodies using a loop with if let
// Extract body from each Message::Sms variant
pub fn extract_all_bodies(messages: &[Message]) -> Vec<&str> {
    todo!()
}

// TODO: Implement is_success using matches! macro
// Returns true if SendResult is Success variant
pub fn is_success(result: &SendResult) -> bool {
    todo!()
}

// TODO: Implement has_phone using matches! macro
// Returns true if user has a phone number
pub fn has_phone(user: &User) -> bool {
    todo!()
}

// TODO: Implement process_user_message using let else chain
// Gets user by id, validates phone, and sends message
pub fn process_user_message(
    users: &HashMap<u32, User>,
    user_id: u32,
    body: &str,
) -> Result<String, String> {
    // Use let else to:
    // 1. Get user from map
    // 2. Get user's phone
    // 3. Return success message with phone number
    todo!()
}

// TODO: Implement classify_and_handle using if let with guards
// Handle different SendResult cases with specific logic:
// - Success with segments > 1: "Multi-part message sent"
// - Success with segments == 1: "Message sent"
// - Failed with code >= 500: "Server error"
// - Failed otherwise: "Client error"
pub fn classify_and_handle(result: &SendResult) -> &'static str {
    todo!()
}

// TODO: Implement extract_recipient_info using if let with struct destructuring
// Returns (to, is_mms) tuple if valid message
pub fn extract_recipient_info(message: &Message) -> Option<(&str, bool)> {
    todo!()
}

// TODO: Implement process_nested_option using if let
// Handle Option<Option<String>> - return inner if both Some
pub fn process_nested_option(nested: Option<Option<String>>) -> Option<String> {
    todo!()
}

fn main() {
    // Test extract_sms_body
    println!("=== extract_sms_body ===");
    let sms = Message::Sms { 
        to: String::from("+1555123456"), 
        body: String::from("Hello!") 
    };
    let mms = Message::Mms { 
        to: String::from("+1555987654"), 
        body: String::from("Image"), 
        media_url: String::from("https://example.com/img.jpg") 
    };
    println!("SMS body: {:?}", extract_sms_body(&sms));
    println!("MMS body: {:?}", extract_sms_body(&mms));

    // Test extract_mms_url
    println!("\n=== extract_mms_url ===");
    println!("MMS URL: {}", extract_mms_url(&mms));
    println!("SMS URL: {}", extract_mms_url(&sms));

    // Test get_scheduled_time
    println!("\n=== get_scheduled_time ===");
    let scheduled = Message::Scheduled { 
        to: String::from("+1555111111"), 
        body: String::from("Later"), 
        send_at: 1700000000 
    };
    println!("Scheduled time: {}", get_scheduled_time(scheduled));
    println!("SMS time: {}", get_scheduled_time(sms.clone()));

    // Test validate_scheduled
    println!("\n=== validate_scheduled ===");
    let valid_scheduled = Message::Scheduled { 
        to: String::from("+1555111111"), 
        body: String::from("Later"), 
        send_at: 2000 
    };
    println!("Valid (min 1000, current 1000): {:?}", validate_scheduled(&valid_scheduled, 1000, 1000));
    println!("Invalid (min 1000, current 1500): {:?}", validate_scheduled(&valid_scheduled, 1000, 1500));
    println!("SMS (min 1000, current 1000): {:?}", validate_scheduled(&sms, 1000, 1000));

    // Test get_user_phone
    println!("\n=== get_user_phone ===");
    let user_with_phone = User { 
        id: 1, 
        username: String::from("alice"), 
        phone: Some(String::from("+1555111111")), 
        email: None,
        credits: 100 
    };
    let user_no_phone = User { 
        id: 2, 
        username: String::from("bob"), 
        phone: None, 
        email: Some(String::from("bob@example.com")),
        credits: 50 
    };
    println!("With phone: {:?}", get_user_phone(&user_with_phone));
    println!("Without phone: {:?}", get_user_phone(&user_no_phone));

    // Test process_send_result
    println!("\n=== process_send_result ===");
    let success = SendResult::Success { id: String::from("msg_123"), segments: 1 };
    let failed = SendResult::Failed { code: 500, reason: String::from("Server error") };
    print!("Success: ");
    process_send_result(&success);
    print!("Failed: ");
    process_send_result(&failed);

    // Test count_successful_messages
    println!("\n=== count_successful_messages ===");
    let results = vec![
        SendResult::Success { id: String::from("msg_1"), segments: 1 },
        SendResult::Failed { code: 400, reason: String::from("Bad request") },
        SendResult::Success { id: String::from("msg_2"), segments: 2 },
    ];
    println!("Success count: {}", count_successful_messages(&results));

    // Test find_first_success
    println!("\n=== find_first_success ===");
    let all_failed = vec![
        SendResult::Failed { code: 500, reason: String::from("Error 1") },
        SendResult::Failed { code: 502, reason: String::from("Error 2") },
    ];
    println!("First success in mixed: {:?}", find_first_success(&results));
    println!("First success in all failed: {:?}", find_first_success(&all_failed));

    // Test extract_all_bodies
    println!("\n=== extract_all_bodies ===");
    let messages = vec![
        Message::Sms { to: String::from("+1"), body: String::from("First") },
        Message::Mms { to: String::from("+2"), body: String::from("Second"), media_url: String::from("url") },
        Message::Sms { to: String::from("+3"), body: String::from("Third") },
    ];
    println!("All SMS bodies: {:?}", extract_all_bodies(&messages));

    // Test is_success and has_phone
    println!("\n=== is_success / has_phone ===");
    println!("is_success(success): {}", is_success(&success));
    println!("is_success(failed): {}", is_success(&failed));
    println!("has_phone(with phone): {}", has_phone(&user_with_phone));
    println!("has_phone(no phone): {}", has_phone(&user_no_phone));

    // Test classify_and_handle
    println!("\n=== classify_and_handle ===");
    let multi_part = SendResult::Success { id: String::from("m1"), segments: 3 };
    let single = SendResult::Success { id: String::from("m2"), segments: 1 };
    let server_err = SendResult::Failed { code: 503, reason: String::from("Unavailable") };
    let client_err = SendResult::Failed { code: 400, reason: String::from("Bad request") };
    println!("Multi-part: {}", classify_and_handle(&multi_part));
    println!("Single: {}", classify_and_handle(&single));
    println!("Server error: {}", classify_and_handle(&server_err));
    println!("Client error: {}", classify_and_handle(&client_err));

    // Test extract_recipient_info
    println!("\n=== extract_recipient_info ===");
    println!("SMS info: {:?}", extract_recipient_info(&sms));
    println!("MMS info: {:?}", extract_recipient_info(&mms));
    println!("Scheduled info: {:?}", extract_recipient_info(&valid_scheduled));

    // Test process_nested_option
    println!("\n=== process_nested_option ===");
    println!("Some(Some(\"value\")): {:?}", process_nested_option(Some(Some(String::from("value")))));
    println!("Some(None): {:?}", process_nested_option(Some(None)));
    println!("None: {:?}", process_nested_option(None));

    // Test process_user_message
    println!("\n=== process_user_message ===");
    let mut users = HashMap::new();
    users.insert(1, user_with_phone);
    users.insert(2, user_no_phone);
    
    println!("User 1: {:?}", process_user_message(&users, 1, "Hello"));
    println!("User 2: {:?}", process_user_message(&users, 2, "Hello"));
    println!("User 3: {:?}", process_user_message(&users, 3, "Hello"));
}

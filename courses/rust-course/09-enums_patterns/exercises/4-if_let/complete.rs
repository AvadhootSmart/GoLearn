// Exercise 4: if let and let else - Complete Solution

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

pub fn extract_sms_body(message: &Message) -> Option<&str> {
    if let Message::Sms { body, .. } = message {
        Some(body)
    } else {
        None
    }
}

pub fn extract_mms_url(message: &Message) -> &str {
    if let Message::Mms { media_url, .. } = message {
        media_url
    } else {
        "Not an MMS"
    }
}

pub fn get_scheduled_time(message: Message) -> u64 {
    let Message::Scheduled { send_at, .. } = message else {
        return 0;
    };
    send_at
}

pub fn validate_scheduled(message: &Message, min_delay: u64, current_time: u64) -> Result<(), String> {
    let Message::Scheduled { send_at, .. } = message else {
        return Err(String::from("Not a scheduled message"));
    };
    
    if *send_at < current_time + min_delay {
        return Err(format!("Send time must be at least {} seconds in the future", min_delay));
    }
    
    Ok(())
}

pub fn get_user_phone(user: &User) -> Result<&str, String> {
    let Some(phone) = &user.phone else {
        return Err(String::from("User has no phone number"));
    };
    Ok(phone)
}

pub fn process_send_result(result: &SendResult) {
    if let SendResult::Success { id, segments } = result {
        println!("Message {} sent successfully in {} segment(s)", id, segments);
    }
}

pub fn count_successful_messages(results: &[SendResult]) -> usize {
    let mut count = 0;
    for result in results {
        if let SendResult::Success { .. } = result {
            count += 1;
        }
    }
    count
}

pub fn find_first_success(results: &[SendResult]) -> Option<&str> {
    let mut iter = results.iter();
    while let Some(result) = iter.next() {
        if let SendResult::Success { id, .. } = result {
            return Some(id);
        }
    }
    None
}

pub fn extract_all_bodies(messages: &[Message]) -> Vec<&str> {
    let mut bodies = Vec::new();
    for message in messages {
        if let Message::Sms { body, .. } = message {
            bodies.push(body.as_str());
        }
    }
    bodies
}

pub fn is_success(result: &SendResult) -> bool {
    matches!(result, SendResult::Success { .. })
}

pub fn has_phone(user: &User) -> bool {
    matches!(user.phone, Some(_))
}

pub fn process_user_message(
    users: &HashMap<u32, User>,
    user_id: u32,
    body: &str,
) -> Result<String, String> {
    let Some(user) = users.get(&user_id) else {
        return Err(format!("User {} not found", user_id));
    };
    
    let Some(phone) = &user.phone else {
        return Err(String::from("User has no phone number"));
    };
    
    Ok(format!("Message '{}' queued for {} (user: {})", body, phone, user.username))
}

pub fn classify_and_handle(result: &SendResult) -> &'static str {
    if let SendResult::Success { segments, .. } = result {
        if *segments > 1 {
            return "Multi-part message sent";
        } else {
            return "Message sent";
        }
    }
    
    if let SendResult::Failed { code, .. } = result {
        if *code >= 500 {
            return "Server error";
        } else {
            return "Client error";
        }
    }
    
    "Unknown"
}

pub fn extract_recipient_info(message: &Message) -> Option<(&str, bool)> {
    match message {
        Message::Sms { to, .. } => Some((to, false)),
        Message::Mms { to, .. } => Some((to, true)),
        Message::Scheduled { .. } => None,
    }
}

pub fn process_nested_option(nested: Option<Option<String>>) -> Option<String> {
    if let Some(inner) = nested {
        if let Some(value) = inner {
            return Some(value);
        }
    }
    None
}

fn main() {
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

    println!("\n=== extract_mms_url ===");
    println!("MMS URL: {}", extract_mms_url(&mms));
    println!("SMS URL: {}", extract_mms_url(&sms));

    println!("\n=== get_scheduled_time ===");
    let scheduled = Message::Scheduled { 
        to: String::from("+1555111111"), 
        body: String::from("Later"), 
        send_at: 1700000000 
    };
    println!("Scheduled time: {}", get_scheduled_time(scheduled));
    println!("SMS time: {}", get_scheduled_time(sms.clone()));

    println!("\n=== validate_scheduled ===");
    let valid_scheduled = Message::Scheduled { 
        to: String::from("+1555111111"), 
        body: String::from("Later"), 
        send_at: 2000 
    };
    println!("Valid (min 1000, current 1000): {:?}", validate_scheduled(&valid_scheduled, 1000, 1000));
    println!("Invalid (min 1000, current 1500): {:?}", validate_scheduled(&valid_scheduled, 1000, 1500));
    println!("SMS (min 1000, current 1000): {:?}", validate_scheduled(&sms, 1000, 1000));

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

    println!("\n=== process_send_result ===");
    let success = SendResult::Success { id: String::from("msg_123"), segments: 1 };
    let failed = SendResult::Failed { code: 500, reason: String::from("Server error") };
    print!("Success: ");
    process_send_result(&success);
    print!("Failed: ");
    process_send_result(&failed);

    println!("\n=== count_successful_messages ===");
    let results = vec![
        SendResult::Success { id: String::from("msg_1"), segments: 1 },
        SendResult::Failed { code: 400, reason: String::from("Bad request") },
        SendResult::Success { id: String::from("msg_2"), segments: 2 },
    ];
    println!("Success count: {}", count_successful_messages(&results));

    println!("\n=== find_first_success ===");
    let all_failed = vec![
        SendResult::Failed { code: 500, reason: String::from("Error 1") },
        SendResult::Failed { code: 502, reason: String::from("Error 2") },
    ];
    println!("First success in mixed: {:?}", find_first_success(&results));
    println!("First success in all failed: {:?}", find_first_success(&all_failed));

    println!("\n=== extract_all_bodies ===");
    let messages = vec![
        Message::Sms { to: String::from("+1"), body: String::from("First") },
        Message::Mms { to: String::from("+2"), body: String::from("Second"), media_url: String::from("url") },
        Message::Sms { to: String::from("+3"), body: String::from("Third") },
    ];
    println!("All SMS bodies: {:?}", extract_all_bodies(&messages));

    println!("\n=== is_success / has_phone ===");
    println!("is_success(success): {}", is_success(&success));
    println!("is_success(failed): {}", is_success(&failed));
    println!("has_phone(with phone): {}", has_phone(&user_with_phone));
    println!("has_phone(no phone): {}", has_phone(&user_no_phone));

    println!("\n=== classify_and_handle ===");
    let multi_part = SendResult::Success { id: String::from("m1"), segments: 3 };
    let single = SendResult::Success { id: String::from("m2"), segments: 1 };
    let server_err = SendResult::Failed { code: 503, reason: String::from("Unavailable") };
    let client_err = SendResult::Failed { code: 400, reason: String::from("Bad request") };
    println!("Multi-part: {}", classify_and_handle(&multi_part));
    println!("Single: {}", classify_and_handle(&single));
    println!("Server error: {}", classify_and_handle(&server_err));
    println!("Client error: {}", classify_and_handle(&client_err));

    println!("\n=== extract_recipient_info ===");
    println!("SMS info: {:?}", extract_recipient_info(&sms));
    println!("MMS info: {:?}", extract_recipient_info(&mms));
    println!("Scheduled info: {:?}", extract_recipient_info(&valid_scheduled));

    println!("\n=== process_nested_option ===");
    println!("Some(Some(\"value\")): {:?}", process_nested_option(Some(Some(String::from("value")))));
    println!("Some(None): {:?}", process_nested_option(Some(None)));
    println!("None: {:?}", process_nested_option(None));

    println!("\n=== process_user_message ===");
    let mut users = HashMap::new();
    users.insert(1, user_with_phone);
    users.insert(2, user_no_phone);
    
    println!("User 1: {:?}", process_user_message(&users, 1, "Hello"));
    println!("User 2: {:?}", process_user_message(&users, 2, "Hello"));
    println!("User 3: {:?}", process_user_message(&users, 3, "Hello"));
}

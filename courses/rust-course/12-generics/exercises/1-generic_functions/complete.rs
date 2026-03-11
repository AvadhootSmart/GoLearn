// Textio Generic Functions Exercise - Complete Solution
// Complete the generic functions to handle different types in Textio's SMS API

use std::fmt::Display;

#[derive(Debug, Clone)]
struct SmsMessage {
    id: u32,
    content: String,
    recipient: String,
}

#[derive(Debug, Clone)]
struct EmailMessage {
    id: u32,
    subject: String,
    body: String,
    recipient: String,
}

#[derive(Debug, Clone)]
struct PushNotification {
    id: u32,
    title: String,
    message: String,
    device_token: String,
}

fn get_first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn get_last<T>(items: &[T]) -> Option<&T> {
    items.last()
}

fn pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn swap<T, U>(tuple: (T, U)) -> (U, T) {
    (tuple.1, tuple.0)
}

fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    items.iter().max()
}

fn find_min<T: Ord>(items: &[T]) -> Option<&T> {
    items.iter().min()
}

fn contains<T: PartialEq>(items: &[T], value: &T) -> bool {
    items.contains(value)
}

fn print_value<T: Display>(value: T) {
    println!("Value: {}", value);
}

#[derive(Debug, Clone)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error_message: Option<String>,
}

fn create_success_response<T>(data: T) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        data: Some(data),
        error_message: None,
    }
}

fn create_error_response<T>(error_message: String) -> ApiResponse<T> {
    ApiResponse {
        success: false,
        data: None,
        error_message: Some(error_message),
    }
}

fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    println!("First number: {:?}", get_first(&numbers));
    println!("Last number: {:?}", get_last(&numbers));

    let messages = vec![
        SmsMessage { id: 1, content: "Hello".to_string(), recipient: "+1234567890".to_string() },
        SmsMessage { id: 2, content: "World".to_string(), recipient: "+0987654321".to_string() },
    ];
    println!("First SMS: {:?}", get_first(&messages));
    println!("Last SMS: {:?}", get_last(&messages));

    let string_and_number = pair("Textio", 42);
    println!("Pair: {:?}", string_and_number);

    let message_and_status = pair(
        SmsMessage { id: 1, content: "Test".to_string(), recipient: "+1111111111".to_string() },
        "delivered"
    );
    println!("Message with status: {:?}", message_and_status);

    let swapped = swap((100, "hello"));
    println!("Swapped: {:?}", swapped);

    let priorities = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Max priority: {:?}", find_max(&priorities));
    println!("Min priority: {:?}", find_min(&priorities));

    let timestamps = vec![1630000000, 1640000000, 1620000000];
    println!("Max timestamp: {:?}", find_max(&timestamps));
    println!("Min timestamp: {:?}", find_min(&timestamps));

    let recipients = vec!["+1234567890", "+0987654321", "+1111111111"];
    println!("Contains +1234567890: {}", contains(&recipients, &"+1234567890".to_string()));
    println!("Contains +9999999999: {}", contains(&recipients, &"+9999999999".to_string()));

    print_value("Hello from Textio!");
    print_value(42);
    print_value(3.14159);

    let success_response = create_success_response(SmsMessage {
        id: 100,
        content: "Your code is 123456".to_string(),
        recipient: "+1234567890".to_string(),
    });
    println!("Success response: {:?}", success_response);

    let error_response: ApiResponse<SmsMessage> = create_error_response("Invalid API key".to_string());
    println!("Error response: {:?}", error_response);

    let original = SmsMessage {
        id: 1,
        content: "Original".to_string(),
        recipient: "+1234567890".to_string(),
    };
    let same = identity(original);
    println!("Identity result: {:?}", same);

    println!("\n--- Turbofish Examples ---");
    
    let explicit_response = create_success_response::<i32>(200);
    println!("Explicit type response: {:?}", explicit_response);

    let explicit_id = identity::<&str>("explicit");
    println!("Explicit identity: {}", explicit_id);
}

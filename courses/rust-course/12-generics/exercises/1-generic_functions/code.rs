// Textio Generic Functions Exercise
// Complete the generic functions to handle different types in Textio's SMS API

use std::fmt::Display;

// Generic message types for Textio
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

// TODO: Implement a generic function `get_first` that:
// - Takes a slice of items of type T
// - Returns Option<&T> (a reference to the first item or None)
// - Works with any type T
// HINT: Use type parameter <T> after the function name

fn get_first<T>(items: &[T]) -> Option<&T> {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `get_last` that:
// - Takes a slice of items of type T
// - Returns Option<&T> (a reference to the last item or None)

fn get_last<T>(items: &[T]) -> Option<&T> {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `pair` that:
// - Takes two values of potentially different types T and U
// - Returns a tuple (T, U)
// - Use two type parameters: <T, U>

fn pair<T, U>(first: T, second: U) -> (T, U) {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `swap` that:
// - Takes a tuple (T, U)
// - Returns a tuple (U, T)

fn swap<T, U>(tuple: (T, U)) -> (U, T) {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `find_max` that:
// - Takes a slice of items of type T
// - Returns Option<&T> (reference to maximum value or None)
// - T must implement the Ord trait for comparison
// HINT: Add trait bound <T: Ord>

fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `find_min` that:
// - Takes a slice of items of type T
// - Returns Option<&T> (reference to minimum value or None)
// - T must implement the Ord trait

fn find_min<T: Ord>(items: &[T]) -> Option<&T> {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `contains` that:
// - Takes a slice of items of type T and a reference to a value
// - Returns true if the slice contains the value
// - T must implement PartialEq for comparison
// HINT: Add trait bound <T: PartialEq>

fn contains<T: PartialEq>(items: &[T], value: &T) -> bool {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `print_value` that:
// - Takes a value of type T
// - Prints "Value: {value}" to stdout
// - T must implement Display trait
// HINT: Add trait bound <T: Display>

fn print_value<T: Display>(value: T) {
    // Your code here
    todo!()
}

// Generic response type for Textio API
#[derive(Debug, Clone)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error_message: Option<String>,
}

// TODO: Implement a generic function `create_success_response` that:
// - Takes data of type T
// - Returns ApiResponse<T> with success: true, data: Some(data), error_message: None

fn create_success_response<T>(data: T) -> ApiResponse<T> {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `create_error_response` that:
// - Takes an error message String
// - Returns ApiResponse<T> with success: false, data: None, error_message: Some(message)
// HINT: T needs to be specified but can be any type

fn create_error_response<T>(error_message: String) -> ApiResponse<T> {
    // Your code here
    todo!()
}

// TODO: Implement a generic function `identity` that:
// - Takes a value of type T
// - Returns the same value unchanged
// This is the simplest generic function!

fn identity<T>(value: T) -> T {
    // Your code here
    todo!()
}

fn main() {
    // Test get_first and get_last with different types
    let numbers = vec![10, 20, 30, 40, 50];
    println!("First number: {:?}", get_first(&numbers));
    println!("Last number: {:?}", get_last(&numbers));

    let messages = vec![
        SmsMessage { id: 1, content: "Hello".to_string(), recipient: "+1234567890".to_string() },
        SmsMessage { id: 2, content: "World".to_string(), recipient: "+0987654321".to_string() },
    ];
    println!("First SMS: {:?}", get_first(&messages));
    println!("Last SMS: {:?}", get_last(&messages));

    // Test pair function
    let string_and_number = pair("Textio", 42);
    println!("Pair: {:?}", string_and_number);

    let message_and_status = pair(
        SmsMessage { id: 1, content: "Test".to_string(), recipient: "+1111111111".to_string() },
        "delivered"
    );
    println!("Message with status: {:?}", message_and_status);

    // Test swap function
    let swapped = swap((100, "hello"));
    println!("Swapped: {:?}", swapped);

    // Test find_max and find_min
    let priorities = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Max priority: {:?}", find_max(&priorities));
    println!("Min priority: {:?}", find_min(&priorities));

    let timestamps = vec![1630000000, 1640000000, 1620000000];
    println!("Max timestamp: {:?}", find_max(&timestamps));
    println!("Min timestamp: {:?}", find_min(&timestamps));

    // Test contains
    let recipients = vec!["+1234567890", "+0987654321", "+1111111111"];
    println!("Contains +1234567890: {}", contains(&recipients, &"+1234567890".to_string()));
    println!("Contains +9999999999: {}", contains(&recipients, &"+9999999999".to_string()));

    // Test print_value
    print_value("Hello from Textio!");
    print_value(42);
    print_value(3.14159);

    // Test API responses
    let success_response = create_success_response(SmsMessage {
        id: 100,
        content: "Your code is 123456".to_string(),
        recipient: "+1234567890".to_string(),
    });
    println!("Success response: {:?}", success_response);

    let error_response: ApiResponse<SmsMessage> = create_error_response("Invalid API key".to_string());
    println!("Error response: {:?}", error_response);

    // Test identity function
    let original = SmsMessage {
        id: 1,
        content: "Original".to_string(),
        recipient: "+1234567890".to_string(),
    };
    let same = identity(original);
    println!("Identity result: {:?}", same);

    // Demonstrate turbofish syntax
    // Sometimes the compiler needs help knowing which type to use
    println!("\n--- Turbofish Examples ---");
    
    // Create a success response with explicit type (turbofish)
    let explicit_response = create_success_response::<i32>(200);
    println!("Explicit type response: {:?}", explicit_response);

    // Identity with explicit type
    let explicit_id = identity::<&str>("explicit");
    println!("Explicit identity: {}", explicit_id);
}

// Textio Generic Structs Exercise
// Create generic structs for Textio's SMS API

use std::fmt::{Display, Debug};

// TODO: Create a generic Point<T> struct with fields x and y
// This will be used for geolocation features in Textio
// Both x and y should be of type T

// struct Point<T> { ... }

// TODO: Implement methods for Point<T>
// impl<T> Point<T> {
//     // Create a new Point
//     fn new(x: T, y: T) -> Self { ... }
// }

// TODO: Implement methods specific to numeric types
// impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
//     // Return x + y
//     fn sum(&self) -> T { ... }
// }

// TODO: Create a generic Pair<T, U> struct with fields first and second
// This allows pairing different types together
// first is type T, second is type U

// struct Pair<T, U> { ... }

// TODO: Implement methods for Pair<T, U>
// impl<T, U> Pair<T, U> {
//     fn new(first: T, second: U) -> Self { ... }
//     fn first(&self) -> &T { ... }
//     fn second(&self) -> &U { ... }
// }

// TODO: Create a generic Container<T> struct
// It should have a single field `value` of type T
// Add methods to get, set, and map the value

// struct Container<T> { ... }

// TODO: Implement Container methods
// impl<T> Container<T> {
//     fn new(value: T) -> Self { ... }
//     fn get(&self) -> &T { ... }
//     fn into_value(self) -> T { ... }
// }

// TODO: Implement map for Container
// impl<T> Container<T> {
//     fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Container<U> { ... }
// }

// TODO: Create a generic ApiResponse<T> struct
// Fields: status (u16), success (bool), data (Option<T>), error_message (Option<String>)

// struct ApiResponse<T> { ... }

// TODO: Implement ApiResponse methods
// impl<T> ApiResponse<T> {
//     fn ok(data: T) -> Self { ... }
//     fn error(status: u16, message: String) -> Self { ... }
//     fn is_success(&self) -> bool { ... }
// }

// TODO: Create a generic Repository<T> struct for storing items
// Fields: items (Vec<T>)

// struct Repository<T> { ... }

// TODO: Implement Repository methods
// impl<T: Clone> Repository<T> {
//     fn new() -> Self { ... }
//     fn add(&mut self, item: T) { ... }
//     fn get_all(&self) -> Vec<T> { ... }
//     fn count(&self) -> usize { ... }
//     fn is_empty(&self) -> bool { ... }
// }

// TODO: Add find method for Repository with PartialEq bound
// impl<T: Clone + PartialEq> Repository<T> {
//     fn find(&self, item: &T) -> Option<T> { ... }
//     fn contains(&self, item: &T) -> bool { ... }
// }

// Sample types for Textio
#[derive(Debug, Clone, PartialEq)]
struct SmsMessage {
    id: u32,
    content: String,
    recipient: String,
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
    username: String,
    email: String,
}

#[derive(Debug, Clone, PartialEq)]
struct DeliveryZone {
    name: String,
    center_lat: f64,
    center_long: f64,
}

fn main() {
    // Test Point struct
    println!("=== Point Tests ===");
    
    let int_point = Point::new(10, 20);
    println!("Integer point: ({}, {})", int_point.x, int_point.y);
    println!("Sum: {}", int_point.sum());

    let float_point = Point::new(3.5, 7.5);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    println!("Sum: {}", float_point.sum());

    // Test Pair struct
    println!("\n=== Pair Tests ===");
    
    let message_with_status = Pair::new(
        SmsMessage {
            id: 1,
            content: "Hello".to_string(),
            recipient: "+1234567890".to_string(),
        },
        "delivered"
    );
    println!("Message ID: {}", message_with_status.first().id);
    println!("Status: {}", message_with_status.second());

    let user_with_score = Pair::new(
        User {
            id: 100,
            username: "alice".to_string(),
            email: "alice@textio.com".to_string(),
        },
        95.5
    );
    println!("User: {} with score: {}", user_with_score.first().username, user_with_score.second());

    // Test Container struct
    println!("\n=== Container Tests ===");
    
    let number_container = Container::new(42);
    println!("Contained value: {}", number_container.get());
    
    let mapped = number_container.map(|n| n * 2);
    println!("Mapped value: {}", mapped.get());

    let message_container = Container::new(SmsMessage {
        id: 1,
        content: "Test".to_string(),
        recipient: "+1111111111".to_string(),
    });
    let content_container = message_container.map(|m| m.content);
    println!("Extracted content: {}", content_container.get());

    // Test ApiResponse struct
    println!("\n=== API Response Tests ===");
    
    let success: ApiResponse<SmsMessage> = ApiResponse::ok(SmsMessage {
        id: 1,
        content: "Your code is 123456".to_string(),
        recipient: "+1234567890".to_string(),
    });
    println!("Success response: success={}, data={:?}", success.is_success(), success.data);

    let error: ApiResponse<SmsMessage> = ApiResponse::error(404, "Message not found".to_string());
    println!("Error response: success={}, error={:?}", error.is_success(), error.error_message);

    // Test Repository struct
    println!("\n=== Repository Tests ===");
    
    let mut message_repo: Repository<SmsMessage> = Repository::new();
    message_repo.add(SmsMessage {
        id: 1,
        content: "First".to_string(),
        recipient: "+1111111111".to_string(),
    });
    message_repo.add(SmsMessage {
        id: 2,
        content: "Second".to_string(),
        recipient: "+2222222222".to_string(),
    });
    println!("Repository count: {}", message_repo.count());
    println!("Is empty: {}", message_repo.is_empty());

    let mut user_repo: Repository<User> = Repository::new();
    user_repo.add(User {
        id: 1,
        username: "admin".to_string(),
        email: "admin@textio.com".to_string(),
    });
    println!("User count: {}", user_repo.count());

    // Test find functionality
    let search_msg = SmsMessage {
        id: 1,
        content: "First".to_string(),
        recipient: "+1111111111".to_string(),
    };
    let found = message_repo.find(&search_msg);
    println!("Found message: {:?}", found);

    let not_found = message_repo.contains(&SmsMessage {
        id: 99,
        content: "Not there".to_string(),
        recipient: "+9999999999".to_string(),
    });
    println!("Contains non-existent: {}", not_found);
}

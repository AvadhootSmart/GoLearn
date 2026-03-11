// Textio Generic Structs Exercise - Complete Solution
// Create generic structs for Textio's SMS API

use std::fmt::{Display, Debug};

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    fn sum(&self) -> T {
        self.x + self.y
    }
}

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn first(&self) -> &T {
        &self.first
    }
    
    fn second(&self) -> &U {
        &self.second
    }
}

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
    
    fn into_value(self) -> T {
        self.value
    }
    
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Container<U> {
        Container { value: f(self.value) }
    }
}

struct ApiResponse<T> {
    status: u16,
    success: bool,
    data: Option<T>,
    error_message: Option<String>,
}

impl<T> ApiResponse<T> {
    fn ok(data: T) -> Self {
        ApiResponse {
            status: 200,
            success: true,
            data: Some(data),
            error_message: None,
        }
    }
    
    fn error(status: u16, message: String) -> Self {
        ApiResponse {
            status,
            success: false,
            data: None,
            error_message: Some(message),
        }
    }
    
    fn is_success(&self) -> bool {
        self.success
    }
}

struct Repository<T> {
    items: Vec<T>,
}

impl<T: Clone> Repository<T> {
    fn new() -> Self {
        Repository { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get_all(&self) -> Vec<T> {
        self.items.clone()
    }
    
    fn count(&self) -> usize {
        self.items.len()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Clone + PartialEq> Repository<T> {
    fn find(&self, item: &T) -> Option<T> {
        self.items.iter().find(|i| *i == item).cloned()
    }
    
    fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }
}

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
    println!("=== Point Tests ===");
    
    let int_point = Point::new(10, 20);
    println!("Integer point: ({}, {})", int_point.x, int_point.y);
    println!("Sum: {}", int_point.sum());

    let float_point = Point::new(3.5, 7.5);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    println!("Sum: {}", float_point.sum());

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

    println!("\n=== API Response Tests ===");
    
    let success: ApiResponse<SmsMessage> = ApiResponse::ok(SmsMessage {
        id: 1,
        content: "Your code is 123456".to_string(),
        recipient: "+1234567890".to_string(),
    });
    println!("Success response: success={}, data={:?}", success.is_success(), success.data);

    let error: ApiResponse<SmsMessage> = ApiResponse::error(404, "Message not found".to_string());
    println!("Error response: success={}, error={:?}", error.is_success(), error.error_message);

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

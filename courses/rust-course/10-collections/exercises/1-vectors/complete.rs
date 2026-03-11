// Vectors Exercise - Textio SMS API - Complete Solution

pub struct MessageBatch {
    messages: Vec<String>,
    phone_numbers: Vec<String>,
}

impl MessageBatch {
    pub fn new() -> Self {
        MessageBatch {
            messages: Vec::with_capacity(50),
            phone_numbers: Vec::with_capacity(50),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn add_phone(&mut self, phone: String) {
        self.phone_numbers.push(phone);
    }

    pub fn peek_last_message(&self) -> Option<&String> {
        self.messages.get(self.messages.len().saturating_sub(1))
    }

    pub fn remove_last_message(&mut self) -> Option<String> {
        self.messages.pop()
    }

    pub fn get_message(&self, index: usize) -> Option<&String> {
        self.messages.get(index)
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn message_capacity(&self) -> usize {
        self.messages.capacity()
    }

    pub fn reserve_capacity(&mut self, additional: usize) {
        self.messages.reserve(additional);
    }

    pub fn optimize_memory(&mut self) {
        self.messages.shrink_to_fit();
    }

    pub fn drain_messages(&mut self, start: usize, end: usize) -> Vec<String> {
        self.messages.drain(start..end).collect()
    }

    pub fn replace_messages(&mut self, start: usize, end: usize, new_messages: Vec<String>) {
        self.messages.splice(start..end, new_messages);
    }

    pub fn all_messages(&self) -> &[String] {
        &self.messages
    }

    pub fn all_phones(&self) -> &[String] {
        &self.phone_numbers
    }
}

pub fn count_delivery_statuses(statuses: &[&str]) -> (usize, usize, usize) {
    let mut delivered = 0;
    let mut pending = 0;
    let mut failed = 0;
    
    for status in statuses {
        match *status {
            "delivered" => delivered += 1,
            "pending" => pending += 1,
            "failed" => failed += 1,
            _ => {}
        }
    }
    
    (delivered, pending, failed)
}

pub fn double_message_ids(ids: &mut Vec<u32>) {
    for id in ids.iter_mut() {
        *id *= 2;
    }
}

fn main() {
    let mut batch = MessageBatch::new();
    
    println!("Initial capacity: {}", batch.message_capacity());
    
    batch.add_message("Hello, World!".to_string());
    batch.add_message("Rust is awesome!".to_string());
    batch.add_message("Textio SMS API".to_string());
    
    batch.add_phone("+1234567890".to_string());
    batch.add_phone("+0987654321".to_string());
    
    println!("Message count: {}", batch.message_count());
    println!("Capacity after adds: {}", batch.message_capacity());
    
    if let Some(msg) = batch.peek_last_message() {
        println!("Last message: {}", msg);
    }
    
    if let Some(msg) = batch.get_message(1) {
        println!("Message at index 1: {}", msg);
    }
    
    if let Some(msg) = batch.remove_last_message() {
        println!("Popped message: {}", msg);
    }
    
    batch.reserve_capacity(100);
    println!("After reserve(100): capacity = {}", batch.message_capacity());
    
    batch.optimize_memory();
    println!("After shrink_to_fit: capacity = {}", batch.message_capacity());
    
    batch.add_message("Msg A".to_string());
    batch.add_message("Msg B".to_string());
    batch.add_message("Msg C".to_string());
    batch.add_message("Msg D".to_string());
    
    let drained = batch.drain_messages(1, 3);
    println!("Drained messages: {:?}", drained);
    println!("Remaining messages: {:?}", batch.all_messages());
    
    batch.replace_messages(0, 1, vec!["New Msg 1".to_string(), "New Msg 2".to_string()]);
    println!("After splice: {:?}", batch.all_messages());
    
    let statuses = vec!["delivered", "pending", "delivered", "failed", "pending", "delivered"];
    let (delivered, pending, failed) = count_delivery_statuses(&statuses);
    println!("Delivery stats - Delivered: {}, Pending: {}, Failed: {}", delivered, pending, failed);
    
    let mut ids = vec![1, 2, 3, 4, 5];
    double_message_ids(&mut ids);
    println!("Doubled IDs: {:?}", ids);
}

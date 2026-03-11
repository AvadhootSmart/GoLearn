// Vectors Exercise - Textio SMS API
// 
// Complete the functions below to work with vectors in the context
// of an SMS messaging system.

/// A structure to manage a batch of SMS messages
pub struct MessageBatch {
    messages: Vec<String>,
    phone_numbers: Vec<String>,
}

impl MessageBatch {
    /// Create a new MessageBatch with pre-allocated capacity
    /// TODO: Use Vec::with_capacity to create vectors with capacity for 50 messages
    pub fn new() -> Self {
        // Your code here
        todo!()
    }

    /// Add a new message to the batch
    /// TODO: Use push to add the message
    pub fn add_message(&mut self, message: String) {
        // Your code here
        todo!()
    }

    /// Add a phone number to the batch
    /// TODO: Use push to add the phone number
    pub fn add_phone(&mut self, phone: String) {
        // Your code here
        todo!()
    }

    /// Get the last message without removing it
    /// TODO: Use get() to safely retrieve the last element
    /// Return None if empty
    pub fn peek_last_message(&self) -> Option<&String> {
        // Your code here
        todo!()
    }

    /// Remove and return the last message
    /// TODO: Use pop() to remove and return the last message
    pub fn remove_last_message(&mut self) -> Option<String> {
        // Your code here
        todo!()
    }

    /// Get a message at a specific index safely
    /// TODO: Use get() to return a reference to the message at index
    pub fn get_message(&self, index: usize) -> Option<&String> {
        // Your code here
        todo!()
    }

    /// Get the current number of messages
    pub fn message_count(&self) -> usize {
        // Your code here
        todo!()
    }

    /// Get the current capacity of messages vector
    pub fn message_capacity(&self) -> usize {
        // Your code here
        todo!()
    }

    /// Reserve additional capacity for messages
    /// TODO: Use reserve() to add capacity for 'additional' more messages
    pub fn reserve_capacity(&mut self, additional: usize) {
        // Your code here
        todo!()
    }

    /// Shrink the capacity to fit the current length
    /// TODO: Use shrink_to_fit() to reduce capacity to match length
    pub fn optimize_memory(&mut self) {
        // Your code here
        todo!()
    }

    /// Remove a range of messages (for batch deletion)
    /// TODO: Use drain() to remove messages from start to end (exclusive)
    /// Return the drained messages as a vector
    pub fn drain_messages(&mut self, start: usize, end: usize) -> Vec<String> {
        // Your code here
        todo!()
    }

    /// Replace a range of messages with new ones
    /// TODO: Use splice() to replace messages from start to end with new_messages
    pub fn replace_messages(&mut self, start: usize, end: usize, new_messages: Vec<String>) {
        // Your code here
        todo!()
    }

    /// Get all messages as a slice
    pub fn all_messages(&self) -> &[String] {
        &self.messages
    }

    /// Get all phone numbers as a slice
    pub fn all_phones(&self) -> &[String] {
        &self.phone_numbers
    }
}

/// Process a vector of delivery statuses
/// TODO: Iterate through the statuses and count each type
/// Return (delivered_count, pending_count, failed_count)
pub fn count_delivery_statuses(statuses: &[&str]) -> (usize, usize, usize) {
    // Your code here
    todo!()
}

/// Double all message IDs in place
/// TODO: Use mutable iteration to multiply each ID by 2
pub fn double_message_ids(ids: &mut Vec<u32>) {
    // Your code here
    todo!()
}

fn main() {
    // Test MessageBatch
    let mut batch = MessageBatch::new();
    
    println!("Initial capacity: {}", batch.message_capacity());
    
    batch.add_message("Hello, World!".to_string());
    batch.add_message("Rust is awesome!".to_string());
    batch.add_message("Textio SMS API".to_string());
    
    batch.add_phone("+1234567890".to_string());
    batch.add_phone("+0987654321".to_string());
    
    println!("Message count: {}", batch.message_count());
    println!("Capacity after adds: {}", batch.message_capacity());
    
    // Test peek
    if let Some(msg) = batch.peek_last_message() {
        println!("Last message: {}", msg);
    }
    
    // Test get
    if let Some(msg) = batch.get_message(1) {
        println!("Message at index 1: {}", msg);
    }
    
    // Test pop
    if let Some(msg) = batch.remove_last_message() {
        println!("Popped message: {}", msg);
    }
    
    // Test reserve and shrink
    batch.reserve_capacity(100);
    println!("After reserve(100): capacity = {}", batch.message_capacity());
    
    batch.optimize_memory();
    println!("After shrink_to_fit: capacity = {}", batch.message_capacity());
    
    // Test drain
    batch.add_message("Msg A".to_string());
    batch.add_message("Msg B".to_string());
    batch.add_message("Msg C".to_string());
    batch.add_message("Msg D".to_string());
    
    let drained = batch.drain_messages(1, 3);
    println!("Drained messages: {:?}", drained);
    println!("Remaining messages: {:?}", batch.all_messages());
    
    // Test splice
    batch.replace_messages(0, 1, vec!["New Msg 1".to_string(), "New Msg 2".to_string()]);
    println!("After splice: {:?}", batch.all_messages());
    
    // Test delivery status counting
    let statuses = vec!["delivered", "pending", "delivered", "failed", "pending", "delivered"];
    let (delivered, pending, failed) = count_delivery_statuses(&statuses);
    println!("Delivery stats - Delivered: {}, Pending: {}, Failed: {}", delivered, pending, failed);
    
    // Test doubling IDs
    let mut ids = vec![1, 2, 3, 4, 5];
    double_message_ids(&mut ids);
    println!("Doubled IDs: {:?}", ids);
}

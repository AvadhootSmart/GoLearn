// HashMap Exercise - Textio SMS API
//
// Complete the functions below to work with HashMaps in the context
// of an SMS messaging system.

use std::collections::HashMap;

/// Tracks delivery status of SMS messages
pub struct DeliveryTracker {
    statuses: HashMap<String, String>,
    retry_counts: HashMap<String, u32>,
}

impl DeliveryTracker {
    /// Create a new delivery tracker
    pub fn new() -> Self {
        // Your code here
        todo!()
    }

    /// Create a tracker with pre-allocated capacity
    /// TODO: Use HashMap::with_capacity for both maps
    pub fn with_capacity(capacity: usize) -> Self {
        // Your code here
        todo!()
    }

    /// Track a new message with "pending" status
    /// TODO: Use insert to add the message with "pending" status
    /// and initialize retry count to 0
    pub fn track_message(&mut self, message_id: String) {
        // Your code here
        todo!()
    }

    /// Update the status of a message
    /// TODO: Use insert to update the status
    pub fn update_status(&mut self, message_id: &str, status: &str) {
        // Your code here
        todo!()
    }

    /// Get the status of a message
    /// TODO: Use get to retrieve the status
    pub fn get_status(&self, message_id: &str) -> Option<&String> {
        // Your code here
        todo!()
    }

    /// Check if a message is being tracked
    /// TODO: Use contains_key
    pub fn is_tracked(&self, message_id: &str) -> bool {
        // Your code here
        todo!()
    }

    /// Remove a message from tracking
    /// TODO: Use remove to delete both status and retry count
    pub fn remove_message(&mut self, message_id: &str) {
        // Your code here
        todo!()
    }

    /// Increment retry count for a message
    /// TODO: Use entry().or_insert() to increment the count
    /// If the message isn't tracked, initialize with count 1
    pub fn increment_retry(&mut self, message_id: &str) {
        // Your code here
        todo!()
    }

    /// Get retry count for a message
    /// TODO: Use get to retrieve the count, return 0 if not found
    pub fn get_retry_count(&self, message_id: &str) -> u32 {
        // Your code here
        todo!()
    }

    /// Get a mutable reference to a status
    /// TODO: Use get_mut to return a mutable reference
    pub fn get_status_mut(&mut self, message_id: &str) -> Option<&mut String> {
        // Your code here
        todo!()
    }

    /// Get or create a status entry
    /// TODO: Use entry().or_insert() to get or create with "unknown"
    pub fn get_or_create_status(&mut self, message_id: &str) -> &mut String {
        // Your code here
        todo!()
    }

    /// Count messages with a specific status
    /// TODO: Iterate through values and count matches
    pub fn count_by_status(&self, status: &str) -> usize {
        // Your code here
        todo!()
    }

    /// Get all message IDs with a specific status
    /// TODO: Iterate and filter by status, collect keys
    pub fn messages_with_status(&self, status: &str) -> Vec<&String> {
        // Your code here
        todo!()
    }

    /// Get total number of tracked messages
    pub fn total_tracked(&self) -> usize {
        // Your code here
        todo!()
    }
}

/// Count word occurrences in a text
/// TODO: Use entry().or_insert() pattern to count each word
pub fn count_words(text: &str) -> HashMap<&str, u32> {
    // Your code here
    todo!()
}

/// Merge two HashMaps, with values from the second taking precedence
/// TODO: Iterate through the second map and insert into the first
pub fn merge_maps(map1: &mut HashMap<String, i32>, map2: HashMap<String, i32>) {
    // Your code here
    todo!()
}

/// Create a HashMap from parallel vectors
/// TODO: Use zip and collect to create the map
pub fn from_parallel_lists(keys: Vec<String>, values: Vec<i32>) -> HashMap<String, i32> {
    // Your code here
    todo!()
}

fn main() {
    // Test DeliveryTracker
    let mut tracker = DeliveryTracker::with_capacity(10);
    
    tracker.track_message("msg-001".to_string());
    tracker.track_message("msg-002".to_string());
    tracker.track_message("msg-003".to_string());
    
    println!("Total tracked: {}", tracker.total_tracked());
    println!("msg-001 status: {:?}", tracker.get_status("msg-001"));
    println!("Is msg-001 tracked: {}", tracker.is_tracked("msg-001"));
    println!("Is msg-999 tracked: {}", tracker.is_tracked("msg-999"));
    
    tracker.update_status("msg-001", "sent");
    tracker.update_status("msg-002", "delivered");
    tracker.update_status("msg-003", "failed");
    
    println!("After updates:");
    println!("  msg-001: {:?}", tracker.get_status("msg-001"));
    println!("  msg-002: {:?}", tracker.get_status("msg-002"));
    println!("  msg-003: {:?}", tracker.get_status("msg-003"));
    
    // Test retry counting
    tracker.increment_retry("msg-003");
    tracker.increment_retry("msg-003");
    tracker.increment_retry("msg-003");
    println!("msg-003 retries: {}", tracker.get_retry_count("msg-003"));
    
    // Test count by status
    println!("Sent count: {}", tracker.count_by_status("sent"));
    println!("Delivered count: {}", tracker.count_by_status("delivered"));
    println!("Failed count: {}", tracker.count_by_status("failed"));
    
    // Test messages with status
    let failed_msgs = tracker.messages_with_status("failed");
    println!("Failed messages: {:?}", failed_msgs);
    
    // Test get_or_create
    let status = tracker.get_or_create_status("msg-004");
    println!("New message status: {}", status);
    
    // Test mutable access
    if let Some(s) = tracker.get_status_mut("msg-001") {
        s.push_str(" (confirmed)");
    }
    println!("Modified msg-001: {:?}", tracker.get_status("msg-001"));
    
    // Test word counting
    let text = "hello world hello rust world world";
    let word_counts = count_words(text);
    println!("Word counts: {:?}", word_counts);
    
    // Test map merging
    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert("a".to_string(), 1);
    map1.insert("b".to_string(), 2);
    
    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("b".to_string(), 20);
    map2.insert("c".to_string(), 3);
    
    merge_maps(&mut map1, map2);
    println!("Merged map: {:?}", map1);
    
    // Test parallel lists
    let keys = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let values = vec![10, 20, 30];
    let from_lists = from_parallel_lists(keys, values);
    println!("From lists: {:?}", from_lists);
    
    // Test remove
    tracker.remove_message("msg-003");
    println!("After removal, total: {}", tracker.total_tracked());
}

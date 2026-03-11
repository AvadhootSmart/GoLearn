// HashMap Exercise - Textio SMS API - Complete Solution

use std::collections::HashMap;

pub struct DeliveryTracker {
    statuses: HashMap<String, String>,
    retry_counts: HashMap<String, u32>,
}

impl DeliveryTracker {
    pub fn new() -> Self {
        DeliveryTracker {
            statuses: HashMap::new(),
            retry_counts: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DeliveryTracker {
            statuses: HashMap::with_capacity(capacity),
            retry_counts: HashMap::with_capacity(capacity),
        }
    }

    pub fn track_message(&mut self, message_id: String) {
        self.statuses.insert(message_id.clone(), "pending".to_string());
        self.retry_counts.insert(message_id, 0);
    }

    pub fn update_status(&mut self, message_id: &str, status: &str) {
        self.statuses.insert(message_id.to_string(), status.to_string());
    }

    pub fn get_status(&self, message_id: &str) -> Option<&String> {
        self.statuses.get(message_id)
    }

    pub fn is_tracked(&self, message_id: &str) -> bool {
        self.statuses.contains_key(message_id)
    }

    pub fn remove_message(&mut self, message_id: &str) {
        self.statuses.remove(message_id);
        self.retry_counts.remove(message_id);
    }

    pub fn increment_retry(&mut self, message_id: &str) {
        *self.retry_counts.entry(message_id.to_string()).or_insert(0) += 1;
    }

    pub fn get_retry_count(&self, message_id: &str) -> u32 {
        *self.retry_counts.get(message_id).unwrap_or(&0)
    }

    pub fn get_status_mut(&mut self, message_id: &str) -> Option<&mut String> {
        self.statuses.get_mut(message_id)
    }

    pub fn get_or_create_status(&mut self, message_id: &str) -> &mut String {
        self.statuses.entry(message_id.to_string()).or_insert("unknown".to_string())
    }

    pub fn count_by_status(&self, status: &str) -> usize {
        self.statuses.values().filter(|s| *s == status).count()
    }

    pub fn messages_with_status(&self, status: &str) -> Vec<&String> {
        self.statuses
            .iter()
            .filter(|(_, s)| *s == status)
            .map(|(id, _)| id)
            .collect()
    }

    pub fn total_tracked(&self) -> usize {
        self.statuses.len()
    }
}

pub fn count_words(text: &str) -> HashMap<&str, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

pub fn merge_maps(map1: &mut HashMap<String, i32>, map2: HashMap<String, i32>) {
    for (key, value) in map2 {
        map1.insert(key, value);
    }
}

pub fn from_parallel_lists(keys: Vec<String>, values: Vec<i32>) -> HashMap<String, i32> {
    keys.into_iter().zip(values.into_iter()).collect()
}

fn main() {
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
    
    tracker.increment_retry("msg-003");
    tracker.increment_retry("msg-003");
    tracker.increment_retry("msg-003");
    println!("msg-003 retries: {}", tracker.get_retry_count("msg-003"));
    
    println!("Sent count: {}", tracker.count_by_status("sent"));
    println!("Delivered count: {}", tracker.count_by_status("delivered"));
    println!("Failed count: {}", tracker.count_by_status("failed"));
    
    let failed_msgs = tracker.messages_with_status("failed");
    println!("Failed messages: {:?}", failed_msgs);
    
    let status = tracker.get_or_create_status("msg-004");
    println!("New message status: {}", status);
    
    if let Some(s) = tracker.get_status_mut("msg-001") {
        s.push_str(" (confirmed)");
    }
    println!("Modified msg-001: {:?}", tracker.get_status("msg-001"));
    
    let text = "hello world hello rust world world";
    let word_counts = count_words(text);
    println!("Word counts: {:?}", word_counts);
    
    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert("a".to_string(), 1);
    map1.insert("b".to_string(), 2);
    
    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert("b".to_string(), 20);
    map2.insert("c".to_string(), 3);
    
    merge_maps(&mut map1, map2);
    println!("Merged map: {:?}", map1);
    
    let keys = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let values = vec![10, 20, 30];
    let from_lists = from_parallel_lists(keys, values);
    println!("From lists: {:?}", from_lists);
    
    tracker.remove_message("msg-003");
    println!("After removal, total: {}", tracker.total_tracked());
}

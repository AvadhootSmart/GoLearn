// Iteration Patterns Exercise - Textio SMS API
//
// Complete the functions below to work with iterators in the context
// of an SMS messaging system.

use std::collections::{HashMap, HashSet};

/// Represents an SMS message
#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub to: String,
    pub body: String,
    pub status: String,
}

/// Statistics about a batch of messages
#[derive(Debug)]
pub struct MessageStats {
    pub total: usize,
    pub delivered: usize,
    pub failed: usize,
    pub pending: usize,
    pub total_characters: usize,
    pub unique_recipients: usize,
}

/// Filter messages by status
/// TODO: Use iter() and filter() to return messages with matching status
pub fn filter_by_status<'a>(messages: &'a [Message], status: &str) -> Vec<&'a Message> {
    // Your code here
    todo!()
}

/// Get all unique recipients from messages
/// TODO: Use iter(), map(), and collect() into a HashSet
pub fn get_unique_recipients(messages: &[Message]) -> HashSet<String> {
    // Your code here
    todo!()
}

/// Count messages by status
/// TODO: Use iter() and fold() to build a HashMap of counts
pub fn count_by_status(messages: &[Message]) -> HashMap<String, usize> {
    // Your code here
    todo!()
}

/// Transform messages to a vector of (id, to) tuples
/// TODO: Use iter() and map() to create the tuples
pub fn extract_id_recipient_pairs(messages: &[Message]) -> Vec<(u64, String)> {
    // Your code here
    todo!()
}

/// Find the first message with a body longer than min_length
/// TODO: Use iter() and find() to return the first match
pub fn find_long_message<'a>(messages: &'a [Message], min_length: usize) -> Option<&'a Message> {
    // Your code here
    todo!()
}

/// Check if any message is failed
/// TODO: Use iter() and any()
pub fn has_failed_messages(messages: &[Message]) -> bool {
    // Your code here
    todo!()
}

/// Check if all messages are delivered
/// TODO: Use iter() and all()
pub fn all_delivered(messages: &[Message]) -> bool {
    // Your code here
    todo!()
}

/// Calculate total character count across all messages
/// TODO: Use iter() and map() and sum()
pub fn total_characters(messages: &[Message]) -> usize {
    // Your code here
    todo!()
}

/// Get the position of the first failed message
/// TODO: Use iter() and position()
pub fn first_failed_position(messages: &[Message]) -> Option<usize> {
    // Your code here
    todo!()
}

/// Partition messages into delivered and not delivered
/// TODO: Use into_iter() and partition()
pub fn partition_by_delivered(messages: Vec<Message>) -> (Vec<Message>, Vec<Message>) {
    // Your code here
    todo!()
}

/// Chain two message vectors together
/// TODO: Use into_iter() and chain() and collect()
pub fn merge_messages(messages1: Vec<Message>, messages2: Vec<Message>) -> Vec<Message> {
    // Your code here
    todo!()
}

/// Take only the first n messages
/// TODO: Use iter() and take() and collect()
pub fn take_first_n<'a>(messages: &'a [Message], n: usize) -> Vec<&'a Message> {
    // Your code here
    todo!()
}

/// Skip the first n messages and get the rest
/// TODO: Use iter() and skip() and collect()
pub fn skip_first_n<'a>(messages: &'a [Message], n: usize) -> Vec<&'a Message> {
    // Your code here
    todo!()
}

/// Create a map from message ID to message body
/// TODO: Use into_iter() and map() with tuples and collect()
pub fn create_id_body_map(messages: Vec<Message>) -> HashMap<u64, String> {
    // Your code here
    todo!()
}

/// Get message bodies that contain a substring
/// TODO: Use iter(), filter(), map(), and collect()
pub fn bodies_containing(messages: &[Message], substring: &str) -> Vec<&str> {
    // Your code here
    todo!()
}

/// Calculate message statistics
/// TODO: Combine multiple iterator operations
pub fn calculate_stats(messages: &[Message]) -> MessageStats {
    // Your code here
    todo!()
}

/// Flatten nested message vectors
/// TODO: Use into_iter() and flatten() and collect()
pub fn flatten_messages(messages: Vec<Vec<Message>>) -> Vec<Message> {
    // Your code here
    todo!()
}

/// Enumerate and format messages
/// TODO: Use iter(), enumerate(), map(), and collect()
/// Format: "1. Message to +123: Hello"
pub fn format_with_numbers(messages: &[Message]) -> Vec<String> {
    // Your code here
    todo!()
}

/// Use filter_map to parse message IDs from strings
/// TODO: Use filter_map() to parse and filter out invalid IDs
pub fn parse_message_ids(id_strings: &[&str]) -> Vec<u64> {
    // Your code here
    todo!()
}

fn main() {
    let messages = vec![
        Message { id: 1, to: "+1111111111".to_string(), body: "Hello there!".to_string(), status: "delivered".to_string() },
        Message { id: 2, to: "+2222222222".to_string(), body: "This is a longer message body".to_string(), status: "delivered".to_string() },
        Message { id: 3, to: "+1111111111".to_string(), body: "Duplicate recipient".to_string(), status: "failed".to_string() },
        Message { id: 4, to: "+3333333333".to_string(), body: "Short".to_string(), status: "pending".to_string() },
        Message { id: 5, to: "+4444444444".to_string(), body: "Another delivered message".to_string(), status: "delivered".to_string() },
    ];

    // Test filter_by_status
    let delivered = filter_by_status(&messages, "delivered");
    println!("Delivered messages: {:?}", delivered.iter().map(|m| m.id).collect::<Vec<_>>());

    // Test unique recipients
    let unique = get_unique_recipients(&messages);
    println!("Unique recipients: {:?}", unique);

    // Test count by status
    let counts = count_by_status(&messages);
    println!("Status counts: {:?}", counts);

    // Test extract pairs
    let pairs = extract_id_recipient_pairs(&messages);
    println!("ID-recipient pairs: {:?}", pairs);

    // Test find long message
    let long = find_long_message(&messages, 15);
    println!("First message > 15 chars: {:?}", long.map(|m| m.id));

    // Test has_failed
    println!("Has failed messages: {}", has_failed_messages(&messages));

    // Test all_delivered
    println!("All delivered: {}", all_delivered(&messages));

    // Test total characters
    println!("Total characters: {}", total_characters(&messages));

    // Test first failed position
    println!("First failed position: {:?}", first_failed_position(&messages));

    // Test partition
    let (delivered_msgs, not_delivered) = partition_by_delivered(messages.clone());
    println!("Delivered count: {}, Not delivered count: {}", delivered_msgs.len(), not_delivered.len());

    // Test merge
    let more_messages = vec![
        Message { id: 6, to: "+5555555555".to_string(), body: "Merged message".to_string(), status: "pending".to_string() },
    ];
    let merged = merge_messages(messages.clone(), more_messages);
    println!("Merged message count: {}", merged.len());

    // Test take and skip
    let first_three = take_first_n(&messages, 3);
    println!("First 3 IDs: {:?}", first_three.iter().map(|m| m.id).collect::<Vec<_>>());

    let skipped = skip_first_n(&messages, 2);
    println!("After skip 2 IDs: {:?}", skipped.iter().map(|m| m.id).collect::<Vec<_>>());

    // Test id-body map
    let id_body_map = create_id_body_map(messages.clone());
    println!("ID 1 body: {:?}", id_body_map.get(&1));

    // Test bodies containing
    let containing = bodies_containing(&messages, "message");
    println!("Bodies containing 'message': {:?}", containing);

    // Test calculate_stats
    let stats = calculate_stats(&messages);
    println!("Stats: {:?}", stats);

    // Test flatten
    let nested = vec![
        vec![messages[0].clone(), messages[1].clone()],
        vec![messages[2].clone()],
    ];
    let flattened = flatten_messages(nested);
    println!("Flattened count: {}", flattened.len());

    // Test format with numbers
    let formatted = format_with_numbers(&messages);
    println!("Formatted messages:");
    for f in &formatted {
        println!("  {}", f);
    }

    // Test parse message IDs
    let id_strings = ["1", "invalid", "42", "also_invalid", "100"];
    let parsed_ids = parse_message_ids(&id_strings);
    println!("Parsed IDs: {:?}", parsed_ids);
}

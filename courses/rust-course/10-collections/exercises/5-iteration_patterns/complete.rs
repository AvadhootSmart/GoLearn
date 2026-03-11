// Iteration Patterns Exercise - Textio SMS API - Complete Solution

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub to: String,
    pub body: String,
    pub status: String,
}

#[derive(Debug)]
pub struct MessageStats {
    pub total: usize,
    pub delivered: usize,
    pub failed: usize,
    pub pending: usize,
    pub total_characters: usize,
    pub unique_recipients: usize,
}

pub fn filter_by_status<'a>(messages: &'a [Message], status: &str) -> Vec<&'a Message> {
    messages.iter().filter(|m| m.status == status).collect()
}

pub fn get_unique_recipients(messages: &[Message]) -> HashSet<String> {
    messages.iter().map(|m| m.to.clone()).collect()
}

pub fn count_by_status(messages: &[Message]) -> HashMap<String, usize> {
    messages.iter().fold(HashMap::new(), |mut acc, m| {
        *acc.entry(m.status.clone()).or_insert(0) += 1;
        acc
    })
}

pub fn extract_id_recipient_pairs(messages: &[Message]) -> Vec<(u64, String)> {
    messages.iter().map(|m| (m.id, m.to.clone())).collect()
}

pub fn find_long_message<'a>(messages: &'a [Message], min_length: usize) -> Option<&'a Message> {
    messages.iter().find(|m| m.body.len() > min_length)
}

pub fn has_failed_messages(messages: &[Message]) -> bool {
    messages.iter().any(|m| m.status == "failed")
}

pub fn all_delivered(messages: &[Message]) -> bool {
    messages.iter().all(|m| m.status == "delivered")
}

pub fn total_characters(messages: &[Message]) -> usize {
    messages.iter().map(|m| m.body.len()).sum()
}

pub fn first_failed_position(messages: &[Message]) -> Option<usize> {
    messages.iter().position(|m| m.status == "failed")
}

pub fn partition_by_delivered(messages: Vec<Message>) -> (Vec<Message>, Vec<Message>) {
    messages.into_iter().partition(|m| m.status == "delivered")
}

pub fn merge_messages(messages1: Vec<Message>, messages2: Vec<Message>) -> Vec<Message> {
    messages1.into_iter().chain(messages2.into_iter()).collect()
}

pub fn take_first_n<'a>(messages: &'a [Message], n: usize) -> Vec<&'a Message> {
    messages.iter().take(n).collect()
}

pub fn skip_first_n<'a>(messages: &'a [Message], n: usize) -> Vec<&'a Message> {
    messages.iter().skip(n).collect()
}

pub fn create_id_body_map(messages: Vec<Message>) -> HashMap<u64, String> {
    messages.into_iter().map(|m| (m.id, m.body)).collect()
}

pub fn bodies_containing(messages: &[Message], substring: &str) -> Vec<&str> {
    messages.iter()
        .filter(|m| m.body.contains(substring))
        .map(|m| m.body.as_str())
        .collect()
}

pub fn calculate_stats(messages: &[Message]) -> MessageStats {
    MessageStats {
        total: messages.len(),
        delivered: messages.iter().filter(|m| m.status == "delivered").count(),
        failed: messages.iter().filter(|m| m.status == "failed").count(),
        pending: messages.iter().filter(|m| m.status == "pending").count(),
        total_characters: total_characters(messages),
        unique_recipients: get_unique_recipients(messages).len(),
    }
}

pub fn flatten_messages(messages: Vec<Vec<Message>>) -> Vec<Message> {
    messages.into_iter().flatten().collect()
}

pub fn format_with_numbers(messages: &[Message]) -> Vec<String> {
    messages.iter()
        .enumerate()
        .map(|(i, m)| format!("{}. Message to {}: {}", i + 1, m.to, m.body))
        .collect()
}

pub fn parse_message_ids(id_strings: &[&str]) -> Vec<u64> {
    id_strings.iter()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn main() {
    let messages = vec![
        Message { id: 1, to: "+1111111111".to_string(), body: "Hello there!".to_string(), status: "delivered".to_string() },
        Message { id: 2, to: "+2222222222".to_string(), body: "This is a longer message body".to_string(), status: "delivered".to_string() },
        Message { id: 3, to: "+1111111111".to_string(), body: "Duplicate recipient".to_string(), status: "failed".to_string() },
        Message { id: 4, to: "+3333333333".to_string(), body: "Short".to_string(), status: "pending".to_string() },
        Message { id: 5, to: "+4444444444".to_string(), body: "Another delivered message".to_string(), status: "delivered".to_string() },
    ];

    let delivered = filter_by_status(&messages, "delivered");
    println!("Delivered messages: {:?}", delivered.iter().map(|m| m.id).collect::<Vec<_>>());

    let unique = get_unique_recipients(&messages);
    println!("Unique recipients: {:?}", unique);

    let counts = count_by_status(&messages);
    println!("Status counts: {:?}", counts);

    let pairs = extract_id_recipient_pairs(&messages);
    println!("ID-recipient pairs: {:?}", pairs);

    let long = find_long_message(&messages, 15);
    println!("First message > 15 chars: {:?}", long.map(|m| m.id));

    println!("Has failed messages: {}", has_failed_messages(&messages));

    println!("All delivered: {}", all_delivered(&messages));

    println!("Total characters: {}", total_characters(&messages));

    println!("First failed position: {:?}", first_failed_position(&messages));

    let (delivered_msgs, not_delivered) = partition_by_delivered(messages.clone());
    println!("Delivered count: {}, Not delivered count: {}", delivered_msgs.len(), not_delivered.len());

    let more_messages = vec![
        Message { id: 6, to: "+5555555555".to_string(), body: "Merged message".to_string(), status: "pending".to_string() },
    ];
    let merged = merge_messages(messages.clone(), more_messages);
    println!("Merged message count: {}", merged.len());

    let first_three = take_first_n(&messages, 3);
    println!("First 3 IDs: {:?}", first_three.iter().map(|m| m.id).collect::<Vec<_>>());

    let skipped = skip_first_n(&messages, 2);
    println!("After skip 2 IDs: {:?}", skipped.iter().map(|m| m.id).collect::<Vec<_>>());

    let id_body_map = create_id_body_map(messages.clone());
    println!("ID 1 body: {:?}", id_body_map.get(&1));

    let containing = bodies_containing(&messages, "message");
    println!("Bodies containing 'message': {:?}", containing);

    let stats = calculate_stats(&messages);
    println!("Stats: {:?}", stats);

    let nested = vec![
        vec![messages[0].clone(), messages[1].clone()],
        vec![messages[2].clone()],
    ];
    let flattened = flatten_messages(nested);
    println!("Flattened count: {}", flattened.len());

    let formatted = format_with_numbers(&messages);
    println!("Formatted messages:");
    for f in &formatted {
        println!("  {}", f);
    }

    let id_strings = ["1", "invalid", "42", "also_invalid", "100"];
    let parsed_ids = parse_message_ids(&id_strings);
    println!("Parsed IDs: {:?}", parsed_ids);
}

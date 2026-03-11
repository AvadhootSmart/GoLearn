// Complete Integration Test Files
// ================================
// These files would be in the tests/ directory

// ==========================================
// tests/common/mod.rs
// ==========================================

pub fn create_test_client() -> textio::Client {
    textio::Client::new("test_api_key_12345")
}

pub fn create_client_with_contacts() -> textio::Client {
    let client = textio::Client::new("test_api_key_12345");
    client.add_contact("Alice", "+1111111111").unwrap();
    client.add_contact("Bob", "+2222222222").unwrap();
    client.add_contact("Charlie", "+3333333333").unwrap();
    client
}

pub fn valid_phone_number() -> &'static str {
    "+1234567890"
}

pub fn invalid_phone_number() -> &'static str {
    "invalid"
}

// ==========================================
// tests/client_test.rs
// ==========================================

mod common;
use common::*;
use textio::*;

#[test]
fn test_client_creation() {
    let client = create_test_client();
    assert_eq!(client.api_key(), "test_api_key_12345");
}

#[test]
fn test_client_new() {
    let client = Client::new("my_key");
    assert!(!client.api_key().is_empty());
}

// ==========================================
// tests/contact_test.rs
// ==========================================

mod common;
use common::*;
use textio::*;

#[test]
fn test_add_contact() {
    let client = create_test_client();
    let result = client.add_contact("Alice", "+1234567890");
    
    assert!(result.is_ok());
    let contact = result.unwrap();
    assert_eq!(contact.name, "Alice");
    assert_eq!(contact.phone, "+1234567890");
    assert!(contact.id.starts_with("contact_"));
}

#[test]
fn test_add_contact_invalid_phone() {
    let client = create_test_client();
    let result = client.add_contact("Alice", "invalid");
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::InvalidPhoneNumber));
}

#[test]
fn test_get_contact() {
    let client = create_test_client();
    let contact = client.add_contact("Bob", "+9876543210").unwrap();
    
    let retrieved = client.get_contact(&contact.id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Bob");
}

#[test]
fn test_get_nonexistent_contact() {
    let client = create_test_client();
    let result = client.get_contact("nonexistent_id");
    assert!(result.is_none());
}

#[test]
fn test_list_contacts() {
    let client = create_client_with_contacts();
    let contacts = client.list_contacts();
    
    assert_eq!(contacts.len(), 3);
    assert!(contacts.iter().any(|c| c.name == "Alice"));
    assert!(contacts.iter().any(|c| c.name == "Bob"));
    assert!(contacts.iter().any(|c| c.name == "Charlie"));
}

#[test]
fn test_send_to_contact() {
    let client = create_test_client();
    let contact = client.add_contact("Test User", "+1234567890").unwrap();
    
    let result = client.send_to_contact(&contact.id, "Hello!");
    assert!(result.is_ok());
    
    let message = result.unwrap();
    assert_eq!(message.to(), "+1234567890");
}

#[test]
fn test_send_to_nonexistent_contact() {
    let client = create_test_client();
    let result = client.send_to_contact("nonexistent", "Hello");
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::ContactNotFound));
}

// ==========================================
// tests/message_test.rs
// ==========================================

mod common;
use common::*;
use textio::*;

#[test]
fn test_send_message() {
    let client = create_test_client();
    let result = client.send("+1234567890", "Hello, World!");
    
    assert!(result.is_ok());
    let message = result.unwrap();
    assert_eq!(message.to(), "+1234567890");
    assert_eq!(message.body(), "Hello, World!");
    assert!(message.id().starts_with("msg_"));
}

#[test]
fn test_send_message_status() {
    let client = create_test_client();
    let message = client.send("+1234567890", "Test").unwrap();
    
    assert_eq!(message.status(), &MessageStatus::Sent);
}

#[test]
fn test_send_invalid_phone() {
    let client = create_test_client();
    let result = client.send("invalid", "Hello");
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::InvalidPhoneNumber));
}

#[test]
fn test_send_empty_message() {
    let client = create_test_client();
    let result = client.send("+1234567890", "");
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::EmptyMessage));
}

#[test]
fn test_get_message_status() {
    let client = create_test_client();
    let message = client.send("+1234567890", "Test").unwrap();
    
    let status = client.get_status(message.id());
    assert!(status.is_ok());
    assert_eq!(status.unwrap(), MessageStatus::Sent);
}

#[test]
fn test_get_status_nonexistent() {
    let client = create_test_client();
    let result = client.get_status("nonexistent_id");
    
    assert!(result.is_err());
}

#[test]
fn test_message_history() {
    let client = create_test_client();
    
    client.send("+1111111111", "First").unwrap();
    client.send("+2222222222", "Second").unwrap();
    client.send("+3333333333", "Third").unwrap();
    
    let history = client.message_history();
    assert_eq!(history.len(), 3);
}

#[test]
fn test_mark_delivered() {
    let client = create_test_client();
    let message = client.send("+1234567890", "Test").unwrap();
    
    let result = client.mark_delivered(message.id());
    assert!(result.is_ok());
    
    let status = client.get_status(message.id()).unwrap();
    assert_eq!(status, MessageStatus::Delivered);
}

#[test]
fn test_complete_message_flow() {
    let client = create_test_client();
    
    // Add a contact
    let contact = client.add_contact("Alice", "+1234567890").unwrap();
    
    // Send message to contact
    let message = client.send_to_contact(&contact.id, "Hello Alice!").unwrap();
    assert_eq!(message.status(), &MessageStatus::Sent);
    
    // Check status
    let status = client.get_status(message.id()).unwrap();
    assert_eq!(status, MessageStatus::Sent);
    
    // Mark as delivered
    client.mark_delivered(message.id()).unwrap();
    
    // Verify delivered
    let final_status = client.get_status(message.id()).unwrap();
    assert_eq!(final_status, MessageStatus::Delivered);
    
    // Check history
    let history = client.message_history();
    assert_eq!(history.len(), 1);
}

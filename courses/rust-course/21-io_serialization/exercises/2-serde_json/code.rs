// Exercise 2: Serde and JSON
// 
// Implement JSON handling for the Textio API.
// Complete the functions marked with TODO.
//
// Add to Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Represents an SMS message in the Textio system
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub recipient: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(default)]
    pub status: MessageStatus,
}

/// Message delivery status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageStatus {
    #[default]
    Pending,
    Sent,
    Delivered,
    Failed,
}

/// API response for sending a message
#[derive(Debug, Serialize, Deserialize)]
pub struct SendResponse {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub status: String,
    #[serde(default)]
    pub segments: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Request to send an SMS
#[derive(Debug, Serialize, Deserialize)]
pub struct SendRequest {
    #[serde(rename = "to")]
    pub recipient: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

/// Contact for import/export
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub subscribed: bool,
}

/// Webhook event types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum WebhookEvent {
    MessageSent { message_id: String, timestamp: String },
    MessageDelivered { 
        message_id: String, 
        timestamp: String,
        recipient: String,
    },
    MessageFailed { 
        message_id: String, 
        error_code: u32,
        error_message: String,
    },
}

/// Parse a JSON API response into a SendResponse struct
pub fn parse_api_response(json: &str) -> Result<SendResponse, serde_json::Error> {
    // TODO: Implement this function
    // Use serde_json::from_str to deserialize the JSON
    
    todo!()
}

/// Create a SendRequest and serialize it to JSON
pub fn create_send_request(
    recipient: &str,
    message: &str,
    sender_id: Option<&str>,
) -> Result<String, serde_json::Error> {
    // TODO: Implement this function
    // 1. Create a SendRequest struct
    // 2. Serialize it to a JSON string using serde_json::to_string
    
    todo!()
}

/// Parse a webhook payload into a WebhookEvent
pub fn parse_webhook(json: &str) -> Result<WebhookEvent, serde_json::Error> {
    // TODO: Implement this function
    // Use serde_json::from_str to deserialize the webhook payload
    
    todo!()
}

/// Export contacts to a JSON string
pub fn export_contacts(contacts: &[Contact]) -> Result<String, serde_json::Error> {
    // TODO: Implement this function
    // 1. Create a wrapper structure with version and exported_at
    // 2. Serialize using serde_json::to_string_pretty
    
    todo!()
}

/// Import contacts from a JSON string
pub fn import_contacts(json: &str) -> Result<Vec<Contact>, serde_json::Error> {
    // TODO: Implement this function
    // Parse JSON and extract contacts array
    
    todo!()
}

/// Parse dynamic JSON into a Value type
pub fn parse_dynamic(json: &str) -> Result<Value, serde_json::Error> {
    // TODO: Implement this function
    // Parse JSON as generic Value
    
    todo!()
}

/// Extract a field from a JSON Value
pub fn get_json_string(value: &Value, field: &str) -> Option<String> {
    // TODO: Implement this function
    // Get a string field from a Value
    
    todo!()
}

/// Merge two JSON objects
pub fn merge_json(base: &Value, overlay: &Value) -> Value {
    // TODO: Implement this function
    // If both are objects, merge them (overlay takes precedence)
    // Otherwise, return overlay
    
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_api_response() {
        let json = r#"{
            "messageId": "msg_123",
            "status": "sent",
            "segments": 1
        }"#;
        
        let response = parse_api_response(json).expect("Failed to parse");
        assert_eq!(response.message_id, "msg_123");
        assert_eq!(response.status, "sent");
        assert_eq!(response.segments, 1);
    }

    #[test]
    fn test_create_send_request() {
        let json = create_send_request("+1234567890", "Hello!", Some("Textio"))
            .expect("Failed to create");
        
        assert!(json.contains("\"to\":\"+1234567890\""));
        assert!(json.contains("\"message\":\"Hello!\""));
        assert!(json.contains("\"sender_id\":\"Textio\""));
    }

    #[test]
    fn test_create_send_request_without_sender() {
        let json = create_send_request("+1234567890", "Hello!", None)
            .expect("Failed to create");
        
        assert!(!json.contains("sender_id"));
    }

    #[test]
    fn test_parse_webhook_sent() {
        let json = r#"{
            "event": "message_sent",
            "message_id": "msg_123",
            "timestamp": "2024-01-15T10:30:00Z"
        }"#;
        
        let event = parse_webhook(json).expect("Failed to parse");
        match event {
            WebhookEvent::MessageSent { message_id, .. } => {
                assert_eq!(message_id, "msg_123");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_parse_webhook_delivered() {
        let json = r#"{
            "event": "message_delivered",
            "message_id": "msg_123",
            "timestamp": "2024-01-15T10:35:00Z",
            "recipient": "+1234567890"
        }"#;
        
        let event = parse_webhook(json).expect("Failed to parse");
        match event {
            WebhookEvent::MessageDelivered { message_id, recipient, .. } => {
                assert_eq!(message_id, "msg_123");
                assert_eq!(recipient, "+1234567890");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_export_import_contacts() {
        let contacts = vec![
            Contact {
                name: "John Doe".to_string(),
                phone: "+1234567890".to_string(),
                email: Some("john@example.com".to_string()),
                subscribed: true,
            },
            Contact {
                name: "Jane Smith".to_string(),
                phone: "+0987654321".to_string(),
                email: None,
                subscribed: false,
            },
        ];
        
        let json = export_contacts(&contacts).expect("Failed to export");
        let imported = import_contacts(&json).expect("Failed to import");
        
        assert_eq!(imported.len(), 2);
        assert_eq!(imported[0].name, "John Doe");
        assert_eq!(imported[1].email, None);
    }

    #[test]
    fn test_parse_dynamic() {
        let json = r#"{"name":"test","count":42}"#;
        let value = parse_dynamic(json).expect("Failed to parse");
        
        assert_eq!(get_json_string(&value, "name"), Some("test".to_string()));
        assert_eq!(get_json_string(&value, "count"), None);
    }

    #[test]
    fn test_merge_json() {
        let base = json!({"a": 1, "b": 2});
        let overlay = json!({"b": 3, "c": 4});
        
        let merged = merge_json(&base, &overlay);
        
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["b"], 3);
        assert_eq!(merged["c"], 4);
    }
}

fn main() {
    println!("Run tests with: cargo test");
}

// Exercise 2: Serde and JSON - Complete Solution
//
// Add to Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageStatus {
    #[default]
    Pending,
    Sent,
    Delivered,
    Failed,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SendRequest {
    #[serde(rename = "to")]
    pub recipient: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub subscribed: bool,
}

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

#[derive(Debug, Serialize, Deserialize)]
struct ContactExport {
    version: String,
    contacts: Vec<Contact>,
    exported_at: String,
}

pub fn parse_api_response(json: &str) -> Result<SendResponse, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn create_send_request(
    recipient: &str,
    message: &str,
    sender_id: Option<&str>,
) -> Result<String, serde_json::Error> {
    let request = SendRequest {
        recipient: recipient.to_string(),
        message: message.to_string(),
        sender_id: sender_id.map(|s| s.to_string()),
    };
    serde_json::to_string(&request)
}

pub fn parse_webhook(json: &str) -> Result<WebhookEvent, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn export_contacts(contacts: &[Contact]) -> Result<String, serde_json::Error> {
    let export = ContactExport {
        version: "1.0".to_string(),
        contacts: contacts.to_vec(),
        exported_at: "2024-01-15T10:30:00Z".to_string(),
    };
    serde_json::to_string_pretty(&export)
}

pub fn import_contacts(json: &str) -> Result<Vec<Contact>, serde_json::Error> {
    let export: ContactExport = serde_json::from_str(json)?;
    Ok(export.contacts)
}

pub fn parse_dynamic(json: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn get_json_string(value: &Value, field: &str) -> Option<String> {
    value.get(field).and_then(|v| v.as_str()).map(|s| s.to_string())
}

pub fn merge_json(base: &Value, overlay: &Value) -> Value {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            let mut merged = Map::new();
            for (k, v) in base_map.iter() {
                merged.insert(k.clone(), v.clone());
            }
            for (k, v) in overlay_map.iter() {
                if let Some(base_val) = base_map.get(k) {
                    merged.insert(k.clone(), merge_json(base_val, v));
                } else {
                    merged.insert(k.clone(), v.clone());
                }
            }
            Value::Object(merged)
        }
        _ => overlay.clone(),
    }
}

fn main() {
    let response_json = r#"{
        "messageId": "msg_abc123",
        "status": "sent",
        "segments": 2
    }"#;
    
    let response = parse_api_response(response_json).expect("Failed to parse");
    println!("API Response: {:?}", response);
    
    let request_json = create_send_request("+1234567890", "Hello, World!", Some("Textio"))
        .expect("Failed to create");
    println!("\nSend Request JSON: {}", request_json);
    
    let webhook_json = r#"{
        "event": "message_delivered",
        "message_id": "msg_abc123",
        "timestamp": "2024-01-15T10:35:00Z",
        "recipient": "+1234567890"
    }"#;
    
    let event = parse_webhook(webhook_json).expect("Failed to parse");
    println!("\nWebhook Event: {:?}", event);
    
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
    
    let exported = export_contacts(&contacts).expect("Failed to export");
    println!("\nExported Contacts:\n{}", exported);
    
    let dynamic = parse_dynamic(r#"{"app":"Textio","version":"2.0"}"#).unwrap();
    if let Some(app) = get_json_string(&dynamic, "app") {
        println!("\nApp name: {}", app);
    }
    
    let base = json!({"api_key": "secret", "timeout": 30});
    let overlay = json!({"timeout": 60, "debug": true});
    let merged = merge_json(&base, &overlay);
    println!("\nMerged config: {}", merged);
}

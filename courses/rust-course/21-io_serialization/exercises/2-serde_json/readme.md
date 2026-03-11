# Exercise 2: Serde and JSON

## Overview

Serialization and deserialization are essential for data exchange in modern applications. Serde (SERialization/DEserialization) is Rust's de facto standard framework for converting data structures to and from various formats. JSON (JavaScript Object Notation) is the most common format for web APIs.

## Learning Objectives

By the end of this exercise, you will be able to:

- Use `#[derive(Serialize, Deserialize)]` for automatic serialization
- Serialize Rust structs to JSON strings
- Deserialize JSON strings to Rust structs
- Work with `serde_json::Value` for dynamic JSON
- Customize serialization with attributes
- Handle optional and default fields
- Work with nested structures and collections

## Core Concepts

### Serde Framework

Serde provides a framework with two main traits:

- `Serialize`: Convert Rust types to data formats
- `Deserialize`: Convert data formats to Rust types

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
    id: u64,
    content: String,
}
```

### Basic Serialization

Convert Rust structs to JSON:

```rust
use serde_json;

let message = Message {
    id: 1,
    content: "Hello".to_string(),
};

// To JSON string (compact)
let json = serde_json::to_string(&message)?;
// Result: {"id":1,"content":"Hello"}

// To JSON string (pretty printed)
let json_pretty = serde_json::to_string_pretty(&message)?;
// Result:
// {
//   "id": 1,
//   "content": "Hello"
// }

// To Vec<u8> (for file/network)
let bytes = serde_json::to_vec(&message)?;
```

### Basic Deserialization

Convert JSON to Rust structs:

```rust
// From string
let json = r#"{"id":1,"content":"Hello"}"#;
let message: Message = serde_json::from_str(json)?;

// From bytes
let bytes = br#"{"id":1,"content":"Hello"}"#;
let message: Message = serde_json::from_slice(bytes)?;

// From reader (file, network stream)
let message: Message = serde_json::from_reader(reader)?;
```

### The Value Type

For dynamic or unknown JSON structures:

```rust
use serde_json::{Value, json};

// Parse as generic Value
let value: Value = serde_json::from_str(json_str)?;

// Access fields dynamically
if let Some(id) = value.get("id").and_then(|v| v.as_u64()) {
    println!("ID: {}", id);
}

// Create JSON values with json! macro
let value = json!({
    "name": "John",
    "age": 30,
    "active": true,
    "tags": ["rust", "serde"]
});
```

### Serialization Attributes

#### Field Renaming

```rust
#[derive(Serialize, Deserialize)]
struct Message {
    #[serde(rename = "messageId")]
    id: u64,
    
    #[serde(rename = "messageBody")]
    content: String,
}
// JSON: {"messageId":1,"messageBody":"Hello"}
```

#### Renaming All Fields

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    first_name: String,
    last_name: String,
}
// JSON: {"firstName":"John","lastName":"Doe"}

#[serde(rename_all = "snake_case")]  // default
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "kebab-case")]
#[serde(rename_all = "PascalCase")]
```

#### Optional Fields

```rust
#[derive(Serialize, Deserialize)]
struct Message {
    id: u64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<u64>,
    
    #[serde(default)]
    priority: u32,  // Defaults to 0 if missing
}
```

#### Default Values

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default = "default_timeout")]
    timeout: u64,
    
    #[serde(default)]
    retries: u32,
}

fn default_timeout() -> u64 {
    30
}
```

#### Skip Fields

```rust
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    
    #[serde(skip)]
    password: String,  // Never serialized
    
    #[serde(skip_serializing)]
    internal_id: u64,  // Not serialized, but can be deserialized
    
    #[serde(skip_deserializing)]
    computed_hash: String,  // Serialized but not deserialized
}
```

### Working with Collections

```rust
#[derive(Serialize, Deserialize)]
struct MessageBatch {
    messages: Vec<Message>,
    total: usize,
}

let batch = MessageBatch {
    messages: vec![
        Message { id: 1, content: "Hello".into() },
        Message { id: 2, content: "World".into() },
    ],
    total: 2,
};

let json = serde_json::to_string_pretty(&batch)?;
```

### Nested Structures

```rust
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    status: String,
    data: MessageData,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MessageData {
    message: Message,
    metadata: MessageMetadata,
}

#[derive(Serialize, Deserialize)]
struct MessageMetadata {
    created_at: String,
    sender: String,
}
```

### Enums

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum MessageStatus {
    Pending,
    Sent { at: String },
    Delivered { at: String, to: String },
    Failed { reason: String },
}

// JSON: {"type":"Sent","at":"2024-01-15T10:30:00Z"}
```

### Error Handling

```rust
use serde_json::Error;

fn parse_message(json: &str) -> Result<Message, Error> {
    serde_json::from_str(json)
}

// Handle specific errors
match parse_message(json_str) {
    Ok(message) => println!("Parsed: {:?}", message),
    Err(e) => {
        if e.is_data() {
            println!("Data error: {}", e);
        } else if e.is_syntax() {
            println!("Syntax error: {}", e);
        } else if e.is_eof() {
            println!("Unexpected end of input");
        }
    }
}
```

## Textio Context

In the Textio SMS API, JSON is used extensively:

### API Request/Response

```rust
#[derive(Serialize, Deserialize)]
struct SendSmsRequest {
    #[serde(rename = "to")]
    recipient: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SendSmsResponse {
    message_id: String,
    status: String,
    #[serde(default)]
    segments: u32,
}
```

### Webhook Payloads

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "event")]
enum WebhookPayload {
    #[serde(rename = "message.sent")]
    Sent { message_id: String, timestamp: String },
    
    #[serde(rename = "message.delivered")]
    Delivered { 
        message_id: String, 
        timestamp: String,
        recipient: String,
    },
    
    #[serde(rename = "message.failed")]
    Failed { 
        message_id: String, 
        error_code: u32,
        error_message: String,
    },
}
```

### Contact Import/Export

```rust
#[derive(Serialize, Deserialize)]
struct ContactExport {
    version: String,
    #[serde(rename = "contacts")]
    items: Vec<Contact>,
    exported_at: String,
}

#[derive(Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(default)]
    subscribed: bool,
}
```

## Best Practices

1. **Use derive macros**: They're safe, fast, and maintainable
2. **Handle missing fields**: Use `Option<T>` or `#[serde(default)]`
3. **Rename for API compatibility**: Use `#[serde(rename = "...")]`
4. **Skip sensitive data**: Use `#[serde(skip)]` for passwords
5. **Use json! macro**: For constructing test data
6. **Validate after deserialization**: Don't trust external data

## Exercise Instructions

You will implement JSON handling for the Textio API:

1. **parse_api_response**: Deserialize a JSON API response
2. **create_send_request**: Create and serialize a send SMS request
3. **parse_webhook**: Parse webhook payloads with different event types
4. **export_contacts**: Serialize contacts to JSON format

## Key Takeaways

- Serde derive macros make serialization automatic
- `serde_json` provides JSON-specific functionality
- Use attributes to customize field names and behavior
- `Value` type handles dynamic/unknown JSON
- Always handle potential deserialization errors

## Further Reading

- [Serde documentation](https://serde.rs/)
- [serde_json documentation](https://docs.rs/serde_json/)
- [Serde attributes reference](https://serde.rs/attributes.html)

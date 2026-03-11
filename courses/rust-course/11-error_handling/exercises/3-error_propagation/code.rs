// Textio SMS API - Error Propagation Exercise
// Practice using the ? operator for clean error handling

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum CampaignError {
    InvalidPhone(String),
    MessageTooLong { max: usize, actual: usize },
    TemplateNotFound(String),
    MissingPlaceholder(String),
    ContactNotFound(String),
    SendFailed(String),
}

impl fmt::Display for CampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CampaignError::InvalidPhone(p) => write!(f, "Invalid phone: {}", p),
            CampaignError::MessageTooLong { max, actual } => {
                write!(f, "Message too long: {} > {}", actual, max)
            }
            CampaignError::TemplateNotFound(t) => write!(f, "Template not found: {}", t),
            CampaignError::MissingPlaceholder(p) => write!(f, "Missing placeholder: {}", p),
            CampaignError::ContactNotFound(c) => write!(f, "Contact not found: {}", c),
            CampaignError::SendFailed(msg) => write!(f, "Send failed: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub body: String,
}

pub struct TemplateStore {
    templates: HashMap<String, Template>,
}

impl TemplateStore {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        templates.insert(
            "welcome".to_string(),
            Template {
                name: "welcome".to_string(),
                body: "Hello {name}, welcome to Textio!".to_string(),
            },
        );
        templates.insert(
            "promo".to_string(),
            Template {
                name: "promo".to_string(),
                body: "Hi {name}, check out our latest offers!".to_string(),
            },
        );
        TemplateStore { templates }
    }

    // TODO: Implement get_template
    // Requirements:
    // - Look up template by name
    // - Return Ok(&Template) if found
    // - Return Err(CampaignError::TemplateNotFound) if not found
    // Use ok_or() to convert Option to Result
    pub fn get_template(&self, name: &str) -> Result<&Template, CampaignError> {
        // Your code here
        todo!()
    }
}

pub struct ContactStore {
    contacts: HashMap<String, Contact>,
}

impl ContactStore {
    pub fn new() -> Self {
        let mut contacts = HashMap::new();
        contacts.insert(
            "alice".to_string(),
            Contact {
                name: "Alice".to_string(),
                phone: "5551234567".to_string(),
            },
        );
        contacts.insert(
            "bob".to_string(),
            Contact {
                name: "Bob".to_string(),
                phone: "5559876543".to_string(),
            },
        );
        ContactStore { contacts }
    }

    // TODO: Implement get_contact
    // Requirements:
    // - Look up contact by name
    // - Return Ok(Contact) if found
    // - Return Err(CampaignError::ContactNotFound) if not found
    pub fn get_contact(&self, name: &str) -> Result<Contact, CampaignError> {
        // Your code here
        todo!()
    }
}

// TODO: Implement parse_phone
// Requirements:
// - Phone must be 10 digits
// - Return Ok(phone) if valid
// - Return Err(CampaignError::InvalidPhone) if invalid
// Use ok_or() to convert validation to Result
pub fn parse_phone(phone: &str) -> Result<String, CampaignError> {
    // Your code here
    todo!()
}

// TODO: Implement validate_message
// Requirements:
// - Message must be 160 chars or less
// - Return Ok(()) if valid
// - Return Err(CampaignError::MessageTooLong) if too long
pub fn validate_message(message: &str) -> Result<(), CampaignError> {
    // Your code here
    todo!()
}

// TODO: Implement render_template
// Requirements:
// - Get the template from store using ?
// - Replace {name} with contact.name
// - Return Ok(rendered_message)
// Note: Just use simple replace, don't validate placeholders
pub fn render_template(
    store: &TemplateStore,
    template_name: &str,
    contact: &Contact,
) -> Result<String, CampaignError> {
    // Your code here - use ? to propagate template lookup error
    todo!()
}

// TODO: Implement send_campaign_message
// Requirements:
// - Use ? to chain all operations:
//   1. Get contact from contact_store
//   2. Parse phone number
//   3. Render template
//   4. Validate message length
// - Return Ok(format!("Sent to: {}", phone))
pub fn send_campaign_message(
    contact_store: &ContactStore,
    template_store: &TemplateStore,
    contact_name: &str,
    template_name: &str,
) -> Result<String, CampaignError> {
    // Your code here - chain all operations with ?
    todo!()
}

// TODO: Implement process_batch
// Requirements:
// - Process a list of (contact_name, template_name) pairs
// - For each pair, call send_campaign_message
// - Collect successful sends into a Vec<String>
// - Return Ok(results) with all successful messages
// - If ANY fails, return the first error
pub fn process_batch(
    contact_store: &ContactStore,
    template_store: &TemplateStore,
    batch: &[(String, String)],
) -> Result<Vec<String>, CampaignError> {
    // Your code here - use a loop with ? for early return
    todo!()
}

// Helper function that always succeeds (for testing)
pub fn send_to_carrier(phone: &str, message: &str) -> Result<String, CampaignError> {
    Ok(format!("Message sent to {}: {} chars", phone, message.len()))
}

fn main() {
    let template_store = TemplateStore::new();
    let contact_store = ContactStore::new();

    println!("=== Textio SMS API - Error Propagation Demo ===\n");

    // Test parse_phone
    println!("--- Phone Parsing ---");
    
    let valid = parse_phone("5551234567");
    println!("Valid phone: {:?}", valid);
    
    let invalid = parse_phone("123");
    println!("Invalid phone: {:?}", invalid);

    // Test validate_message
    println!("\n--- Message Validation ---");
    
    let short = validate_message("Hello!");
    println!("Short message: {:?}", short);
    
    let long = validate_message(&"x".repeat(200));
    println!("Long message: {:?}", long);

    // Test get_template
    println!("\n--- Template Lookup ---");
    
    let found = template_store.get_template("welcome");
    println!("Found 'welcome': {:?}", found.map(|t| &t.body));
    
    let missing = template_store.get_template("unknown");
    println!("Missing 'unknown': {:?}", missing);

    // Test render_template
    println!("\n--- Template Rendering ---");
    
    let contact = Contact {
        name: "Test".to_string(),
        phone: "5551111111".to_string(),
    };
    
    let rendered = render_template(&template_store, "welcome", &contact);
    println!("Rendered welcome: {:?}", rendered);
    
    let failed = render_template(&template_store, "missing", &contact);
    println!("Missing template: {:?}", failed);

    // Test send_campaign_message (chaining with ?)
    println!("\n--- Send Campaign Message (Full Chain) ---");
    
    let success = send_campaign_message(&contact_store, &template_store, "alice", "welcome");
    println!("Success case: {:?}", success);
    
    let bad_contact = send_campaign_message(&contact_store, &template_store, "unknown", "welcome");
    println!("Bad contact: {:?}", bad_contact);
    
    let bad_template = send_campaign_message(&contact_store, &template_store, "bob", "unknown");
    println!("Bad template: {:?}", bad_template);

    // Test process_batch
    println!("\n--- Batch Processing ---");
    
    let batch1 = vec![
        ("alice".to_string(), "welcome".to_string()),
        ("bob".to_string(), "promo".to_string()),
    ];
    let result1 = process_batch(&contact_store, &template_store, &batch1);
    println!("Valid batch: {:?}", result1);
    
    let batch2 = vec![
        ("alice".to_string(), "welcome".to_string()),
        ("unknown".to_string(), "promo".to_string()),
    ];
    let result2 = process_batch(&contact_store, &template_store, &batch2);
    println!("Mixed batch: {:?}", result2);

    // Demonstrate map_err for adding context
    println!("\n--- Adding Context with map_err ---");
    
    let result = template_store
        .get_template("missing")
        .map_err(|e| format!("Campaign failed: {}", e));
    println!("With context: {:?}", result);

    println!("\n=== Exercise Complete ===");
}

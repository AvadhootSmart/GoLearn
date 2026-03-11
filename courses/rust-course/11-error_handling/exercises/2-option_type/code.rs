// Textio SMS API - Option Type Exercise
// Practice working with Option<T> for nullable values

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct Message {
    pub id: u64,
    pub to: String,
    pub status: String,
}

pub struct TextioDb {
    contacts: HashMap<String, Contact>,
    messages: HashMap<u64, Message>,
}

impl TextioDb {
    pub fn new() -> Self {
        let mut contacts = HashMap::new();
        contacts.insert(
            "alice".to_string(),
            Contact {
                name: "Alice Smith".to_string(),
                phone: "5551234567".to_string(),
                active: true,
            },
        );
        contacts.insert(
            "bob".to_string(),
            Contact {
                name: "Bob Jones".to_string(),
                phone: "5559876543".to_string(),
                active: true,
            },
        );
        contacts.insert(
            "inactive".to_string(),
            Contact {
                name: "Old User".to_string(),
                phone: "5550000000".to_string(),
                active: false,
            },
        );

        let mut messages = HashMap::new();
        messages.insert(
            1,
            Message {
                id: 1,
                to: "5551234567".to_string(),
                status: "delivered".to_string(),
            },
        );
        messages.insert(
            2,
            Message {
                id: 2,
                to: "5559876543".to_string(),
                status: "pending".to_string(),
            },
        );

        TextioDb { contacts, messages }
    }

    // TODO: Implement find_contact
    // Requirements:
    // - Look up contact by name (case-insensitive, use to_lowercase())
    // - Return Some(Contact) if found, None otherwise
    // - Use get() on the HashMap
    pub fn find_contact(&self, name: &str) -> Option<Contact> {
        // Your code here
        todo!()
    }

    // TODO: Implement find_active_contact
    // Requirements:
    // - Find the contact by name
    // - Use filter() to only return Some if contact.active is true
    // - Return None if not found OR if inactive
    pub fn find_active_contact(&self, name: &str) -> Option<Contact> {
        // Your code here
        todo!()
    }

    // TODO: Implement get_message
    // Requirements:
    // - Look up message by id
    // - Return Some(&Message) if found, None otherwise
    pub fn get_message(&self, id: u64) -> Option<&Message> {
        // Your code here
        todo!()
    }

    // TODO: Implement get_message_status
    // Requirements:
    // - Find message by id
    // - Use map() to extract just the status string
    // - Return None if message not found
    pub fn get_message_status(&self, id: u64) -> Option<String> {
        // Your code here
        todo!()
    }
}

// TODO: Implement format_message
// Requirements:
// - Take an optional template and a contact
// - If template is Some, replace "{name}" with contact.name
// - If template is None, return "Hi {name}!" as default
// - Use map() and unwrap_or()
pub fn format_message(template: Option<&str>, contact: &Contact) -> String {
    // Your code here
    todo!()
}

// TODO: Implement get_phone_or_default
// Requirements:
// - Take an Option<Contact>
// - Use map() to extract the phone field
// - Use unwrap_or_default() to return empty string if None
pub fn get_phone_or_default(contact: Option<Contact>) -> String {
    // Your code here
    todo!()
}

// TODO: Implement find_and_send
// Requirements:
// - Use and_then() to chain operations
// - First find the contact
// - Then check if active (use filter)
// - Return the formatted message or None
pub fn find_and_send(db: &TextioDb, name: &str, template: Option<&str>) -> Option<String> {
    // Your code here - use and_then and filter
    todo!()
}

// TODO: Implement contact_to_result
// Requirements:
// - Convert Option<Contact> to Result<Contact, String>
// - Use ok_or() with error message "Contact not found"
pub fn contact_to_result(contact: Option<Contact>) -> Result<Contact, String> {
    // Your code here
    todo!()
}

fn main() {
    let db = TextioDb::new();

    println!("=== Textio SMS API - Option Type Demo ===\n");

    // Test find_contact
    println!("--- Finding Contacts ---");
    
    let alice = db.find_contact("alice");
    println!("Find 'alice': {:?}", alice);
    
    let unknown = db.find_contact("unknown");
    println!("Find 'unknown': {:?}", unknown);
    
    let case_test = db.find_contact("ALICE");
    println!("Find 'ALICE' (case insensitive): {:?}", case_test);

    // Test is_some() and is_none()
    println!("\n--- Checking Option State ---");
    println!("alice.is_some(): {}", alice.is_some());
    println!("unknown.is_none(): {}", unknown.is_none());

    // Test find_active_contact with filter
    println!("\n--- Finding Active Contacts ---");
    
    let active_alice = db.find_active_contact("alice");
    println!("Active 'alice': {:?}", active_alice);
    
    let active_inactive = db.find_active_contact("inactive");
    println!("Active 'inactive' (should be None): {:?}", active_inactive);

    // Test get_message_status with map
    println!("\n--- Message Status with map() ---");
    
    let status1 = db.get_message_status(1);
    println!("Status of message 1: {:?}", status1);
    
    let status99 = db.get_message_status(99);
    println!("Status of message 99: {:?}", status99);

    // Test format_message
    println!("\n--- Formatting Messages ---");
    
    let contact = alice.as_ref().unwrap();
    let custom = format_message(Some("Hello, {name}! Welcome to Textio."), contact);
    println!("Custom template: {}", custom);
    
    let default = format_message(None, contact);
    println!("Default template: {}", default);

    // Test get_phone_or_default
    println!("\n--- Phone with Default ---");
    
    let phone1 = get_phone_or_default(alice.clone());
    println!("Phone from Some: {}", phone1);
    
    let phone2 = get_phone_or_default(None);
    println!("Phone from None: '{}'", phone2);

    // Test find_and_send with and_then
    println!("\n--- Find and Send (and_then + filter) ---");
    
    let msg1 = find_and_send(&db, "alice", Some("Hi {name}!"));
    println!("Send to alice: {:?}", msg1);
    
    let msg2 = find_and_send(&db, "unknown", Some("Hi {name}!"));
    println!("Send to unknown: {:?}", msg2);
    
    let msg3 = find_and_send(&db, "inactive", Some("Hi {name}!"));
    println!("Send to inactive: {:?}", msg3);

    // Test contact_to_result
    println!("\n--- Converting Option to Result ---");
    
    let result1 = contact_to_result(alice);
    println!("Some contact as Result: {:?}", result1);
    
    let result2 = contact_to_result(None);
    println!("None contact as Result: {:?}", result2);

    // Test unwrap_or_default
    println!("\n--- unwrap_or_default() ---");
    
    let some_phone: Option<String> = Some("5551112222".to_string());
    let none_phone: Option<String> = None;
    
    println!("Some phone: {}", some_phone.unwrap_or_default());
    println!("None phone: '{}'", none_phone.unwrap_or_default());

    println!("\n=== Exercise Complete ===");
}

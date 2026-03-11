// Textio SMS API - Option Type Exercise (Complete Solution)
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

    pub fn find_contact(&self, name: &str) -> Option<Contact> {
        self.contacts.get(&name.to_lowercase()).cloned()
    }

    pub fn find_active_contact(&self, name: &str) -> Option<Contact> {
        self.find_contact(name)
            .filter(|c| c.active)
    }

    pub fn get_message(&self, id: u64) -> Option<&Message> {
        self.messages.get(&id)
    }

    pub fn get_message_status(&self, id: u64) -> Option<String> {
        self.get_message(id)
            .map(|m| m.status.clone())
    }
}

pub fn format_message(template: Option<&str>, contact: &Contact) -> String {
    template
        .map(|t| t.replace("{name}", &contact.name))
        .unwrap_or(format!("Hi {}!", contact.name))
}

pub fn get_phone_or_default(contact: Option<Contact>) -> String {
    contact
        .map(|c| c.phone)
        .unwrap_or_default()
}

pub fn find_and_send(db: &TextioDb, name: &str, template: Option<&str>) -> Option<String> {
    db.find_active_contact(name)
        .map(|c| format_message(template, &c))
}

pub fn contact_to_result(contact: Option<Contact>) -> Result<Contact, String> {
    contact.ok_or("Contact not found".to_string())
}

fn main() {
    let db = TextioDb::new();

    println!("=== Textio SMS API - Option Type Demo ===\n");

    println!("--- Finding Contacts ---");
    
    let alice = db.find_contact("alice");
    println!("Find 'alice': {:?}", alice);
    
    let unknown = db.find_contact("unknown");
    println!("Find 'unknown': {:?}", unknown);
    
    let case_test = db.find_contact("ALICE");
    println!("Find 'ALICE' (case insensitive): {:?}", case_test);

    println!("\n--- Checking Option State ---");
    println!("alice.is_some(): {}", alice.is_some());
    println!("unknown.is_none(): {}", unknown.is_none());

    println!("\n--- Finding Active Contacts ---");
    
    let active_alice = db.find_active_contact("alice");
    println!("Active 'alice': {:?}", active_alice);
    
    let active_inactive = db.find_active_contact("inactive");
    println!("Active 'inactive' (should be None): {:?}", active_inactive);

    println!("\n--- Message Status with map() ---");
    
    let status1 = db.get_message_status(1);
    println!("Status of message 1: {:?}", status1);
    
    let status99 = db.get_message_status(99);
    println!("Status of message 99: {:?}", status99);

    println!("\n--- Formatting Messages ---");
    
    let contact = alice.as_ref().unwrap();
    let custom = format_message(Some("Hello, {name}! Welcome to Textio."), contact);
    println!("Custom template: {}", custom);
    
    let default = format_message(None, contact);
    println!("Default template: {}", default);

    println!("\n--- Phone with Default ---");
    
    let phone1 = get_phone_or_default(alice.clone());
    println!("Phone from Some: {}", phone1);
    
    let phone2 = get_phone_or_default(None);
    println!("Phone from None: '{}'", phone2);

    println!("\n--- Find and Send (and_then + filter) ---");
    
    let msg1 = find_and_send(&db, "alice", Some("Hi {name}!"));
    println!("Send to alice: {:?}", msg1);
    
    let msg2 = find_and_send(&db, "unknown", Some("Hi {name}!"));
    println!("Send to unknown: {:?}", msg2);
    
    let msg3 = find_and_send(&db, "inactive", Some("Hi {name}!"));
    println!("Send to inactive: {:?}", msg3);

    println!("\n--- Converting Option to Result ---");
    
    let result1 = contact_to_result(alice);
    println!("Some contact as Result: {:?}", result1);
    
    let result2 = contact_to_result(None);
    println!("None contact as Result: {:?}", result2);

    println!("\n--- unwrap_or_default() ---");
    
    let some_phone: Option<String> = Some("5551112222".to_string());
    let none_phone: Option<String> = None;
    
    println!("Some phone: {}", some_phone.unwrap_or_default());
    println!("None phone: '{}'", none_phone.unwrap_or_default());

    println!("\n=== Exercise Complete ===");
}

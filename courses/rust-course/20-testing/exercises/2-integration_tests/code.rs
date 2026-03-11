// Textio Integration Testing Exercise
//
// This file represents src/lib.rs - the main library code
// Integration tests would be in the tests/ directory
//
// In this exercise, you'll:
// 1. Create integration tests in tests/ directory
// 2. Test the public API of the Textio library
// 3. Create shared test utilities

// src/lib.rs - Library code (provided)

pub struct Client {
    api_key: String,
    messages: std::cell::RefCell<Vec<Message>>,
    contacts: std::cell::RefCell<Vec<Contact>>,
}

#[derive(Debug, Clone)]
pub struct Message {
    id: String,
    to: String,
    body: String,
    status: MessageStatus,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

#[derive(Debug)]
pub enum Error {
    InvalidPhoneNumber,
    EmptyMessage,
    ContactNotFound,
    RateLimited,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidPhoneNumber => write!(f, "Invalid phone number"),
            Error::EmptyMessage => write!(f, "Message cannot be empty"),
            Error::ContactNotFound => write!(f, "Contact not found"),
            Error::RateLimited => write!(f, "Rate limit exceeded"),
        }
    }
}

impl std::error::Error for Error {}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
            messages: std::cell::RefCell::new(Vec::new()),
            contacts: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn send(&self, to: &str, body: &str) -> Result<Message, Error> {
        if !to.starts_with('+') || to.len() < 10 {
            return Err(Error::InvalidPhoneNumber);
        }
        
        if body.is_empty() {
            return Err(Error::EmptyMessage);
        }

        let message = Message {
            id: format!("msg_{}", uuid::Uuid::new_v4()),
            to: to.to_string(),
            body: body.to_string(),
            status: MessageStatus::Sent,
        };
        
        self.messages.borrow_mut().push(message.clone());
        Ok(message)
    }

    pub fn get_status(&self, id: &str) -> Result<MessageStatus, Error> {
        self.messages
            .borrow()
            .iter()
            .find(|m| m.id == id)
            .map(|m| m.status.clone())
            .ok_or(Error::ContactNotFound)
    }

    pub fn add_contact(&self, name: &str, phone: &str) -> Result<Contact, Error> {
        if !phone.starts_with('+') {
            return Err(Error::InvalidPhoneNumber);
        }

        let contact = Contact {
            id: format!("contact_{}", uuid::Uuid::new_v4()),
            name: name.to_string(),
            phone: phone.to_string(),
        };
        
        self.contacts.borrow_mut().push(contact.clone());
        Ok(contact)
    }

    pub fn get_contact(&self, id: &str) -> Option<Contact> {
        self.contacts
            .borrow()
            .iter()
            .find(|c| c.id == id)
            .cloned()
    }

    pub fn list_contacts(&self) -> Vec<Contact> {
        self.contacts.borrow().clone()
    }

    pub fn send_to_contact(&self, contact_id: &str, body: &str) -> Result<Message, Error> {
        let contact = self.get_contact(contact_id).ok_or(Error::ContactNotFound)?;
        self.send(&contact.phone, body)
    }

    pub fn message_history(&self) -> Vec<Message> {
        self.messages.borrow().clone()
    }

    pub fn mark_delivered(&self, message_id: &str) -> Result<(), Error> {
        let mut messages = self.messages.borrow_mut();
        if let Some(msg) = messages.iter_mut().find(|m| m.id == message_id) {
            msg.status = MessageStatus::Delivered;
            Ok(())
        } else {
            Err(Error::ContactNotFound)
        }
    }
}

impl Message {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn status(&self) -> &MessageStatus {
        &self.status
    }
}

// Note: For this exercise, create these files in a tests/ directory:
//
// tests/common/mod.rs - Shared test utilities
// tests/client_test.rs - Tests for Client operations
// tests/contact_test.rs - Tests for Contact operations
// tests/message_test.rs - Tests for Message operations

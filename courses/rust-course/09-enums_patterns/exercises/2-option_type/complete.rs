// Exercise 2: The Option Type - Complete Solution

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub to: String,
    pub body: String,
    pub from: Option<String>,
    pub media_url: Option<String>,
    pub status_code: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

pub struct MessageStore {
    messages: Vec<Message>,
}

impl MessageStore {
    pub fn new() -> Self {
        MessageStore {
            messages: Vec::new(),
        }
    }
    
    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }
    
    pub fn find(&self, id: u64) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }
    
    pub fn find_by_phone(&self, phone: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.to == phone)
    }
    
    pub fn get_sender(&self, message_id: u64) -> Option<&str> {
        self.find(message_id).and_then(|m| m.from.as_deref())
    }
    
    pub fn get_media_url(&self, message_id: u64) -> Option<&str> {
        self.find(message_id).and_then(|m| m.media_url.as_deref())
    }
    
    pub fn find_latest(&self, phone: &str) -> Option<&Message> {
        self.messages.iter().rev().find(|m| m.to == phone)
    }
    
    pub fn count_with_status(&self, status: u16) -> Option<usize> {
        let count = self.messages.iter()
            .filter_map(|m| m.status_code)
            .filter(|&s| s == status)
            .count();
        
        if count > 0 { Some(count) } else { None }
    }
}

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            users: Vec::new(),
        }
    }
    
    pub fn add(&mut self, user: User) {
        self.users.push(user);
    }
    
    pub fn find(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    
    pub fn get_phone(&self, user_id: u32) -> Option<&str> {
        self.find(user_id).and_then(|u| u.phone.as_deref())
    }
    
    pub fn get_email(&self, user_id: u32) -> Option<&str> {
        self.find(user_id).and_then(|u| u.email.as_deref())
    }
}

pub fn optional_length(s: Option<&str>) -> usize {
    s.map(|s| s.len()).unwrap_or(0)
}

pub fn format_phone(phone: Option<&str>) -> String {
    match phone {
        Some(p) => format!("+1{}", p),
        None => String::from("Unknown"),
    }
}

pub fn get_message_sender(store: &MessageStore, _user_mgr: &UserManager, message_id: u64) -> Option<String> {
    store.find(message_id).and_then(|m| m.from.clone())
}

pub fn get_contact_info(user_mgr: &UserManager, user_id: u32) -> Option<String> {
    let user = user_mgr.find(user_id)?;
    
    user.phone.clone()
        .or_else(|| user.email.clone())
}

pub fn parse_optional_number(s: Option<&str>) -> Option<i32> {
    s.and_then(|val| val.parse().ok())
}

pub fn first_some<T: Clone>(options: &[Option<T>]) -> Option<T> {
    options.iter().find_map(|opt| opt.clone())
}

pub fn collect_somes<T: Clone>(options: &[Option<T>]) -> Vec<T> {
    options.iter().filter_map(|opt| opt.clone()).collect()
}

fn main() {
    let mut store = MessageStore::new();
    
    store.add(Message {
        id: 1,
        to: String::from("+1555123456"),
        body: String::from("Hello!"),
        from: Some(String::from("TEXTIO")),
        media_url: None,
        status_code: Some(200),
    });
    
    store.add(Message {
        id: 2,
        to: String::from("+1555987654"),
        body: String::from("Check out this image"),
        from: None,
        media_url: Some(String::from("https://example.com/image.jpg")),
        status_code: Some(200),
    });
    
    store.add(Message {
        id: 3,
        to: String::from("+1555123456"),
        body: String::from("Second message"),
        from: Some(String::from("SUPPORT")),
        media_url: None,
        status_code: Some(404),
    });
    
    let mut users = UserManager::new();
    users.add(User {
        id: 1,
        username: String::from("alice"),
        phone: Some(String::from("+1555111111")),
        email: Some(String::from("alice@example.com")),
    });
    users.add(User {
        id: 2,
        username: String::from("bob"),
        phone: None,
        email: Some(String::from("bob@example.com")),
    });
    users.add(User {
        id: 3,
        username: String::from("charlie"),
        phone: None,
        email: None,
    });
    
    println!("=== MessageStore ===");
    
    println!("Find message 1: {:?}", store.find(1));
    println!("Find message 99: {:?}", store.find(99));
    
    println!("\nFind by phone +1555123456: {:?}", store.find_by_phone("+1555123456"));
    println!("Find by phone +1999999999: {:?}", store.find_by_phone("+1999999999"));
    
    println!("\nSender for message 1: {:?}", store.get_sender(1));
    println!("Sender for message 2: {:?}", store.get_sender(2));
    
    println!("\nMedia URL for message 2: {:?}", store.get_media_url(2));
    println!("Media URL for message 1: {:?}", store.get_media_url(1));
    
    println!("\nLatest for +1555123456: {:?}", store.find_latest("+1555123456"));
    
    println!("\nCount with status 200: {:?}", store.count_with_status(200));
    println!("Count with status 500: {:?}", store.count_with_status(500));
    
    println!("\n=== UserManager ===");
    
    println!("User 1 phone: {:?}", users.get_phone(1));
    println!("User 2 phone: {:?}", users.get_phone(2));
    println!("User 99 phone: {:?}", users.get_phone(99));
    
    println!("\nUser 1 email: {:?}", users.get_email(1));
    println!("User 3 email: {:?}", users.get_email(3));
    
    println!("\n=== Helper Functions ===");
    
    println!("Length of Some(\"hello\"): {}", optional_length(Some("hello")));
    println!("Length of None: {}", optional_length(None));
    
    println!("\nFormat Some(\"5551234\"): {}", format_phone(Some("5551234")));
    println!("Format None: {}", format_phone(None));
    
    println!("\nContact for user 1: {:?}", get_contact_info(&users, 1));
    println!("Contact for user 2: {:?}", get_contact_info(&users, 2));
    println!("Contact for user 3: {:?}", get_contact_info(&users, 3));
    
    println!("\nParse Some(\"42\"): {:?}", parse_optional_number(Some("42")));
    println!("Parse Some(\"abc\"): {:?}", parse_optional_number(Some("abc")));
    println!("Parse None: {:?}", parse_optional_number(None));
    
    let options = vec![None, Some(1), None, Some(3), Some(5)];
    println!("\nFirst Some from {:?}: {:?}", options, first_some(&options));
    println!("All Somes: {:?}", collect_somes(&options));
}

// String Type Exercise - Textio SMS API - Complete Solution

pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}

impl SmsMessage {
    pub fn new(to: &str, from: &str, body: &str) -> Self {
        SmsMessage {
            to: to.to_string(),
            from: from.to_string(),
            body: body.to_string(),
        }
    }

    pub fn preview(&self, len: usize) -> &str {
        self.body.get(0..len).unwrap_or(&self.body)
    }

    pub fn add_signature(&mut self, signature: &str) {
        self.body.push_str(signature);
    }

    pub fn append_char(&mut self, c: char) {
        self.body.push(c);
    }

    pub fn truncate(&mut self, max_chars: usize) {
        let chars: Vec<char> = self.body.chars().take(max_chars).collect();
        self.body = chars.into_iter().collect();
    }

    pub fn body_byte_length(&self) -> usize {
        self.body.len()
    }

    pub fn body_char_count(&self) -> usize {
        self.body.chars().count()
    }

    pub fn censor(&mut self, word: &str, replacement: &str) {
        self.body = self.body.replace(word, replacement);
    }
}

pub fn format_phone_list(phones: &[&str], separator: &str) -> String {
    phones.join(separator)
}

pub fn build_message(greeting: &str, name: &str, content: &str) -> String {
    format!("{}, {}! {}", greeting, name, content)
}

pub fn concatenate(s1: String, s2: &str) -> String {
    s1 + s2
}

pub fn create_with_capacity(capacity: usize) -> String {
    String::with_capacity(capacity)
}

pub fn split_message(message: &str, delimiter: char) -> Vec<&str> {
    message.split(delimiter).collect()
}

pub fn clean_message(message: &str) -> &str {
    message.trim()
}

pub fn has_country_code(phone: &str, code: &str) -> bool {
    phone.starts_with(code)
}

fn main() {
    let mut msg = SmsMessage::new("+1234567890", "+0987654321", "Hello, World!");
    
    println!("To: {}", msg.to);
    println!("From: {}", msg.from);
    println!("Body: {}", msg.body);
    println!("Preview (5 chars): {}", msg.preview(5));
    println!("Byte length: {}", msg.body_byte_length());
    println!("Char count: {}", msg.body_char_count());
    
    msg.add_signature(" - Textio");
    println!("With signature: {}", msg.body);
    
    msg.append_char('!');
    println!("With char appended: {}", msg.body);
    
    msg.truncate(15);
    println!("Truncated to 15 chars: {}", msg.body);
    
    let mut censored = SmsMessage::new("+1", "+2", "The secret word is PASSWORD");
    censored.censor("PASSWORD", "*****");
    println!("Censored: {}", censored.body);
    
    let phones = ["+1234567890", "+0987654321", "+1122334455"];
    println!("Phone list: {}", format_phone_list(&phones, ", "));
    
    let built = build_message("Hello", "Alice", "Your package has been delivered!");
    println!("Built message: {}", built);
    
    let s1 = String::from("Hello, ");
    let s2 = "World!";
    let concat = concatenate(s1, s2);
    println!("Concatenated: {}", concat);
    
    let mut s = create_with_capacity(100);
    println!("Empty string capacity: {}", s.capacity());
    s.push_str("Hello");
    println!("After push capacity: {}", s.capacity());
    
    let parts = split_message("one,two,three,four", ',');
    println!("Split parts: {:?}", parts);
    
    let trimmed = clean_message("  hello world  ");
    println!("Trimmed: '{}'", trimmed);
    
    let phone = "+1-555-1234";
    println!("Has +1 code: {}", has_country_code(phone, "+1"));
    println!("Has +44 code: {}", has_country_code(phone, "+44"));
    
    let unicode_msg = SmsMessage::new("+1", "+2", "Hello 🎉 日");
    println!("Unicode message: {}", unicode_msg.body);
    println!("Unicode byte length: {}", unicode_msg.body_byte_length());
    println!("Unicode char count: {}", unicode_msg.body_char_count());
}

// Exercise 2: Mutable References (&mut T) - Complete Solution

fn main() {
    println!("=== Textio Message Modification ===\n");
    
    // Part 1: Basic Mutable References
    // =================================
    
    let mut message = String::from("Hello");
    
    let msg_ref = &mut message;
    
    msg_ref.push_str(", Textio!");
    
    println!("Modified: {}", msg_ref);
    
    println!();
    
    // Part 2: One Mutable Reference at a Time
    // =======================================
    
    let mut data = String::from("Initial");
    
    let ref1 = &mut data;
    ref1.push_str(" value");
    
    let ref2 = &mut data;
    ref2.push('!');
    
    println!("Final value: {}", data);
    
    println!();
    
    // Part 3: Mutable References in Functions
    // =======================================
    
    let mut sms_body = String::from("Your code is ");
    
    add_verification_code(&mut sms_body, "123456");
    add_signature(&mut sms_body);
    
    println!("SMS Body: {}", sms_body);
    
    println!();
    
    // Part 4: Modifying Structs Through References
    // ============================================
    
    let mut message = Message {
        id: 1001,
        recipient: String::from("+15551234567"),
        body: String::from("Hello"),
        status: MessageStatus::Pending,
    };
    
    mark_message_sent(&mut message);
    append_to_body(&mut message, "!");
    
    println!("Message ID: {}", message.id);
    println!("Status: {:?}", message.status);
    println!("Body: {}", message.body);
    
    println!();
    
    // Part 5: Iterating Mutably Over Collections
    // ==========================================
    
    let mut messages = vec![
        String::from("hello"),
        String::from("world"),
        String::from("textio"),
    ];
    
    for msg in messages.iter_mut() {
        if let Some(first_char) = msg.chars().next() {
            let capitalized = first_char.to_uppercase().collect::<String>();
            msg.replace_range(..1, &capitalized);
        }
    }
    
    println!("Capitalized messages: {:?}", messages);
    
    println!();
    
    // Part 6: The Dereference Operator (*)
    // ====================================
    
    let mut count = 0;
    
    let count_ref = &mut count;
    
    *count_ref += 1;
    
    println!("Count: {}", count);
    
    // Part 7: Real-World Textio Pattern - Message Builder
    // ===================================================
    
    let mut builder = MessageBuilder::new();
    
    builder.set_to("+15551112222");
    builder.set_from("+15553334444");
    builder.set_body("Your verification code is 654321");
    
    let final_message = builder.build();
    
    println!("Built message:");
    println!("  To: {}", final_message.to);
    println!("  From: {}", final_message.from);
    println!("  Body: {}", final_message.body);
}

#[derive(Debug)]
enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

struct Message {
    id: u32,
    recipient: String,
    body: String,
    status: MessageStatus,
}

fn add_verification_code(body: &mut String, code: &str) {
    body.push_str(code);
}

fn add_signature(body: &mut String) {
    body.push_str("\n\n- Textio SMS Service");
}

fn mark_message_sent(msg: &mut Message) {
    msg.status = MessageStatus::Sent;
}

fn append_to_body(msg: &mut Message, text: &str) {
    msg.body.push_str(text);
}

struct BuiltMessage {
    to: String,
    from: String,
    body: String,
}

struct MessageBuilder {
    to: String,
    from: String,
    body: String,
}

impl MessageBuilder {
    fn new() -> Self {
        MessageBuilder {
            to: String::new(),
            from: String::new(),
            body: String::new(),
        }
    }
    
    fn set_to(&mut self, to: &str) -> &mut Self {
        self.to = to.to_string();
        self
    }
    
    fn set_from(&mut self, from: &str) -> &mut Self {
        self.from = from.to_string();
        self
    }
    
    fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
        self
    }
    
    fn build(self) -> BuiltMessage {
        BuiltMessage {
            to: self.to,
            from: self.from,
            body: self.body,
        }
    }
}

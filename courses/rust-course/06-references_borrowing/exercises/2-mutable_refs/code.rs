// Exercise 2: Mutable References (&mut T)
// 
// In this exercise, you'll learn to create and use mutable references.
// Follow the TODO comments to complete the code.
//
// Key concepts:
// - Creating mutable references with &mut
// - The one mutable reference rule
// - Modifying data through references
// - Mutable references in functions

fn main() {
    println!("=== Textio Message Modification ===\n");
    
    // Part 1: Basic Mutable References
    // =================================
    
    // TODO: Create a mutable String called message with value "Hello"
    // let mut message = ???;
    
    // TODO: Create a mutable reference to message
    // let msg_ref = ???;
    
    // TODO: Use the reference to append ", Textio!" to the string
    // Hint: Use push_str method
    // ???.push_str(???);
    
    // TODO: Print the modified message through the reference
    // println!("Modified: {}", ???);
    
    println!();
    
    // Part 2: One Mutable Reference at a Time
    // =======================================
    
    let mut data = String::from("Initial");
    
    // TODO: Create one mutable reference and modify the data
    // let ref1 = ???;
    // ???.push_str(" value");
    
    // Note: You cannot create another mutable reference here while ref1 is active!
    // But after ref1 is no longer used, you can create another.
    
    // TODO: Create a new mutable reference after ref1 is done being used
    // let ref2 = ???;
    // ???.push('!');
    
    // println!("Final value: {}", data);
    
    println!();
    
    // Part 3: Mutable References in Functions
    // =======================================
    
    let mut sms_body = String::from("Your code is ");
    
    // TODO: Call add_verification_code to append "123456" to sms_body
    // Hint: Pass a mutable reference
    // ???;
    
    // TODO: Call add_signature to append a signature to sms_body
    // ???;
    
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
    
    // TODO: Call mark_message_sent with a mutable reference to message
    // ???;
    
    // TODO: Call append_to_body with a mutable reference and the text "!"
    // ???;
    
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
    
    // TODO: Use iter_mut() to capitalize each message
    // Hint: You'll need to modify each string in place
    // for msg in messages.??? {
    //     if let Some(first_char) = msg.chars().next() {
    //         let capitalized = first_char.???.collect::<String>();
    //         msg.???(..1, &capitalized);
    //     }
    // }
    
    println!("Capitalized messages: {:?}", messages);
    
    println!();
    
    // Part 6: The Dereference Operator (*)
    // ====================================
    
    let mut count = 0;
    
    // TODO: Create a mutable reference to count
    // let count_ref = ???;
    
    // TODO: Use the * operator to increment count through the reference
    // ??? += 1;
    
    println!("Count: {}", count);
    
    // Part 7: Real-World Textio Pattern - Message Builder
    // ===================================================
    
    let mut builder = MessageBuilder::new();
    
    // TODO: Use the builder to set fields through mutable references
    // Call set_to, set_from, set_body, and then build
    
    // ???;
    // ???;
    // ???;
    
    // let final_message = builder.???;
    
    // println!("Built message:");
    // println!("  To: {}", final_message.to);
    // println!("  From: {}", final_message.from);
    // println!("  Body: {}", final_message.body);
}

// Struct for Part 4
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

// Functions for Part 3

fn add_verification_code(body: &mut String, code: &str) {
    body.push_str(code);
}

fn add_signature(body: &mut String) {
    body.push_str("\n\n- Textio SMS Service");
}

// Functions for Part 4

fn mark_message_sent(msg: &mut Message) {
    msg.status = MessageStatus::Sent;
}

fn append_to_body(msg: &mut Message, text: &str) {
    msg.body.push_str(text);
}

// Struct for Part 7
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

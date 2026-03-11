// Associated Functions - Textio Constructors

#[derive(Debug)]
struct Message {
    to: String,
    from: String,
    body: String,
    priority: u8,
}

impl Message {
    const MAX_BODY_LENGTH: usize = 160;
    const MAX_PRIORITY: u8 = 5;
    const DEFAULT_PRIORITY: u8 = 1;

    fn new(to: String, from: String, body: String) -> Self {
        Self {
            to,
            from,
            body,
            priority: Self::DEFAULT_PRIORITY,
        }
    }

    fn empty() -> Self {
        Self {
            to: String::new(),
            from: String::new(),
            body: String::new(),
            priority: Self::DEFAULT_PRIORITY,
        }
    }

    fn notification(to: String, body: String) -> Self {
        Self {
            to,
            from: String::from("Textio"),
            body,
            priority: Self::DEFAULT_PRIORITY,
        }
    }

    fn urgent(to: String, from: String, body: String) -> Self {
        Self {
            to,
            from,
            body,
            priority: Self::MAX_PRIORITY,
        }
    }

    fn with_priority(to: String, from: String, body: String, priority: u8) -> Self {
        Self {
            to,
            from,
            body,
            priority: if priority > Self::MAX_PRIORITY {
                Self::MAX_PRIORITY
            } else {
                priority
            },
        }
    }

    fn is_valid(&self) -> bool {
        self.body.len() <= Self::MAX_BODY_LENGTH
    }

    fn remaining_chars(&self) -> i32 {
        Self::MAX_BODY_LENGTH as i32 - self.body.len() as i32
    }
}

fn main() {
    println!("Max body length: {}", Message::MAX_BODY_LENGTH);

    let msg1 = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Hello from Textio!"),
    );
    println!("Basic message: {:?}", msg1);

    let msg2 = Message::empty();
    println!("Empty message: {:?}", msg2);

    let msg3 = Message::notification(
        String::from("+15550001"),
        String::from("Your code is 12345"),
    );
    println!("Notification: {:?}", msg3);

    let msg4 = Message::urgent(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("URGENT: Server down!"),
    );
    println!("Urgent message: {:?}", msg4);

    let msg5 = Message::with_priority(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Medium priority"),
        3,
    );
    println!("Priority 3 message: {:?}", msg5);

    let msg6 = Message::with_priority(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Over max priority"),
        10,
    );
    println!("Priority 10 (capped): {:?}", msg6);

    let short_msg = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Short message"),
    );
    println!("Short message valid: {}", short_msg.is_valid());

    let long_body = "x".repeat(200);
    let long_msg = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        long_body,
    );
    println!("Long message valid: {}", long_msg.is_valid());

    let msg7 = Message::new(
        String::from("+15550001"),
        String::from("+15550002"),
        String::from("Short"),
    );
    println!("Remaining chars: {}", msg7.remaining_chars());
}

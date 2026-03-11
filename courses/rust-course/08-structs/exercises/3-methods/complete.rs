// Methods - Textio Message Implementation

#[derive(Debug, Clone)]
struct Message {
    to: String,
    from: String,
    body: String,
    delivered: bool,
    attempts: u8,
}

impl Message {
    fn is_long(&self) -> bool {
        self.body.len() > 160
    }

    fn recipient(&self) -> &str {
        &self.to
    }

    fn sender(&self) -> &str {
        &self.from
    }

    fn char_count(&self) -> usize {
        self.body.len()
    }
}

impl Message {
    fn mark_delivered(&mut self) {
        self.delivered = true;
    }

    fn increment_attempts(&mut self) {
        self.attempts += 1;
    }

    fn append(&mut self, text: &str) {
        self.body.push_str(text);
    }

    fn reset_attempts(&mut self) {
        self.attempts = 0;
    }
}

fn main() {
    let mut msg = Message {
        to: String::from("+15550001"),
        from: String::from("+15550002"),
        body: String::from("Hello, this is a test message from Textio!"),
        delivered: false,
        attempts: 0,
    };

    println!("Is long: {}", msg.is_long());

    println!("Recipient: {}", msg.recipient());

    println!("Character count: {}", msg.char_count());

    msg.increment_attempts();
    msg.increment_attempts();

    println!("Attempts: {}", msg.attempts);

    msg.append(" More text.");

    println!("Is long after append: {}", msg.is_long());

    msg.mark_delivered();

    println!("Delivered: {}", msg.delivered);

    println!("Full message: {:?}", msg);
}

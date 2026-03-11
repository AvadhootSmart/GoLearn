// Exercise 1: Defining Traits - Complete Solution

// Define a Message trait with methods for accessing message properties
pub trait Message {
    fn content(&self) -> &str;
    fn recipient(&self) -> &str;
    fn sender(&self) -> &str;
    fn is_empty(&self) -> bool;
}

// SMS message struct
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}

impl Message for SmsMessage {
    fn content(&self) -> &str {
        &self.body
    }

    fn recipient(&self) -> &str {
        &self.to
    }

    fn sender(&self) -> &str {
        &self.from
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

// MMS message struct
pub struct MmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
    pub media_url: String,
}

impl Message for MmsMessage {
    fn content(&self) -> &str {
        &self.body
    }

    fn recipient(&self) -> &str {
        &self.to
    }

    fn sender(&self) -> &str {
        &self.from
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

// Define a StatusReport trait
pub trait StatusReport {
    fn message_id(&self) -> &str;
    fn status(&self) -> &str;
    fn timestamp(&self) -> u64;
}

pub struct MessageStatus {
    pub id: String,
    pub current_status: String,
    pub time: u64,
    pub attempts: u32,
}

impl StatusReport for MessageStatus {
    fn message_id(&self) -> &str {
        &self.id
    }

    fn status(&self) -> &str {
        &self.current_status
    }

    fn timestamp(&self) -> u64 {
        self.time
    }
}

// Define an Updatable trait
pub trait Updatable {
    fn increment_attempts(&mut self);
    fn set_status(&mut self, new_status: &str);
}

impl Updatable for MessageStatus {
    fn increment_attempts(&mut self) {
        self.attempts += 1;
    }

    fn set_status(&mut self, new_status: &str) {
        self.current_status = new_status.to_string();
    }
}

fn display_message_info<M: Message>(msg: &M) {
    println!("From: {}", msg.sender());
    println!("To: {}", msg.recipient());
    println!("Content: {}", msg.content());
    println!("Is empty: {}", msg.is_empty());
}

fn display_status<S: StatusReport>(status: &S) {
    println!("Message ID: {}", status.message_id());
    println!("Status: {}", status.status());
    println!("Timestamp: {}", status.timestamp());
}

fn main() {
    let sms = SmsMessage {
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: String::from("Hello from Textio!"),
    };

    println!("=== SMS Message ===");
    display_message_info(&sms);

    let mms = MmsMessage {
        to: String::from("+15551112222"),
        from: String::from("+15553334444"),
        body: String::from("Check out this image!"),
        media_url: String::from("https://example.com/image.jpg"),
    };

    println!("\n=== MMS Message ===");
    display_message_info(&mms);
    println!("Media URL: {}", mms.media_url);

    let mut status = MessageStatus {
        id: String::from("msg_12345"),
        current_status: String::from("pending"),
        time: 1699999999,
        attempts: 0,
    };

    println!("\n=== Message Status ===");
    display_status(&status);

    println!("\n=== After Update ===");
    status.increment_attempts();
    status.set_status("sent");
    display_status(&status);
    println!("Attempts: {}", status.attempts);

    let empty_sms = SmsMessage {
        to: String::from("+15550000000"),
        from: String::from("+15551111111"),
        body: String::from(""),
    };

    println!("\n=== Empty Message Test ===");
    println!("Is empty: {}", empty_sms.is_empty());
}

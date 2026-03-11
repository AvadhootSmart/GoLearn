// Exercise 1: Defining Traits
// 
// In this exercise, you'll define and implement traits for the Textio SMS API.
// Follow the TODO comments to complete the implementation.

// TODO: Define a Message trait with the following methods:
// - content(&self) -> &str: Returns the message body
// - recipient(&self) -> &str: Returns the recipient's phone number
// - sender(&self) -> &str: Returns the sender's phone number
// - is_empty(&self) -> bool: Returns true if the message content is empty



// SMS message struct - represents a standard text message
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
}

// TODO: Implement the Message trait for SmsMessage



// MMS message struct - represents a multimedia message with an attachment
pub struct MmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
    pub media_url: String,
}

// TODO: Implement the Message trait for MmsMessage
// Note: MMS messages have a media_url in addition to the body



// TODO: Define a StatusReport trait with:
// - message_id(&self) -> &str: Returns the unique message identifier
// - status(&self) -> &str: Returns the current status (pending, sent, delivered, failed)
// - timestamp(&self) -> u64: Returns the Unix timestamp of the status update



// Message status tracker
pub struct MessageStatus {
    pub id: String,
    pub current_status: String,
    pub time: u64,
    pub attempts: u32,
}

// TODO: Implement the StatusReport trait for MessageStatus



// TODO: Define a trait named Updatable with:
// - increment_attempts(&mut self): Increases the attempt count by 1
// - set_status(&mut self, new_status: &str): Updates the status



// TODO: Implement the Updatable trait for MessageStatus



// Helper function to display message information
fn display_message_info<M: Message>(msg: &M) {
    println!("From: {}", msg.sender());
    println!("To: {}", msg.recipient());
    println!("Content: {}", msg.content());
    println!("Is empty: {}", msg.is_empty());
}

// Helper function to display status
fn display_status<S: StatusReport>(status: &S) {
    println!("Message ID: {}", status.message_id());
    println!("Status: {}", status.status());
    println!("Timestamp: {}", status.timestamp());
}

fn main() {
    // Test SMS message
    let sms = SmsMessage {
        to: String::from("+15551234567"),
        from: String::from("+15559876543"),
        body: String::from("Hello from Textio!"),
    };

    println!("=== SMS Message ===");
    display_message_info(&sms);

    // Test MMS message
    let mms = MmsMessage {
        to: String::from("+15551112222"),
        from: String::from("+15553334444"),
        body: String::from("Check out this image!"),
        media_url: String::from("https://example.com/image.jpg"),
    };

    println!("\n=== MMS Message ===");
    display_message_info(&mms);
    println!("Media URL: {}", mms.media_url);

    // Test message status
    let mut status = MessageStatus {
        id: String::from("msg_12345"),
        current_status: String::from("pending"),
        time: 1699999999,
        attempts: 0,
    };

    println!("\n=== Message Status ===");
    display_status(&status);

    // Test updating status
    println!("\n=== After Update ===");
    status.increment_attempts();
    status.set_status("sent");
    display_status(&status);
    println!("Attempts: {}", status.attempts);

    // Test empty message
    let empty_sms = SmsMessage {
        to: String::from("+15550000000"),
        from: String::from("+15551111111"),
        body: String::from(""),
    };

    println!("\n=== Empty Message Test ===");
    println!("Is empty: {}", empty_sms.is_empty());
}

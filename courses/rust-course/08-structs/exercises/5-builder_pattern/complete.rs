// Builder Pattern - Textio Message Builder

#[derive(Debug)]
struct Message {
    to: String,
    from: String,
    body: String,
    priority: u8,
    delay_seconds: u32,
    callback_url: Option<String>,
}

struct MessageBuilder {
    to: Option<String>,
    from: Option<String>,
    body: Option<String>,
    priority: Option<u8>,
    delay_seconds: Option<u32>,
    callback_url: Option<String>,
}

impl MessageBuilder {
    fn new() -> Self {
        MessageBuilder {
            to: None,
            from: None,
            body: None,
            priority: None,
            delay_seconds: None,
            callback_url: None,
        }
    }

    fn to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }

    fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }

    fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    fn delay(mut self, seconds: u32) -> Self {
        self.delay_seconds = Some(seconds);
        self
    }

    fn callback(mut self, url: String) -> Self {
        self.callback_url = Some(url);
        self
    }

    fn build(self) -> Result<Message, String> {
        let to = self.to.ok_or("'to' field is required".to_string())?;
        
        if !to.starts_with('+') {
            return Err("'to' must start with '+'".to_string());
        }
        
        let body = self.body.ok_or("'body' field is required".to_string())?;
        
        if body.len() > 160 {
            return Err("'body' exceeds 160 characters".to_string());
        }
        
        Ok(Message {
            to,
            from: self.from.unwrap_or_else(|| String::from("Textio")),
            body,
            priority: self.priority.unwrap_or(1),
            delay_seconds: self.delay_seconds.unwrap_or(0),
            callback_url: self.callback_url,
        })
    }
}

fn main() {
    let msg1 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .from(String::from("+15550002"))
        .body(String::from("Hello from Textio!"))
        .priority(3)
        .delay(60)
        .callback(String::from("https://example.com/callback"))
        .build();

    match msg1 {
        Ok(msg) => println!("Message built: {:?}", msg),
        Err(e) => println!("Error: {}", e),
    }

    let msg2 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .body(String::from("Minimal message"))
        .build();

    match msg2 {
        Ok(msg) => println!("Minimal message: {:?}", msg),
        Err(e) => println!("Error: {}", e),
    }

    let msg3 = MessageBuilder::new()
        .body(String::from("Missing recipient"))
        .build();

    match msg3 {
        Ok(msg) => println!("Unexpected success: {:?}", msg),
        Err(e) => println!("Expected error: {}", e),
    }

    let msg4 = MessageBuilder::new()
        .to(String::from("15550001"))
        .body(String::from("Invalid phone"))
        .build();

    match msg4 {
        Ok(msg) => println!("Unexpected success: {:?}", msg),
        Err(e) => println!("Expected error: {}", e),
    }

    let long_body = "x".repeat(200);
    let msg5 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .body(long_body)
        .build();

    match msg5 {
        Ok(msg) => println!("Unexpected success: {:?}", msg),
        Err(e) => println!("Expected error: {}", e),
    }

    let msg6 = MessageBuilder::new()
        .to(String::from("+15550001"))
        .body(String::from("With priority"))
        .priority(5)
        .build();

    match msg6 {
        Ok(msg) => {
            println!("Priority message: {:?}", msg);
            println!("From field defaulted to: {}", msg.from);
        },
        Err(e) => println!("Error: {}", e),
    }
}

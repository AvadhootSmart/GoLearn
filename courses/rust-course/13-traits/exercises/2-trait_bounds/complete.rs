// Exercise 2: Trait Bounds - Complete Solution

use std::fmt::Display;

pub trait Message {
    fn content(&self) -> &str;
    fn recipient(&self) -> &str;
}

pub trait Validatable {
    fn validate(&self) -> Result<(), String>;
}

pub trait CostCalculator {
    fn calculate_cost(&self) -> f64;
}

#[derive(Clone)]
pub struct SmsMessage {
    pub to: String,
    pub body: String,
    pub cost_per_char: f64,
}

impl Message for SmsMessage {
    fn content(&self) -> &str {
        &self.body
    }
    fn recipient(&self) -> &str {
        &self.to
    }
}

impl Validatable for SmsMessage {
    fn validate(&self) -> Result<(), String> {
        if self.to.is_empty() {
            return Err("Recipient is empty".to_string());
        }
        if self.body.is_empty() {
            return Err("Message is empty".to_string());
        }
        Ok(())
    }
}

impl CostCalculator for SmsMessage {
    fn calculate_cost(&self) -> f64 {
        self.body.len() as f64 * self.cost_per_char
    }
}

#[derive(Clone)]
pub struct PremiumMessage {
    pub to: String,
    pub body: String,
    pub base_cost: f64,
}

impl Message for PremiumMessage {
    fn content(&self) -> &str {
        &self.body
    }
    fn recipient(&self) -> &str {
        &self.to
    }
}

impl Validatable for PremiumMessage {
    fn validate(&self) -> Result<(), String> {
        if self.to.len() < 10 {
            return Err("Invalid phone number".to_string());
        }
        Ok(())
    }
}

impl CostCalculator for PremiumMessage {
    fn calculate_cost(&self) -> f64 {
        self.base_cost * 1.5
    }
}

// Generic function with basic trait bound
fn send_message<T: Message>(msg: &T) {
    println!("Sending to: {}", msg.recipient());
    println!("Content: {}", msg.content());
}

// Generic function with multiple trait bounds
fn validate_and_send<T: Message + Validatable>(msg: &T) -> Result<(), String> {
    msg.validate()?;
    println!("Validated!");
    send_message(msg);
    Ok(())
}

// Generic function with where clause
fn process_batch<T>(messages: Vec<T>)
where
    T: Message + CostCalculator,
{
    let mut total_cost = 0.0;
    let count = messages.len();
    
    for msg in &messages {
        total_cost += msg.calculate_cost();
    }
    
    println!("Total cost: ${:.2}", total_cost);
    println!("Messages processed: {}", count);
}

// Generic struct with trait bound
struct MessageProcessor<T: Display> {
    processor_id: T,
    processed_count: u32,
}

impl<T: Display> MessageProcessor<T> {
    fn new(processor_id: T) -> Self {
        MessageProcessor {
            processor_id,
            processed_count: 0,
        }
    }

    fn process<M: Message>(&mut self, msg: &M) {
        println!("[{}] Processing message for {}", self.processor_id, msg.recipient());
        self.processed_count += 1;
    }
}

// Function accepting slice of trait implementors
fn estimate_total_cost(items: &[&dyn CostCalculator]) -> f64 {
    items.iter().map(|item| item.calculate_cost()).sum()
}

// Generic function with Clone and Validatable bounds
fn clone_and_validate<T: Clone + Validatable>(item: &T) -> Result<T, String> {
    let cloned = item.clone();
    cloned.validate()?;
    Ok(cloned)
}

fn main() {
    let sms = SmsMessage {
        to: String::from("+15551234567"),
        body: String::from("Hello Textio!"),
        cost_per_char: 0.01,
    };

    let premium = PremiumMessage {
        to: String::from("+15559998888"),
        body: String::from("Premium message content"),
        base_cost: 0.50,
    };

    println!("=== Testing send_message ===");
    send_message(&sms);
    send_message(&premium);

    println!("\n=== Testing validate_and_send ===");
    match validate_and_send(&sms) {
        Ok(_) => println!("Message sent!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Testing process_batch ===");
    let messages: Vec<&SmsMessage> = vec![&sms, &sms];
    process_batch(messages);

    println!("\n=== Testing MessageProcessor ===");
    let mut processor = MessageProcessor::new(String::from("PROC-001"));
    processor.process(&sms);
    processor.process(&premium);
    println!("Total processed: {}", processor.processed_count);

    println!("\n=== Testing estimate_total_cost ===");
    let items: Vec<&dyn CostCalculator> = vec![&sms, &premium];
    let total = estimate_total_cost(&items);
    println!("Estimated total cost: ${:.2}", total);

    println!("\n=== Testing clone_and_validate ===");
    match clone_and_validate(&sms) {
        Ok(cloned) => println!("Cloned message content: {}", cloned.content()),
        Err(e) => println!("Validation failed: {}", e),
    }
}

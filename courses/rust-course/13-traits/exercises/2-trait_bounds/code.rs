// Exercise 2: Trait Bounds
//
// Learn to constrain generic types with trait bounds.
// Complete the TODO sections to make the code compile and run.

use std::fmt::Display;

// Basic traits for our Textio system
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

// SMS Message implementation
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

// Premium Message implementation
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

// TODO: Create a generic function `send_message` that:
// - Takes a reference to any type that implements Message
// - Prints "Sending to: {recipient}" and "Content: {content}"
// - Returns nothing



// TODO: Create a generic function `validate_and_send` that:
// - Takes a reference to any type that implements Message + Validatable
// - Validates the message, prints "Validated!" if successful
// - Calls send_message internally
// - Returns Result<(), String>



// TODO: Create a generic function `process_batch` with a where clause that:
// - Takes a vector of items implementing Message + CostCalculator
// - Prints the total cost (sum of all calculate_cost() values)
// - Prints the number of messages processed



// TODO: Create a generic struct `MessageProcessor` that:
// - Has a generic field `processor_id` that implements Display
// - Has a field `processed_count` of type u32
// - Implement a `new` method that creates a new processor



// TODO: Implement a method `process` for MessageProcessor that:
// - Takes a reference to any type implementing Message
// - Prints "[{processor_id}] Processing message for {recipient}"
// - Increments processed_count



// TODO: Create a function `estimate_total_cost` that:
// - Takes a slice of items implementing CostCalculator
// - Returns the total cost as f64



// TODO: Create a generic function `clone_and_validate` that:
// - Takes a reference to any type implementing Clone + Validatable
// - Clones the item, validates the clone
// - Returns the cloned item if valid, or an error string



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

    // Test send_message
    println!("=== Testing send_message ===");
    send_message(&sms);
    send_message(&premium);

    // Test validate_and_send
    println!("\n=== Testing validate_and_send ===");
    match validate_and_send(&sms) {
        Ok(_) => println!("Message sent!"),
        Err(e) => println!("Error: {}", e),
    }

    // Test process_batch
    println!("\n=== Testing process_batch ===");
    let messages: Vec<&SmsMessage> = vec![&sms, &sms];
    process_batch(messages);

    // Test MessageProcessor
    println!("\n=== Testing MessageProcessor ===");
    let mut processor = MessageProcessor::new(String::from("PROC-001"));
    processor.process(&sms);
    processor.process(&premium);
    println!("Total processed: {}", processor.processed_count);

    // Test estimate_total_cost
    println!("\n=== Testing estimate_total_cost ===");
    let items: Vec<&dyn CostCalculator> = vec![&sms, &premium];
    let total = estimate_total_cost(&items);
    println!("Estimated total cost: ${:.2}", total);

    // Test clone_and_validate
    println!("\n=== Testing clone_and_validate ===");
    match clone_and_validate(&sms) {
        Ok(cloned) => println!("Cloned message content: {}", cloned.content()),
        Err(e) => println!("Validation failed: {}", e),
    }
}

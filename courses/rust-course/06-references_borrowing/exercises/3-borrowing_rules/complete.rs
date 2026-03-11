// Exercise 3: Borrowing Rules - Complete Solution

fn main() {
    println!("=== Textio Borrowing Rules Practice ===\n");
    
    // Part 1: Fix the Multiple Mutable Borrows Error
    // ==============================================
    
    println!("Part 1: Multiple Mutable Borrows");
    
    let mut message = String::from("Hello");
    
    let r1 = &mut message;
    r1.push_str(", there");
    // r1 is no longer used after this point (NLL)
    
    let r2 = &mut message;  // Now OK with NLL
    r2.push('!');
    
    println!("{}", message);
    
    println!();
    
    // Part 2: Fix the Mixed Borrow Error
    // ==================================
    
    println!("Part 2: Mixed Immutable and Mutable Borrows");
    
    let mut messages = vec![
        String::from("First"),
        String::from("Second"),
        String::from("Third"),
    ];
    
    // Fix: Use the reference before modifying
    let first_message = &messages[0];
    println!("First: {}", first_message);
    // first_message is no longer used
    
    messages.push(String::from("Fourth"));  // Now OK
    
    println!("Messages: {:?}", messages);
    
    println!();
    
    // Part 3: Use Scopes to Manage Borrows
    // ====================================
    
    println!("Part 3: Using Scopes");
    
    let mut data = String::from("Initial value");
    
    {
        let r1 = &mut data;
        r1.push_str(" (modified once)");
    }
    
    {
        let r2 = &mut data;
        r2.push_str(" (modified twice)");
    }
    
    println!("Final data: {}", data);
    
    println!();
    
    // Part 4: Extract Values to Avoid Borrow Conflicts
    // ================================================
    
    println!("Part 4: Extracting Values");
    
    let mut queue = MessageQueue::new();
    queue.add("Hello", 1);
    queue.add("World", 2);
    queue.add("Important", 10);
    
    // Fix: Clone the body before modifying
    let highest_body = queue.get_highest_priority_body();
    queue.add("New message", 5);
    
    if let Some(body) = highest_body {
        println!("Highest priority: {}", body);
    }
    
    println!();
    
    // Part 5: Working with Indices Instead of References
    // ==================================================
    
    println!("Part 5: Using Indices");
    
    let mut numbers = vec![10, 20, 30, 40, 50];
    
    let max_index = numbers.iter()
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap();
    
    numbers[max_index] *= 2;
    
    println!("Modified: {:?}", numbers);
    
    println!();
    
    // Part 6: Split Borrowing
    // ======================
    
    println!("Part 6: Split Borrowing");
    
    let mut data = [1, 2, 3, 4, 5, 6];
    
    let (left, right) = data.split_at_mut(3);
    for val in left.iter_mut() {
        *val *= 2;
    }
    for val in right.iter_mut() {
        *val *= 3;
    }
    
    println!("Split and modified: {:?}", data);
    
    println!();
    
    // Part 7: Real-World Textio Scenario
    // ==================================
    
    println!("Part 7: Textio Message Processing");
    
    let mut processor = MessageProcessor::new();
    
    processor.add_message("+15551112222", "Hello there!");
    processor.add_message("+15553334444", "Your code is 123456");
    processor.add_message("+15551112222", "Second message");
    
    let count = processor.count_to_recipient("+15551112222");
    println!("Messages to +15551112222: {}", count);
    
    processor.add_message("+15551112222", "Third message");
    
    processor.process_all();
    
    println!("Total processed: {}", processor.processed_count);
}

struct Message {
    body: String,
    priority: u8,
}

struct MessageQueue {
    messages: Vec<Message>,
}

impl MessageQueue {
    fn new() -> Self {
        MessageQueue {
            messages: Vec::new(),
        }
    }
    
    fn add(&mut self, body: &str, priority: u8) {
        self.messages.push(Message {
            body: body.to_string(),
            priority,
        });
    }
    
    fn get_highest_priority(&self) -> Option<&Message> {
        self.messages.iter().max_by_key(|m| m.priority)
    }
    
    fn get_highest_priority_body(&self) -> Option<String> {
        self.messages.iter()
            .max_by_key(|m| m.priority)
            .map(|m| m.body.clone())
    }
}

struct SmsMessage {
    recipient: String,
    body: String,
    processed: bool,
}

struct MessageProcessor {
    messages: Vec<SmsMessage>,
    pub processed_count: u32,
}

impl MessageProcessor {
    fn new() -> Self {
        MessageProcessor {
            messages: Vec::new(),
            processed_count: 0,
        }
    }
    
    fn add_message(&mut self, recipient: &str, body: &str) {
        self.messages.push(SmsMessage {
            recipient: recipient.to_string(),
            body: body.to_string(),
            processed: false,
        });
    }
    
    fn count_to_recipient(&self, recipient: &str) -> usize {
        self.messages.iter()
            .filter(|m| m.recipient == recipient)
            .count()
    }
    
    fn process_all(&mut self) {
        for msg in self.messages.iter_mut() {
            if !msg.processed {
                msg.processed = true;
                self.processed_count += 1;
            }
        }
    }
}

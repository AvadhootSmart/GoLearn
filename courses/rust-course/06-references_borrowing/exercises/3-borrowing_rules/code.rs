// Exercise 3: Borrowing Rules
// 
// In this exercise, you'll work with Rust's borrowing rules and
// learn how to fix common borrow checker errors.
//
// Key concepts:
// - One mutable OR many immutable (never both)
// - Non-Lexical Lifetimes (NLL)
// - Restructuring code to satisfy the borrow checker
// - Working with the borrow checker, not against it

fn main() {
    println!("=== Textio Borrowing Rules Practice ===\n");
    
    // Part 1: Fix the Multiple Mutable Borrows Error
    // ==============================================
    
    println!("Part 1: Multiple Mutable Borrows");
    
    let mut message = String::from("Hello");
    
    // PROBLEM: This code has two mutable borrows at once
    // TODO: Fix this code so it compiles and runs
    // Hint: Use the fact that NLL ends borrows when they're last used
    
    // let r1 = &mut message;
    // r1.push_str(", there");
    // let r2 = &mut message;  // ERROR! Can't have two mutable borrows
    // r2.push('!');
    // println!("{}", message);
    
    // Your fix here:
    
    println!();
    
    // Part 2: Fix the Mixed Borrow Error
    // ==================================
    
    println!("Part 2: Mixed Immutable and Mutable Borrows");
    
    let mut messages = vec![
        String::from("First"),
        String::from("Second"),
        String::from("Third"),
    ];
    
    // PROBLEM: Can't modify while we have an immutable reference
    // TODO: Fix this code by reordering operations
    
    // let first_message = &messages[0];
    // messages.push(String::from("Fourth"));  // ERROR!
    // println!("First: {}", first_message);
    
    // Your fix here:
    
    println!();
    
    // Part 3: Use Scopes to Manage Borrows
    // ====================================
    
    println!("Part 3: Using Scopes");
    
    let mut data = String::from("Initial value");
    
    // TODO: Create a scope where you borrow and modify data
    // Then create another scope for a different modification
    
    // Your code here:
    // {
    //     // First modification
    // }
    // {
    //     // Second modification
    // }
    
    println!("Final data: {}", data);
    
    println!();
    
    // Part 4: Extract Values to Avoid Borrow Conflicts
    // ================================================
    
    println!("Part 4: Extracting Values");
    
    let mut queue = MessageQueue::new();
    queue.add("Hello", 1);
    queue.add("World", 2);
    queue.add("Important", 10);
    
    // PROBLEM: We want to get the highest priority and then add a new message
    // TODO: Fix by extracting the value we need before modifying
    
    // let highest = queue.get_highest_priority();  // Returns Option<&Message>
    // queue.add("New message", 5);  // ERROR! Can't add while borrowed
    // if let Some(msg) = highest {
    //     println!("Highest priority: {}", msg.body);
    // }
    
    // Your fix here:
    
    println!();
    
    // Part 5: Working with Indices Instead of References
    // ==================================================
    
    println!("Part 5: Using Indices");
    
    let mut numbers = vec![10, 20, 30, 40, 50];
    
    // TODO: Find the index of the maximum value, then double it
    // Hint: Store the index, not a reference
    
    // let max_index = ???;
    // // Now modify the vector
    // numbers[max_index] *= 2;
    
    // println!("Modified: {:?}", numbers);
    
    println!();
    
    // Part 6: Split Borrowing
    // ======================
    
    println!("Part 6: Split Borrowing");
    
    let mut data = [1, 2, 3, 4, 5, 6];
    
    // TODO: Use split_at_mut to get two mutable slices
    // Double all values in the left half, triple all in the right half
    
    // let (left, right) = data.???;
    // for val in left.iter_mut() {
    //     *val *= 2;
    // }
    // for val in right.iter_mut() {
    //     *val *= 3;
    // }
    
    // println!("Split and modified: {:?}", data);
    
    println!();
    
    // Part 7: Real-World Textio Scenario
    // ==================================
    
    println!("Part 7: Textio Message Processing");
    
    let mut processor = MessageProcessor::new();
    
    processor.add_message("+15551112222", "Hello there!");
    processor.add_message("+15553334444", "Your code is 123456");
    processor.add_message("+15551112222", "Second message");
    
    // TODO: Get the count of messages to a specific recipient
    // Then add another message
    // Hint: The count method returns a value (not a reference)
    
    // let count = processor.count_to_recipient("???");
    // println!("Messages to +15551112222: {}", count);
    
    // Now add another message
    // processor.add_message("???", "Third message");
    
    // TODO: Process all pending messages
    // processor.???;
    
    // println!("Total processed: {}", processor.processed_count);
}

// MessageQueue for Part 4
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

// MessageProcessor for Part 7
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

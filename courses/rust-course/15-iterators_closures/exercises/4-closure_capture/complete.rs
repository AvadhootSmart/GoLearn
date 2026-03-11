// Exercise 4: Closure Capture Modes - Complete Solution

use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Message {
    id: u32,
    content: String,
    recipient: String,
}

impl Message {
    fn new(id: u32, content: &str, recipient: &str) -> Self {
        Message {
            id,
            content: content.to_string(),
            recipient: recipient.to_string(),
        }
    }
}

fn consume_with<F>(value: Message, consumer: F) 
where
    F: FnOnce(Message),
{
    consumer(value);
}

fn process_messages_mut<F>(messages: Vec<Message>, mut processor: F) -> usize
where
    F: FnMut(&Message),
{
    for msg in &messages {
        processor(msg);
    }
    messages.len()
}

fn find_matching<F>(messages: &[Message], predicate: F) -> Option<&Message>
where
    F: Fn(&Message) -> bool,
{
    messages.iter().find(|m| predicate(m))
}

fn create_prefixer(prefix: String) -> impl Fn(&str) -> String {
    move |text| format!("{}{}", prefix, text)
}

fn create_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

fn main() {
    println!("=== Part 1: Understanding Capture Modes ===\n");
    
    let text = String::from("Hello");
    
    let print_len = || println!("Length: {}", text.len());
    print_len();
    println!("Text still accessible: {}", text);
    
    let mut mutable_text = String::from("Hello");
    let mut add_world = || {
        mutable_text.push_str(" World");
    };
    add_world();
    println!("After mutation: {}", mutable_text);
    
    println!("\n=== Part 2: The move Keyword ===\n");
    
    let owned_string = String::from("I belong to the closure");
    
    let owned_closure = move || println!("{}", owned_string);
    owned_closure();
    println!("Closure called successfully");
    
    let prefix = "[TEXTIO] ".to_string();
    let prefixer = create_prefixer(prefix);
    
    let result = prefixer("Hello World");
    println!("Prefixed: {}", result);
    
    println!("\n=== Part 3: FnOnce - Consuming Closures ===\n");
    
    let consumable = Message::new(999, "This will be consumed", "+1999");
    
    let consume_message = |m: Message| {
        println!("Consuming message ID: {}", m.id);
    };
    
    consume_with(consumable, consume_message);
    
    println!("\n=== Part 4: FnMut - Mutable Closures ===\n");
    
    let messages = vec![
        Message::new(1, "First", "+1"),
        Message::new(2, "Second", "+2"),
        Message::new(3, "Third", "+3"),
    ];
    
    let mut processed_count = 0;
    let counter_closure = |_: &Message| {
        processed_count += 1;
    };
    
    let count = process_messages_mut(messages.clone(), counter_closure);
    println!("Processed {} messages, counted {}", count, processed_count);
    
    let mut counter = create_counter(10);
    println!("Counter: {}", counter());
    println!("Counter: {}", counter());
    println!("Counter: {}", counter());
    
    println!("\n=== Part 5: Fn - Immutable Closures ===\n");
    
    let search_messages = vec![
        Message::new(1, "Hello Alice", "+15550001"),
        Message::new(2, "Hello Bob", "+15550002"),
        Message::new(3, "Hello Charlie", "+15550003"),
    ];
    
    let target = "+15550002";
    let found = find_matching(&search_messages, |m| m.recipient == target);
    println!("Found message: {:?}", found);
    
    let is_to_alice = |m: &Message| m.recipient == "+15550001";
    let alice_msg = find_matching(&search_messages, is_to_alice);
    println!("Alice's message: {:?}", alice_msg);
    
    println!("\n=== Part 6: move with Threads ===\n");
    
    let thread_data = vec![1, 2, 3, 4, 5];
    let multiplier = 2;
    
    let handle = thread::spawn(move || {
        let sum: i32 = thread_data.iter().map(|x| x * multiplier).sum();
        format!("Sum from thread: {}", sum)
    });
    
    let result = handle.join().unwrap();
    println!("{}", result);
    
    println!("\n=== Part 7: Textio Message Handler ===\n");
    
    let config_threshold = 10;
    let blocked_list = vec!["+1999", "+1888"];
    
    let message_filter = move |msg: &Message| {
        msg.content.len() >= config_threshold 
            && !blocked_list.contains(&msg.recipient.as_str())
    };
    
    let test_messages = vec![
        Message::new(1, "Short", "+15550001"),
        Message::new(2, "This is a longer message", "+15550002"),
        Message::new(3, "Blocked user", "+1999"),
        Message::new(4, "Valid long message here", "+15550003"),
    ];
    
    let valid_messages: Vec<&Message> = test_messages.iter()
        .filter(|m| message_filter(m))
        .collect();
    
    println!("Valid messages:");
    for msg in valid_messages {
        println!("  ID {}: {:?}", msg);
    }
    
    println!("\n=== Part 8: Complex Capture Scenario ===\n");
    
    let mut request_count = 0;
    let max_requests = 3;
    
    let mut rate_limited_processor = || {
        if request_count < max_requests {
            request_count += 1;
            Some(format!("Request {} processed", request_count))
        } else {
            None
        }
    };
    
    for i in 1..=5 {
        match rate_limited_processor() {
            Some(result) => println!("Attempt {}: {}", i, result),
            None => println!("Attempt {}: Rate limited!", i),
        }
    }
}

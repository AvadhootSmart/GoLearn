// Exercise 4: Closure Capture Modes
// Learn move keyword, Fn/FnMut/FnOnce traits, and capture modes

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

// TODO: Define a function that takes a FnOnce closure
// This closure can only be called once
fn consume_with<F>(value: Message, consumer: F) 
where
    F: FnOnce(Message),
{
    // TODO: Call the consumer with the value
}

// TODO: Define a function that takes a FnMut closure
// This closure can be called multiple times and may mutate state
fn process_messages_mut<F>(messages: Vec<Message>, mut processor: F) -> usize
where
    F: FnMut(&Message),
{
    // TODO: Process each message and return the count
    0
}

// TODO: Define a function that takes a Fn closure
// This closure can be called multiple times without mutation
fn find_matching<F>(messages: &[Message], predicate: F) -> Option<&Message>
where
    F: Fn(&Message) -> bool,
{
    // TODO: Find and return the first matching message
    None
}

// TODO: Create a function that returns a closure
// The closure must use 'move' to capture its environment
fn create_prefixer(prefix: String) -> impl Fn(&str) -> String {
    // TODO: Return a closure that prepends prefix to the input
    // Remember to use 'move'!
    move |_| String::new()
}

// TODO: Create a function that returns a mutable closure
fn create_counter(start: i32) -> impl FnMut() -> i32 {
    // TODO: Return a closure that increments and returns a counter
    // The counter should start at 'start' and increment each call
    move || 0
}

fn main() {
    println!("=== Part 1: Understanding Capture Modes ===\n");
    
    let text = String::from("Hello");
    
    // This closure captures by reference
    let print_len = || println!("Length: {}", text.len());
    print_len();
    println!("Text still accessible: {}", text);
    
    // This closure would capture by mutable reference
    let mut mutable_text = String::from("Hello");
    let mut add_world = || {
        mutable_text.push_str(" World");
    };
    // println!("{}", mutable_text);  // This would fail - borrowed!
    add_world();
    println!("After mutation: {}", mutable_text);
    
    println!("\n=== Part 2: The move Keyword ===\n");
    
    let owned_string = String::from("I belong to the closure");
    
    // TODO: Create a closure with 'move' that takes ownership
    let owned_closure = || (); // Fix: add move and use owned_string
    
    // TODO: Verify that owned_string is moved (uncomment to see error)
    // println!("{}", owned_string);  // This should fail if move works
    
    owned_closure();
    println!("Closure called successfully");
    
    // TODO: Create and use the prefixer function
    let prefix = "[TEXTIO] ".to_string();
    let prefixer = create_prefixer(prefix);
    
    let result = prefixer("Hello World");
    println!("Prefixed: {}", result);
    
    // Original prefix should be gone (moved into closure)
    
    println!("\n=== Part 3: FnOnce - Consuming Closures ===\n");
    
    let consumable = Message::new(999, "This will be consumed", "+1999");
    
    // TODO: Create a closure that consumes the message
    let consume_message = |m: Message| {
        println!("Consuming message ID: {}", m.id);
        // Message is dropped at end of closure
    };
    
    // TODO: Use consume_with to call the closure
    // consume_with(consumable, consume_message);
    
    // consumable is no longer valid here
    
    println!("\n=== Part 4: FnMut - Mutable Closures ===\n");
    
    let messages = vec![
        Message::new(1, "First", "+1"),
        Message::new(2, "Second", "+2"),
        Message::new(3, "Third", "+3"),
    ];
    
    // TODO: Create a counter closure that mutates state
    let mut processed_count = 0;
    let mut counter_closure = |_: &Message| {
        processed_count += 1;
    };
    
    // TODO: Use process_messages_mut
    // let count = process_messages_mut(messages.clone(), counter_closure);
    // println!("Processed {} messages, counted {}", count, processed_count);
    
    // TODO: Create and use the counter function
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
    
    // TODO: Use find_matching with a predicate closure
    let target = "+15550002";
    // let found = find_matching(&search_messages, |m| m.recipient == target);
    let found: Option<&Message> = None; // Replace with actual call
    println!("Found message: {:?}", found);
    
    // The same closure can be called multiple times
    let is_to_alice = |m: &Message| m.recipient == "+15550001";
    // let alice_msg = find_matching(&search_messages, is_to_alice);
    let alice_msg: Option<&Message> = None; // Replace with actual call
    println!("Alice's message: {:?}", alice_msg);
    
    println!("\n=== Part 6: move with Threads ===\n");
    
    let thread_data = vec![1, 2, 3, 4, 5];
    let multiplier = 2;
    
    // TODO: Spawn a thread that uses thread_data and multiplier
    // Both must be moved into the thread
    let handle = thread::spawn(move || {
        // Use thread_data and multiplier here
        let sum: i32 = thread_data.iter().map(|x| x * multiplier).sum();
        format!("Sum from thread: {}", sum)
    });
    
    // thread_data is no longer valid here - it was moved
    // println!("{:?}", thread_data);  // This would fail
    
    let result = handle.join().unwrap();
    println!("{}", result);
    
    println!("\n=== Part 7: Textio Message Handler ===\n");
    
    let config_threshold = 10;
    let blocked_list = vec!["+1999", "+1888"];
    
    // TODO: Create a closure that captures config by move
    // The closure should filter messages:
    // - Content length must be >= threshold
    // - Recipient must not be in blocked_list
    let message_filter = move |msg: &Message| {
        // Fix this implementation
        true
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
        println!("  ID {}: {:?}", msg.id, msg);
    }
    
    println!("\n=== Part 8: Complex Capture Scenario ===\n");
    
    // Scenario: Create a rate limiter using closure state
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
    
    // Process requests
    for i in 1..=5 {
        match rate_limited_processor() {
            Some(result) => println!("Attempt {}: {}", i, result),
            None => println!("Attempt {}: Rate limited!", i),
        }
    }
}

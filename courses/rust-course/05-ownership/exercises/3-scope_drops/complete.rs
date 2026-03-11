use std::mem;

struct SmsMessage {
    id: u32,
    to: String,
    body: String,
}

impl Drop for SmsMessage {
    fn drop(&mut self) {
        println!("  [DROP] Message {} to '{}': '{}'", self.id, self.to, self.body);
    }
}

struct Connection {
    id: u32,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("  [DROP] Connection {} closed", self.id);
    }
}

struct ConnectionPool {
    name: String,
    connections: Vec<Connection>,
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        println!("  [DROP] Connection pool '{}' shutting down", self.name);
    }
}

struct MessageQueue {
    name: String,
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        println!("  [DROP] Message queue '{}' cleared", self.name);
    }
}

fn process_batch(should_continue: bool) {
    println!("  Entering process_batch({})", should_continue);
    
    let msg1 = SmsMessage {
        id: 1,
        to: String::from("Alice"),
        body: String::from("Hello"),
    };
    
    let msg2 = SmsMessage {
        id: 2,
        to: String::from("Bob"),
        body: String::from("World"),
    };
    
    if !should_continue {
        println!("  Early return!");
        return;
    }
    
    println!("  Processing messages normally");
}

fn main() {
    println!("=== Exercise 1: Basic Drop ===\n");
    
    let msg = SmsMessage {
        id: 1,
        to: String::from("Alice"),
        body: String::from("Hello"),
    };
    println!("Message created");
    
    println!("\n=== Exercise 2: Drop Order ===\n");
    
    let msg_a = SmsMessage {
        id: 1,
        to: String::from("A"),
        body: String::from("Message A"),
    };
    let msg_b = SmsMessage {
        id: 2,
        to: String::from("B"),
        body: String::from("Message B"),
    };
    let msg_c = SmsMessage {
        id: 3,
        to: String::from("C"),
        body: String::from("Message C"),
    };
    println!("Created messages A, B, C");
    drop(msg_c);
    drop(msg_b);
    drop(msg_a);
    
    println!("\n=== Exercise 3: Nested Scopes ===\n");
    
    let outer = SmsMessage {
        id: 10,
        to: String::from("Outer"),
        body: String::from("Outer message"),
    };
    {
        let inner = SmsMessage {
            id: 20,
            to: String::from("Inner"),
            body: String::from("Inner message"),
        };
        println!("Inner scope ending");
    }
    println!("Outer scope continuing");
    drop(outer);
    
    println!("\n=== Exercise 4: Explicit Drop ===\n");
    
    let msg = SmsMessage {
        id: 100,
        to: String::from("Explicit"),
        body: String::from("Will be dropped early"),
    };
    println!("Before explicit drop");
    drop(msg);
    println!("After explicit drop");
    
    println!("\n=== Exercise 5: Vec Drop Order ===\n");
    
    let messages = vec![
        SmsMessage {
            id: 1,
            to: String::from("A"),
            body: String::from("First"),
        },
        SmsMessage {
            id: 2,
            to: String::from("B"),
            body: String::from("Second"),
        },
        SmsMessage {
            id: 3,
            to: String::from("C"),
            body: String::from("Third"),
        },
    ];
    println!("Messages in vec: {} messages", messages.len());
    drop(messages);
    
    println!("\n=== Exercise 6: Complex Nested Structure ===\n");
    
    let pool = ConnectionPool {
        name: String::from("TextioPool"),
        connections: vec![
            Connection { id: 1 },
            Connection { id: 2 },
        ],
    };
    let queue = MessageQueue {
        name: String::from("OutboundQueue"),
    };
    println!("Pool and queue created");
    drop(queue);
    drop(pool);
    
    println!("\n=== Exercise 7: Shadowing and Drop ===\n");
    
    let msg = SmsMessage {
        id: 1,
        to: String::from("A"),
        body: String::from("First"),
    };
    let msg = SmsMessage {
        id: 2,
        to: String::from("B"),
        body: String::from("Second"),
    };
    let msg = SmsMessage {
        id: 3,
        to: String::from("C"),
        body: String::from("Third"),
    };
    println!("Created three shadowed messages, current id: {}", msg.id);
    drop(msg);
    
    println!("\n=== Exercise 8: Early Return Drops ===\n");
    
    process_batch(true);
    process_batch(false);
    
    println!("\n=== Exercise 9: Reassignment Drops Old Value ===\n");
    
    let mut msg = SmsMessage {
        id: 1,
        to: String::from("Original"),
        body: String::from("Original message"),
    };
    println!("Created message 1");
    msg = SmsMessage {
        id: 2,
        to: String::from("New"),
        body: String::from("New message"),
    };
    println!("Reassigned to message 2");
    drop(msg);
    
    println!("\n=== Exercise 10: Real Textio Scenario ===\n");
    
    let pool = ConnectionPool {
        name: String::from("ProductionPool"),
        connections: vec![
            Connection { id: 1 },
            Connection { id: 2 },
            Connection { id: 3 },
        ],
    };
    let messages = vec![
        SmsMessage {
            id: 1,
            to: String::from("+1234567890"),
            body: String::from("Message 1"),
        },
        SmsMessage {
            id: 2,
            to: String::from("+0987654321"),
            body: String::from("Message 2"),
        },
        SmsMessage {
            id: 3,
            to: String::from("+5555555555"),
            body: String::from("Message 3"),
        },
    ];
    println!("Processing 3 messages with pool");
    drop(messages);
    drop(pool);
    
    println!("\n=== End of main ===\n");
}

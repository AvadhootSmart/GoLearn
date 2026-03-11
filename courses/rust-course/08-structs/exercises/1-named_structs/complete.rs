// Named Structs - Textio SMS Message System

#[derive(Debug)]
struct Message {
    to: String,
    from: String,
    body: String,
    priority: u8,
    delivered: bool,
}

#[derive(Debug)]
struct MessageStats {
    total_sent: u64,
    total_delivered: u64,
    total_failed: u64,
}

fn create_message(to: String, from: String, body: String) -> Message {
    Message {
        to,
        from,
        body,
        priority: 1,
        delivered: false,
    }
}

fn main() {
    let to = String::from("+15550001");
    let from = String::from("+15550002");
    let body = String::from("Hello from Textio!");
    
    let msg1 = create_message(to, from, body);

    println!("Message 1: {:?}", msg1);

    let msg2 = Message {
        body: String::from("Priority message!"),
        ..msg1
    };

    println!("Message 2: {:#?}", msg2);

    let msg3 = Message {
        priority: 5,
        delivered: true,
        ..msg1
    };

    println!("Message 3: {:?}", msg3);

    println!("msg1.to is still valid: {}", msg1.to);

    let stats = MessageStats {
        total_sent: 1000,
        total_delivered: 950,
        total_failed: 50,
    };

    println!("Stats: {:?}", stats);
}

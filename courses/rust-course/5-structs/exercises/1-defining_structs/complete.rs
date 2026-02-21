// 1. Define struct Message here
struct Message {
    body: String,
    recipient: String,
}

fn main() {
    // 2. Instantiate a Message
    let msg = Message {
        body: String::from("Hey there"),
        recipient: String::from("555-0199"),
    };
    
    // 3. Print the message body
    println!("Body: {}", msg.body);
}

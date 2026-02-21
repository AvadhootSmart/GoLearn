// 1. Define enum MessageStatus here
enum MessageStatus {
    Sent,
    Failed,
}

// 2. Write print_status function here
fn print_status(status: MessageStatus) {
    match status {
        MessageStatus::Sent => println!("Message was sent"),
        MessageStatus::Failed => println!("Message failed"),
    }
}

fn main() {
    // 3. Call print_status with Sent variant
    let status = MessageStatus::Sent;
    print_status(status);
}

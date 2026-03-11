// Tuple Structs - Textio Type-Safe Identifiers

#[derive(Debug)]
struct PhoneNumber(String);

#[derive(Debug)]
struct AccountId(String);

#[derive(Debug)]
struct MessageId(u64);

#[derive(Debug)]
struct Priority(u8);

struct SystemReady;

fn main() {
    let phone = PhoneNumber(String::from("+15550001"));
    println!("Phone number: {}", phone.0);

    let account = AccountId(String::from("acc_12345678"));
    println!("Account ID: {}", account.0);

    let msg_id = MessageId(42);
    println!("Message ID: {:?}", msg_id);

    let priority = Priority(3);
    let Priority(level) = priority;
    println!("Priority level: {}", level);

    let _ready = SystemReady;
    println!("System is ready!");

    let user_id = AccountId(String::from("user_001"));
    let msg_sender = AccountId(String::from("user_002"));
    println!("User: {:?}, Sender: {:?}", user_id, msg_sender);

    fn send_notification(phone: PhoneNumber) {
        println!("Sending notification to: {}", phone.0);
    }

    send_notification(PhoneNumber(String::from("+15550002")));
}

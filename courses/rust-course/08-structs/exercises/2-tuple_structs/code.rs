// Tuple Structs - Textio Type-Safe Identifiers
// 
// Learn to use tuple structs and the newtype pattern
// for type safety in the Textio SMS system.

// TODO: Define a tuple struct PhoneNumber that wraps a String
// Don't forget to derive Debug!



// TODO: Define a tuple struct AccountId that wraps a String
// Derive Debug



// TODO: Define a tuple struct MessageId that wraps a u64
// Derive Debug



// TODO: Define a tuple struct Priority that wraps a u8
// Derive Debug



// TODO: Define a unit struct called SystemReady
// This represents a marker that the system is initialized



fn main() {
    // TODO: Create a PhoneNumber instance
    let phone = PhoneNumber(String::from("+15550001"));
    
    // TODO: Print the phone number using .0 access
    

    // TODO: Create an AccountId
    let account = AccountId(String::from("acc_12345678"));
    
    // TODO: Print the account ID
    

    // TODO: Create a MessageId with value 42
    let msg_id = MessageId(42);
    
    // TODO: Print the message ID using debug format
    

    // TODO: Destructure the Priority tuple struct
    let priority = Priority(3);
    let Priority(level) = priority;
    println!("Priority level: {}", level);
    

    // TODO: Create an instance of the unit struct
    let _ready = SystemReady;
    println!("System is ready!");
    

    // TODO: Demonstrate type safety
    // Create two different "ids" that would be the same type
    // without the newtype pattern
    let user_id = AccountId(String::from("user_001"));
    let msg_sender = AccountId(String::from("user_002"));
    
    // Both are AccountId type - compiler knows they're the same type
    println!("User: {:?}, Sender: {:?}", user_id, msg_sender);
    

    // TODO: Create a function that only accepts PhoneNumber
    // This shows how newtype pattern provides type safety
    
    fn send_notification(phone: PhoneNumber) {
        println!("Sending notification to: {}", phone.0);
    }
    
    // This works - correct type
    send_notification(PhoneNumber(String::from("+15550002")));
    
    // This would not compile - wrong type:
    // send_notification(String::from("+15550002"));  // Error!
    // send_notification(account);  // Error!
    
}

use std::mem;

#[derive(Copy, Clone, Debug, PartialEq)]
struct PhoneNumber {
    country_code: u16,
    area_code: u16,
    exchange: u16,
    line: u16,
}

#[derive(Copy, Clone, Debug)]
struct MessageId(u64);

#[derive(Debug)]
struct SmsMessage {
    id: MessageId,
    to: PhoneNumber,
    body: String,
}

fn main() {
    println!("=== Exercise 1: Identifying Copy Types ===\n");
    
    // TASK 1: For each type, predict if it's Copy or not
    // Then verify by trying to use the original after assignment
    
    // 1a: i32 - is it Copy?
    let num1: i32 = 42;
    let num2 = num1;
    println!("i32: num1 = {}, num2 = {} (both valid = Copy!)", num1, num2);
    
    // 1b: String - is it Copy?
    // Create a String, try to assign and use both
    // What happens? Write your code here:
    
    // 1c: bool - is it Copy?
    // Create a bool, try to assign and use both
    
    // 1d: Vec<i32> - is it Copy?
    // Create a Vec<i32>, try to assign and use both
    
    println!("\n=== Exercise 2: Tuple Copy Semantics ===\n");
    
    // TASK 2: Determine if each tuple is Copy
    
    // 2a: Tuple of Copy types
    let tuple1 = (1, 2.0, true);
    let tuple2 = tuple1;
    println!("Tuple (i32, f64, bool): both valid = {:?}", tuple1);
    
    // 2b: Tuple with one non-Copy type
    // Create (1, String::from("hello"), true)
    // Try to copy and see what happens
    
    println!("\n=== Exercise 3: Array Copy Semantics ===\n");
    
    // TASK 3: Arrays and Copy
    
    // 3a: Array of Copy types
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;
    println!("Array [i32; 5]: both valid = {:?}", arr1);
    
    // 3b: Array of non-Copy type
    // Create [String::from("a"), String::from("b")]
    // Try to copy and see what happens
    
    println!("\n=== Exercise 4: Custom Copy Type - PhoneNumber ===\n");
    
    // TASK 4: Work with PhoneNumber (which derives Copy)
    
    // Create a phone number
    let phone1 = PhoneNumber {
        country_code: 1,
        area_code: 555,
        exchange: 123,
        line: 4567,
    };
    
    // Copy it
    let phone2 = phone1;
    
    // Both should be valid
    println!("Phone 1: {:?}", phone1);
    println!("Phone 2: {:?}", phone2);
    
    // Pass to function and show it's still valid after
    // Call validate_phone(phone1) and then print phone1 again
    
    println!("\n=== Exercise 5: Custom Copy Type - MessageId ===\n");
    
    // TASK 5: Work with MessageId (newtype pattern with Copy)
    
    let id1 = MessageId(12345);
    let id2 = id1;
    
    println!("ID 1: {:?}", id1);
    println!("ID 2: {:?}", id2);
    
    // Show both are valid by using them
    println!("ID 1 value: {}", id1.0);
    println!("ID 2 value: {}", id2.0);
    
    println!("\n=== Exercise 6: Mixed Struct - Copy and Non-Copy ===\n");
    
    // TASK 6: SmsMessage has both Copy (PhoneNumber, MessageId)
    // and non-Copy (String) fields
    
    let msg = SmsMessage {
        id: MessageId(1),
        to: PhoneNumber {
            country_code: 1,
            area_code: 555,
            exchange: 123,
            line: 4567,
        },
        body: String::from("Hello from Textio!"),
    };
    
    // Can we copy just the Copy fields?
    let msg_id = msg.id;
    let msg_to = msg.to;
    
    // These should work (Copy types)
    println!("Copied ID: {:?}", msg_id);
    println!("Copied phone: {:?}", msg_to);
    
    // Can we still use msg.id and msg.to?
    println!("Original ID still valid: {:?}", msg.id);
    println!("Original phone still valid: {:?}", msg.to);
    
    // What about body (String)?
    // Try to copy it - what happens?
    
    println!("\n=== Exercise 7: Size Comparison ===\n");
    
    // TASK 7: Compare sizes of Copy vs non-Copy types
    
    println!("Size of i32: {} bytes", mem::size_of::<i32>());
    println!("Size of bool: {} bytes", mem::size_of::<bool>());
    println!("Size of f64: {} bytes", mem::size_of::<f64>());
    println!("Size of PhoneNumber: {} bytes", mem::size_of::<PhoneNumber>());
    println!("Size of MessageId: {} bytes", mem::size_of::<MessageId>());
    println!("Size of String: {} bytes", mem::size_of::<String>());
    println!("Size of Vec<i32>: {} bytes", mem::size_of::<Vec<i32>>());
    
    println!("\n=== Exercise 8: Copy Through Functions ===\n");
    
    // TASK 8: Pass Copy types to functions
    
    let number = 42;
    let result = double(number);
    println!("Original: {}, Result: {}", number, result);
    
    let phone = PhoneNumber {
        country_code: 1,
        area_code: 555,
        exchange: 123,
        line: 4567,
    };
    print_phone(phone);
    println!("Phone still valid: {:?}", phone);
    
    println!("\n=== Exercise 9: Clone vs Copy Performance ===\n");
    
    // TASK 9: Show that Copy is implicit, Clone is explicit
    
    let p1 = PhoneNumber {
        country_code: 1,
        area_code: 555,
        exchange: 123,
        line: 4567,
    };
    
    // Implicit copy
    let p2 = p1;
    
    // Explicit clone (same result for Copy types)
    let p3 = p1.clone();
    
    println!("All three valid: {:?}, {:?}, {:?}", p1, p2, p3);
    
    // For String, we MUST use clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("Both Strings: {}, {}", s1, s2);
    
    println!("\n=== Exercise 10: Real Textio Scenario ===\n");
    
    // TASK 10: Process a batch of SMS messages using Copy types
    
    let batch_id = MessageId(1000);
    let default_phone = PhoneNumber {
        country_code: 1,
        area_code: 800,
        exchange: 555,
        line: 1234,
    };
    
    // Process the batch multiple times (Copy allows this!)
    process_batch(batch_id, default_phone);
    process_batch(batch_id, default_phone);
    process_batch(batch_id, default_phone);
    
    println!("Batch ID still valid: {:?}", batch_id);
    println!("Default phone still valid: {:?}", default_phone);
}

fn validate_phone(phone: PhoneNumber) -> bool {
    phone.country_code > 0 && phone.area_code >= 100
}

fn double(n: i32) -> i32 {
    n * 2
}

fn print_phone(phone: PhoneNumber) {
    println!("Phone: +{} ({}) {}-{}", 
        phone.country_code, 
        phone.area_code, 
        phone.exchange, 
        phone.line
    );
}

fn process_batch(batch_id: MessageId, phone: PhoneNumber) {
    println!("Processing batch {:?} for phone {:?}", batch_id, phone);
}

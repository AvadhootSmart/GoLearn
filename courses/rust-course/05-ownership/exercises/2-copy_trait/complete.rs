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

fn main() {
    println!("=== Exercise 1: Identifying Copy Types ===\n");
    
    let num1: i32 = 42;
    let num2 = num1;
    println!("i32: num1 = {}, num2 = {} (both valid = Copy!)", num1, num2);
    
    let str1 = String::from("hello");
    let str2 = str1;
    println!("String: str2 = {} (str1 moved, NOT Copy!)", str2);
    
    let bool1 = true;
    let bool2 = bool1;
    println!("bool: bool1 = {}, bool2 = {} (both valid = Copy!)", bool1, bool2);
    
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1;
    println!("Vec<i32>: vec2 = {:?} (vec1 moved, NOT Copy!)", vec2);
    
    println!("\n=== Exercise 2: Tuple Copy Semantics ===\n");
    
    let tuple1 = (1, 2.0, true);
    let tuple2 = tuple1;
    println!("Tuple (i32, f64, bool): both valid = {:?}", tuple1);
    
    let tuple3 = (1, String::from("hello"), true);
    let tuple4 = tuple3;
    println!("Tuple (i32, String, bool): tuple4 = {:?}", tuple4);
    println!("tuple3.0 and tuple3.2 still valid: {}, {}", tuple3.0, tuple3.2);
    
    println!("\n=== Exercise 3: Array Copy Semantics ===\n");
    
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;
    println!("Array [i32; 5]: both valid = {:?}", arr1);
    
    let arr3 = [String::from("a"), String::from("b")];
    let arr4 = arr3;
    println!("Array [String; 2]: arr4 = {:?}", arr4);
    println!("arr3 moved, not Copy");
    
    println!("\n=== Exercise 4: Custom Copy Type - PhoneNumber ===\n");
    
    let phone1 = PhoneNumber {
        country_code: 1,
        area_code: 555,
        exchange: 123,
        line: 4567,
    };
    
    let phone2 = phone1;
    
    println!("Phone 1: {:?}", phone1);
    println!("Phone 2: {:?}", phone2);
    
    let is_valid = validate_phone(phone1);
    println!("Phone valid: {}", is_valid);
    println!("Phone still valid after function: {:?}", phone1);
    
    println!("\n=== Exercise 5: Custom Copy Type - MessageId ===\n");
    
    let id1 = MessageId(12345);
    let id2 = id1;
    
    println!("ID 1: {:?}", id1);
    println!("ID 2: {:?}", id2);
    
    println!("ID 1 value: {}", id1.0);
    println!("ID 2 value: {}", id2.0);
    
    println!("\n=== Exercise 6: Mixed Struct - Copy and Non-Copy ===\n");
    
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
    
    let msg_id = msg.id;
    let msg_to = msg.to;
    
    println!("Copied ID: {:?}", msg_id);
    println!("Copied phone: {:?}", msg_to);
    
    println!("Original ID still valid: {:?}", msg.id);
    println!("Original phone still valid: {:?}", msg.to);
    
    let body_clone = msg.body.clone();
    println!("Body cloned: {}", body_clone);
    println!("Original body: {}", msg.body);
    
    println!("\n=== Exercise 7: Size Comparison ===\n");
    
    println!("Size of i32: {} bytes", mem::size_of::<i32>());
    println!("Size of bool: {} bytes", mem::size_of::<bool>());
    println!("Size of f64: {} bytes", mem::size_of::<f64>());
    println!("Size of PhoneNumber: {} bytes", mem::size_of::<PhoneNumber>());
    println!("Size of MessageId: {} bytes", mem::size_of::<MessageId>());
    println!("Size of String: {} bytes", mem::size_of::<String>());
    println!("Size of Vec<i32>: {} bytes", mem::size_of::<Vec<i32>>());
    
    println!("\n=== Exercise 8: Copy Through Functions ===\n");
    
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
    
    let p1 = PhoneNumber {
        country_code: 1,
        area_code: 555,
        exchange: 123,
        line: 4567,
    };
    
    let p2 = p1;
    let p3 = p1.clone();
    
    println!("All three valid: {:?}, {:?}, {:?}", p1, p2, p3);
    
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("Both Strings: {}, {}", s1, s2);
    
    println!("\n=== Exercise 10: Real Textio Scenario ===\n");
    
    let batch_id = MessageId(1000);
    let default_phone = PhoneNumber {
        country_code: 1,
        area_code: 800,
        exchange: 555,
        line: 1234,
    };
    
    process_batch(batch_id, default_phone);
    process_batch(batch_id, default_phone);
    process_batch(batch_id, default_phone);
    
    println!("Batch ID still valid: {:?}", batch_id);
    println!("Default phone still valid: {:?}", default_phone);
}

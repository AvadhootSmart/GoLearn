# Function Ownership: Taking and Giving Ownership

## Ownership Through Functions

Functions are where ownership really matters. Understanding how ownership flows through functions is essential.

```
┌──────────────────────────────────────────────────────────────┐
│                OWNERSHIP FLOW IN FUNCTIONS                   │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  When you pass a value to a function:                        │
│                                                              │
│  1. MOVE: Ownership transfers TO the function                │
│     fn take(s: String) { }  // Takes ownership               │
│                                                              │
│  2. BORROW: Temporary access, no transfer                    │
│     fn borrow(s: &String) { }  // Borrows                    │
│                                                              │
│  3. COPY: For Copy types, a copy is made                     │
│     fn copy(n: i32) { }  // Gets a copy                      │
│                                                              │
│  When a function returns:                                    │
│                                                              │
│  1. RETURN: Ownership transfers TO caller                     │
│     fn create() -> String { String::from("hi") }             │
│                                                              │
│  2. NOTHING: If nothing returned, values drop in function    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Taking Ownership

```
┌──────────────────────────────────────────────────────────────┐
│                    TAKING OWNERSHIP                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  fn process(message: String) {                               │
│      println!("Processing: {}", message);                    │
│  }  // message drops here!                                   │
│                                                              │
│  fn main() {                                                 │
│      let msg = String::from("Hello");                        │
│      process(msg);  // Ownership moves to process            │
│      // println!("{}", msg);  // ERROR! msg was moved        │
│  }                                                           │
│                                                              │
│  Memory visualization:                                       │
│                                                              │
│  Before call:                                                │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   msg ──► "Hello"                  │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  During call:                                                │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   msg ╳ (invalidated)              │                     │
│  └────────────────────────────────────┘                     │
│  ┌────────────────────────────────────┐                     │
│  │ process's stack:                   │                     │
│  │   message ──► "Hello"              │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  After call:                                                 │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   msg ╳ (still invalid)            │                     │
│  │   "Hello" was dropped in process!  │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Giving Ownership (Return Values)

```
┌──────────────────────────────────────────────────────────────┐
│                    GIVING OWNERSHIP                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  fn create_message() -> String {                             │
│      let message = String::from("Hello from Textio!");       │
│      message  // Ownership moves to caller                   │
│  }                                                           │
│                                                              │
│  fn main() {                                                 │
│      let msg = create_message();  // Ownership received      │
│      println!("{}", msg);  // Valid!                         │
│  }                                                           │
│                                                              │
│  Memory visualization:                                       │
│                                                              │
│  In create_message:                                          │
│  ┌────────────────────────────────────┐                     │
│  │ create_message's stack:            │                     │
│  │   message ──► "Hello from Textio!" │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  Returning (ownership moves):                                │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   msg ──► "Hello from Textio!"     │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  Heap data is NOT copied - just pointer moves!               │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Take and Give Back Pattern

```
┌──────────────────────────────────────────────────────────────┐
│                TAKE AND GIVE BACK PATTERN                    │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Sometimes you need to modify and return:                    │
│                                                              │
│  fn add_greeting(mut message: String) -> String {            │
│      message.push_str(" - Textio");                          │
│      message  // Return ownership                            │
│  }                                                           │
│                                                              │
│  fn main() {                                                 │
│      let msg = String::from("Hello");                        │
│      let msg = add_greeting(msg);  // Take and give back     │
│      println!("{}", msg);  // "Hello - Textio"               │
│  }                                                           │
│                                                              │
│  This pattern is common but can be tedious.                  │
│  Better alternative: use mutable references!                 │
│                                                              │
│  fn add_greeting(message: &mut String) {                     │
│      message.push_str(" - Textio");                          │
│  }                                                           │
│                                                              │
│  fn main() {                                                 │
│      let mut msg = String::from("Hello");                    │
│      add_greeting(&mut msg);                                 │
│      println!("{}", msg);                                    │
│  }                                                           │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Multiple Return Values

```
┌──────────────────────────────────────────────────────────────┐
│                MULTIPLE OWNERSHIP TRANSFERS                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Return multiple owned values with tuples:                   │
│                                                              │
│  fn split_message(message: String) -> (String, String) {     │
│      let parts: Vec<&str> = message.splitn(2, ':').collect();│
│      (                                                       │
│          parts[0].to_string(),                               │
│          parts.get(1).unwrap_or(&"").to_string(),            │
│      )                                                       │
│  }                                                           │
│                                                              │
│  fn main() {                                                 │
│      let msg = String::from("TO:+1234567890");               │
│      let (prefix, number) = split_message(msg);              │
│      // msg is now invalid (moved into function)             │
│      // But we own prefix and number!                        │
│  }                                                           │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Copy Types Through Functions

```
┌──────────────────────────────────────────────────────────────┐
│                COPY TYPES IN FUNCTIONS                       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Copy types don't move - they're copied:                     │
│                                                              │
│  fn double(n: i32) -> i32 {                                  │
│      n * 2                                                   │
│  }                                                           │
│                                                              │
│  fn main() {                                                 │
│      let x = 5;                                              │
│      let y = double(x);                                      │
│      println!("x = {}, y = {}", x, y);  // Both valid!       │
│  }                                                           │
│                                                              │
│  Memory visualization:                                       │
│                                                              │
│  Before call:                                                │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   x = 5                            │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  During call:                                                │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   x = 5  (still valid!)            │                     │
│  └────────────────────────────────────┘                     │
│  ┌────────────────────────────────────┐                     │
│  │ double's stack:                    │                     │
│  │   n = 5  (copy of x)               │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  After call:                                                 │
│  ┌────────────────────────────────────┐                     │
│  │ main's stack:                      │                     │
│  │   x = 5                            │                     │
│  │   y = 10                           │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## The Three Patterns

```
┌──────────────────────────────────────────────────────────────┐
│            THREE OWNERSHIP PATTERNS                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Pattern 1: TAKE (consume)                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ fn send(message: String) { }                         │   │
│  │                                                      │   │
│  │ let msg = String::from("Hello");                     │   │
│  │ send(msg);                                           │   │
│  │ // msg invalid after call                            │   │
│  │                                                      │   │
│  │ Use when: Function consumes the value                │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  Pattern 2: BORROW (reference)                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ fn print(message: &String) { }                       │   │
│  │                                                      │   │
│  │ let msg = String::from("Hello");                     │   │
│  │ print(&msg);                                         │   │
│  │ // msg still valid after call                        │   │
│  │                                                      │   │
│  │ Use when: Function only needs to read                │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  Pattern 3: RETURN (give)                                    │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ fn create() -> String { String::from("Hello") }      │   │
│  │                                                      │   │
│  │ let msg = create();                                  │   │
│  │ // msg owns the returned value                       │   │
│  │                                                      │   │
│  │ Use when: Function creates new owned value           │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## C++ Comparison: What Would Crash

```
┌──────────────────────────────────────────────────────────────┐
│            C++ CRASH vs RUST COMPILE ERROR                   │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  C++ USE AFTER FREE (crashes at runtime):                    │
│                                                              │
│  void process(string s) {                                    │
│      cout << s << endl;                                      │
│  }  // s destructor called, memory freed                     │
│                                                              │
│  int main() {                                                │
│      string msg = "hello";                                   │
│      process(std::move(msg));                                │
│      cout << msg << endl;  // CRASH! Use after free!         │
│  }                                                           │
│                                                              │
│  ─────────────────────────────────────────────────────────   │
│                                                              │
│  RUST (caught at compile time):                              │
│                                                              │
│  fn process(s: String) {                                     │
│      println!("{}", s);                                      │
│  }  // s dropped, memory freed                               │
│                                                              │
│  fn main() {                                                 │
│      let msg = String::from("hello");                        │
│      process(msg);                                           │
│      // println!("{}", msg);  // COMPILE ERROR!              │
│      // error: value borrowed here after move                │
│  }                                                           │
│                                                              │
│  Rust catches the bug BEFORE your program runs!              │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Function Ownership with Structs

```rust
struct SmsMessage {
    to: String,
    body: String,
}

// Takes ownership of entire struct
fn send(message: SmsMessage) {
    println!("Sending to {}: {}", message.to, message.body);
}  // message dropped here

// Creates and returns ownership
fn create_message(to: &str, body: &str) -> SmsMessage {
    SmsMessage {
        to: to.to_string(),
        body: body.to_string(),
    }
}

// Takes ownership, modifies, returns
fn add_signature(mut message: SmsMessage) -> SmsMessage {
    message.body.push_str("\n\n- Textio");
    message
}

fn main() {
    // Create gives us ownership
    let msg = create_message("+1234567890", "Hello!");
    
    // Take and give back
    let msg = add_signature(msg);
    
    // Final send consumes the message
    send(msg);
    // msg is now invalid!
}
```

## Ownership Flow Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                OWNERSHIP FLOW VISUALIZATION                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  fn create_message() -> SmsMessage { ... }                   │
│  fn validate(msg: SmsMessage) -> SmsMessage { ... }          │
│  fn send(msg: SmsMessage) { ... }                            │
│                                                              │
│  fn main() {                                                 │
│      let msg = create_message();                             │
│      //           │                                          │
│      //           ▼                                          │
│      //    ┌──────────────┐                                  │
│      //    │ msg received │                                  │
│      //    └──────────────┘                                  │
│      //           │                                          │
│      let msg = validate(msg);                                │
│      //           │      │                                   │
│      //           ▼      ▼                                   │
│      //    ┌──────────────────┐                              │
│      //    │ msg moved in,    │                              │
│      //    │ msg moved out    │                              │
│      //    └──────────────────┘                              │
│      //           │                                          │
│      send(msg);                                              │
│      //           │                                          │
│      //           ▼                                          │
│      //    ┌──────────────────┐                              │
│      //    │ msg consumed,    │                              │
│      //    │ dropped inside   │                              │
│      //    └──────────────────┘                              │
│      //                                                      │
│      // msg invalid here!                                    │
│  }                                                           │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Builder Pattern with Ownership

```rust
struct SmsBuilder {
    to: Option<String>,
    from: Option<String>,
    body: Option<String>,
}

impl SmsBuilder {
    fn new() -> Self {
        SmsBuilder {
            to: None,
            from: None,
            body: None,
        }
    }
    
    // Takes &mut self, returns &mut self for chaining
    fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }
    
    fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }
    
    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
    
    // Consumes self and returns the final message
    fn build(self) -> Result<SmsMessage, &'static str> {
        Ok(SmsMessage {
            to: self.to.ok_or("to is required")?,
            from: self.from.ok_or("from is required")?,
            body: self.body.ok_or("body is required")?,
        })
    }
}

fn main() {
    let message = SmsBuilder::new()
        .to("+1234567890")
        .from("+0987654321")
        .body("Hello from Textio!")
        .build()
        .unwrap();
}
```

## Option and Ownership

```rust
fn find_message(id: u32, messages: &mut Vec<SmsMessage>) -> Option<SmsMessage> {
    // Remove transfers ownership out of the Vec
    if id < messages.len() as u32 {
        Some(messages.remove(id as usize))
    } else {
        None
    }
}

fn main() {
    let mut messages = vec![
        SmsMessage { to: String::from("+1"), body: String::from("Hi") },
        SmsMessage { to: String::from("+2"), body: String::from("Hello") },
    ];
    
    // Option<SmsMessage> - we might or might not get ownership
    match find_message(0, &mut messages) {
        Some(msg) => println!("Found: {}", msg.body),
        None => println!("Not found"),
    }
    
    // First message was removed from Vec, only one left
    println!("Messages remaining: {}", messages.len());
}
```

## Result and Ownership

```rust
fn parse_phone_number(input: String) -> Result<PhoneNumber, String> {
    if input.starts_with('+') && input.len() >= 10 {
        Ok(PhoneNumber { number: input })
    } else {
        // We own input, we can return it as the error!
        Err(format!("Invalid phone number: {}", input))
    }
}

fn main() {
    let input = String::from("+1234567890");
    
    match parse_phone_number(input) {
        Ok(phone) => println!("Valid: {:?}", phone),
        Err(e) => println!("Error: {}", e),
    }
    // input is moved into the function
}
```

## Textio Example: Full Pipeline

```rust
struct SmsMessage {
    id: u32,
    to: String,
    body: String,
}

fn create(id: u32, to: &str, body: &str) -> SmsMessage {
    SmsMessage {
        id,
        to: to.to_string(),
        body: body.to_string(),
    }
}

fn validate(message: SmsMessage) -> Result<SmsMessage, String> {
    if message.to.starts_with('+') {
        Ok(message)
    } else {
        Err(format!("Invalid phone: {}", message.to))
    }
}

fn format(message: SmsMessage) -> String {
    format!("[{}] To: {} | {}", message.id, message.to, message.body)
}

fn send(formatted: String) -> bool {
    println!("Sending: {}", formatted);
    true
}

fn main() {
    let msg = create(1, "+1234567890", "Hello!");
    
    // Validate takes ownership, returns Result with ownership
    let msg = validate(msg).expect("Validation failed");
    
    // Format takes ownership, returns owned String
    let formatted = format(msg);
    // msg is now invalid!
    
    // Send takes ownership of formatted
    let success = send(formatted);
    // formatted is now invalid!
    
    println!("Success: {}", success);
}
```

## Exercises

In this exercise, you'll work with Textio's message pipeline:

1. Take ownership through function parameters
2. Give ownership through return values
3. Use take-and-give-back patterns
4. Build complete ownership pipelines

Complete the tasks in `code.rs` to master function ownership!

## Key Takeaways

```
┌──────────────────────────────────────────────────────────────┐
│                    REMEMBER                                  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Passing owned value to function = MOVE                   │
│                                                              │
│  2. Returning owned value = GIVE ownership to caller         │
│                                                              │
│  3. Copy types are copied, not moved                         │
│                                                              │
│  4. Take-and-give-back is a common pattern                   │
│                                                              │
│  5. Consider references if you don't need ownership          │
│                                                              │
│  6. Option/Result can transfer ownership                     │
│                                                              │
│  7. Builder pattern consumes self, returns built value       │
│                                                              │
│  8. Ownership flow creates clear responsibility chains       │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

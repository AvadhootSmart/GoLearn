// Exercise 5: Custom Iterators - Complete Solution

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

struct Counter {
    start: i32,
    end: i32,
    current: i32,
}

impl Counter {
    fn new(start: i32, end: i32) -> Self {
        Counter {
            start,
            end,
            current: start - 1,
        }
    }
}

impl Iterator for Counter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end - self.current).max(0) as usize;
        (remaining, Some(remaining))
    }
}

struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        let next_val = self.current + self.next;
        self.current = self.next;
        self.next = next_val;
        Some(result)
    }
}

struct MessageQueue {
    messages: Vec<Message>,
}

impl MessageQueue {
    fn new() -> Self {
        MessageQueue { messages: Vec::new() }
    }
    
    fn add(&mut self, message: Message) {
        self.messages.push(message);
    }
    
    fn iter(&self) -> Iter<'_> {
        Iter { 
            inner: self.messages.iter() 
        }
    }
    
    fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut { 
            inner: self.messages.iter_mut() 
        }
    }
}

pub struct Iter<'a> {
    inner: std::slice::Iter<'a, Message>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Message;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct IterMut<'a> {
    inner: std::slice::IterMut<'a, Message>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Message;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for MessageQueue {
    type Item = Message;
    type IntoIter = std::vec::IntoIter<Message>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

struct Paginated<'a> {
    messages: &'a [Message],
    page_size: usize,
    current_page: usize,
}

impl<'a> Paginated<'a> {
    fn new(messages: &'a [Message], page_size: usize) -> Self {
        Paginated {
            messages,
            page_size,
            current_page: 0,
        }
    }
}

impl<'a> Iterator for Paginated<'a> {
    type Item = Vec<&'a Message>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.current_page * self.page_size;
        if start >= self.messages.len() {
            return None;
        }
        let end = (start + self.page_size).min(self.messages.len());
        let page: Vec<&'a Message> = self.messages[start..end].iter().collect();
        self.current_page += 1;
        Some(page)
    }
}

struct Filtered<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Filtered<I, P> {
    fn new(iter: I, predicate: P) -> Self {
        Filtered { iter, predicate }
    }
}

impl<I, P> Iterator for Filtered<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
}

struct MessageIdRange {
    start: u32,
    end: u32,
    current: u32,
}

impl MessageIdRange {
    fn new(start: u32, end: u32) -> Self {
        MessageIdRange {
            start,
            end,
            current: start,
        }
    }
}

impl Iterator for MessageIdRange {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Part 1: Counter Iterator ===\n");
    
    println!("Counting 1 to 5:");
    for n in Counter::new(1, 5) {
        println!("  {}", n);
    }
    
    let count_vec: Vec<i32> = Counter::new(10, 15).collect();
    println!("Counter vector: {:?}", count_vec);
    
    println!("\n=== Part 2: Fibonacci Iterator ===\n");
    
    println!("First 10 Fibonacci numbers:");
    for fib in Fibonacci::new().take(10) {
        println!("  {}", fib);
    }
    
    let sum: u64 = Fibonacci::new().take(20).sum();
    println!("Sum of first 20 Fibonacci numbers: {}", sum);
    
    println!("\n=== Part 3: MessageQueue Iterator ===\n");
    
    let mut queue = MessageQueue::new();
    queue.add(Message::new(1, "Hello", "+15550001"));
    queue.add(Message::new(2, "World", "+15550002"));
    queue.add(Message::new(3, "Test", "+15550003"));
    
    println!("All messages:");
    for msg in queue.iter() {
        println!("  {:?}", msg);
    }
    
    println!("Consuming queue:");
    for msg in queue {
        println!("  Consumed: {:?}", msg.id);
    }
    
    println!("\n=== Part 4: Paginated Iterator ===\n");
    
    let messages = vec![
        Message::new(1, "Msg 1", "+1"),
        Message::new(2, "Msg 2", "+2"),
        Message::new(3, "Msg 3", "+3"),
        Message::new(4, "Msg 4", "+4"),
        Message::new(5, "Msg 5", "+5"),
        Message::new(6, "Msg 6", "+6"),
        Message::new(7, "Msg 7", "+7"),
    ];
    
    println!("Pages of 3 messages:");
    for (page_num, page) in Paginated::new(&messages, 3).enumerate() {
        println!("Page {}:", page_num);
        for msg in page {
            println!("  {:?}", msg.id);
        }
    }
    
    println!("\n=== Part 5: Filtered Iterator ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = Filtered::new(numbers.into_iter(), |n| n % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
    
    println!("\n=== Part 6: MessageIdRange Iterator ===\n");
    
    let ids: Vec<u32> = MessageIdRange::new(100, 105).collect();
    println!("Message IDs: {:?}", ids);
    
    println!("\n=== Part 7: Textio Message Stream ===\n");
    
    let textio_messages = vec![
        Message::new(101, "Verification code: 1234", "+15550001"),
        Message::new(102, "Your order shipped", "+15550002"),
        Message::new(103, "Verification code: 5678", "+15550001"),
        Message::new(104, "Meeting reminder", "+15550003"),
        Message::new(105, "Verification code: 9999", "+15550004"),
    ];
    
    println!("Messages with verification codes (paginated):");
    let code_messages: Vec<&Message> = textio_messages.iter()
        .filter(|m| m.content.contains("code"))
        .collect();
    
    for (page_num, page) in Paginated::new(&code_messages, 2).enumerate() {
        println!("Page {}:", page_num);
        for msg in page {
            println!("  {:?}", msg);
        }
    }
    
    println!("\n=== Part 8: Combining Custom Iterators ===\n");
    
    let combined: Vec<i32> = Counter::new(1, 3)
        .chain(Counter::new(10, 12))
        .collect();
    println!("Chained counters: {:?}", combined);
    
    let processed: Vec<String> = MessageIdRange::new(1, 5)
        .map(|id| format!("MSG-{}", id))
        .collect();
    println!("Processed IDs: {:?}", processed);
}

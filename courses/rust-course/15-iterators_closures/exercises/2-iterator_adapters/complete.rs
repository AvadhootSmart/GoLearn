// Exercise 2: Iterator Adapters - Complete Solution

#[derive(Debug, Clone)]
struct Message {
    id: u32,
    phone: String,
    content: String,
    priority: u8,
}

impl Message {
    fn new(id: u32, phone: &str, content: &str, priority: u8) -> Self {
        Message {
            id,
            phone: phone.to_string(),
            content: content.to_string(),
            priority,
        }
    }
    
    fn summary(&self) -> String {
        format!("[{}] To {}: {}", self.id, self.phone, self.content)
    }
}

fn main() {
    println!("=== Part 1: map() and filter() ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Evens: {:?}", evens);
    
    let odd_squares: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 1).map(|x| x * x).collect();
    println!("Odd squares: {:?}", odd_squares);
    
    println!("\n=== Part 2: take() and skip() ===\n");
    
    let first_five: Vec<i32> = (1..).take(5).collect();
    println!("First five from infinite: {:?}", first_five);
    
    let skipped: Vec<i32> = (1..=10).skip(3).take(4).collect();
    println!("Skip 3, take 4: {:?}", skipped);
    
    println!("\n=== Part 3: enumerate() ===\n");
    
    let fruits = vec!["apple", "banana", "cherry", "date", "elderberry"];
    
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }
    
    let indexed: Vec<(usize, &str)> = fruits.iter().enumerate().map(|(i, f)| (i, *f)).collect();
    println!("Indexed: {:?}", indexed);
    
    println!("\n=== Part 4: zip() ===\n");
    
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![28, 32, 25];
    let cities = vec!["New York", "San Francisco", "Chicago"];
    
    let name_ages: Vec<(&str, i32)> = names.iter().zip(ages.iter()).map(|(n, a)| (*n, *a)).collect();
    println!("Name-Age pairs: {:?}", name_ages);
    
    let full_info: Vec<(&str, i32, &str)> = names.iter()
        .zip(ages.iter())
        .zip(cities.iter())
        .map(|((n, a), c)| (*n, *a, *c))
        .collect();
    println!("Full info: {:?}", full_info);
    
    println!("\n=== Part 5: chain() and flatten() ===\n");
    
    let queue1 = vec![1, 2, 3];
    let queue2 = vec![4, 5, 6];
    let queue3 = vec![7, 8, 9];
    
    let combined: Vec<i32> = queue1.iter()
        .chain(queue2.iter())
        .chain(queue3.iter())
        .cloned()
        .collect();
    println!("Combined queues: {:?}", combined);
    
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    
    let flat: Vec<i32> = nested.iter().flatten().cloned().collect();
    println!("Flattened: {:?}", flat);
    
    let sentences = vec!["Hello world", "Rust is great", "Learn iterators"];
    let all_words: Vec<&str> = sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("All words: {:?}", all_words);
    
    println!("\n=== Part 6: Textio Message Pipeline ===\n");
    
    let messages = vec![
        Message::new(1, "+15550001", "Your code is 1234", 9),
        Message::new(2, "+15550002", "Meeting reminder", 5),
        Message::new(3, "+15550001", "Your code is 5678", 9),
        Message::new(4, "+15550003", "Promo: 50% off!", 2),
        Message::new(5, "+15550002", "Delivery update", 7),
        Message::new(6, "+15550001", "Account alert", 8),
        Message::new(7, "+15550003", "Your code is 9999", 9),
        Message::new(8, "+15550002", "Weekly digest", 1),
    ];
    
    let urgent_summaries: Vec<String> = messages.iter()
        .filter(|m| m.priority >= 7)
        .map(|m| m.summary())
        .collect();
    println!("Urgent messages:");
    for summary in urgent_summaries {
        println!("  {}", summary);
    }
    
    let page_size = 3;
    let page_number = 1;
    let page: Vec<&Message> = messages.iter()
        .skip(page_number * page_size)
        .take(page_size)
        .collect();
    println!("\nPage {} of messages:", page_number);
    for msg in page {
        println!("  {:?}", msg);
    }
    
    let mut unique_phones: Vec<&String> = messages.iter().map(|m| &m.phone).collect();
    unique_phones.sort();
    unique_phones.dedup();
    println!("\nUnique phone numbers: {:?}", unique_phones);
    
    let count_for_001: usize = messages.iter().filter(|m| m.phone == "+15550001").count();
    println!("Messages for +15550001: {}", count_for_001);
}

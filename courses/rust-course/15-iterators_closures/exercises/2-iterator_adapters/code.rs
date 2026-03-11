// Exercise 2: Iterator Adapters
// Learn map, filter, take, skip, enumerate, zip, chain, and flatten

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
    
    // TODO: Use map to double each number and collect into Vec
    let doubled: Vec<i32> = vec![]; // Replace with your code
    println!("Doubled: {:?}", doubled);
    
    // TODO: Use filter to keep only even numbers
    let evens: Vec<i32> = vec![]; // Replace with your code
    println!("Evens: {:?}", evens);
    
    // TODO: Chain map and filter: square only the odd numbers
    let odd_squares: Vec<i32> = vec![]; // Replace with your code
    println!("Odd squares: {:?}", odd_squares);
    
    println!("\n=== Part 2: take() and skip() ===\n");
    
    // TODO: Take only the first 5 numbers from an infinite range
    let first_five: Vec<i32> = vec![]; // Replace: (1..).take(5).collect()
    println!("First five from infinite: {:?}", first_five);
    
    // TODO: Skip the first 3 and take the next 4
    let skipped: Vec<i32> = vec![]; // Replace: (1..=10).skip(3).take(4).collect()
    println!("Skip 3, take 4: {:?}", skipped);
    
    println!("\n=== Part 3: enumerate() ===\n");
    
    let fruits = vec!["apple", "banana", "cherry", "date", "elderberry"];
    
    // TODO: Use enumerate to print each fruit with its index
    // Example output: "0: apple", "1: banana", etc.
    
    // TODO: Collect enumerated pairs into a Vec
    let indexed: Vec<(usize, &str)> = vec![]; // Replace with your code
    println!("Indexed: {:?}", indexed);
    
    println!("\n=== Part 4: zip() ===\n");
    
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![28, 32, 25];
    let cities = vec!["New York", "San Francisco", "Chicago"];
    
    // TODO: Zip names and ages into pairs
    let name_ages: Vec<(&str, i32)> = vec![]; // Replace with your code
    println!("Name-Age pairs: {:?}", name_ages);
    
    // TODO: Zip all three together (names, ages, cities)
    // Hint: zip names with ages, then zip that result with cities
    let full_info: Vec<(&str, i32, &str)> = vec![]; // Replace with your code
    println!("Full info: {:?}", full_info);
    
    println!("\n=== Part 5: chain() and flatten() ===\n");
    
    let queue1 = vec![1, 2, 3];
    let queue2 = vec![4, 5, 6];
    let queue3 = vec![7, 8, 9];
    
    // TODO: Chain all three queues together
    let combined: Vec<i32> = vec![]; // Replace with your code
    println!("Combined queues: {:?}", combined);
    
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    
    // TODO: Flatten the nested vectors
    let flat: Vec<i32> = vec![]; // Replace with your code
    println!("Flattened: {:?}", flat);
    
    // TODO: Use flat_map to split sentences into words
    let sentences = vec!["Hello world", "Rust is great", "Learn iterators"];
    let all_words: Vec<&str> = vec![]; // Hint: use flat_map with split_whitespace
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
    
    // TODO: Get summaries of high-priority messages (priority >= 7)
    let urgent_summaries: Vec<String> = vec![]; // Replace with your code
    println!("Urgent messages:");
    for summary in urgent_summaries {
        println!("  {}", summary);
    }
    
    // TODO: Implement pagination - get page 1 (items 2-4, 0-indexed)
    let page_size = 3;
    let page_number = 1;
    let page: Vec<&Message> = vec![]; // Replace: use skip and take
    println!("\nPage {} of messages:", page_number);
    for msg in page {
        println!("  {:?}", msg);
    }
    
    // TODO: Find all unique phone numbers
    // Hint: use map to get phones, then collect and dedup
    let unique_phones: Vec<&String> = vec![]; // Replace with your code
    println!("\nUnique phone numbers: {:?}", unique_phones);
    
    // TODO: Count messages per phone number using enumerate and filter
    // Get count for "+15550001"
    let count_for_001: usize = 0; // Replace with your code
    println!("Messages for +15550001: {}", count_for_001);
}

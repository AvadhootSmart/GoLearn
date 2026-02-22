use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("alice"), 10);
    scores.insert(String::from("bob"), 20);

    if let Some(score) = scores.get("alice") {
        println!("alice {}", score);
    }
}

fn main() {
    // 1. Create mutable vector user_ids
    let mut user_ids: Vec<i32> = Vec::new();
    
    // 2. Push 101, 102, 103
    user_ids.push(101);
    user_ids.push(102);
    user_ids.push(103);
    
    // 3. Iterate and print
    for id in user_ids {
        println!("User ID: {}", id);
    }
}

fn main() {
    // 1. Create a vector numbers using vec![1, 2, 3]
    let numbers = vec![1, 2, 3];
    
    // 2. Use map and a closure to add 1 to each number
    // We must annotate the type of added_one so collect() knows what to build
    let added_one: Vec<i32> = numbers.iter().map(|x| x + 1).collect();
    
    // 3. Print added_one
    println!("{:?}", added_one);
}

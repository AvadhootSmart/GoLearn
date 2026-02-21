fn main() {
    // 1. Allocate a Box on the heap with value 5
    let b = Box::new(5);
    
    // 2. Print the Box value
    println!("b = {}", b);
}

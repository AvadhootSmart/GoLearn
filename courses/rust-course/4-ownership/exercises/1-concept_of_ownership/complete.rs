fn main() {
    // 1. Create a String named s1 with the value "hello" (use String::from)
    let s1 = String::from("hello");
    
    // 2. Move ownership of s1 to s2
    let s2 = s1;
    
    // 3. Print s2
    println!("{}", s2);
}

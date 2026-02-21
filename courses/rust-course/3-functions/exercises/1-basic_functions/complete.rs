fn main() {
    let greeting = get_greeting();
    println!("{}", greeting);
}

// 1. Write a function called get_greeting
// 2. Return the string "Welcome to Textio!" using String::from
fn get_greeting() -> String {
    String::from("Welcome to Textio!")
}

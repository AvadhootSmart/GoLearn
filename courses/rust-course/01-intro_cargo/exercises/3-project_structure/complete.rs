mod greeting {
    pub fn say_hello() -> &'static str {
        "Hello from Textio!"
    }
}

fn main() {
    println!("{}", greeting::say_hello());
}

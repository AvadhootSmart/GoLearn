fn build_message(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let message = build_message("Textio");
    println!("{}", message);
}

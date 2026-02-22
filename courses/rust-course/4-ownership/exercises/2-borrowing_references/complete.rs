fn message_len(value: &String) -> usize {
    value.len()
}

fn main() {
    let msg = String::from("status");
    let length = message_len(&msg);
    println!("{} {}", length, msg);
}

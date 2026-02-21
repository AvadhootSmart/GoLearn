// 1. Write read_config function here
fn read_config() -> Result<String, String> {
    Ok(String::from("config loaded"))
}

fn main() {
    // 2. Call read_config() and match on the result
    match read_config() {
        Ok(msg) => println!("Success: {}", msg),
        Err(err) => println!("Error: {}", err),
    }
}

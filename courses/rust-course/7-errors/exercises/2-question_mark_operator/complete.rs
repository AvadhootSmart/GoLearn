use std::num::ParseIntError;

fn parse_port(input: &str) -> Result<u16, ParseIntError> {
    let port = input.parse::<u16>()?;
    Ok(port)
}

fn main() {
    match parse_port("8080") {
        Ok(port) => println!("port {}", port),
        Err(e) => println!("error {}", e),
    }
}

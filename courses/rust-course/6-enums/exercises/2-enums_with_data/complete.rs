enum ApiResponse {
    Ok(u16),
    Err(String),
}

fn print_response(response: ApiResponse) {
    match response {
        ApiResponse::Ok(code) => println!("status {}", code),
        ApiResponse::Err(message) => println!("error {}", message),
    }
}

fn main() {
    print_response(ApiResponse::Ok(200));
}

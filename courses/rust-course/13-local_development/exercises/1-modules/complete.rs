// 1. Define mod server
mod server {
    // 2. Inside server, define pub mod network
    pub mod network {
        // 3. Inside network, define pub fn connect
        pub fn connect() {
            println!("connected");
        }
    }
}

fn main() {
    // 4. Call connect
    crate::server::network::connect();
}

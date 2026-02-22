mod server {
    pub mod network {
        pub fn connect() {
            println!("connected");
        }
    }
}

use crate::server::network::connect;

fn main() {
    connect();
    println!("boot complete");
}

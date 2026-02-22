use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("ready")).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("message {}", received);
}

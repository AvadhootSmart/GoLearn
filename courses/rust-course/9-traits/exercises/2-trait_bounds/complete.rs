trait Loggable {
    fn log_line(&self) -> String;
}

struct Event {
    id: u32,
}

impl Loggable for Event {
    fn log_line(&self) -> String {
        format!("event-{}", self.id)
    }
}

fn print_log<T: Loggable>(item: &T) {
    println!("{}", item.log_line());
}

fn main() {
    let event = Event { id: 7 };
    print_log(&event);
}

use std::rc::Rc;

fn main() {
    let name = Rc::new(String::from("Textio"));
    let name_clone = Rc::clone(&name);

    println!("{} {}", name, name_clone);
}

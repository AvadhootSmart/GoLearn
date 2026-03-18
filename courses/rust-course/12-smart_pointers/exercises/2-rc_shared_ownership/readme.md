# Shared Ownership with `Rc<T>`

## Why this matters
Some data needs multiple owners in single-threaded programs (graphs, shared config, AST nodes). `Rc<T>` enables that pattern safely.

## Core concept
`Rc<T>` uses reference counting:
- cloning an `Rc` increments count,
- dropping an `Rc` decrements count,
- value is dropped when count reaches zero.

## Worked example
```rust
use std::rc::Rc;

fn main() {
    let name = Rc::new(String::from("Textio"));
    let name_clone = Rc::clone(&name);

    println!("{} {}", name, name_clone);
}
```

## Common mistakes
- Using `Rc<T>` across threads (use `Arc<T>` instead).
- Assuming `Rc` gives mutability (pair with `RefCell` for interior mutability when needed).

## Exercise task

1. Import `std::rc::Rc`.
2. Create `name = Rc::new(String::from("Textio"))`.
3. Clone it into `name_clone` using `Rc::clone(&name)`.
4. Print both values.

## Quick recap
`Rc<T>` is for shared ownership in single-threaded contexts, not for mutation or multithreading by itself.

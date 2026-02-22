# Shared Ownership with `Rc<T>`

## Concept first: multiple owners in single-threaded code

`Rc<T>` enables shared ownership by reference counting.

## Exercise task

1. Import `std::rc::Rc`.
2. Create `name = Rc::new(String::from("Textio"))`.
3. Clone it into `name_clone` using `Rc::clone(&name)`.
4. Print both values.

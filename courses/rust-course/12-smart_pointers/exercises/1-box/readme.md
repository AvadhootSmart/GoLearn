# Smart Pointers

Smart pointers are data structures that act like a pointer but have additional metadata and capabilities. `Box<T>` is the most straightforward smart pointer, letting you allocate values on the heap instead of the stack.

## Assignment

1. Use `Box::new` to allocate an `i32` with the value `5` on the heap, binding it to the variable `b`.
2. Print the value of `b` by using string interpolation `println!("b = {}", b);`.

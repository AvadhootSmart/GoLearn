# Iterators and Closures

Rust's iterators are lazy, meaning they have no effect until you call methods that consume them. Closures are anonymous functions you can save in a variable or pass as arguments.

## Assignment

1. Create a vector `numbers` containing `1, 2, 3`.
2. Use `.iter().map()` with a closure to add `1` to each number, and `.collect()` it into a new vector `added_one`.
3. Print the `added_one` vector using `println!("{:?}", added_one);`.

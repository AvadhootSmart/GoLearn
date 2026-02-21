# Squashing Bugs

## Concept first: using compiler errors as guidance

A core Rust workflow is: write code → compile/run → read compiler feedback → fix the issue. Rust's compiler errors are usually descriptive and often include hints that point directly to the fix.

Example debugging loop:

```rust
fn main() {
    let status = "Server boot complete";
    println!("{}", status);
}
```

If a variable name is misspelled or a type is used incorrectly, the compiler reports what it expected and where the issue is.

Nuances to remember:

- Fix the **first** compile error first; later errors are often side effects.
- Pay attention to line numbers and suggested fixes in the error output.
- Re-run after each small fix.

## Exercise task

There is a bug in the starter code. Run it, read the Rust compiler error, and fix the code so the program prints the expected message correctly.

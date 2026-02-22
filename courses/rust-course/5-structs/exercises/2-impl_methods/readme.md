# Struct Methods with `impl`

## Concept first: behavior on data types

Use `impl` blocks to define methods for structs. Methods receive `self`, `&self`, or `&mut self`.

## Exercise task

1. Define a struct `Counter` with field `value: i32`.
2. Add an `impl` method `increment(&mut self)` that increases `value` by 1.
3. In `main`, create a mutable `Counter` with `value: 0`, call `increment()` twice, and print `value`.

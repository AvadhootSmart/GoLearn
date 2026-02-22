# Propagating Errors with `?`

## Concept first: early return on failure

The `?` operator returns early when a `Result` is `Err`, reducing nested matches.

## Exercise task

1. Write `parse_port(input: &str) -> Result<u16, std::num::ParseIntError>`.
2. Parse the input with `input.parse::<u16>()?`.
3. In `main`, call `parse_port("8080")` and print `port <value>` on success.

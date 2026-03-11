# Exercise 3: Error Propagation in Rust

## Introduction

Error propagation is the process of passing errors up the call stack to a caller that can handle them. Rust provides the `?` operator for elegant and concise error propagation, making it easy to write clean error-handling code without verbose boilerplate.

## The Problem Without ?

Consider reading and parsing a file without the `?` operator:

```rust
fn read_config() -> Result<Config, io::Error> {
    let file_result = File::open("config.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    
    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);
    match read_result {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    
    let config_result = parse_config(&contents);
    match config_result {
        Ok(c) => Ok(c),
        Err(e) => Err(e),
    }
}
```

This is verbose and repetitive. Each operation that can fail requires a match statement and early return.

## The ? Operator

The `?` operator simplifies error propagation:

```rust
fn read_config() -> Result<Config, io::Error> {
    let mut file = File::open("config.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = parse_config(&contents)?;
    Ok(config)
}
```

## How ? Works

The `?` operator does two things:

1. **On Ok/Some**: Extracts the value and continues execution
2. **On Err/None**: Returns early from the function with the error

### With Result

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn calculate() -> Result<i32, String> {
    let x = divide(10, 2)?;  // x = 5
    let y = divide(x, 0)?;   // Returns Err early!
    Ok(y + 1)                // Never reached
}
```

### With Option

```rust
fn get_first(items: &Vec<i32>) -> Option<&i32> {
    items.first()
}

fn process() -> Option<i32> {
    let first = get_first(&vec![1, 2, 3])?;  // first = &1
    let doubled = first * 2;
    Some(doubled)
}
```

## Early Return Behavior

The `?` operator causes early return on error:

```rust
fn step1() -> Result<i32, Error> { ... }
fn step2(n: i32) -> Result<String, Error> { ... }
fn step3(s: &str) -> Result<bool, Error> { ... }

fn process() -> Result<bool, Error> {
    let n = step1()?;    // If Err, returns Err immediately
    let s = step2(n)?;   // If Err, returns Err immediately
    let b = step3(&s)?;  // If Err, returns Err immediately
    Ok(b)
}
```

## Return Type Requirements

Functions using `?` must return:

1. **Result** if using `?` on Result
2. **Option** if using `?` on Option
3. **Any type implementing Try** (advanced)

```rust
// This compiles - Result with ?
fn use_result() -> Result<i32, String> {
    let n: i32 = "42".parse()?;
    Ok(n)
}

// This compiles - Option with ?
fn use_option() -> Option<i32> {
    let items = vec![1, 2, 3];
    let first = items.first()?;
    Some(*first)
}

// This DOES NOT compile - mismatched types
fn mixed() -> Result<i32, String> {
    let items = vec![1, 2, 3];
    // let first = items.first()?;  // Error: Option in Result function
    Ok(1)
}
```

## Converting Between Option and Result

Use `?` across types by converting:

```rust
fn find_user(id: u64) -> Option<User> { ... }
fn load_config(user: &User) -> Result<Config, io::Error> { ... }

fn get_user_config(id: u64) -> Result<Config, String> {
    let user = find_user(id)
        .ok_or_else(|| format!("User {} not found", id))?;
    
    let config = load_config(&user)
        .map_err(|e| format!("Config error: {}", e))?;
    
    Ok(config)
}
```

## The Try Trait (Advanced)

The `?` operator is powered by the `Try` trait (still unstable for custom implementations). Built-in implementations exist for:

- `Result<T, E>`
- `Option<T>`
- `Poll<Option<T>>` (for async)

## Chaining Multiple Operations

The `?` operator shines when chaining fallible operations:

```rust
fn process_sms(api: &SmsApi, to: &str, template_id: u64) -> Result<MessageId, SmsError> {
    // All these can fail, ? propagates errors automatically
    let template = api.get_template(template_id)?;
    let contact = api.find_contact(to)?;
    let message = template.render(&contact)?;
    let validated = api.validate_message(&message)?;
    let id = api.send(&contact.phone, &validated)?;
    Ok(id)
}
```

## Adding Context with map_err

When propagating errors, add context for better debugging:

```rust
fn load_user_config(path: &str) -> Result<Config, AppError> {
    let contents = fs::read_to_string(path)
        .map_err(|e| AppError::Io(path.to_string(), e))?;
    
    let config: Config = toml::from_str(&contents)
        .map_err(|e| AppError::Parse(path.to_string(), e))?;
    
    Ok(config)
}
```

## Textio SMS API Example

```rust
fn send_campaign(
    db: &Database,
    template_name: &str,
    contact_name: &str,
) -> Result<MessageId, CampaignError> {
    // Each step can fail, ? propagates with context
    let template = db.get_template(template_name)
        .ok_or(CampaignError::TemplateNotFound(template_name.to_string()))?;
    
    let contact = db.find_contact(contact_name)
        .ok_or(CampaignError::ContactNotFound(contact_name.to_string()))?;
    
    let message = template.render(&contact)
        .map_err(|e| CampaignError::RenderError(e))?;
    
    let id = send_to_carrier(&contact.phone, &message)
        .map_err(|e| CampaignError::CarrierError(e))?;
    
    Ok(id)
}
```

## ? vs unwrap() vs match

| Approach | Best For |
|----------|----------|
| `?` | Propagating errors to caller |
| `match` | Complex error handling logic |
| `unwrap()` | Prototypes, tests, guaranteed success |
| `expect()` | Prototypes with custom panic message |

## Common Patterns

### Function Falls Through

```rust
fn operation() -> Result<Data, Error> {
    step1()?;  // Early return on error
    step2()?;  // Early return on error
    step3()?;  // Early return on error
    final_step()  // Return the final result
}
```

### Conditional Early Return

```rust
fn process(input: &str) -> Result<Output, Error> {
    if input.is_empty() {
        return Err(Error::EmptyInput);
    }
    
    let parsed = parse(input)?;
    let processed = transform(parsed)?;
    Ok(processed)
}
```

### Main Function

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml")?;
    let data = read_data(&config.input_file)?;
    let result = process_data(&data)?;
    write_output(&config.output_file, &result)?;
    Ok(())
}
```

## Best Practices

1. **Use ? for propagation**, not for handling
2. **Add context** with `map_err()` when crossing module boundaries
3. **Convert Option to Result** with `ok_or()` before using `?`
4. **Keep functions focused** - each function handles one level of abstraction
5. **Return Result from main** for command-line tools

## Exercise Task

In this exercise, you'll implement an SMS campaign system for Textio:

1. `parse_phone_number` - Parse phone with error propagation
2. `validate_message` - Validate message content
3. `load_template` - Load and parse message templates
4. `send_campaign_message` - Chain all operations with `?`
5. `process_batch` - Process multiple messages

You'll practice:
- Using `?` for early returns
- Converting Option to Result with `ok_or()`
- Adding context with `map_err()`
- Chaining fallible operations

Run your code with:
```bash
rustc code.rs && ./code
```

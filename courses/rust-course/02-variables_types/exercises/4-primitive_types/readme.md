# Exercise 4: Primitive Types

## Why This Concept Exists

Rust provides a rich set of primitive types that form the foundation of all data manipulation. Understanding these types is essential because:

1. **Memory Efficiency**: Choose the right size for your data (u8 vs u64)
2. **Performance**: Smaller types can be faster and use less memory
3. **Safety**: Overflow and underflow are caught at runtime in debug mode
4. **Domain Modeling**: Different types model different real-world data

For Textio, primitive types store everything: message IDs (u64), character counts (usize), prices (f64), flags (bool), and message content (char, &str).

---

## Integer Types

Rust provides signed and unsigned integers in various sizes:

### Signed Integers (can be negative)

| Type | Min | Max | Bytes |
|------|-----|-----|-------|
| i8 | -128 | 127 | 1 |
| i16 | -32,768 | 32,767 | 2 |
| i32 | -2.1 billion | 2.1 billion | 4 |
| i64 | -9.2 quintillion | 9.2 quintillion | 8 |
| i128 | -1.7 × 10^38 | 1.7 × 10^38 | 16 |
| isize | Platform-dependent | Platform-dependent | 4 or 8 |

### Unsigned Integers (positive only)

| Type | Min | Max | Bytes |
|------|-----|-----|-------|
| u8 | 0 | 255 | 1 |
| u16 | 0 | 65,535 | 2 |
| u32 | 0 | 4.3 billion | 4 |
| u64 | 0 | 18.4 quintillion | 8 |
| u128 | 0 | 3.4 × 10^38 | 16 |
| usize | Platform-dependent | Platform-dependent | 4 or 8 |

```rust
fn main() {
    // Signed integers
    let small: i8 = -10;
    let medium: i16 = -1000;
    let standard: i32 = -100000;
    let large: i64 = -10000000000;
    let huge: i128 = -10000000000000000000;
    
    // Unsigned integers
    let byte: u8 = 255;
    let word: u16 = 60000;
    let dword: u32 = 4000000000;
    let qword: u64 = 18000000000000000000;
    
    // Architecture-dependent
    let pointer_sized: isize = -100;
    let unsigned_pointer: usize = 100;
}
```

---

## Floating-Point Types

Rust has two floating-point types following IEEE 754 standard:

```rust
fn main() {
    // f32: 32-bit float (single precision)
    let single: f32 = 3.14159;
    
    // f64: 64-bit float (double precision) - DEFAULT
    let double: f64 = 3.141592653589793;
    
    // f64 is the default for floating-point literals
    let inferred = 3.14;  // f64
    
    // Scientific notation
    let scientific = 2.5e10;  // 2.5 × 10^10
    let tiny = 1e-10;        // 0.0000000001
}
```

### Float Precision Warning

```rust
fn main() {
    // Floating-point precision issues
    let x: f32 = 0.1 + 0.2;
    println!("f32: {}", x);  // May not be exactly 0.3
    
    let y: f64 = 0.1 + 0.2;
    println!("f64: {}", y);  // More precise, but still has issues
    
    // Never compare floats for exact equality!
    // if x == 0.3 { }  // DANGEROUS
}
```

---

## Boolean Type

```rust
fn main() {
    let is_active: bool = true;
    let is_deleted: bool = false;
    
    // Boolean operations
    let and_result = true && false;   // false
    let or_result = true || false;    // true
    let not_result = !true;           // false
    
    // Comparison produces booleans
    let is_greater = 5 > 3;           // true
    let is_equal = "hello" == "hello"; // true
    
    // Used in control flow
    if is_active {
        println!("Active!");
    }
}
```

---

## Character Type

Rust's `char` is a Unicode scalar value (4 bytes):

```rust
fn main() {
    // Basic ASCII
    let letter: char = 'A';
    let digit: char = '7';
    let symbol: char = '@';
    
    // Unicode characters
    let emoji: char = '😀';
    let chinese: char = '中';
    let greek: char = 'α';
    let math: char = '∑';
    
    // Escape sequences
    let newline: char = '\n';
    let tab: char = '\t';
    let quote: char = '\'';
    let backslash: char = '\\';
    
    // Unicode escape
    let heart: char = '\u{2764}';  // ❤
}
```

---

## String Slice Type (&str)

```rust
fn main() {
    // String literals are &str
    let message: &str = "Hello, Textio!";
    
    // &str is a "fat pointer" (pointer + length)
    // It doesn't own the data, just references it
    
    // String operations
    let length = message.len();
    let is_empty = message.is_empty();
    let contains = message.contains("Textio");
    
    // Slicing (be careful with Unicode!)
    let first_five = &message[0..5];  // "Hello"
    
    // &str vs String (covered in later modules)
    // &str: borrowed, fixed-size
    // String: owned, growable
}
```

---

## Type Inference and Explicit Types

```rust
fn main() {
    // Inference - Rust figures out the type
    let number = 42;        // i32 (default integer)
    let decimal = 3.14;     // f64 (default float)
    let text = "hello";     // &str
    let flag = true;        // bool
    
    // Explicit types when needed
    let small: u8 = 255;
    let precise: f32 = 3.14;
    let big_id: u64 = 12345678901234;
    
    // Type suffix
    let inferred_u8 = 255u8;
    let inferred_i64 = 100i64;
    let inferred_f32 = 3.14f32;
}
```

---

## Type Casting with `as`

Convert between types using the `as` keyword:

```rust
fn main() {
    // Integer to integer
    let small: i8 = 100;
    let large: i32 = small as i32;  // 100
    
    // Larger to smaller (truncation)
    let big: i32 = 300;
    let tiny: i8 = big as i8;       // 44 (truncated!)
    
    // Float to integer (truncates decimal)
    let float_val: f64 = 3.99;
    let int_val: i32 = float_val as i32;  // 3
    
    // Integer to float
    let int_num: i32 = 42;
    let float_num: f64 = int_num as f64;  // 42.0
    
    // Character to integer
    let char_val: char = 'A';
    let char_code: u32 = char_val as u32;  // 65
    
    // Integer to character
    let code: u32 = 66;
    let char_from_code: char = code as char;  // 'B'
}
```

---

## Numeric Overflow

In debug mode, overflow causes a panic. In release mode, it wraps:

```rust
fn main() {
    // In debug mode, this PANICS
    // let mut byte: u8 = 255;
    // byte = byte + 1;  // panic: overflow
    
    // Explicit wrapping
    let wrapped = 255u8.wrapping_add(1);  // 0
    
    // Checked addition (returns Option)
    let checked = 255u8.checked_add(1);  // None
    
    // Saturating addition (stays at max)
    let saturated = 255u8.saturating_add(1);  // 255
    
    // Overflowing addition (returns (value, did_overflow))
    let (result, overflowed) = 255u8.overflowing_add(1);
    // result = 0, overflowed = true
}
```

---

## Debug Printing

```rust
fn main() {
    let number = 42;
    let text = "hello";
    let flag = true;
    let letter = 'A';
    
    // Debug format {:?}
    println!("Debug number: {:?}", number);
    println!("Debug text: {:?}", text);
    println!("Debug flag: {:?}", flag);
    println!("Debug letter: {:?}", letter);
    
    // dbg! macro - prints to stderr with location
    dbg!(number);
    dbg!(text);
    
    // dbg! returns the value
    let doubled = dbg!(number * 2);
    println!("Doubled: {}", doubled);
    
    // Multiple values
    dbg!(number, text, flag);
}
```

---

## Mental Model: Type Sizes

```
Memory Layout (bytes):

u8/i8:    [XX]                    1 byte
u16/i16:  [XX XX]                 2 bytes
u32/i32:  [XX XX XX XX]           4 bytes
u64/i64:  [XX XX XX XX XX XX XX XX] 8 bytes

f32:      [XX XX XX XX]           4 bytes (IEEE 754)
f64:      [XX XX XX XX XX XX XX XX] 8 bytes (IEEE 754)

bool:     [00 or 01]              1 byte (0 or 1)
char:     [XX XX XX XX]           4 bytes (Unicode)
&str:     [ptr][len]              16 bytes (fat pointer)
```

---

## Common Pitfalls

### Pitfall 1: Integer Overflow

```rust
fn main() {
    // WRONG - Will panic in debug mode
    // let max: u8 = 255;
    // let overflow = max + 1;  // panic!
    
    // CORRECT - Use explicit overflow handling
    let max: u8 = 255;
    let wrapped = max.wrapping_add(1);  // 0
    let saturated = max.saturating_add(1);  // 255
}
```

### Pitfall 2: Float Comparison

```rust
fn main() {
    // WRONG - Floats may not be exactly equal
    let x = 0.1 + 0.2;
    // if x == 0.3 { }  // May fail!
    
    // CORRECT - Use epsilon comparison
    let epsilon = 1e-10;
    let is_close = (x - 0.3).abs() < epsilon;
}
```

### Pitfall 3: Wrong Type for Domain

```rust
fn main() {
    // WRONG - i32 can't hold large IDs
    // let message_id: i32 = 5000000000;  // overflow!
    
    // CORRECT - Use u64 for IDs
    let message_id: u64 = 5000000000;
    
    // WRONG - f32 for money (precision issues)
    // let price: f32 = 19.99;
    
    // BETTER - Use i32 (cents) or specialized decimal type
    let price_cents: i32 = 1999;
}
```

### Pitfall 4: Truncation in Casting

```rust
fn main() {
    // WRONG - Data loss when casting
    let big: i32 = 300;
    let small: i8 = big as i8;  // 44, not 300!
    
    // CORRECT - Check before casting
    if big <= i8::MAX as i32 && big >= i8::MIN as i32 {
        let small: i8 = big as i8;
    }
    
    // Or use try_into (covered later)
}
```

---

## Under the Hood

### Memory Representation

```rust
fn main() {
    // Integers stored in two's complement (signed) or binary (unsigned)
    let x: i8 = -1;      // Binary: 11111111
    let y: u8 = 255;     // Binary: 11111111 (same bits!)
    
    // Floats stored in IEEE 754 format
    let f: f32 = 1.0;    // Sign: 0, Exponent: 127, Mantissa: 0
    
    // bool is a full byte (not a bit)
    let b: bool = true;  // Stored as 0x01
}
```

### Type Size Constants

```rust
fn main() {
    // Each type has MIN and MAX constants
    println!("i8: {} to {}", i8::MIN, i8::MAX);
    println!("u8: {} to {}", u8::MIN, u8::MAX);
    println!("i32: {} to {}", i32::MIN, i32::MAX);
    println!("u64: {} to {}", u64::MIN, u64::MAX);
    
    // Check size
    println!("bool size: {} bytes", std::mem::size_of::<bool>());
    println!("char size: {} bytes", std::mem::size_of::<char>());
}
```

---

## Real-World Textio Examples

### Message ID Storage

```rust
fn main() {
    // Message IDs can be very large
    let message_id: u64 = 18446744073709551615;
    let batch_id: u32 = 4294967295;
    
    println!("Message ID: {}", message_id);
    println!("Batch ID: {}", batch_id);
}
```

### Character Counting

```rust
fn main() {
    // Use usize for counts (matches platform pointer size)
    let message = "Hello, Textio!";
    let char_count: usize = message.len();
    
    println!("Message length: {} bytes", char_count);
}
```

### Pricing

```rust
fn main() {
    // Use integers for money to avoid float issues
    let cost_per_sms_cents: u32 = 5;  // $0.05
    let monthly_budget_cents: u64 = 100000;  // $1000.00
    
    // Or use f64 for approximate calculations
    let cost_per_sms: f64 = 0.05;
    let total_cost: f64 = 100.0 * cost_per_sms;
    
    println!("Cost per SMS: ${:.2}", cost_per_sms);
    println!("Total for 100: ${:.2}", total_cost);
}
```

### Status Flags

```rust
fn main() {
    let is_delivered: bool = true;
    let is_read: bool = false;
    let is_premium_user: bool = true;
    
    // Combine flags
    let should_notify = is_delivered && !is_read;
    let has_priority = is_premium_user && is_delivered;
    
    println!("Should notify: {}", should_notify);
    println!("Has priority: {}", has_priority);
}
```

### Unicode Messages

```rust
fn main() {
    // Support international characters
    let greeting = "Hello 你好 مرحبا";
    
    // Each char is a Unicode scalar value
    for c in greeting.chars() {
        println!("Character: '{}', Code: {}", c, c as u32);
    }
}
```

---

## Exercise Task

Create a program demonstrating all primitive types in Textio context:

1. Create variables for each integer type (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize)
2. Create f32 and f64 variables
3. Create bool and char variables
4. Create &str variables
5. Demonstrate type casting with `as`
6. Show overflow handling methods
7. Use debug printing with {:?} and dbg!()

**Starter Code**: See `code.rs`
**Solution**: See `complete.rs`
**Expected Output**: See `expected.txt`

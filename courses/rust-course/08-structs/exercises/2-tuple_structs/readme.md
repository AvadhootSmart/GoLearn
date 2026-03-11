# Tuple Structs

Tuple structs are structs with unnamed fields accessed by position. They combine the naming benefits of structs with the simplicity of tuples.

## Syntax

```rust
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0, 0.0);
```

Key differences from named structs:
- Fields have no names, only positions
- Accessed via `.0`, `.1`, `.2`, etc.
- Syntax looks like a tuple but with a type name

## When to Use Tuple Structs

### 1. When Field Names Would Be Redundant

```rust
// Named struct - fields have same meaning
struct Coordinate3D {
    x: f64,
    y: f64,
    z: f64,
}

// Tuple struct - cleaner when fields are uniform
struct Point3D(f64, f64, f64);
```

### 2. Wrapping a Single Type (Newtype Pattern)

The newtype pattern wraps a type to create a distinct new type:

```rust
struct Millimeters(u32);
struct Meters(u32);

let length_mm = Millimeters(1000);
let length_m = Meters(1);
```

Even though both wrap `u32`, they are different types:

```rust
// This won't compile!
// let total = length_mm + length_m;  // Error: different types
```

### 3. Type Safety Without Runtime Overhead

```rust
struct PhoneNumber(String);
struct AccountId(String);
struct ApiKey(String);

fn send_sms(to: PhoneNumber, from: PhoneNumber, account: AccountId) {
    // Can't accidentally mix up parameters
}

let phone = PhoneNumber(String::from("+15550001"));
let account = AccountId(String::from("acc_12345"));

// Compiler ensures correct types
send_sms(phone.clone(), PhoneNumber(String::from("+15550002")), account);
```

## Creating Tuple Structs

```rust
struct Rgb(u8, u8, u8);

// Create instance - like function call
let red = Rgb(255, 0, 0);
let green = Rgb(0, 255, 0);

// With type annotation
let blue: Rgb = Rgb(0, 0, 255);
```

## Accessing Fields

```rust
struct Dimensions(u32, u32, u32);  // width, height, depth

let box_size = Dimensions(10, 20, 30);

println!("Width: {}", box_size.0);
println!("Height: {}", box_size.1);
println!("Depth: {}", box_size.2);
```

## Destructuring Tuple Structs

```rust
struct Point(i32, i32);

let p = Point(10, 20);

// Destructure into variables
let Point(x, y) = p;
println!("x = {}, y = {}", x, y);

// Or with underscores for unused fields
let Point(first, _) = p;
println!("First coordinate: {}", first);
```

## Mutable Tuple Structs

```rust
let mut position = Point(0, 0);

position.0 += 10;
position.1 += 20;

println!("New position: ({}, {})", position.0, position.1);
```

## The Newtype Pattern in Depth

The newtype pattern provides type safety and encapsulation:

### Preventing Confusion

```rust
// Without newtype - easy to mix up
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
calculate_area(10.0, 20.0);  // Which is width? Which is height?

// With newtype - compiler helps
struct Width(f64);
struct Height(f64);

fn calculate_area_safe(width: Width, height: Height) -> f64 {
    width.0 * height.0
}
calculate_area_safe(Width(10.0), Height(20.0));  // Clear!
```

### Adding Validation

```rust
struct PositiveInt(u32);

impl PositiveInt {
    fn new(value: u32) -> Option<Self> {
        if value > 0 {
            Some(PositiveInt(value))
        } else {
            None
        }
    }
    
    fn value(&self) -> u32 {
        self.0
    }
}

let valid = PositiveInt::new(42).unwrap();
// let invalid = PositiveInt::new(0).unwrap();  // Would panic
```

### Adding Domain Logic

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl Celsius {
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

let temp = Celsius(0.0);
let f = temp.to_fahrenheit();
println!("{}°C = {}°F", temp.0, f.0);
```

## Unit-Like Structs

Structs with no fields:

```rust
struct AlwaysEqual;
struct Unit;

let unit = Unit;
let equal = AlwaysEqual;
```

Use cases:
- Implementing traits without data
- Marker types (like `PhantomData` patterns)
- State machine states

```rust
struct Connected;
struct Disconnected;

enum ConnectionState {
    Connected(Connected),
    Disconnected(Disconnected),
}
```

## Comparing Struct Types

| Feature | Named Struct | Tuple Struct | Unit Struct |
|---------|--------------|--------------|-------------|
| Fields | Named | Positional | None |
| Access | `.field` | `.0`, `.1` | N/A |
| Best for | Different meanings | Same type fields | No data needed |
| Example | `User { name, age }` | `Point(x, y)` | `AlwaysEqual` |

## Visibility

```rust
pub struct PublicWrapper(pub i32, i32);
//                     ^^^^^^    ^^^^^
//                     field 0   field 1
//                     public    private

let w = PublicWrapper(1, 2);
println!("{}", w.0);  // OK - public
// println!("{}", w.1);  // Error - private
```

## Debug with Tuple Structs

```rust
#[derive(Debug)]
struct PhoneNumber(String);

let number = PhoneNumber(String::from("+15550001"));
println!("{:?}", number);  // PhoneNumber("+15550001")
```

## Memory Layout

Tuple structs have the same memory layout as tuples:

```rust
struct Point(f64, f64);  // 16 bytes
struct Coordinate(f64, f64, f64);  // 24 bytes
```

No runtime overhead compared to raw tuples.

## Textio Example

In Textio's SMS system, we use tuple structs for:

```rust
// Phone numbers - can't mix up with other strings
struct PhoneNumber(String);

// Account identifiers - distinct from user IDs
struct AccountId(String);

// Message IDs for tracking
struct MessageId(u64);

// Priority levels
struct Priority(u8);  // 1-5
```

## Exercise: Textio Type Safety

In this exercise, you'll create tuple structs for Textio's type-safe identifiers and measurements. You'll see how the newtype pattern prevents common bugs.

### Tasks

1. Define tuple structs for PhoneNumber, AccountId, and MessageId
2. Use the newtype pattern for type safety
3. Create instances and access fields
4. Destructure tuple structs
5. Add Debug derivation for printing

# Custom Iterators

## Introduction

While Rust provides iterators for all standard collections, creating your own iterators is a powerful skill. Custom iterators allow you to define iteration behavior for your own types, enabling them to work seamlessly with iterator adapters and the `for` loop syntax.

## The Iterator Trait

To create a custom iterator, implement the `Iterator` trait:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The `Item` associated type defines what the iterator produces. The `next()` method returns `Some(item)` when there's another element, or `None` when exhausted.

## Basic Custom Iterator

Here's a simple counter iterator:

```rust
struct Counter {
    count: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Usage
for i in Counter::new(5) {
    println!("{}", i);  // Prints 1, 2, 3, 4, 5
}
```

## Iterator State Management

The iterator struct holds the state needed to track iteration progress:

```rust
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// Note: This iterator is infinite!
let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
```

## IntoIterator Trait

The `IntoIterator` trait allows a type to be converted into an iterator:

```rust
pub trait IntoIterator where Self::IntoIter::Item == Self::Item {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

This enables `for` loops to work directly with your type:

```rust
struct SimpleCollection {
    items: Vec<i32>,
}

impl IntoIterator for SimpleCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// Now works with for loop
let collection = SimpleCollection { items: vec![1, 2, 3] };
for item in collection {
    println!("{}", item);
}
```

## Iterating Over Custom Types

For a custom collection, you often want multiple iteration modes:

```rust
struct MessageQueue {
    messages: Vec<Message>,
}

// IntoIter - takes ownership
pub struct IntoIter {
    messages: std::vec::IntoIter<Message>,
}

impl Iterator for IntoIter {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        self.messages.next()
    }
}

// Iter - borrows
pub struct Iter<'a> {
    messages: std::slice::Iter<'a, Message>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Message;
    fn next(&mut self) -> Option<Self::Item> {
        self.messages.next()
    }
}

// IterMut - mutably borrows
pub struct IterMut<'a> {
    messages: std::slice::IterMut<'a, Message>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Message;
    fn next(&mut self) -> Option<Self::Item> {
        self.messages.next()
    }
}
```

## Providing iter(), iter_mut(), into_iter() Methods

```rust
impl MessageQueue {
    pub fn iter(&self) -> Iter<'_> {
        Iter { messages: self.messages.iter() }
    }
    
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut { messages: self.messages.iter_mut() }
    }
}

impl IntoIterator for MessageQueue {
    type Item = Message;
    type IntoIter = IntoIter;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { messages: self.messages.into_iter() }
    }
}
```

## Implementing Other Iterator Traits

### DoubleEndedIterator

For iterators that can be traversed from both ends:

```rust
impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.max > self.count {
            self.max -= 1;
            Some(self.max)
        } else {
            None
        }
    }
}
```

### ExactSizeIterator

When you know the exact number of elements:

```rust
impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max - self.count
    }
}
```

## The size_hint Method

Providing a size hint enables optimizations:

```rust
impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.max - self.count;
        (remaining, Some(remaining))
    }
}
```

## Common Patterns

### Wrapping Iterator

```rust
struct Cycle<T> {
    original: Vec<T>,
    index: usize,
}

impl<T: Clone> Iterator for Cycle<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.original.is_empty() {
            return None;
        }
        let item = self.original[self.index].clone();
        self.index = (self.index + 1) % self.original.len();
        Some(item)
    }
}
```

### Filtering Iterator

```rust
struct FilterIter<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Iterator for FilterIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
}
```

### Transforming Iterator

```rust
struct MapIter<I, F> {
    iter: I,
    func: F,
}

impl<I, F, B> Iterator for MapIter<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.func)
    }
}
```

## Textio Example: Message Stream Iterator

```rust
struct MessageStream {
    messages: Vec<Message>,
    filter: Option<Box<dyn Fn(&Message) -> bool>>,
}

impl Iterator for MessageStream {
    type Item = Message;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(msg) = self.messages.pop() {
            if let Some(ref filter) = self.filter {
                if filter(&msg) {
                    return Some(msg);
                }
            } else {
                return Some(msg);
            }
        }
        None
    }
}
```

## Performance Considerations

1. **Inline small iterators** - The compiler can optimize small iterator structs
2. **Provide size_hint** - Enables `Vec::with_capacity` optimizations
3. **Use iterator adapters** - They're zero-cost abstractions
4. **Consider ownership** - Borrowing is often cheaper than cloning

## Exercise Overview

In this exercise, you will:
1. Implement the `Iterator` trait for custom types
2. Create iterators with different state management
3. Implement `IntoIterator` for collections
4. Build a paginated message iterator for Textio
5. Create a streaming iterator for message batches

## Key Takeaways

- Implement `Iterator` with `type Item` and `fn next(&mut self)`
- The iterator struct holds state between iterations
- Return `Some(item)` for values, `None` when exhausted
- Implement `IntoIterator` to enable `for` loops
- Provide `iter()`, `iter_mut()`, `into_iter()` for collections
- `size_hint` enables optimizations
- `DoubleEndedIterator` and `ExactSizeIterator` add capabilities

## Further Reading

- [Rust Book: Implementing Iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html#implementing-iterator)
- [Iterator Trait Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [IntoIterator Documentation](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)

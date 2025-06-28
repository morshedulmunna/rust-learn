# Loop.rs - Rust Loop Types and Patterns

## Overview

The `loop.rs` file demonstrates the three main types of loops in Rust (`loop`, `while`, `for`) along with advanced patterns and iterator methods.

## Code Structure

The file contains multiple functions demonstrating different loop concepts:

1. `loop_example()` - Basic infinite loop
2. `while_example()` - While loop
3. `for_example()` - For loop with arrays
4. `for_range_example()` - For loop with ranges
5. Advanced patterns with various iterator methods

## Key Concepts

### 1. Infinite Loop (`loop`)

```rust
fn loop_example() {
    loop {
        println!("again!");
    }
}
```

**Characteristics:**

- Runs indefinitely until explicitly broken
- Use `break` to exit the loop
- Can return values using `break value`
- Useful for event loops and game loops

### 2. While Loop

```rust
fn while_example() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

**Characteristics:**

- Continues while condition is true
- Condition is checked before each iteration
- Useful when you don't know the number of iterations
- Can become infinite if condition never becomes false

### 3. For Loop

```rust
fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}
```

**Characteristics:**

- Iterates over collections
- Safer than manual indexing
- Prevents out-of-bounds errors
- Most common loop type in Rust

### 4. Range-based For Loop

```rust
fn for_range_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

**Characteristics:**

- Uses range syntax (`start..end`, `start..=end`)
- Can use range methods like `.rev()`, `.step_by()`
- Inclusive ranges use `..=`
- Exclusive ranges use `..`

## Advanced Loop Patterns

### 1. Loop with Break Value

```rust
fn loop_with_break_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Result: {}", result);
}
```

**Key Points:**

- `break` can return a value
- The value becomes the result of the loop expression
- Useful for finding values or accumulating results

### 2. Loop Labels

```rust
fn loop_with_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // Break outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}
```

**Key Points:**

- Labels start with `'` (single quote)
- Use `break 'label` to break specific loops
- Useful for nested loops
- Helps avoid confusion in complex nested structures

### 3. While Let Pattern

```rust
fn while_let_example() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

**Key Points:**

- Combines `while` with pattern matching
- Continues while pattern matches
- Useful for processing `Option` or `Result` types
- Cleaner than manual `match` in loops

## Iterator Methods

### 1. Enumerate

```rust
fn for_with_enumerate() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

### 2. References and Mutability

```rust
fn for_with_reference() {
    let v = vec![100, 32, 57];
    for i in &v {
        // Using reference to avoid moving
        println!("{}", i);
    }
    println!("Vector is still available: {:?}", v);
}

fn for_with_mut_reference() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // Mutable reference to modify values
        *i += 50;
    }
    println!("Modified vector: {:?}", v);
}
```

### 3. Range Methods

```rust
fn for_with_range_inclusive() {
    for number in 1..=5 {
        // Inclusive range (includes 5)
        println!("{}", number);
    }
}

fn for_with_step_by() {
    for number in (0..10).step_by(2) {
        // Step by 2
        println!("{}", number);
    }
}
```

### 4. Filtering and Mapping

```rust
fn for_with_filter() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter().filter(|&&x| x % 2 == 0) {
        println!("Even number: {}", number);
    }
}

fn for_with_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    for doubled in numbers.iter().map(|x| x * 2) {
        println!("Doubled: {}", doubled);
    }
}
```

### 5. Combining Iterators

```rust
fn for_with_zip() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
}

fn for_with_chain() {
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];

    for number in first.iter().chain(second.iter()) {
        println!("{}", number);
    }
}
```

### 6. Taking and Skipping

```rust
fn for_with_take() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter().take(3) {
        // Only take first 3
        println!("{}", number);
    }
}

fn for_with_skip() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter().skip(3) {
        // Skip first 3
        println!("{}", number);
    }
}
```

### 7. Advanced Iterator Methods

```rust
fn for_with_rev() {
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter().rev() {
        // Reverse iteration
        println!("{}", number);
    }
}

fn for_with_cycle() {
    let colors = vec!["red", "green", "blue"];
    for (i, color) in colors.iter().cycle().take(7).enumerate() {
        println!("Item {}: {}", i, color);
    }
}

fn for_with_windows() {
    let numbers = vec![1, 2, 3, 4, 5];
    for window in numbers.windows(3) {
        // Sliding window of size 3
        println!("Window: {:?}", window);
    }
}

fn for_with_chunks() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for chunk in numbers.chunks(3) {
        // Split into chunks of size 3
        println!("Chunk: {:?}", chunk);
    }
}
```

## Best Practices

### 1. Choose the Right Loop Type

```rust
// Use for when iterating over collections
for item in collection {
    // Process item
}

// Use while when condition-based
while condition {
    // Process until condition is false
}

// Use loop when you need control over exit
loop {
    if should_exit {
        break;
    }
    // Process
}
```

### 2. Avoid Manual Indexing

```rust
// Bad: Manual indexing
for i in 0..array.len() {
    println!("{}", array[i]);
}

// Good: Direct iteration
for item in array {
    println!("{}", item);
}

// Good: When you need index
for (i, item) in array.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

### 3. Use Iterator Methods

```rust
// Bad: Manual filtering
for item in collection {
    if condition(item) {
        println!("{}", item);
    }
}

// Good: Use filter
for item in collection.iter().filter(|item| condition(item)) {
    println!("{}", item);
}
```

## Common Patterns

### 1. Processing Collections

```rust
let numbers = vec![1, 2, 3, 4, 5];
let sum: i32 = numbers.iter().sum();
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

### 2. Finding Elements

```rust
let numbers = vec![1, 2, 3, 4, 5];
if let Some(found) = numbers.iter().find(|&&x| x > 3) {
    println!("Found: {}", found);
}
```

### 3. Conditional Loops

```rust
let mut data = vec![1, 2, 3, 4, 5];
while let Some(item) = data.pop() {
    if item > 3 {
        break;
    }
    println!("Processing: {}", item);
}
```

## Common Mistakes

### ❌ Infinite loops without exit condition

```rust
// Bad: No way to exit
loop {
    println!("This runs forever!");
}
```

### ❌ Modifying collection while iterating

```rust
// Bad: Can cause issues
let mut numbers = vec![1, 2, 3, 4, 5];
for number in &numbers {
    numbers.push(number * 2); // This can cause problems
}
```

### ❌ Using wrong loop type

```rust
// Bad: Using while for collection iteration
let array = [1, 2, 3, 4, 5];
let mut i = 0;
while i < array.len() {
    println!("{}", array[i]);
    i += 1;
}

// Good: Use for loop
for item in &array {
    println!("{}", item);
}
```

### ✅ Correct patterns

```rust
// Good: Proper exit condition
let mut counter = 0;
loop {
    counter += 1;
    if counter >= 10 {
        break;
    }
}

// Good: Safe collection modification
let mut numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// Good: Appropriate loop type
for item in collection {
    println!("{}", item);
}
```

## Performance Considerations

### 1. Iterator Chaining

```rust
// Efficient: Chain operations
let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

### 2. Avoiding Unnecessary Allocations

```rust
// Good: Iterate without collecting
for item in numbers.iter().filter(|&&x| x > 0) {
    println!("{}", item);
}
```

## Exercises

1. **Number Counter**: Create a loop that counts from 1 to 10 and prints each number
2. **Sum Calculator**: Use a loop to calculate the sum of numbers in a vector
3. **Pattern Printer**: Print patterns using nested loops (triangles, squares)
4. **Data Processor**: Process a collection using various iterator methods
5. **Game Loop**: Create a simple game loop with user input and exit conditions

## Related Concepts

- **Iterators**: Understanding Rust's iterator trait
- **Collections**: Working with vectors, arrays, and other collections
- **Pattern Matching**: Using patterns in loops
- **Ownership**: Understanding how loops interact with ownership

# Vectors.rs - Dynamic Arrays in Rust

## Overview

The `vectors.rs` file demonstrates comprehensive vector operations in Rust including creation, modification, iteration, and advanced usage patterns. Vectors are growable arrays that can store multiple values of the same type.

## Code Analysis

```rust
fn vectors() {
    println!("=== Vectors Learning Examples ===\n");

    // 1. Creating Vectors
    create_vectors();

    // 2. Adding and Removing Elements
    modify_vectors();

    // 3. Accessing Vector Elements
    access_elements();

    // 4. Iterating Over Vectors
    iterate_vectors();

    // 5. Vector Methods
    vector_methods();

    // 6. Vector with User Input
    vector_with_input();

    // 7. Vector of Different Types (using enums)
    vector_of_different_types();
}
```

## Key Concepts

### 1. Creating Vectors

```rust
// Empty vector with type annotation
let mut v1: Vec<i32> = Vec::new();

// Vector with initial values using vec! macro
let v2 = vec![1, 2, 3, 4, 5];

// Vector with repeated values
let v3 = vec![0; 5];  // Creates [0, 0, 0, 0, 0]

// Vector with capacity
let mut v4 = Vec::with_capacity(10);
```

**Key Points:**

- `Vec::new()` creates an empty vector
- `vec!` macro creates a vector with initial values
- `vec![value; count]` creates a vector with repeated values
- `Vec::with_capacity(n)` pre-allocates memory for efficiency

### 2. Adding and Removing Elements

```rust
let mut v = Vec::new();

// Adding elements
v.push(1);
v.push(2);
v.push(3);

// Removing elements
let last = v.pop();

// Inserting at specific index
v.insert(1, 10);

// Removing at specific index
v.remove(0);

// Clearing the vector
v.clear();
```

**Key Points:**

- `push()` adds element to the end
- `pop()` removes and returns the last element
- `insert(index, value)` inserts at specific position
- `remove(index)` removes element at specific position
- `clear()` removes all elements

### 3. Accessing Vector Elements

```rust
let v = vec![10, 20, 30, 40, 50];

// Using indexing (panics if out of bounds)
println!("First element: {}", v[0]);

// Using get() method (returns Option)
match v.get(2) {
    Some(value) => println!("Element at index 2: {}", value),
    None => println!("Index 2 is out of bounds"),
}

// Using if let for cleaner code
if let Some(value) = v.get(1) {
    println!("Element at index 1: {}", value);
}

// Last element
if let Some(last) = v.last() {
    println!("Last element: {}", last);
}
```

**Key Points:**

- Indexing with `v[index]` panics if out of bounds
- `get(index)` returns `Option<T>` for safe access
- `last()` returns the last element as `Option<T>`
- Always use `get()` when index might be invalid

### 4. Iterating Over Vectors

```rust
let v = vec![1, 2, 3, 4, 5];

// Iterating with for loop (immutable)
for value in &v {
    print!("{} ", value);
}

// Iterating with indices
for (index, value) in v.iter().enumerate() {
    print!("[{}:{}] ", index, value);
}

// Iterating with mutable reference
let mut v2 = vec![1, 2, 3, 4, 5];
for value in &mut v2 {
    *value *= 2;  // Double each value
}

// Using iterators
let sum: i32 = v2.iter().sum();
let doubled: Vec<i32> = v2.iter().map(|x| x * 2).collect();
```

**Key Points:**

- `for value in &v` iterates immutably
- `for value in &mut v` iterates mutably
- `enumerate()` provides index-value pairs
- Iterator methods like `sum()`, `map()`, `filter()` are powerful

### 5. Vector Methods

```rust
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

// Length and capacity
println!("Length: {}, Capacity: {}", v.len(), v.capacity());

// Check if empty
println!("Is empty: {}", v.is_empty());

// Sort
v.sort();

// Reverse
v.reverse();

// Deduplicate
v.sort();
v.dedup();

// Contains
println!("Contains 5: {}", v.contains(&5));

// Find
if let Some(index) = v.iter().position(|&x| x == 5) {
    println!("5 found at index: {}", index);
}

// Filter
let even_numbers: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).cloned().collect();
```

**Key Points:**

- `len()` returns current length
- `capacity()` returns allocated capacity
- `sort()` sorts in ascending order
- `dedup()` removes consecutive duplicates
- `contains()` checks for element presence
- `position()` finds element index

### 6. Vector with User Input

```rust
let mut numbers = Vec::new();

println!("Enter numbers (type 'done' to finish):");

loop {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if input == "done" {
        break;
    }

    match input.parse::<i32>() {
        Ok(num) => {
            numbers.push(num);
            println!("Added: {}", num);
        }
        Err(_) => {
            println!("Invalid number, please try again");
        }
    }
}

if !numbers.is_empty() {
    println!("Your numbers: {:?}", numbers);
    println!("Sum: {}", numbers.iter().sum::<i32>());
    println!("Average: {:.2}", numbers.iter().sum::<i32>() as f64 / numbers.len() as f64);
}
```

**Key Points:**

- Use `loop` for continuous input
- `parse()` converts string to number
- Handle parsing errors gracefully
- Calculate statistics using iterator methods

### 7. Vector of Different Types (using enums)

```rust
#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f64),
    Text(String),
}

let mut mixed_data = Vec::new();

mixed_data.push(Value::Integer(42));
mixed_data.push(Value::Float(3.14));
mixed_data.push(Value::Text(String::from("Hello, Rust!")));

// Iterate and match
for (index, value) in mixed_data.iter().enumerate() {
    match value {
        Value::Integer(i) => println!("[{}] Integer: {}", index, i),
        Value::Float(f) => println!("[{}] Float: {:.2}", index, f),
        Value::Text(s) => println!("[{}] Text: {}", index, s),
    }
}

// Filter by type
let integers: Vec<i32> = mixed_data
    .iter()
    .filter_map(|v| {
        if let Value::Integer(i) = v {
            Some(*i)
        } else {
            None
        }
    })
    .collect();
```

**Key Points:**

- Use enums to store different types in one vector
- `match` expressions handle different enum variants
- `filter_map()` combines filtering and mapping
- Pattern matching is powerful for type-safe operations

## Common Patterns

### Vector Initialization

```rust
// Common initialization patterns
let empty: Vec<i32> = Vec::new();
let with_values = vec![1, 2, 3, 4, 5];
let with_capacity = Vec::with_capacity(100);
let repeated = vec![0; 10];
```

### Safe Element Access

```rust
// Always prefer safe access
let v = vec![1, 2, 3];

// Safe way
if let Some(value) = v.get(5) {
    println!("Value: {}", value);
} else {
    println!("Index out of bounds");
}

// Unsafe way (panics)
// let value = v[5];  // This will panic!
```

### Vector Transformation

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Transform and collect
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// Filter and collect
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

// Chain operations
let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x > 2)
    .map(|x| x * x)
    .collect();
```

### Vector as Stack

```rust
let mut stack = Vec::new();

// Push operations
stack.push(1);
stack.push(2);
stack.push(3);

// Pop operations
while let Some(value) = stack.pop() {
    println!("Popped: {}", value);
}
```

## Best Practices

1. **Use `get()` for safe element access**
2. **Pre-allocate capacity when you know the size**
3. **Use iterator methods for functional programming**
4. **Handle parsing errors when reading input**
5. **Use enums for vectors with mixed types**
6. **Clear vectors when done to free memory**

## Common Mistakes

### ❌ Unsafe indexing

```rust
let v = vec![1, 2, 3];
let value = v[10];  // This will panic!
```

### ✅ Safe indexing

```rust
let v = vec![1, 2, 3];
if let Some(value) = v.get(10) {
    println!("Value: {}", value);
} else {
    println!("Index out of bounds");
}
```

### ❌ Forgetting to handle parsing errors

```rust
let input = "abc";
let number = input.parse::<i32>().unwrap();  // This will panic!
```

### ✅ Proper error handling

```rust
let input = "abc";
match input.parse::<i32>() {
    Ok(number) => println!("Number: {}", number),
    Err(_) => println!("Invalid number"),
}
```

### ❌ Inefficient vector operations

```rust
let mut v = Vec::new();
for i in 0..1000 {
    v.push(i);  // Multiple reallocations
}
```

### ✅ Efficient vector operations

```rust
let mut v = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);  // No reallocations
}
```

## Exercises

1. **Number Statistics**: Create a program that reads numbers and calculates min, max, mean, and median
2. **Vector Operations**: Implement vector addition, subtraction, and dot product
3. **Data Processing**: Read CSV-like data and store in vectors of different types
4. **Stack Implementation**: Use a vector to implement a stack with push, pop, and peek operations
5. **Vector Sorting**: Implement custom sorting algorithms using vectors

## Related Concepts

- **Ownership**: Understanding vector ownership and borrowing
- **Iterators**: Powerful functional programming with vectors
- **Generics**: Creating generic vector functions
- **Error Handling**: Safe vector operations with Result and Option
- **Memory Management**: Understanding vector capacity and reallocation

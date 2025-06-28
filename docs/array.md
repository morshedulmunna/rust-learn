# Array.rs - Rust Arrays

## Overview

The `array.rs` file demonstrates how to work with arrays in Rust, including basic array operations, iterator methods, and common array manipulations.

## Code Analysis

```rust
fn array_example() {
    let a = [1, 2, 3, 4, 5];
    println!("a is {a:?}");
}

fn array_with_type() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a is {a:?}");
}

fn array_with_default_value() {
    let a: [i32; 5] = [3; 5];
    // Array methods and iterator operations
    a.len();
    a.is_empty();
    a.contains(&3);
    a.iter().sum();
    a.iter().product();
    a.iter().max();
    a.iter().min();
    a.iter().count();
    a.iter().nth(2);
    a.iter().position(|x| x == &3);
    a.iter().rposition(|x| x == &3);
    a.iter().rev().collect::<Vec<&i32>>();
    a.iter().skip(2).collect::<Vec<&i32>>();
    a.iter().take(2).collect::<Vec<&i32>>();
    a.iter().filter(|x| x % 2 == 0).collect::<Vec<&i32>>();
    a.iter().filter(|x| x % 2 != 0).collect::<Vec<&i32>>();
    a.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("a is {a:?}");
}

fn array_with_index() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first is {first}, second is {second}");
}
```

## Key Concepts

### 1. Basic Array Declaration

```rust
let a = [1, 2, 3, 4, 5];
```

**Characteristics:**

- Fixed size (cannot grow or shrink)
- All elements must be of the same type
- Type and size are inferred from the initial values
- Stored on the stack (not heap)
- Zero-based indexing

### 2. Array with Explicit Type Annotation

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

**Key Points:**

- `[i32; 5]` means "array of 5 i32 elements"
- Type annotation format: `[Type; Size]`
- Useful when you want to be explicit about types
- Helps with type inference in complex scenarios

### 3. Array with Default Values

```rust
let a: [i32; 5] = [3; 5];
```

**Key Points:**

- Creates an array with 5 elements, all initialized to 3
- Format: `[value; size]`
- Useful for creating arrays with repeated values
- More efficient than manually specifying each element

### 4. Array Indexing

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

**Key Points:**

- Zero-based indexing (first element is at index 0)
- Bounds checking at runtime (prevents buffer overflows)
- Panics if index is out of bounds
- Safe by default

## Array Methods and Properties

### 1. Basic Array Methods

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Get array length
let length = a.len(); // 5

// Check if array is empty
let is_empty = a.is_empty(); // false

// Check if array contains a value
let contains_three = a.contains(&3); // true
```

### 2. Iterator Methods for Aggregation

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Sum all elements
let sum: i32 = a.iter().sum(); // 15

// Product of all elements
let product: i32 = a.iter().product(); // 120

// Find maximum value
let max = a.iter().max(); // Some(5)

// Find minimum value
let min = a.iter().min(); // Some(1)

// Count elements
let count = a.iter().count(); // 5
```

### 3. Iterator Methods for Access

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Get element at specific position
let third = a.iter().nth(2); // Some(3)

// Find first occurrence of value
let position = a.iter().position(|x| x == &3); // Some(2)

// Find last occurrence of value
let rposition = a.iter().rposition(|x| x == &3); // Some(2)
```

## Iterator Transformations

### 1. Reversing and Slicing

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Reverse the array
let reversed: Vec<&i32> = a.iter().rev().collect(); // [5, 4, 3, 2, 1]

// Skip first n elements
let skipped: Vec<&i32> = a.iter().skip(2).collect(); // [3, 4, 5]

// Take first n elements
let taken: Vec<&i32> = a.iter().take(2).collect(); // [1, 2]
```

### 2. Filtering Elements

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Filter even numbers
let evens: Vec<&i32> = a.iter().filter(|x| x % 2 == 0).collect(); // [2, 4]

// Filter odd numbers
let odds: Vec<&i32> = a.iter().filter(|x| x % 2 != 0).collect(); // [1, 3, 5]
```

### 3. Mapping Elements

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Double each element
let doubled: Vec<i32> = a.iter().map(|x| x * 2).collect(); // [2, 4, 6, 8, 10]

// Square each element
let squared: Vec<i32> = a.iter().map(|x| x * x).collect(); // [1, 4, 9, 16, 25]
```

## Advanced Array Patterns

### 1. Chaining Iterator Methods

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Filter even numbers, double them, then sum
let result: i32 = a.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum(); // 12 (2*2 + 4*2)

// Take first 3 elements, reverse them
let result: Vec<&i32> = a.iter()
    .take(3)
    .rev()
    .collect(); // [3, 2, 1]
```

### 2. Conditional Operations

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Find first element greater than 3
let first_greater: Option<&i32> = a.iter().find(|x| x > &&3); // Some(4)

// Check if all elements are positive
let all_positive: bool = a.iter().all(|x| x > &0); // true

// Check if any element is even
let has_even: bool = a.iter().any(|x| x % 2 == 0); // true
```

### 3. Array Slicing

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Create slices
let slice = &a[1..4]; // [2, 3, 4]
let full_slice = &a[..]; // [1, 2, 3, 4, 5]
let start_slice = &a[..3]; // [1, 2, 3]
let end_slice = &a[2..]; // [3, 4, 5]
```

## Common Array Operations

### 1. Finding and Searching

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Find element with condition
if let Some(element) = a.iter().find(|&&x| x > 3) {
    println!("First element > 3: {}", element);
}

// Find index of element
if let Some(index) = a.iter().position(|&x| x == 3) {
    println!("3 found at index: {}", index);
}

// Find last occurrence
if let Some(index) = a.iter().rposition(|&x| x == 3) {
    println!("Last 3 found at index: {}", index);
}
```

### 2. Statistical Operations

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Calculate statistics
let sum: i32 = a.iter().sum();
let count = a.iter().count();
let average = sum as f64 / count as f64;

let max = a.iter().max().unwrap();
let min = a.iter().min().unwrap();

println!("Sum: {}, Count: {}, Average: {:.2}", sum, count, average);
println!("Max: {}, Min: {}", max, min);
```

### 3. Array Transformation

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Transform to different types
let strings: Vec<String> = a.iter().map(|x| x.to_string()).collect();
let floats: Vec<f64> = a.iter().map(|x| *x as f64).collect();

// Filter and transform
let even_doubles: Vec<i32> = a.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

## Best Practices

### 1. Choose Arrays vs Vectors

```rust
// Use arrays when:
// - Size is known at compile time
// - Size is small and fixed
// - Performance is critical
let coordinates = [x, y, z]; // 3D point

// Use vectors when:
// - Size is dynamic
// - Size is large
// - Size is unknown at compile time
let mut dynamic_list = Vec::new();
```

### 2. Use Iterator Methods Efficiently

```rust
// Good: Chain operations efficiently
let result: i32 = a.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();

// Bad: Multiple iterations
let evens: Vec<&i32> = a.iter().filter(|x| x % 2 == 0).collect();
let doubled: Vec<i32> = evens.iter().map(|x| x * 2).collect();
let sum: i32 = doubled.iter().sum();
```

### 3. Handle Optional Results

```rust
// Good: Handle None cases
match a.iter().max() {
    Some(max) => println!("Maximum: {}", max),
    None => println!("Array is empty"),
}

// Also good: Use unwrap_or for defaults
let max = a.iter().max().unwrap_or(&0);
```

### 4. Use Appropriate Types

```rust
// Good: Use specific types
let ages: [u8; 5] = [25, 30, 35, 40, 45]; // u8 is sufficient for ages

// Bad: Using generic types
let ages: [i32; 5] = [25, 30, 35, 40, 45]; // i32 is unnecessarily large
```

## Common Mistakes

### ❌ Out of bounds access

```rust
// Bad: Will panic at runtime
let a = [1, 2, 3, 4, 5];
let element = a[10]; // Panic: index out of bounds
```

### ❌ Trying to modify array size

```rust
// Bad: Arrays have fixed size
let mut a = [1, 2, 3, 4, 5];
a.push(6); // This won't compile - arrays don't have push method
```

### ❌ Mixing types in array

```rust
// Bad: All elements must be same type
let a = [1, "hello", 3.14]; // This won't compile
```

### ❌ Using wrong index type

```rust
// Bad: Index must be usize
let a = [1, 2, 3, 4, 5];
let index: i32 = 2;
let element = a[index]; // This won't compile

// Good: Use usize for indexing
let index: usize = 2;
let element = a[index];
```

### ❌ Forgetting to handle None results

```rust
// Bad: Will panic if array is empty
let a: [i32; 0] = [];
let max = a.iter().max().unwrap(); // Panic!

// Good: Handle None case
let max = a.iter().max().unwrap_or(&0);
```

### ✅ Correct patterns

```rust
// Good: Safe array access
let a = [1, 2, 3, 4, 5];
if let Some(element) = a.get(2) {
    println!("Element: {}", element);
}

// Good: Use vectors for dynamic size
let mut v = vec![1, 2, 3, 4, 5];
v.push(6); // This works

// Good: Handle optional results
let max = a.iter().max().unwrap_or(&0);
let position = a.iter().position(|x| x == &3).unwrap_or(usize::MAX);
```

## Performance Considerations

### 1. Iterator Efficiency

```rust
// Good: Single pass through array
let result: i32 = a.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();

// Bad: Multiple passes
let evens: Vec<&i32> = a.iter().filter(|x| x % 2 == 0).collect();
let doubled: Vec<i32> = evens.iter().map(|x| x * 2).collect();
let sum: i32 = doubled.iter().sum();
```

### 2. Memory Usage

```rust
// Good: Use references when possible
for element in &a {
    println!("{}", element);
}

// Bad: Copying elements unnecessarily
for element in a {
    println!("{}", element);
}
```

### 3. Bounds Checking

```rust
// Rust performs bounds checking by default
let a = [1, 2, 3, 4, 5];
let element = a[3]; // Bounds checked at runtime

// Use iterators to avoid bounds checking
for element in &a {
    // No bounds checking needed
    println!("{}", element);
}
```

## Exercises

1. **Array Reverser**: Write a function that reverses an array in place
2. **Array Rotator**: Create a function that rotates array elements by a given number of positions
3. **Array Statistics**: Calculate mean, median, and mode of an array of numbers
4. **Array Searcher**: Implement binary search on a sorted array
5. **Matrix Operations**: Create functions for matrix addition, multiplication, and transposition
6. **Array Filter**: Create a function that filters an array based on a predicate
7. **Array Mapper**: Create a function that applies a transformation to each element
8. **Array Reducer**: Implement a reduce function that combines all elements

## Related Concepts

- **Vectors**: Dynamic arrays that can grow and shrink
- **Slices**: References to portions of arrays
- **Iterators**: Processing array elements efficiently
- **Ownership**: How arrays interact with Rust's ownership system
- **Closures**: Using closures with iterator methods

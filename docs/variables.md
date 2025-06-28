# Variables.rs - Basic Variable Concepts

## Overview

The `variables.rs` file demonstrates fundamental variable concepts in Rust including mutability, shadowing, and user input handling.

## Code Analysis

```rust
fn variables() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y + 2;
        println!("y is {y}");
    }
    println!("y is {y}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is {spaces}");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("guess is {guess}");

    let mut x = String::from("hello");
    x.push_str(", world");
    println!("x is {x}");
}
```

## Key Concepts

### 1. Mutability

```rust
let mut x = 5;  // Mutable variable
x = 6;          // Can be changed
```

**Key Points:**

- Variables are immutable by default
- Use `mut` keyword to make variables mutable
- Mutable variables can be reassigned

### 2. Variable Shadowing

```rust
let y = 5;
let y = y + 1;  // Shadowing with same type
```

**Key Points:**

- Shadowing creates a new variable with the same name
- Can change the type of the variable
- Useful for transformations

### 3. Scope and Shadowing

```rust
let y = 5;
{
    let y = y + 2;  // New scope, new variable
    println!("y is {y}");
}
println!("y is {y}");  // Back to original y
```

**Key Points:**

- Inner scope can shadow outer variables
- Original variable is restored when inner scope ends
- Useful for temporary transformations

### 4. Type Changing with Shadowing

```rust
let spaces = "   ";        // String slice
let spaces = spaces.len(); // usize (number)
```

**Key Points:**

- Shadowing allows changing variable types
- Original variable is completely replaced
- No need for different variable names

### 5. User Input

```rust
let mut guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

**Key Points:**

- `String::new()` creates an empty string
- `io::stdin()` reads from standard input
- `read_line()` appends to the string
- `expect()` handles potential errors

### 6. String Manipulation

```rust
let mut x = String::from("hello");
x.push_str(", world");
```

**Key Points:**

- `String::from()` creates a new owned string
- `push_str()` appends to the string
- Strings are growable and mutable

## Common Patterns

### Variable Transformation Chain

```rust
let value = "123";
let value = value.parse::<i32>().unwrap();
let value = value * 2;
println!("Result: {}", value);
```

### Input Validation

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");
let input = input.trim();  // Remove whitespace
```

### Conditional Shadowing

```rust
let value = 5;
let value = if value > 0 { value * 2 } else { 0 };
```

## Best Practices

1. **Use meaningful variable names**
2. **Prefer immutability when possible**
3. **Use shadowing for type transformations**
4. **Handle input errors gracefully**
5. **Use appropriate string types (`&str` vs `String`)**

## Common Mistakes

### ❌ Trying to modify immutable variable

```rust
let x = 5;
x = 6;  // This will not compile
```

### ✅ Use mut for mutable variables

```rust
let mut x = 5;
x = 6;  // This works
```

### ❌ Forgetting to handle input errors

```rust
io::stdin().read_line(&mut input);  // Missing error handling
```

### ✅ Proper error handling

```rust
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
```

## Exercises

1. **Type Converter**: Create a program that reads a string and converts it to different types
2. **Input Calculator**: Read two numbers and perform basic arithmetic
3. **String Builder**: Build a string by reading multiple lines of input
4. **Variable Scope**: Demonstrate different scoping scenarios with shadowing

## Related Concepts

- **Ownership**: Understanding who owns the data
- **References**: Borrowing data without taking ownership
- **String Types**: Difference between `&str` and `String`
- **Error Handling**: Using `Result` and `Option` types

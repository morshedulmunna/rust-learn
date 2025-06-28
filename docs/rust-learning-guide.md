# Rust Learning Guide

A comprehensive guide to Rust programming fundamentals based on practical examples.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Comments](#comments)
3. [Variables and Mutability](#variables-and-mutability)
4. [Constants](#constants)
5. [Variable Shadowing](#variable-shadowing)
6. [Data Types](#data-types)
7. [Control Flow](#control-flow)
8. [Loops](#loops)
9. [Pattern Matching](#pattern-matching)
10. [Enums](#enums)
11. [User Input](#user-input)
12. [Command Line Arguments](#command-line-arguments)

---

## Getting Started

### Project Structure

```
rust-learn/
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
└── src/
    ├── main.rs         # Entry point
    ├── comments.rs     # Comment examples
    ├── variables.rs    # Basic variable examples
    ├── const_let_mut_variables.rs  # Advanced variable concepts
    ├── conditonal.rs   # Conditional statements
    ├── loop.rs         # Loop examples
    ├── match.rs        # Pattern matching
    ├── enum.rs         # Enumerations
    └── user_input.rs   # User input handling
```

### Running the Project

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run with arguments
cargo run -- arg1 arg2
```

---

## Comments

Rust supports several types of comments for documentation and code explanation.

### Single Line Comments

```rust
// This is a single line comment
/// This is a documentation comment (doc comment)
```

### Multi-line Comments

```rust
/*
This is a multi-line comment
that can span multiple lines
*/

/**
This is also a multi-line comment
commonly used for documentation
*/
```

**Key Points:**

- `//` - Regular single-line comment
- `///` - Documentation comment (used for generating docs)
- `/* */` - Multi-line comment block
- `/** */` - Multi-line documentation comment

---

## Variables and Mutability

### Basic Variable Declaration

```rust
fn variables() {
    // Immutable variable (default)
    let x = 5;
    println!("x is {x}");

    // Mutable variable
    let mut y = 5;
    y = 6;  // Can be changed
    println!("y is {y}");
}
```

### Variable Shadowing

```rust
fn shadowing_example() {
    let x = 5;
    let x = x + 1;  // Shadowing with same type

    {
        let x = x + 2;  // New scope, new variable
        println!("x in inner scope: {x}");
    }

    println!("x in outer scope: {x}");

    // Shadowing with different type
    let spaces = "   ";
    let spaces = spaces.len();  // Now it's a number, not a string
}
```

**Key Concepts:**

- Variables are immutable by default
- Use `mut` keyword for mutable variables
- Shadowing allows reusing variable names with different types
- Shadowing creates a new variable, doesn't modify the original

---

## Constants

Constants are compile-time values that cannot be changed.

```rust
fn constants_example() {
    // Constants must have type annotations
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
    const TWO_HOURS_IN_SECONDS: u32 = ONE_HOUR_IN_SECONDS * 2;

    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
```

**Key Points:**

- Use `const` keyword
- Must have explicit type annotation
- Evaluated at compile time
- Use SCREAMING_SNAKE_CASE naming convention
- Can be used in other constant expressions

---

## Variable Shadowing

Variable shadowing allows you to declare a new variable with the same name as a previous variable.

```rust
fn shadowing_detailed() {
    let x = 5;
    println!("x is {x}");

    let x = 6;  // Shadowing - creates new variable
    println!("x is now {x}");

    let x = "hello";  // Can change type
    println!("x is now a string: {x}");

    let x = x.len();  // Can change type again
    println!("x is now the length: {x}");
}
```

**Benefits:**

- Change variable types
- Reuse variable names
- Create new variables without different names
- Useful for transformations

---

## Data Types

### Scalar Types

```rust
fn data_types_example() {
    // Integers
    let x: i32 = 5;        // 32-bit signed integer
    let y: u32 = 5;        // 32-bit unsigned integer
    let z: i64 = 5;        // 64-bit signed integer

    // Floating-point
    let a: f32 = 3.14;     // 32-bit float
    let b: f64 = 3.14;     // 64-bit float

    // Boolean
    let c: bool = true;

    // Character
    let d: char = 'A';

    // Type inference
    let inferred_int = 5;      // Rust infers i32
    let inferred_float = 5.0;  // Rust infers f64
    let inferred_bool = true;  // Rust infers bool
}
```

### Compound Types

```rust
fn compound_types() {
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // Destructuring

    // Arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];

    // Vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
}
```

---

## Control Flow

### If Statements

```rust
fn conditional_example() {
    let number = 6;

    // Basic if-else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Multiple conditions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // If as expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {number}");
}
```

**Key Points:**

- Conditions must be `bool` type
- No parentheses around conditions (optional)
- Can use `if` as an expression
- All branches must return the same type

---

## Loops

### Loop Types

#### 1. Infinite Loop

```rust
fn infinite_loop() {
    loop {
        println!("This runs forever!");
        // Use break to exit
        break;
    }
}
```

#### 2. While Loop

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

#### 3. For Loop

```rust
fn for_example() {
    let a = [10, 20, 30, 40, 50];

    // Iterate over array
    for element in a {
        println!("the value is: {element}");
    }

    // Range-based loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
```

### Advanced Loop Patterns

#### Loop with Break Value

```rust
fn loop_with_break_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };
    println!("Result: {}", result);
}
```

#### Loop Labels

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
                break 'counting_up;  // Break outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
}
```

#### While Let Pattern

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

### Iterator Methods

```rust
fn iterator_examples() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Enumerate
    for (index, value) in numbers.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Filter
    for number in numbers.iter().filter(|&&x| x % 2 == 0) {
        println!("Even number: {}", number);
    }

    // Map
    for doubled in numbers.iter().map(|x| x * 2) {
        println!("Doubled: {}", doubled);
    }

    // Take and Skip
    for number in numbers.iter().take(3) {
        println!("First 3: {}", number);
    }

    for number in numbers.iter().skip(2) {
        println!("Skip first 2: {}", number);
    }
}
```

---

## Pattern Matching

### Basic Match

```rust
fn basic_match() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),  // Default case
    }
}
```

### Multiple Patterns

```rust
fn multiple_patterns() {
    let x = 5;
    match x {
        1 | 2 | 3 => println!("one, two, or three"),
        _ => println!("anything"),
    }
}
```

### Range Patterns

```rust
fn range_patterns() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
```

### Pattern Binding

```rust
fn pattern_binding() {
    let x = 5;
    match x {
        x => println!("x is {x}"),  // Binds the value to x
    }
}
```

### Guards

```rust
fn match_with_guards() {
    let x = 5;
    match x {
        x if x % 2 == 0 => println!("x is even"),
        x if x % 2 != 0 => println!("x is odd"),
        _ => println!("x is not a number"),
    }
}
```

---

## Enums

### Basic Enum

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_example() {
    let msg = Message::Move { x: 1, y: 2 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(s) => println!("Write: {s}"),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: ({r}, {g}, {b})"),
    }
}
```

### If Let Pattern

```rust
fn if_let_example() {
    let msg = Message::Move { x: 1, y: 2 };

    if let Message::Move { x, y } = msg {
        println!("Move to ({x}, {y})");
    } else {
        println!("Not a move message");
    }
}
```

**Key Points:**

- Enums can hold data
- Each variant can have different types and amounts of data
- Use `match` for exhaustive pattern matching
- Use `if let` for single pattern matching

---

## User Input

### Basic String Input

```rust
use std::io;

fn basic_input() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You entered: {}", guess);
}
```

### Numeric Input

```rust
fn numeric_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    println!("You entered: {}", number);
}
```

### Multiple Values

```rust
fn multiple_values() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("You entered: {:?}", values);
}
```

### Safe Input with Error Handling

```rust
fn safe_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
```

### Input with Prompt

```rust
fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
```

### Input with Validation

```rust
fn validated_input() -> i32 {
    loop {
        let input = prompt_input("Enter a number between 1-10: ");
        match input.parse::<i32>() {
            Ok(num) if num >= 1 && num <= 10 => return num,
            Ok(_) => println!("Number must be between 1 and 10"),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}
```

---

## Command Line Arguments

### Reading Command Line Arguments

```rust
fn command_line_args() {
    let args: Vec<String> = std::env::args().collect();
    println!("All arguments: {:?}", args);

    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    } else {
        println!("No arguments provided");
    }
}
```

**Key Points:**

- `std::env::args()` returns an iterator of command line arguments
- First argument (`args[0]`) is the program name
- Arguments start from `args[1]`
- Use `collect()` to convert iterator to vector

---

## Best Practices

### 1. Error Handling

- Use `Result` types for functions that can fail
- Handle errors gracefully instead of using `expect()`
- Use `?` operator for propagating errors

### 2. Variable Naming

- Use `snake_case` for variables and functions
- Use `SCREAMING_SNAKE_CASE` for constants
- Use descriptive names

### 3. Type Safety

- Let Rust infer types when possible
- Explicitly specify types when needed for clarity
- Use appropriate integer types (`i32`, `u32`, etc.)

### 4. Memory Management

- Understand ownership rules
- Use references when you don't need ownership
- Use `clone()` sparingly

### 5. Documentation

- Use doc comments (`///`) for public APIs
- Write clear, concise comments
- Include examples in documentation

---

## Common Pitfalls

### 1. Mutability

```rust
// ❌ Wrong - trying to modify immutable variable
let x = 5;
x = 6;  // This will not compile

// ✅ Correct - use mut
let mut x = 5;
x = 6;
```

### 2. Type Mismatch

```rust
// ❌ Wrong - type mismatch
let x: i32 = 5.0;  // This will not compile

// ✅ Correct - matching types
let x: i32 = 5;
let y: f64 = 5.0;
```

### 3. Unused Variables

```rust
// ❌ Warning - unused variable
let x = 5;  // Rust will warn about unused variable

// ✅ Correct - use the variable or prefix with _
let _x = 5;  // Prefix with _ to indicate intentionally unused
```

### 4. Infinite Loops

```rust
// ❌ Dangerous - infinite loop without exit condition
loop {
    println!("This runs forever!");
}

// ✅ Correct - provide exit condition
let mut counter = 0;
loop {
    counter += 1;
    if counter >= 10 {
        break;
    }
}
```

---

## Exercises

Try implementing these exercises to practice the concepts:

1. **Temperature Converter**: Create a program that converts between Celsius and Fahrenheit
2. **Number Guessing Game**: Implement a simple number guessing game
3. **Simple Calculator**: Create a calculator that can perform basic operations
4. **Word Counter**: Count words in a string
5. **Fibonacci Sequence**: Generate Fibonacci numbers up to a given limit

---

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

---

_This guide covers the fundamental concepts of Rust programming. Continue practicing and exploring more advanced topics as you become comfortable with these basics._

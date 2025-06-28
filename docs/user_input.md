# User Input.rs - User Input Handling

## Overview

The `user_input.rs` file demonstrates various methods for handling user input in Rust, from basic string input to advanced patterns with error handling and validation.

## Code Structure

The file contains multiple functions demonstrating different input handling approaches:

1. `user_input()` - Basic string input
2. `numeric_input()` - Reading numeric input with parsing
3. `multiple_values()` - Reading multiple values on one line
4. `safe_input()` - Safe input with error handling
5. `prompt_input()` - Input with custom prompts
6. `read_until_quit()` - Reading until specific condition
7. `validated_input()` - Input with validation
8. `command_line_args()` - Reading command line arguments
9. Advanced patterns for different input scenarios

## Key Concepts

### 1. Basic String Input

```rust
use std::io;

fn user_input() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("guess is {guess}");
}
```

**Key Points:**

- `String::new()` creates an empty string
- `io::stdin()` gets a handle to standard input
- `read_line()` reads a line and appends to the string
- `expect()` handles potential errors (panics on error)

### 2. Numeric Input with Parsing

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

**Key Points:**

- `trim()` removes whitespace and newlines
- `parse()` converts string to number
- Type annotation `: i32` specifies the target type
- `expect()` provides error message for parsing failures

### 3. Multiple Values on One Line

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

**Key Points:**

- `split_whitespace()` splits on any whitespace
- `map()` applies parsing to each part
- `collect()` gathers results into a vector
- Useful for reading multiple numbers or words

### 4. Safe Input with Error Handling

```rust
fn safe_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
```

**Key Points:**

- Returns `Result` type for proper error handling
- `?` operator propagates errors
- No panic on input failure
- Caller can handle errors gracefully

### 5. Input with Custom Prompt

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

**Key Points:**

- `print!()` prints without newline
- `flush()` ensures prompt is displayed immediately
- Returns trimmed string
- Provides better user experience

### 6. Reading Until Specific Condition

```rust
fn read_until_quit() {
    loop {
        let input = prompt_input("Enter something (or 'quit' to exit): ");
        if input.to_lowercase() == "quit" {
            break;
        }
        println!("You said: {}", input);
    }
}
```

**Key Points:**

- Uses `loop` for continuous input
- `to_lowercase()` for case-insensitive comparison
- `break` exits the loop
- Useful for interactive programs

### 7. Input with Validation

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

**Key Points:**

- Uses `loop` for retry logic
- `match` handles parsing and validation
- Guards (`if num >= 1 && num <= 10`) add conditions
- Returns only valid input

## Advanced Patterns

### 1. Timeout Input (Conceptual)

```rust
fn input_with_timeout_example() {
    // This would require the 'tokio' crate for async input
    use tokio::time::{Duration, timeout};

    async fn async_input() -> Result<String, Box<dyn std::error::Error>> {
        let input = timeout(Duration::from_secs(5), async {
            let mut input = String::new();
            tokio::io::AsyncReadExt::read_line(&mut tokio::io::stdin(), &mut input).await?;
            Ok(input)
        })
        .await??;
        Ok(input.trim().to_string())
    }

    println!("Timeout input example - requires tokio crate");
}
```

### 2. File Input

```rust
fn read_from_file_example() {
    // Reading from a file instead of user input
    match std::fs::read_to_string("input.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

### 3. Command Line Arguments

```rust
fn command_line_args() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    } else {
        println!("No arguments provided");
    }
}
```

## Common Input Patterns

### 1. Menu Selection

```rust
fn menu_selection() -> u32 {
    loop {
        println!("1. Option A");
        println!("2. Option B");
        println!("3. Option C");
        println!("0. Exit");

        let input = prompt_input("Select an option: ");
        match input.parse::<u32>() {
            Ok(choice) if choice <= 3 => return choice,
            Ok(0) => {
                println!("Goodbye!");
                std::process::exit(0);
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
```

### 2. Password Input

```rust
fn password_input() -> String {
    use std::io::{self, Write};

    print!("Enter password: ");
    io::stdout().flush().expect("Failed to flush");

    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password");

    password.trim().to_string()
}
```

### 3. Confirmation Input

```rust
fn confirm_action() -> bool {
    loop {
        let input = prompt_input("Are you sure? (y/n): ");
        match input.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}
```

### 4. CSV Input

```rust
fn csv_input() -> Vec<Vec<String>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().to_string())
                .collect()
        })
        .collect()
}
```

## Best Practices

### 1. Always Handle Errors

```rust
// Good: Proper error handling
fn safe_numeric_input() -> Result<i32, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let number = input.trim().parse()?;
    Ok(number)
}

// Bad: Panics on error
fn unsafe_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    input.trim().parse().expect("Invalid number")
}
```

### 2. Use Appropriate Types

```rust
// Good: Use appropriate numeric types
let age: u8 = input.parse().expect("Invalid age");
let temperature: f64 = input.parse().expect("Invalid temperature");

// Bad: Using generic types
let age: i32 = input.parse().expect("Invalid age"); // Could be negative
```

### 3. Provide Clear Prompts

```rust
// Good: Clear and helpful prompts
let name = prompt_input("Enter your full name: ");
let age = prompt_input("Enter your age (1-120): ");

// Bad: Unclear prompts
let name = prompt_input("Name: ");
let age = prompt_input("Age: ");
```

### 4. Validate Input

```rust
// Good: Validate input before using
fn get_positive_number() -> u32 {
    loop {
        let input = prompt_input("Enter a positive number: ");
        match input.parse::<u32>() {
            Ok(num) if num > 0 => return num,
            _ => println!("Please enter a positive number"),
        }
    }
}
```

## Common Mistakes

### ❌ Not handling input errors

```rust
// Bad: Will panic on input failure
let mut input = String::new();
io::stdin().read_line(&mut input); // Missing expect or error handling
```

### ❌ Not trimming input

```rust
// Bad: Input includes newline character
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
println!("Input: '{}'", input); // Includes \n

// Good: Trim whitespace and newlines
let input = input.trim();
```

### ❌ Not validating parsed input

```rust
// Bad: No validation
let age: i32 = input.parse().expect("Invalid number");
// age could be negative or unreasonably large

// Good: Validate after parsing
let age: i32 = input.parse().expect("Invalid number");
if age < 0 || age > 150 {
    println!("Invalid age range");
    return;
}
```

### ❌ Using expect in production code

```rust
// Bad: Panics in production
let input = io::stdin().read_line(&mut buffer).expect("Failed");

// Good: Handle errors gracefully
match io::stdin().read_line(&mut buffer) {
    Ok(_) => println!("Input: {}", buffer),
    Err(e) => eprintln!("Error reading input: {}", e),
}
```

### ✅ Correct patterns

```rust
// Good: Safe input handling
fn get_user_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Good: Validated numeric input
fn get_valid_number() -> u32 {
    loop {
        match prompt_input("Enter a number: ").parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}
```

## Performance Considerations

### 1. Buffer Reuse

```rust
// Good: Reuse buffer for multiple inputs
fn efficient_input() {
    let mut buffer = String::new();

    for i in 0..5 {
        buffer.clear(); // Clear instead of creating new string
        io::stdin().read_line(&mut buffer).expect("Failed");
        println!("Input {}: {}", i + 1, buffer.trim());
    }
}
```

### 2. Avoiding Unnecessary Allocations

```rust
// Good: Use references when possible
fn process_input(input: &str) {
    // Process without taking ownership
    println!("Processing: {}", input);
}

// Usage
let input = prompt_input("Enter text: ");
process_input(&input);
```

## Exercises

1. **Calculator**: Create a simple calculator that reads two numbers and an operation
2. **Form Validator**: Build a form that validates name, email, and age inputs
3. **File Processor**: Read multiple lines from a file and process them
4. **Interactive Menu**: Create a menu system with numbered options
5. **Data Parser**: Parse different data formats (CSV, JSON-like) from user input

## Related Concepts

- **Error Handling**: Using `Result` and `Option` types
- **String Manipulation**: Working with strings and parsing
- **Collections**: Using vectors and iterators for multiple inputs
- **Control Flow**: Using loops and conditionals for input validation

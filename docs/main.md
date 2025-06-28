# Main.rs - Command Line Arguments

## Overview

The `main.rs` file demonstrates how to handle command line arguments in Rust using the standard library.

## Code Analysis

```rust
fn main() {
    command_line_args();
}

fn command_line_args() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    } else {
        println!("No arguments provided");
    }
}
```

## Key Concepts

### 1. Command Line Arguments

- `std::env::args()` returns an iterator of command line arguments
- The first argument (`args[0]`) is always the program name
- User-provided arguments start from `args[1]`

### 2. Vector Collection

- `collect()` converts the iterator into a `Vec<String>`
- This allows us to access arguments by index

### 3. Argument Validation

- Check `args.len()` to ensure arguments were provided
- Always validate before accessing `args[1]` to avoid panic

## Usage Examples

```bash
# Run without arguments
cargo run
# Output: No arguments provided

# Run with arguments
cargo run -- hello world
# Output:
# args: ["target/debug/rust-learn", "hello", "world"]
# First argument: hello
```

## Best Practices

1. **Always check argument count** before accessing specific indices
2. **Use descriptive error messages** when arguments are missing
3. **Consider using argument parsing libraries** like `clap` for complex applications
4. **Handle edge cases** like empty strings or invalid input

## Common Patterns

### Basic Argument Handling

```rust
fn basic_args() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("No arguments provided"),
        2 => println!("One argument: {}", args[1]),
        _ => println!("Multiple arguments: {:?}", &args[1..]),
    }
}
```

### Argument Validation

```rust
fn validate_args() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <argument>", args[0]);
        std::process::exit(1);
    }

    let argument = &args[1];
    println!("Processing: {}", argument);
}
```

## Related Concepts

- **Error Handling**: Using `Result` types for robust argument processing
- **String Manipulation**: Working with `String` and `&str` types
- **Collections**: Understanding `Vec<T>` and iterators
- **Control Flow**: Using `if` statements for conditional logic

## Exercises

1. **Argument Counter**: Modify the program to count and display the total number of arguments
2. **Argument Reverser**: Print all arguments in reverse order
3. **Argument Validator**: Add validation to ensure arguments are numeric
4. **Help System**: Implement a `--help` flag that displays usage information

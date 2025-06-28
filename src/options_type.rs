/// Option Type in Rust - Handling Optional Values
///
/// The Option type represents a value that might or might not exist.
/// It's Rust's way of handling null values safely without null pointer errors.
use std::io;

pub fn options_type() {
    println!("=== Option Type Learning Examples ===\n");

    // 1. Creating Option Values
    create_options();

    // 2. Pattern Matching with Option
    pattern_matching();

    // 3. Option Methods
    option_methods();

    // 4. Option with Functions
    option_with_functions();

    // 5. Option with Collections
    option_with_collections();

    // 6. Option with User Input
    option_with_input();

    // 7. Advanced Option Patterns
    advanced_patterns();
}

fn create_options() {
    println!("1. Creating Option Values:");

    // Some value
    let some_number: Option<i32> = Some(42);
    println!("Some number: {:?}", some_number);

    // None value
    let no_number: Option<i32> = None;
    println!("No number: {:?}", no_number);

    // Option from function that might fail
    let result = divide(10, 2);
    println!("Division result: {:?}", result);

    let result = divide(10, 0);
    println!("Division by zero: {:?}", result);

    // Option from string parsing
    let parsed: Option<i32> = "123".parse().ok();
    println!("Parsed number: {:?}", parsed);

    let parsed: Option<i32> = "abc".parse().ok();
    println!("Parsed invalid: {:?}", parsed);

    println!();
}

fn pattern_matching() {
    println!("2. Pattern Matching with Option:");

    let some_value = Some(42);
    let no_value: Option<i32> = None;

    // Match expression
    match some_value {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value"),
    }

    match no_value {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value"),
    }

    // If let pattern (cleaner for single case)
    if let Some(value) = some_value {
        println!("If let got value: {}", value);
    }

    if let Some(value) = no_value {
        println!("This won't print: {}", value);
    } else {
        println!("No value found");
    }

    // While let pattern
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("Popping from stack:");
    while let Some(value) = stack.pop() {
        print!("{} ", value);
    }
    println!();

    println!();
}

fn option_methods() {
    println!("3. Option Methods:");

    let some_value = Some(42);
    let no_value: Option<i32> = None;

    // unwrap() - panics if None
    println!("Unwrapped some: {}", some_value.unwrap());
    // println!("Unwrapped none: {}", no_value.unwrap()); // This would panic!

    // unwrap_or() - provides default value
    println!("Unwrap or default: {}", some_value.unwrap_or(0));
    println!("Unwrap or default: {}", no_value.unwrap_or(0));

    // unwrap_or_else() - provides default from closure
    println!(
        "Unwrap or else: {}",
        some_value.unwrap_or_else(|| {
            println!("Computing default...");
            100
        })
    );
    println!(
        "Unwrap or else: {}",
        no_value.unwrap_or_else(|| {
            println!("Computing default...");
            100
        })
    );

    // map() - transform Some value
    let doubled = some_value.map(|x| x * 2);
    println!("Mapped some: {:?}", doubled);

    let doubled = no_value.map(|x| x * 2);
    println!("Mapped none: {:?}", doubled);

    // and_then() - chain operations
    let result = some_value
        .and_then(|x| if x > 0 { Some(x * 2) } else { None })
        .and_then(|x| if x < 100 { Some(x + 10) } else { None });
    println!("Chained result: {:?}", result);

    // filter() - filter Some values
    let filtered = some_value.filter(|&x| x > 40);
    println!("Filtered > 40: {:?}", filtered);

    let filtered = some_value.filter(|&x| x > 50);
    println!("Filtered > 50: {:?}", filtered);

    // is_some() and is_none()
    println!("Is some: {}", some_value.is_some());
    println!("Is none: {}", some_value.is_none());
    println!("Is some: {}", no_value.is_some());
    println!("Is none: {}", no_value.is_none());

    println!();
}

fn option_with_functions() {
    println!("4. Option with Functions:");

    // Function that returns Option
    let result = find_number(&vec![1, 2, 3, 4, 5], 3);
    println!("Found 3: {:?}", result);

    let result = find_number(&vec![1, 2, 3, 4, 5], 10);
    println!("Found 10: {:?}", result);

    // Function that takes Option
    print_if_some(Some("Hello"));
    print_if_some(None);

    // Chaining function calls
    let numbers = vec![1, 2, 3, 4, 5];
    let result = numbers
        .get(2)
        .map(|&x| x * 2)
        .and_then(|x| if x > 5 { Some(x) } else { None });
    println!("Chained function result: {:?}", result);

    // Option as function parameter
    let result = process_optional_value(Some(42));
    println!("Processed some: {}", result);

    let result = process_optional_value(None);
    println!("Processed none: {}", result);

    println!();
}

fn option_with_collections() {
    println!("5. Option with Collections:");

    let numbers = vec![1, 2, 3, 4, 5];

    // Safe indexing
    let first = numbers.get(0);
    println!("First element: {:?}", first);

    let tenth = numbers.get(10);
    println!("Tenth element: {:?}", tenth);

    // Finding elements
    let found = numbers.iter().find(|&&x| x == 3);
    println!("Found 3: {:?}", found);

    let found = numbers.iter().find(|&&x| x == 10);
    println!("Found 10: {:?}", found);

    // Position of element
    let position = numbers.iter().position(|&x| x == 4);
    println!("Position of 4: {:?}", position);

    let position = numbers.iter().position(|&x| x == 10);
    println!("Position of 10: {:?}", position);

    // Filtering and mapping with Option
    let results: Vec<i32> = numbers
        .iter()
        .filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None })
        .collect();
    println!("Filtered and mapped: {:?}", results);

    // Option in struct
    let person = Person {
        name: "Alice".to_string(),
        age: Some(30),
        email: None,
    };
    println!("Person: {:?}", person);

    println!();
}

fn option_with_input() {
    println!("6. Option with User Input:");

    println!("Enter a number (or 'quit' to exit):");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "quit" {
            break;
        }

        // Parse input safely
        match input.parse::<i32>() {
            Ok(number) => {
                let result = process_number(number);
                match result {
                    Some(value) => println!("Processed result: {}", value),
                    None => println!("Number was too large to process"),
                }
            }
            Err(_) => {
                println!("Invalid number, please try again");
            }
        }
    }

    println!();
}

fn advanced_patterns() {
    println!("7. Advanced Option Patterns:");

    // Option with Result
    let result = parse_and_validate("42");
    match result {
        Ok(Some(value)) => println!("Valid number: {}", value),
        Ok(None) => println!("Number was zero"),
        Err(_) => println!("Invalid input"),
    }

    let result = parse_and_validate("abc");
    match result {
        Ok(Some(value)) => println!("Valid number: {}", value),
        Ok(None) => println!("Number was zero"),
        Err(_) => println!("Invalid input"),
    }

    // Option with custom types
    let user = find_user_by_id(1);
    match user {
        Some(user) => println!("Found user: {}", user.name),
        None => println!("User not found"),
    }

    let user = find_user_by_id(999);
    match user {
        Some(user) => println!("Found user: {}", user.name),
        None => println!("User not found"),
    }

    // Option with closures
    let numbers = vec![1, 2, 3, 4, 5];
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);

    first_even
        .map(|&x| {
            println!("First even number: {}", x);
            x * 2
        })
        .map(|x| {
            println!("Doubled: {}", x);
            x
        });

    // Option with default values
    let config_value = get_config_value("timeout");
    let timeout = config_value.unwrap_or(30);
    println!("Timeout: {} seconds", timeout);

    println!();
}

// Helper functions

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    numbers.iter().position(|&x| x == target)
}

fn print_if_some(value: Option<&str>) {
    if let Some(text) = value {
        println!("Got text: {}", text);
    } else {
        println!("No text provided");
    }
}

fn process_optional_value(value: Option<i32>) -> i32 {
    value.unwrap_or(0)
}

fn process_number(num: i32) -> Option<i32> {
    if num > 1000 { None } else { Some(num * 2) }
}

fn parse_and_validate(input: &str) -> Result<Option<i32>, std::num::ParseIntError> {
    let number = input.parse::<i32>()?;
    if number == 0 {
        Ok(None)
    } else {
        Ok(Some(number))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: Option<u32>,
    email: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
}

fn find_user_by_id(id: u32) -> Option<User> {
    // Simulate database lookup
    if id == 1 {
        Some(User {
            id: 1,
            name: "Alice".to_string(),
        })
    } else {
        None
    }
}

fn get_config_value(key: &str) -> Option<u32> {
    // Simulate config lookup
    match key {
        "timeout" => Some(60),
        "port" => Some(8080),
        _ => None,
    }
}

// Main function to run all option examples
fn main() {
    options_type();
}

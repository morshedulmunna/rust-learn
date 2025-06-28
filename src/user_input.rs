use std::io;

/// user input - Method 1: Basic string input
fn user_input() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("guess is {guess}");
}

/// Method 2: Reading numeric input with parsing
fn numeric_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse to integer
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    println!("You entered: {}", number);
}

/// Method 3: Reading multiple values on one line
fn multiple_values() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Split by whitespace and parse
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("You entered: {:?}", values);
}

/// Method 4: Reading with error handling (no panic)
fn safe_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Method 5: Reading with custom prompt
fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

/// Method 6: Reading until specific condition
fn read_until_quit() {
    loop {
        let input = prompt_input("Enter something (or 'quit' to exit): ");
        if input.to_lowercase() == "quit" {
            break;
        }
        println!("You said: {}", input);
    }
}

/// Method 7: Reading with validation
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

/// Method 8: Reading from command line arguments
fn command_line_args() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    } else {
        println!("No arguments provided");
    }
}

/// Method 9: Reading with timeout (requires external crate)
fn input_with_timeout_example() {
    // This would require the 'tokio' crate for async input
    // Example structure:

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

/// Method 10: Reading from file instead of stdin
fn read_from_file_example() {
    // Reading from a file instead of user input
    match std::fs::read_to_string("input.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}

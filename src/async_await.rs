use std::time::Duration;
use tokio::time::sleep;

// Basic async function
async fn hello_world() {
    println!("Hello from async function!");
}

// Async function that simulates some work
async fn do_work(work_id: u32) {
    println!("Starting work {}", work_id);

    // Simulate some async work (like I/O or network request)
    sleep(Duration::from_millis(100)).await;

    println!("Finished work {}", work_id);
}

// Async function that returns a value
async fn calculate_sum(a: u32, b: u32) -> u32 {
    // Simulate some computation time
    sleep(Duration::from_millis(50)).await;
    a + b
}

// Async function that demonstrates error handling
async fn risky_operation(should_fail: bool) -> Result<String, &'static str> {
    sleep(Duration::from_millis(75)).await;

    if should_fail {
        Err("Operation failed!")
    } else {
        Ok("Operation succeeded!".to_string())
    }
}

// Function that demonstrates concurrent execution
async fn run_concurrent_tasks() {
    println!("\n--- Running concurrent tasks ---");

    // Spawn multiple tasks concurrently
    let task1 = do_work(1);
    let task2 = do_work(2);
    let task3 = do_work(3);

    // Wait for all tasks to complete
    let (_, _, _) = tokio::join!(task1, task2, task3);

    println!("All concurrent tasks completed!");
}

// Function that demonstrates sequential vs concurrent execution
async fn compare_execution_modes() {
    println!("\n--- Sequential vs Concurrent Execution ---");

    // Sequential execution
    println!("Sequential execution:");
    let start = std::time::Instant::now();

    do_work(1).await;
    do_work(2).await;
    do_work(3).await;

    let sequential_duration = start.elapsed();
    println!("Sequential took: {:?}", sequential_duration);

    // Concurrent execution
    println!("\nConcurrent execution:");
    let start = std::time::Instant::now();

    let task1 = do_work(4);
    let task2 = do_work(5);
    let task3 = do_work(6);

    tokio::join!(task1, task2, task3);

    let concurrent_duration = start.elapsed();
    println!("Concurrent took: {:?}", concurrent_duration);

    println!(
        "Concurrent was {:.2}x faster!",
        sequential_duration.as_millis() as f64 / concurrent_duration.as_millis() as f64
    );
}

// Function that demonstrates async/await with Result handling
async fn handle_async_results() {
    println!("\n--- Handling Async Results ---");

    // Handle successful operation
    match risky_operation(false).await {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Handle failed operation
    match risky_operation(true).await {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// Function that demonstrates async operations with different return types
async fn demonstrate_async_operations() {
    println!("\n--- Async Operations with Return Values ---");

    // Calculate sums concurrently
    let sum1 = calculate_sum(10, 20);
    let sum2 = calculate_sum(30, 40);
    let sum3 = calculate_sum(50, 60);

    let (result1, result2, result3) = tokio::join!(sum1, sum2, sum3);

    println!("Sum 1: {}", result1);
    println!("Sum 2: {}", result2);
    println!("Sum 3: {}", result3);
    println!("Total: {}", result1 + result2 + result3);
}

// Main async function that orchestrates all examples
async fn run_async_examples() {
    println!("=== Rust Async/Await Examples ===\n");

    // Basic async function call
    hello_world().await;

    // Run all demonstration functions
    run_concurrent_tasks().await;
    compare_execution_modes().await;
    handle_async_results().await;
    demonstrate_async_operations().await;

    println!("\n=== All async examples completed! ===");
}

// Main function that sets up the async runtime
fn main() {
    // Create and run the async runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(run_async_examples());
}

// Example of how to use this module from other parts of your code
pub async fn public_async_function() -> String {
    "This is a public async function".to_string()
}

// Example of async function that can be used in tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculate_sum() {
        let result = calculate_sum(5, 3).await;
        assert_eq!(result, 8);
    }

    #[tokio::test]
    async fn test_risky_operation_success() {
        let result = risky_operation(false).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Operation succeeded!");
    }

    #[tokio::test]
    async fn test_risky_operation_failure() {
        let result = risky_operation(true).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Operation failed!");
    }
}

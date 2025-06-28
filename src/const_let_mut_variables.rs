/// const, let, mut variables
///
/// This module demonstrates the three main ways to declare variables in Rust:
/// - const: compile-time constants, immutable, cannot be changed
/// - let: immutable variables, cannot be reassigned after declaration
/// - mut: mutable variables, can be changed after declaration

fn const_let_mut_variables() {
    // CONSTANTS
    // Constants are declared with 'const' and must have a type annotation
    // They are evaluated at compile time and are immutable
    // Convention: Use SCREAMING_SNAKE_CASE for constant names
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // You cannot change a constant - this would cause a compile error:
    // THREE_HOURS_IN_SECONDS = 10800; // ❌ This won't compile

    // Constants can be used in other constant expressions
    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
    const TWO_HOURS_IN_SECONDS: u32 = ONE_HOUR_IN_SECONDS * 2;
    println!("TWO_HOURS_IN_SECONDS is {TWO_HOURS_IN_SECONDS}");
}

fn let_variables() {
    // IMMUTABLE VARIABLES (let)
    // Variables declared with 'let' are immutable by default
    // This means once assigned, they cannot be changed
    let x = 5;
    println!("x is {x}");

    // This line would cause a compile error because x is immutable:
    // x = 6; // ❌ This won't compile - cannot assign twice to immutable variable

    // However, you can "shadow" a variable by declaring it again with the same name
    // This creates a new variable that shadows the previous one
    let x = 6; // This is called "variable shadowing"
    println!("x is now {x}");

    // Shadowing allows you to change the type of a variable
    let x = "hello"; // x is now a string, not a number
    println!("x is now a string: {x}");

    // You can also shadow with a different type
    let x = x.len(); // x is now a usize (the length of the string)
    println!("x is now the length: {x}");
}

fn mut_variables() {
    // MUTABLE VARIABLES (let mut)
    // Variables declared with 'let mut' can be changed after declaration
    // Use 'mut' when you need to modify the value later
    let mut x = 5;
    println!("x is {x}");

    // Now we can change the value
    x = 6;
    println!("x is now {x}");

    // We can also modify the value in other ways
    x += 1;
    println!("x after adding 1: {x}");

    x *= 2;
    println!("x after multiplying by 2: {x}");

    // Mutable variables are useful for counters, accumulators, etc.
    let mut counter = 0;
    counter += 1;
    counter += 1;
    counter += 1;
    println!("Counter is: {counter}");
}

fn variable_scope_example() {
    // VARIABLE SCOPE
    // Variables are only valid within their scope (the block where they're declared)

    let outer_variable = "I'm in the outer scope";
    println!("Outer: {outer_variable}");

    {
        // This is a new scope (block)
        let inner_variable = "I'm in the inner scope";
        println!("Inner: {inner_variable}");
        println!("Outer (from inner): {outer_variable}"); // Can access outer variables

        // inner_variable goes out of scope here
    }

    // This would cause an error - inner_variable is not in scope here:
    // println!("Outer trying to access inner: {inner_variable}"); // ❌ Won't compile

    println!("Outer: {outer_variable}"); // This works fine
}

fn type_inference_example() {
    // TYPE INFERENCE
    // Rust can often infer the type of a variable from its value

    let x = 5; // Rust infers this is i32
    let y = 5.0; // Rust infers this is f64
    let z = true; // Rust infers this is bool
    let text = "hello"; // Rust infers this is &str

    // You can also explicitly specify the type
    let explicit_int: i32 = 5;
    let explicit_float: f64 = 5.0;
    let explicit_bool: bool = true;
    let explicit_string: &str = "hello";

    println!("Inferred types: x={}, y={}, z={}, text={}", x, y, z, text);
    println!(
        "Explicit types: explicit_int={}, explicit_float={}, explicit_bool={}, explicit_string={}",
        explicit_int, explicit_float, explicit_bool, explicit_string
    );
}

// Main function to demonstrate all concepts
pub fn demonstrate_variables() {
    println!("=== CONSTANTS ===");
    const_let_mut_variables();

    println!("\n=== IMMUTABLE VARIABLES (let) ===");
    let_variables();

    println!("\n=== MUTABLE VARIABLES (let mut) ===");
    mut_variables();

    println!("\n=== VARIABLE SCOPE ===");
    variable_scope_example();

    println!("\n=== TYPE INFERENCE ===");
    type_inference_example();
}

/// Vectors in Rust - Dynamic Arrays
///
/// Vectors are growable arrays that can store multiple values of the same type.
/// They are one of the most commonly used data structures in Rust.
use std::io;

pub fn vectors() {
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

fn create_vectors() {
    println!("1. Creating Vectors:");

    // Empty vector with type annotation
    let v1: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v1);

    // Vector with initial values using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector with values: {:?}", v2);

    // Vector with repeated values
    let v3 = vec![0; 5]; // Creates [0, 0, 0, 0, 0]
    println!("Vector with repeated values: {:?}", v3);

    // Vector with capacity
    let v4: Vec<i32> = Vec::with_capacity(10);
    println!(
        "Vector with capacity 10: {:?} (len: {}, capacity: {})",
        v4,
        v4.len(),
        v4.capacity()
    );

    println!();
}

fn modify_vectors() {
    println!("2. Adding and Removing Elements:");

    let mut v = Vec::new();

    // Adding elements
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After pushing: {:?}", v);

    // Removing elements
    let last = v.pop();
    println!("Popped value: {:?}", last);
    println!("After popping: {:?}", v);

    // Inserting at specific index
    v.insert(1, 10);
    println!("After inserting 10 at index 1: {:?}", v);

    // Removing at specific index
    v.remove(0);
    println!("After removing at index 0: {:?}", v);

    // Clearing the vector
    v.clear();
    println!("After clearing: {:?}", v);

    println!();
}

fn access_elements() {
    println!("3. Accessing Vector Elements:");

    let v = vec![10, 20, 30, 40, 50];

    // Using indexing (panics if out of bounds)
    println!("First element: {}", v[0]);
    println!("Third element: {}", v[2]);

    // Using get() method (returns Option)
    match v.get(2) {
        Some(value) => println!("Element at index 2: {}", value),
        None => println!("Index 2 is out of bounds"),
    }

    match v.get(10) {
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("Index 10 is out of bounds"),
    }

    // Using if let for cleaner code
    if let Some(value) = v.get(1) {
        println!("Element at index 1: {}", value);
    }

    // Last element
    if let Some(last) = v.last() {
        println!("Last element: {}", last);
    }

    println!();
}

fn iterate_vectors() {
    println!("4. Iterating Over Vectors:");

    let v = vec![1, 2, 3, 4, 5];

    // Iterating with for loop (immutable)
    print!("Values: ");
    for value in &v {
        print!("{} ", value);
    }
    println!();

    // Iterating with indices
    print!("Indices and values: ");
    for (index, value) in v.iter().enumerate() {
        print!("[{}:{}] ", index, value);
    }
    println!();

    // Iterating with mutable reference
    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", v2);

    for value in &mut v2 {
        *value *= 2; // Double each value
    }
    println!("After doubling: {:?}", v2);

    // Using iterators
    let sum: i32 = v2.iter().sum();
    println!("Sum of all elements: {}", sum);

    let doubled: Vec<i32> = v2.iter().map(|x| x * 2).collect();
    println!("Doubled again: {:?}", doubled);

    println!();
}

fn vector_methods() {
    println!("5. Vector Methods:");

    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Original vector: {:?}", v);

    // Length and capacity
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());

    // Check if empty
    println!("Is empty: {}", v.is_empty());

    // Sort
    v.sort();
    println!("After sorting: {:?}", v);

    // Reverse
    v.reverse();
    println!("After reversing: {:?}", v);

    // Deduplicate
    v.sort();
    v.dedup();
    println!("After deduplication: {:?}", v);

    // Contains
    println!("Contains 5: {}", v.contains(&5));
    println!("Contains 7: {}", v.contains(&7));

    // Find
    if let Some(index) = v.iter().position(|&x| x == 5) {
        println!("5 found at index: {}", index);
    }

    // Filter
    let even_numbers: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", even_numbers);

    println!();
}

fn vector_with_input() {
    println!("6. Vector with User Input:");

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
        println!(
            "Average: {:.2}",
            numbers.iter().sum::<i32>() as f64 / numbers.len() as f64
        );
    } else {
        println!("No numbers entered");
    }

    println!();
}

fn vector_of_different_types() {
    println!("7. Vector of Different Types (using enums):");

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
    mixed_data.push(Value::Integer(100));
    mixed_data.push(Value::Float(2.718));

    println!("Mixed data vector:");
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

    println!("Only integers: {:?}", integers);

    println!();
}

// Main function to run all vector examples
fn main() {
    vectors();
}

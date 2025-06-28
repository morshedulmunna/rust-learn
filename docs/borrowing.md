# Borrowing - Accessing Data Without Ownership

## Overview

The `browing.rs` file demonstrates Rust's borrowing system, which allows you to access data without taking ownership. Borrowing is a fundamental concept that enables safe concurrent access, efficient memory usage, and prevents data races at compile time. This guide covers everything from basic borrowing concepts to advanced patterns and best practices.

## Code Analysis

```rust
pub fn borrowing() {
    println!("=== Borrowing Learning Examples ===\n");

    // 1. Basic Borrowing Concepts
    basic_borrowing_concepts();

    // 2. Immutable Borrowing
    immutable_borrowing();

    // 3. Mutable Borrowing
    mutable_borrowing();

    // 4. Borrowing Rules
    borrowing_rules();

    // 5. Borrowing with Functions
    borrowing_with_functions();

    // 6. Advanced Patterns
    advanced_patterns();
}
```

## Key Concepts

### 1. Basic Borrowing Concepts

```rust
// WHAT IS BORROWING?
// Borrowing allows you to access data without taking ownership.
// It's like borrowing a book from a library - you can read it, but you don't own it.

let s1 = String::from("hello");
println!("s1 owns: '{}'", s1);

// Immutable borrow
let len = calculate_length(&s1);
println!("Length of '{}' is {}", s1, len);
println!("s1 is still valid after borrowing!");

// BORROWING vs OWNERSHIP
// With ownership (moves the data)
let s2 = String::from("world");
takes_ownership(s2);
// println!("s2: {}", s2);  // COMPILE ERROR: s2 was moved!

// With borrowing (keeps the data)
let s3 = String::from("world");
borrows_data(&s3);
println!("s3 is still valid: '{}'", s3);
```

**Key Points:**

- **Borrowing** allows access to data without taking ownership
- **Immutable borrows** (`&`) provide read-only access
- **Mutable borrows** (`&mut`) provide read-write access
- Borrowing prevents data from being moved or dropped
- Borrowing is essential for efficient memory usage

### 2. Immutable Borrowing

```rust
// IMMUTABLE BORROWING - Read-Only Access
let s = String::from("hello world");
let s_ref = &s;  // Immutable borrow
println!("s: '{}', s_ref: '{}'", s, s_ref);

// MULTIPLE IMMUTABLE BORROWS
let data = String::from("shared data");
let ref1 = &data;
let ref2 = &data;
let ref3 = &data;
println!("All references point to the same data");

// IMMUTABLE BORROWING LIMITATIONS
let mut s = String::from("hello");
let s_ref = &s;  // Immutable borrow
// s.push_str(" world");  // COMPILE ERROR: cannot borrow as mutable!
println!("Cannot modify data while it's immutably borrowed");
```

**Key Points:**

- Immutable borrows provide read-only access
- You can have multiple immutable borrows simultaneously
- Immutable borrows prevent modification of the data
- Immutable borrows are safe for concurrent access

### 3. Mutable Borrowing

```rust
// MUTABLE BORROWING - Read-Write Access
let mut s = String::from("hello");
println!("Before modification: '{}'", s);

let s_ref = &mut s;  // Mutable borrow
s_ref.push_str(" world");  // Modify through reference
println!("After modification: '{}'", s);

// EXCLUSIVE MUTABLE ACCESS
let mut data = String::from("original");
let ref1 = &mut data;  // First mutable borrow
ref1.push_str(" modified");
println!("ref1: '{}'", ref1);

// let ref2 = &mut data;  // COMPILE ERROR: cannot borrow as mutable more than once!
println!("Cannot have multiple mutable borrows simultaneously");
```

**Key Points:**

- Mutable borrows provide read-write access
- You can have only one mutable borrow at a time
- Mutable borrows allow modification of the data
- Mutable borrows prevent data races

### 4. Borrowing Rules

```rust
// THE BORROWING RULES:
// 1. You can have any number of immutable borrows
// 2. You can have exactly one mutable borrow
// 3. You cannot have both immutable and mutable borrows
// 4. References must always be valid

// RULE 1: Multiple Immutable Borrows
let data = String::from("shared");
let ref1 = &data;
let ref2 = &data;
let ref3 = &data;
println!("Multiple immutable borrows: '{}', '{}', '{}'", ref1, ref2, ref3);

// RULE 2: Single Mutable Borrow
let mut data = String::from("mutable");
let ref1 = &mut data;
ref1.push_str(" data");
println!("Single mutable borrow: '{}'", ref1);

// RULE 3: No Mixing Immutable and Mutable
let mut data = String::from("mixed");
let immut_ref = &data;  // Immutable borrow
let immut_ref2 = &data; // Another immutable borrow
// let mut_ref = &mut data;  // COMPILE ERROR: cannot borrow as mutable!

// BORROWING SCOPE
let mut data = String::from("scope test");
{
    let ref1 = &data;  // Immutable borrow
    let ref2 = &data;  // Another immutable borrow
    // ref1 and ref2 go out of scope here
}
let ref3 = &mut data;  // Now we can have a mutable borrow
```

**Key Points:**

- **Rule 1**: Multiple immutable borrows are allowed
- **Rule 2**: Only one mutable borrow at a time
- **Rule 3**: Cannot mix immutable and mutable borrows
- **Rule 4**: References must always be valid
- Borrowing scope determines when references are valid

### 5. Borrowing with Functions

```rust
// FUNCTION PARAMETERS - Borrowing
let text = String::from("hello world");
let length = get_length(&text);  // Borrow text
let word_count = count_words(&text);  // Borrow text again
println!("text is still valid after function calls");

// MUTABLE FUNCTION PARAMETERS
let mut text = String::from("hello");
append_world(&mut text);  // Mutable borrow
println!("After: '{}'", text);

// RETURNING REFERENCES
let text = String::from("hello world");
let first_word = get_first_word(&text);
println!("First word: '{}'", first_word);

// BORROWING WITH COLLECTIONS
let mut numbers = vec![1, 2, 3, 4, 5];

// Immutable borrow
let sum: i32 = numbers.iter().sum();
println!("numbers is still valid: {:?}", numbers);

// Mutable borrow
let numbers_ref = &mut numbers;
numbers_ref.push(6);
numbers_ref[0] = 10;
println!("Modified: {:?}", numbers);
```

**Key Points:**

- Function parameters can borrow data
- Functions can return references
- Collections support both immutable and mutable borrowing
- Borrowing with functions enables efficient data access

### 6. Advanced Patterns

```rust
// BORROWING WITH CLOSURES
let mut list = vec![1, 2, 3, 4, 5];

// Closure that borrows immutably
let print_list = || println!("List: {:?}", list);
print_list();

// Closure that borrows mutably
let mut add_element = || list.push(6);
add_element();

// BORROWING WITH ITERATORS
let numbers = vec![1, 2, 3, 4, 5];

// Iterator with immutable borrows
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("Original: {:?}", numbers);

// Iterator with mutable borrows
let mut numbers = vec![1, 2, 3, 4, 5];
for num in &mut numbers {
    *num *= 2;
}

// BORROWING WITH STRUCTS
let mut person = Person {
    name: String::from("Alice"),
    age: 30,
};

// Immutable borrow
let name_ref = &person.name;
let age_ref = &person.age;

// Mutable borrow
let person_ref = &mut person;
person_ref.age += 1;
person_ref.name.push_str(" Smith");

// BORROWING WITH LIFETIMES
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**Key Points:**

- Closures can borrow data immutably or mutably
- Iterators work with borrowing patterns
- Structs support borrowing of individual fields
- Lifetimes ensure references remain valid

## Common Patterns

### Avoiding Moving with References

```rust
// Instead of moving
fn process_string(s: String) -> String {
    // Process s
    s
}

let s = String::from("hello");
let result = process_string(s);
// s is no longer valid here

// Use references to avoid moving
fn process_string_ref(s: &String) -> usize {
    s.len()
}

let s = String::from("hello");
let len = process_string_ref(&s);
// s is still valid here
```

### Borrowing with Collections

```rust
let mut numbers = vec![1, 2, 3, 4, 5];

// Safe indexing with borrowing
if let Some(num) = numbers.get(2) {
    println!("Element at index 2: {}", num);
}

// Iterating with borrowing
for num in &numbers {
    println!("Number: {}", num);
}

// Modifying with borrowing
for num in &mut numbers {
    *num *= 2;
}
```

### Borrowing with Structs

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

let mut person = Person {
    name: String::from("Alice"),
    age: 30,
};

// Borrow individual fields
let name_ref = &person.name;
let age_ref = &person.age;

// Borrow entire struct
let person_ref = &mut person;
person_ref.age += 1;
```

### Borrowing with Enums

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let message = Message::Write(String::from("hello"));

match &message {  // Borrow the enum
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(s) => println!("Write: '{}'", s),
    Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
}

// message is still valid after the match
```

## Best Practices

1. **Use the smallest scope possible for borrows**
2. **Prefer immutable borrows when possible**
3. **Use references to avoid unnecessary copying**
4. **Understand the borrowing rules thoroughly**
5. **Use appropriate lifetime annotations**
6. **Avoid long-lived mutable borrows**
7. **Use borrowing with collections efficiently**
8. **Understand borrowing scope and lifetime**

## Common Mistakes

### ❌ Trying to modify immutably borrowed data

```rust
let mut s = String::from("hello");
let s_ref = &s;  // Immutable borrow
s.push_str(" world");  // This will not compile!
```

### ✅ Use mutable borrows for modification

```rust
let mut s = String::from("hello");
let s_ref = &mut s;  // Mutable borrow
s_ref.push_str(" world");  // This works
```

### ❌ Multiple mutable borrows

```rust
let mut s = String::from("hello");
let ref1 = &mut s;
let ref2 = &mut s;  // This will not compile!
```

### ✅ Use one mutable borrow at a time

```rust
let mut s = String::from("hello");
{
    let ref1 = &mut s;
    ref1.push_str(" world");
}  // ref1 goes out of scope
let ref2 = &mut s;  // Now this works
```

### ❌ Mixing immutable and mutable borrows

```rust
let mut s = String::from("hello");
let immut_ref = &s;  // Immutable borrow
let mut_ref = &mut s;  // This will not compile!
```

### ✅ Use appropriate borrow types

```rust
let mut s = String::from("hello");
{
    let immut_ref = &s;  // Immutable borrow
    println!("{}", immut_ref);
}  // immut_ref goes out of scope
let mut_ref = &mut s;  // Now this works
```

## Exercises

1. **Basic Borrowing**: Create functions that demonstrate immutable and mutable borrowing
2. **Collection Borrowing**: Work with vectors and other collections using borrowing
3. **Struct Borrowing**: Create structs and demonstrate field borrowing
4. **Function Borrowing**: Write functions that borrow parameters and return references
5. **Advanced Borrowing**: Implement complex borrowing patterns with closures and iterators
6. **Lifetime Annotations**: Write functions with explicit lifetime parameters
7. **Borrowing Rules**: Demonstrate all borrowing rules with examples
8. **Error Handling**: Use borrowing with Result and Option types
9. **Performance Optimization**: Use borrowing for efficient memory usage
10. **Real-world Scenarios**: Implement practical borrowing patterns

## Related Concepts

- **Ownership**: The foundation that borrowing builds upon
- **Lifetimes**: How long references are valid
- **References**: The mechanism for borrowing
- **Memory Safety**: How borrowing prevents common errors
- **Data Races**: How borrowing prevents concurrent access issues
- **Zero-Cost Abstractions**: How borrowing provides safety without runtime cost
- **Smart Pointers**: Advanced borrowing patterns with Rc and Arc
- **Iterators**: Working with borrowed data efficiently

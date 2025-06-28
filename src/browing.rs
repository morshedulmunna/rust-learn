/// Borrowing in Rust - Accessing Data Without Ownership
///
/// Borrowing is Rust's way of allowing you to access data without taking ownership.
/// It's a fundamental concept that enables safe concurrent access and efficient memory usage.
/// This comprehensive guide covers all aspects of borrowing from basic to advanced patterns.
use std::io;

pub fn borrowing() {
    println!("=== Borrowing Learning Examples ===\n");

    // 1. Basic Borrowing Concepts
    basic_borrowing_concepts();

    // 2. Immutable Borrowing
    immutable_borrowing();

    // 3. Mutable Borrowing
    mutable_borrowing();

    // 4. Borrowing Rules and Restrictions
    borrowing_rules();

    // 5. Borrowing with Functions
    borrowing_with_functions();

    // 6. Borrowing with Collections
    borrowing_with_collections();

    // 7. Borrowing with Structs
    borrowing_with_structs();

    // 8. Advanced Borrowing Patterns
    advanced_borrowing_patterns();

    // 9. Borrowing and Lifetimes
    borrowing_and_lifetimes();

    // 10. Common Borrowing Scenarios
    common_borrowing_scenarios();
}

fn basic_borrowing_concepts() {
    println!("1. Basic Borrowing Concepts:");
    println!("============================\n");

    println!("WHAT IS BORROWING?");
    println!("==================");
    println!("Borrowing allows you to access data without taking ownership.");
    println!("It's like borrowing a book from a library - you can read it, but you don't own it.");

    let s1 = String::from("hello");
    println!("s1 owns: '{}'", s1);

    // Immutable borrow
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);
    println!("s1 is still valid after borrowing!");

    println!("\nBORROWING vs OWNERSHIP:");
    println!("=======================");

    // With ownership (moves the data)
    let s2 = String::from("world");
    takes_ownership(s2);
    // println!("s2: {}", s2);  // COMPILE ERROR: s2 was moved!

    // With borrowing (keeps the data)
    let s3 = String::from("world");
    borrows_data(&s3);
    println!("s3 is still valid: '{}'", s3);

    println!("\nBORROWING OPERATORS:");
    println!("===================");
    println!("&  - Immutable borrow (read-only access)");
    println!("&mut - Mutable borrow (read-write access)");
    println!("*  - Dereference operator (access the value)");

    let x = 42;
    let ref_x = &x; // Immutable borrow
    println!("x: {}, ref_x: {}, *ref_x: {}", x, ref_x, *ref_x);

    let mut y = 10;
    let ref_y = &mut y; // Mutable borrow
    *ref_y += 5; // Dereference and modify
    println!("y: {}, ref_y: {}", y, ref_y);

    println!();
}

fn immutable_borrowing() {
    println!("2. Immutable Borrowing:");
    println!("=======================\n");

    println!("IMMUTABLE BORROWING - Read-Only Access:");
    println!("======================================");

    let s = String::from("hello world");
    let s_ref = &s; // Immutable borrow
    println!("s: '{}', s_ref: '{}'", s, s_ref);
    println!("Both s and s_ref can access the same data");

    println!("\nMULTIPLE IMMUTABLE BORROWS:");
    println!("==========================");

    let data = String::from("shared data");
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;

    println!("ref1: '{}'", ref1);
    println!("ref2: '{}'", ref2);
    println!("ref3: '{}'", ref3);
    println!("All references point to the same data");

    println!("\nIMMUTABLE BORROWING WITH FUNCTIONS:");
    println!("===================================");

    let text = String::from("hello world");
    let word_count = count_words(&text);
    let char_count = count_chars(&text);

    println!("Text: '{}'", text);
    println!("Word count: {}", word_count);
    println!("Character count: {}", char_count);
    println!("text is still valid after multiple function calls");

    println!("\nIMMUTABLE BORROWING LIMITATIONS:");
    println!("===============================");

    let mut s = String::from("hello");
    let s_ref = &s; // Immutable borrow
    // s.push_str(" world");  // COMPILE ERROR: cannot borrow as mutable!
    println!("Cannot modify data while it's immutably borrowed");
    println!("s_ref: '{}'", s_ref);

    println!();
}

fn mutable_borrowing() {
    println!("3. Mutable Borrowing:");
    println!("=====================\n");

    println!("MUTABLE BORROWING - Read-Write Access:");
    println!("====================================");

    let mut s = String::from("hello");
    println!("Before modification: '{}'", s);

    let s_ref = &mut s; // Mutable borrow
    s_ref.push_str(" world"); // Modify through reference
    println!("After modification: '{}'", s);

    println!("\nEXCLUSIVE MUTABLE ACCESS:");
    println!("=========================");

    let mut data = String::from("original");
    let ref1 = &mut data; // First mutable borrow
    ref1.push_str(" modified");
    println!("ref1: '{}'", ref1);

    // let ref2 = &mut data;  // COMPILE ERROR: cannot borrow as mutable more than once!
    println!("Cannot have multiple mutable borrows simultaneously");

    println!("\nMUTABLE BORROWING WITH FUNCTIONS:");
    println!("=================================");

    let mut text = String::from("hello");
    println!("Before function: '{}'", text);

    modify_string(&mut text);
    println!("After function: '{}'", text);

    println!("\nMUTABLE BORROWING PATTERNS:");
    println!("===========================");

    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", numbers);

    // Borrow mutably to modify
    let numbers_ref = &mut numbers;
    numbers_ref.push(6);
    numbers_ref[0] = 10;
    println!("Modified: {:?}", numbers);

    println!();
}

fn borrowing_rules() {
    println!("4. Borrowing Rules and Restrictions:");
    println!("===================================\n");

    println!("THE BORROWING RULES:");
    println!("===================");
    println!("1. You can have any number of immutable borrows");
    println!("2. You can have exactly one mutable borrow");
    println!("3. You cannot have both immutable and mutable borrows");
    println!("4. References must always be valid");

    println!("\nRULE 1: Multiple Immutable Borrows:");
    println!("==================================");

    let data = String::from("shared");
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;
    println!(
        "Multiple immutable borrows: '{}', '{}', '{}'",
        ref1, ref2, ref3
    );

    println!("\nRULE 2: Single Mutable Borrow:");
    println!("=============================");

    let mut data = String::from("mutable");
    let ref1 = &mut data;
    ref1.push_str(" data");
    println!("Single mutable borrow: '{}'", ref1);
    // let ref2 = &mut data;  // COMPILE ERROR: cannot borrow as mutable more than once!

    println!("\nRULE 3: No Mixing Immutable and Mutable:");
    println!("=======================================");

    let mut data = String::from("mixed");
    let immut_ref = &data; // Immutable borrow
    let immut_ref2 = &data; // Another immutable borrow
    println!("Immutable borrows: '{}', '{}'", immut_ref, immut_ref2);

    // let mut_ref = &mut data;  // COMPILE ERROR: cannot borrow as mutable!
    println!("Cannot have mutable borrow while immutable borrows exist");

    println!("\nRULE 4: References Must Be Valid:");
    println!("=================================");

    let valid_ref = create_valid_reference();
    println!("Valid reference: '{}'", valid_ref);

    // This would not compile:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // COMPILE ERROR: returns a reference to data owned by the current function
    // }

    println!("\nBORROWING SCOPE:");
    println!("===============");

    let mut data = String::from("scope test");
    {
        let ref1 = &data; // Immutable borrow
        let ref2 = &data; // Another immutable borrow
        println!("In scope: '{}', '{}'", ref1, ref2);
        // ref1 and ref2 go out of scope here
    }

    let ref3 = &mut data; // Now we can have a mutable borrow
    ref3.push_str(" modified");
    println!("After scope: '{}'", ref3);

    println!();
}

fn borrowing_with_functions() {
    println!("5. Borrowing with Functions:");
    println!("===========================\n");

    println!("FUNCTION PARAMETERS - Borrowing:");
    println!("===============================");

    let text = String::from("hello world");
    let length = get_length(&text); // Borrow text
    let word_count = count_words(&text); // Borrow text again
    println!("Text: '{}'", text);
    println!("Length: {}, Words: {}", length, word_count);
    println!("text is still valid after function calls");

    println!("\nMUTABLE FUNCTION PARAMETERS:");
    println!("============================");

    let mut text = String::from("hello");
    println!("Before: '{}'", text);

    append_world(&mut text); // Mutable borrow
    println!("After: '{}'", text);

    println!("\nRETURNING REFERENCES:");
    println!("====================");

    let text = String::from("hello world");
    let first_word = get_first_word(&text);
    println!("Text: '{}'", text);
    println!("First word: '{}'", first_word);

    println!("\nBORROWING WITH OPTION:");
    println!("=====================");

    let text = Some(String::from("hello"));
    if let Some(ref s) = text {
        // Borrow the String
        println!("Borrowed: '{}'", s);
    }
    println!("text is still valid: {:?}", text);

    println!("\nBORROWING WITH RESULT:");
    println!("=====================");

    let result: Result<String, &str> = Ok(String::from("success"));
    match &result {
        // Borrow the Result
        Ok(s) => println!("Success: '{}'", s),
        Err(e) => println!("Error: {}", e),
    }
    println!("result is still valid: {:?}", result);

    println!();
}

fn borrowing_with_collections() {
    println!("6. Borrowing with Collections:");
    println!("=============================\n");

    println!("VECTOR BORROWING:");
    println!("================");

    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", numbers);

    // Immutable borrow
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    println!("numbers is still valid: {:?}", numbers);

    // Mutable borrow
    let numbers_ref = &mut numbers;
    numbers_ref.push(6);
    numbers_ref[0] = 10;
    println!("Modified: {:?}", numbers);

    println!("\nITERATION WITH BORROWING:");
    println!("=========================");

    let words = vec![String::from("hello"), String::from("world")];

    // Iterate with immutable borrows
    for word in &words {
        println!("Word: '{}'", word);
    }
    println!("words is still valid: {:?}", words);

    // Iterate with mutable borrows
    let mut mutable_words = vec![String::from("hello"), String::from("world")];
    for word in &mut mutable_words {
        word.push_str("!");
    }
    println!("Modified words: {:?}", mutable_words);

    println!("\nCOLLECTION METHODS WITH BORROWING:");
    println!("=================================");

    let numbers = vec![1, 2, 3, 4, 5];

    // get() returns Option<&T>
    if let Some(num) = numbers.get(2) {
        println!("Element at index 2: {}", num);
    }

    // first() and last() return Option<&T>
    if let Some(first) = numbers.first() {
        println!("First element: {}", first);
    }

    if let Some(last) = numbers.last() {
        println!("Last element: {}", last);
    }

    println!("\nBORROWING WITH HASHMAP:");
    println!("=======================");

    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(String::from("key1"), 42);
    map.insert(String::from("key2"), 100);

    // Immutable borrow
    if let Some(value) = map.get("key1") {
        println!("Value for key1: {}", value);
    }

    // Mutable borrow
    if let Some(value) = map.get_mut("key2") {
        *value += 50;
        println!("Modified value for key2: {}", value);
    }

    println!("Map: {:?}", map);

    println!();
}

fn borrowing_with_structs() {
    println!("7. Borrowing with Structs:");
    println!("=========================\n");

    println!("STRUCT FIELDS - Borrowing:");
    println!("=========================");

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: Some(String::from("alice@example.com")),
    };

    // Borrow individual fields
    let name_ref = &person.name;
    let age_ref = &person.age;
    println!("Name: '{}', Age: {}", name_ref, age_ref);

    // Borrow the entire struct
    let person_ref = &person;
    println!("Person: {:?}", person_ref);

    println!("\nMUTABLE STRUCT BORROWING:");
    println!("=========================");

    let mut person = Person {
        name: String::from("Bob"),
        age: 25,
        email: None,
    };

    // Mutable borrow of individual field
    let age_ref = &mut person.age;
    *age_ref += 1;
    println!("Updated age: {}", age_ref);

    // Mutable borrow of entire struct
    let person_ref = &mut person;
    person_ref.name.push_str(" Smith");
    person_ref.email = Some(String::from("bob.smith@example.com"));
    println!("Updated person: {:?}", person_ref);

    println!("\nSTRUCT METHODS WITH BORROWING:");
    println!("==============================");

    let mut person = Person {
        name: String::from("Charlie"),
        age: 35,
        email: Some(String::from("charlie@example.com")),
    };

    // Immutable method
    person.print_info();

    // Mutable method
    person.have_birthday();
    person.print_info();

    println!("\nBORROWING WITH ENUMS:");
    println!("=====================");

    let message = Message::Write(String::from("hello"));

    match &message {
        // Borrow the enum
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(s) => println!("Write: '{}'", s),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }

    println!("message is still valid: {:?}", message);

    println!();
}

fn advanced_borrowing_patterns() {
    println!("8. Advanced Borrowing Patterns:");
    println!("==============================\n");

    println!("BORROWING WITH CLOSURES:");
    println!("=======================");

    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before closure: {:?}", list);

    // Closure that borrows immutably
    let print_list = || println!("List: {:?}", list);
    print_list();

    // Closure that borrows mutably
    let mut add_element = || list.push(6);
    add_element();
    println!("After closure: {:?}", list);

    println!("\nBORROWING WITH ITERATORS:");
    println!("=========================");

    let numbers = vec![1, 2, 3, 4, 5];

    // Iterator with immutable borrows
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);

    // Iterator with mutable borrows
    let mut numbers = vec![1, 2, 3, 4, 5];
    for num in &mut numbers {
        *num *= 2;
    }
    println!("Modified in place: {:?}", numbers);

    println!("\nBORROWING WITH SLICES:");
    println!("=====================");

    let text = String::from("hello world");
    let hello = &text[0..5]; // Borrow a slice
    let world = &text[6..11]; // Borrow another slice

    println!("Text: '{}'", text);
    println!("Hello: '{}'", hello);
    println!("World: '{}'", world);

    println!("\nBORROWING WITH SMART POINTERS:");
    println!("=============================");

    use std::rc::Rc;
    let data = Rc::new(String::from("shared data"));
    let ref1 = Rc::clone(&data);
    let ref2 = Rc::clone(&data);

    println!("data: '{}'", data);
    println!("ref1: '{}'", ref1);
    println!("ref2: '{}'", ref2);
    println!("Reference count: {}", Rc::strong_count(&data));

    println!("\nBORROWING WITH BOX:");
    println!("==================");

    let boxed_data = Box::new(42);
    let box_ref = &boxed_data;
    println!("Boxed value: {}", box_ref);

    println!();
}

fn borrowing_and_lifetimes() {
    println!("9. Borrowing and Lifetimes:");
    println!("==========================\n");

    println!("LIFETIMES - Ensuring Reference Validity:");
    println!("=======================================");

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("Longest string: '{}'", result);

    println!("\nLIFETIME ANNOTATIONS:");
    println!("====================");

    // Function with explicit lifetime
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("hello");
    let string2 = String::from("world");
    let result = longest(&string1, &string2);
    println!("Longest: '{}'", result);

    println!("\nSTRUCTS WITH LIFETIMES:");
    println!("======================");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: '{}'", i.part);

    println!("\nLIFETIME ELISION:");
    println!("================");

    // These functions have elided lifetimes
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let text = String::from("hello world");
    let word = first_word(&text);
    println!("First word: '{}'", word);

    println!("\nSTATIC LIFETIME:");
    println!("===============");

    let s: &'static str = "I have a static lifetime.";
    println!("Static string: '{}'", s);

    println!();
}

fn common_borrowing_scenarios() {
    println!("10. Common Borrowing Scenarios:");
    println!("==============================\n");

    println!("SCENARIO 1: Reading Configuration:");
    println!("=================================");

    let config = Config {
        host: String::from("localhost"),
        port: 8080,
        timeout: 30,
    };

    let host = &config.host;
    let port = &config.port;
    println!("Host: {}, Port: {}", host, port);

    println!("\nSCENARIO 2: Modifying Data in Place:");
    println!("====================================");

    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Before: {:?}", numbers);

    // Modify in place using mutable borrow
    let numbers_ref = &mut numbers;
    for num in numbers_ref {
        *num *= 2;
    }
    println!("After: {:?}", numbers);

    println!("\nSCENARIO 3: Conditional Borrowing:");
    println!("==================================");

    let mut data = Some(String::from("hello"));

    if let Some(ref s) = data {
        println!("Data exists: '{}'", s);
    }

    if let Some(ref mut s) = data {
        s.push_str(" world");
        println!("Modified data: '{}'", s);
    }

    println!("\nSCENARIO 4: Borrowing with Error Handling:");
    println!("==========================================");

    let result: Result<String, &str> = Ok(String::from("success"));

    match &result {
        Ok(s) => println!("Success: '{}'", s),
        Err(e) => println!("Error: {}", e),
    }

    // result is still valid after the match
    println!("Result is still valid: {:?}", result);

    println!("\nSCENARIO 5: Borrowing in Loops:");
    println!("===============================");

    let mut items = vec![1, 2, 3, 4, 5];

    // Immutable borrow in loop
    for item in &items {
        println!("Item: {}", item);
    }

    // Mutable borrow in loop
    for item in &mut items {
        *item += 10;
    }

    println!("Modified items: {:?}", items);

    println!("\nBORROWING BEST PRACTICES:");
    println!("========================");
    println!("1. Use the smallest scope possible for borrows");
    println!("2. Prefer immutable borrows when possible");
    println!("3. Use references to avoid unnecessary copying");
    println!("4. Understand the borrowing rules thoroughly");
    println!("5. Use appropriate lifetime annotations");

    println!();
}

// Helper functions

fn calculate_length(s: &String) -> usize {
    println!("Calculating length of '{}'", s);
    s.len()
}

fn takes_ownership(s: String) {
    println!("Takes ownership of '{}'", s);
}

fn borrows_data(s: &String) {
    println!("Borrows data: '{}'", s);
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn count_chars(s: &str) -> usize {
    s.chars().count()
}

fn modify_string(s: &mut String) {
    s.push_str(" modified");
    println!("Modified string to: '{}'", s);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn create_valid_reference() -> &'static str {
    "This is a valid static reference"
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

impl Person {
    fn print_info(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
        if let Some(ref email) = self.email {
            println!("Email: {}", email);
        }
    }

    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday! You are now {} years old.", self.age);
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
}

// Main function to run all borrowing examples
fn main() {
    borrowing();
}

/// Ownership in Rust - Memory Safety Without Garbage Collection
///
/// Ownership is Rust's most unique feature and has deep implications for the language.
/// It enables Rust to make memory safety guarantees without needing a garbage collector.
/// This comprehensive guide covers from basic concepts to advanced patterns.
use std::io;

pub fn ownership() {
    println!("=== Ownership Learning Examples ===\n");

    // 1. Basic Ownership Rules
    basic_ownership_rules();

    // 2. Ownership and Functions
    ownership_and_functions();

    // 3. References and Borrowing
    references_and_borrowing();

    // 4. Mutable References
    mutable_references();

    // 5. Slices
    slices();

    // 6. Ownership with Collections
    ownership_with_collections();

    // 7. Advanced Ownership Patterns
    advanced_ownership_patterns();

    // 8. Memory Management Deep Dive
    memory_management_deep_dive();

    // 9. Ownership with Custom Types
    ownership_with_custom_types();

    // 10. Advanced Borrowing Patterns
    advanced_borrowing_patterns();
}

fn basic_ownership_rules() {
    println!("1. Basic Ownership Rules:");
    println!("========================\n");

    println!("RULE 1: Each value has exactly one owner (only heap-allocated values can be owned)");
    println!("----------------------------------------------------------------------------");
    let s1 = String::from("hello"); // s1 is the owner of this heap-allocated String
    println!("s1 owns: '{}' (heap-allocated)", s1);

    let x = 5; // x is NOT an owner - this is stack-allocated
    println!("x: {} (stack-allocated, not owned)", x);

    println!("\nRULE 2: There can only be one owner at a time");
    println!("--------------------------------------------");
    let s2 = s1; // s1's value MOVES to s2 (ownership transfer)
    // println!("s1: {}", s1);  // COMPILE ERROR: s1 no longer owns the value!
    println!("s2 now owns: '{}'", s2);
    println!("s1 is no longer valid after the move");

    println!("\nRULE 3: When the owner goes out of scope, the value is dropped");
    println!("----------------------------------------------------------");
    {
        let s3 = String::from("world");
        println!("s3 in scope: '{}'", s3);
        // s3 will be dropped when this block ends
    } // s3 goes out of scope and is dropped (memory freed)
    println!("s3 has been dropped and memory freed");

    println!("\nSTACK vs HEAP - Understanding Memory Allocation:");
    println!("===============================================");

    // Stack allocation (Copy types)
    let x = 5; // Stack allocated - fixed size, fast access
    let y = x; // COPY (not move) - because i32 implements Copy trait
    println!("Stack: x = {}, y = {} (both valid after assignment)", x, y);
    println!("Stack allocation: fixed size, fast, automatic cleanup");

    // Heap allocation (Move types)
    let s4 = String::from("hello"); // Heap allocated - dynamic size
    let s5 = s4; // MOVE (not copy) - ownership transferred
    println!("Heap: s5 = '{}' (s4 is no longer valid)", s5);
    println!("Heap allocation: dynamic size, slower, manual cleanup via ownership");

    println!("\nCopy vs Move - Understanding the Difference:");
    println!("===========================================");

    // Copy types (stack-allocated)
    let a = 42;
    let b = a; // Copy
    println!("Copy: a = {}, b = {} (both valid)", a, b);

    // Move types (heap-allocated)
    let s6 = String::from("hello");
    let s7 = s6; // Move
    println!("Move: s7 = '{}' (s6 is invalid)", s7);

    println!();
}

fn ownership_and_functions() {
    println!("2. Ownership and Functions:");
    println!("==========================\n");

    println!("FUNCTION PARAMETERS - Ownership Transfer:");
    println!("=======================================");

    let s = String::from("hello");
    println!("Before function call: '{}'", s);
    println!("s owns the String");

    takes_ownership(s); // s's value MOVES into the function
    println!("After function call: s is no longer valid");
    // println!("s: {}", s);  // COMPILE ERROR: s was moved!

    println!("\nCOPY TYPES - No Ownership Transfer:");
    println!("===================================");

    let x = 5;
    println!("Before function call: x = {}", x);
    makes_copy(x); // x is COPIED (not moved) because i32 implements Copy
    println!("After copy function: x = {} (still valid)", x);

    println!("\nRETURN VALUES - Ownership Transfer Back:");
    println!("=======================================");

    let s1 = gives_ownership(); // Function gives ownership to s1
    println!("s1 received ownership: '{}'", s1);

    let s2 = String::from("hello");
    println!("s2 owns: '{}'", s2);
    let s3 = takes_and_gives_back(s2); // s2 moves in, return value moves to s3
    println!("s3 received ownership: '{}'", s3);
    println!("s2 is no longer valid");

    println!("\nOWNERSHIP FLOW - Understanding the Journey:");
    println!("==========================================");

    let original = String::from("original");
    println!("1. original owns: '{}'", original);

    let moved = original; // Move 1: original → moved
    println!("2. moved owns: '{}'", moved);
    println!("   original is no longer valid");

    let returned = takes_and_gives_back(moved); // Move 2: moved → function → returned
    println!("3. returned owns: '{}'", returned);
    println!("   moved is no longer valid");

    println!();
}

fn references_and_borrowing() {
    println!("3. References and Borrowing:");
    println!("============================\n");

    println!("BORROWING - Access Without Ownership:");
    println!("====================================");

    let s1 = String::from("hello");
    println!("s1 owns: '{}'", s1);

    let len = calculate_length(&s1); // &s1 creates a reference (borrow)
    println!("The length of '{}' is {}.", s1, len);
    println!("s1 is still valid after borrowing!");

    println!("\nIMMUTABLE REFERENCES - Read-Only Access:");
    println!("=======================================");

    let s2 = String::from("hello");
    // change(&s2);  // COMPILE ERROR: cannot borrow as mutable!
    println!("Immutable references cannot modify the data");

    println!("\nMULTIPLE IMMUTABLE REFERENCES - Shared Read Access:");
    println!("==================================================");

    let s3 = String::from("hello");
    let r1 = &s3; // First immutable reference
    let r2 = &s3; // Second immutable reference
    let r3 = &s3; // Third immutable reference
    println!(
        "Multiple immutable references: r1='{}', r2='{}', r3='{}'",
        r1, r2, r3
    );
    println!("All can read the same data simultaneously");

    println!("\nREFERENCE LIFETIME - Understanding Scope:");
    println!("========================================");

    let s4 = String::from("hello");
    {
        let r1 = &s4; // Reference created
        println!("r1: '{}'", r1);
        // r1 goes out of scope here
    }
    println!("s4 is still valid: '{}'", s4);

    println!("\nNO DANGLING REFERENCES - Rust Prevents This:");
    println!("============================================");

    let reference_to_nothing = dangle();
    println!("Reference: '{}'", reference_to_nothing);
    println!("Rust prevents dangling references at compile time");

    println!("\nBORROWING RULES SUMMARY:");
    println!("========================");
    println!("1. You can have any number of immutable references");
    println!("2. You can have exactly one mutable reference");
    println!("3. You cannot have both immutable and mutable references");
    println!("4. References must always be valid");

    println!();
}

fn mutable_references() {
    println!("4. Mutable References:");
    println!("======================\n");

    println!("MUTABLE BORROWING - Modify Without Ownership:");
    println!("============================================");

    let mut s = String::from("hello");
    println!("Before change: '{}'", s);
    change(&mut s); // &mut s creates a mutable reference
    println!("After change: '{}'", s);

    println!("\nEXCLUSIVE MUTABLE ACCESS - Only One at a Time:");
    println!("==============================================");

    let mut s1 = String::from("hello");
    let r1 = &mut s1; // First mutable reference
    println!("r1: '{}'", r1);
    // let r2 = &mut s1;  // COMPILE ERROR: cannot borrow as mutable more than once!
    println!("Cannot have multiple mutable references simultaneously");

    println!("\nIMMUTABLE vs MUTABLE - Cannot Mix:");
    println!("==================================");

    let mut s2 = String::from("hello");
    let r1 = &s2; // Immutable reference
    let r2 = &s2; // Another immutable reference
    println!("Immutable references: r1='{}', r2='{}'", r1, r2);
    // let r3 = &mut s2;  // COMPILE ERROR: cannot borrow as mutable!
    println!("Cannot have mutable reference while immutable ones exist");

    println!("\nREFERENCE SCOPE - Understanding When References End:");
    println!("==================================================");

    let mut s3 = String::from("hello");
    {
        let r1 = &s3; // Immutable reference
        let r2 = &s3; // Another immutable reference
        println!("Immutable references: r1='{}', r2='{}'", r1, r2);
        // r1 and r2 go out of scope here
    }

    let r3 = &mut s3; // Now we can have a mutable reference
    println!("Mutable reference: r3='{}'", r3);
    println!("Previous immutable references are out of scope");

    println!("\nMUTABLE REFERENCE RULES:");
    println!("========================");
    println!("1. Only one mutable reference at a time");
    println!("2. Cannot have mutable and immutable references simultaneously");
    println!("3. Mutable references can modify the data");
    println!("4. Reference scope ends at last use");

    println!();
}

fn slices() {
    println!("5. Slices:");
    println!("==========\n");

    println!("STRING SLICES - References to String Data:");
    println!("=========================================");

    let s = String::from("hello world");
    println!("Original string: '{}'", s);

    let hello = &s[0..5]; // Slice from index 0 to 4 (exclusive)
    let world = &s[6..11]; // Slice from index 6 to 10 (exclusive)
    println!("Slices: hello='{}', world='{}'", hello, world);

    // Shorthand syntax
    let hello_short = &s[..5]; // From start to index 4
    let world_short = &s[6..]; // From index 6 to end
    let full_short = &s[..]; // Entire string
    println!(
        "Shorthand: hello='{}', world='{}', full='{}'",
        hello_short, world_short, full_short
    );

    println!("\nSLICE TYPE - &str:");
    println!("==================");

    let s1 = String::from("hello world");
    let word = first_word(&s1);
    println!("First word of '{}': '{}'", s1, word);
    println!("word is of type &str (string slice)");

    println!("\nARRAY SLICES - References to Array Data:");
    println!("========================================");

    let a = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", a);

    let slice = &a[1..3]; // Slice from index 1 to 2 (exclusive)
    println!("Array slice: {:?}", slice);
    println!("Slice type: &[i32]");

    println!("\nSLICE ADVANTAGES:");
    println!("================");
    println!("1. No copying of data");
    println!("2. Efficient memory usage");
    println!("3. Type safety (bounds checking)");
    println!("4. Clear ownership semantics");

    println!("\nSLICE BOUNDS - Runtime Safety:");
    println!("=============================");

    let s2 = String::from("hello");
    // let slice = &s2[0..10];  // This would panic at runtime!
    println!("Slices are bounds-checked at runtime");
    println!("Invalid slice ranges cause panic");

    println!();
}

fn ownership_with_collections() {
    println!("6. Ownership with Collections:");
    println!("=============================\n");

    println!("VECTOR OWNERSHIP - Collections Own Their Data:");
    println!("=============================================");

    let mut v = Vec::new();
    v.push(String::from("hello")); // Vector takes ownership
    v.push(String::from("world")); // Vector takes ownership
    println!("Vector owns: {:?}", v);

    println!("\nMOVING OUT OF COLLECTIONS:");
    println!("==========================");

    let first = v.remove(0); // Ownership transferred from vector to first
    println!("Removed: '{}'", first);
    println!("Vector after removal: {:?}", v);
    println!("first now owns the String");

    println!("\nITERATION WITH OWNERSHIP:");
    println!("=========================");

    let v2 = vec![String::from("hello"), String::from("world")];
    println!("Before iteration: {:?}", v2);

    for s in v2 {
        // v2 is MOVED into the for loop
        println!("String: '{}'", s);
        // s owns each String during iteration
    }
    // println!("v2: {:?}", v2);  // COMPILE ERROR: v2 was moved!
    println!("v2 is no longer valid after iteration");

    println!("\nITERATION WITH REFERENCES:");
    println!("==========================");

    let v3 = vec![String::from("hello"), String::from("world")];
    println!("Before iteration: {:?}", v3);

    for s in &v3 {
        // v3 is BORROWED (not moved)
        println!("String: '{}'", s);
        // s is a reference to each String
    }
    println!("v3 after iteration: {:?}", v3);
    println!("v3 is still valid after iteration");

    println!("\nITERATION WITH MUTABLE REFERENCES:");
    println!("==================================");

    let mut v4 = vec![String::from("hello"), String::from("world")];
    println!("Before modification: {:?}", v4);

    for s in &mut v4 {
        // v4 is mutably borrowed
        s.push_str("!"); // Modify each String
        println!("Modified: '{}'", s);
    }
    println!("v4 after modification: {:?}", v4);

    println!("\nCOLLECTION OWNERSHIP RULES:");
    println!("===========================");
    println!("1. Collections own their elements");
    println!("2. Moving out transfers ownership");
    println!("3. Iterating with 'for' moves the collection");
    println!("4. Iterating with 'for &' borrows the collection");
    println!("5. Iterating with 'for &mut' mutably borrows the collection");

    println!();
}

fn advanced_ownership_patterns() {
    println!("7. Advanced Ownership Patterns:");
    println!("==============================\n");

    println!("CLONE - When You Need Ownership:");
    println!("===============================");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy - both own their data
    println!("s1: '{}', s2: '{}'", s1, s2);
    println!("Both s1 and s2 are valid after cloning");
    println!("Clone is expensive but gives you ownership");

    println!("\nCOPY TRAIT - Automatic Copying:");
    println!("==============================");

    let x = 5;
    let y = x; // Copy (not move) - because i32 implements Copy
    println!("x: {}, y: {} (both valid after assignment)", x, y);
    println!("Copy is cheap and automatic for simple types");

    println!("\nSTRUCT OWNERSHIP - Fields Can Be Moved:");
    println!("======================================");

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person: {:?}", person);

    let name = person.name; // person.name is moved out
    println!("name: '{}'", name);
    // println!("person: {:?}", person);  // COMPILE ERROR: person.name was moved!
    println!("person.age is still valid: {}", person.age);

    println!("\nBOX<T> - Heap Allocation with Single Ownership:");
    println!("==============================================");

    let b = Box::new(5); // Allocate on heap, b owns the Box
    println!("Boxed value: {}", b);
    println!("Box provides heap allocation with single ownership");

    println!("\nRC<T> - Shared Ownership (Single Thread):");
    println!("=========================================");

    use std::rc::Rc;
    let data = Rc::new(String::from("shared data"));
    println!("Reference count: {}", Rc::strong_count(&data));

    let data_clone1 = Rc::clone(&data); // Share ownership
    println!("Reference count: {}", Rc::strong_count(&data));

    let data_clone2 = Rc::clone(&data); // Share ownership
    println!("Reference count: {}", Rc::strong_count(&data));

    println!(
        "data: '{}', clone1: '{}', clone2: '{}'",
        data, data_clone1, data_clone2
    );
    println!("All references point to the same data");

    println!("\nARC<T> - Thread-Safe Shared Ownership:");
    println!("======================================");

    use std::sync::Arc;
    let shared_data = Arc::new(String::from("thread-safe data"));
    let shared_clone = Arc::clone(&shared_data);
    println!("shared_data: '{}', clone: '{}'", shared_data, shared_clone);
    println!("Arc provides thread-safe reference counting");

    println!();
}

fn memory_management_deep_dive() {
    println!("8. Memory Management Deep Dive:");
    println!("==============================\n");

    println!("STACK vs HEAP - Detailed Comparison:");
    println!("===================================");

    println!("STACK:");
    println!("- Fixed size, known at compile time");
    println!("- Fast allocation and deallocation");
    println!("- Automatic cleanup when variable goes out of scope");
    println!("- LIFO (Last In, First Out) structure");
    println!("- Used for: local variables, function parameters");

    println!("\nHEAP:");
    println!("- Dynamic size, unknown at compile time");
    println!("- Slower allocation and deallocation");
    println!("- Manual cleanup via ownership system");
    println!("- Can be fragmented");
    println!("- Used for: large data, data that outlives function");

    println!("\nOWNERSHIP AND MEMORY SAFETY:");
    println!("============================");

    println!("1. No null pointer dereferences");
    println!("2. No dangling pointers");
    println!("3. No double frees");
    println!("4. No use-after-free errors");
    println!("5. No data races (with proper borrowing)");

    println!("\nMEMORY LEAK PREVENTION:");
    println!("=======================");

    println!("- Automatic cleanup when owner goes out of scope");
    println!("- No manual memory management required");
    println!("- Compiler ensures all memory is freed");
    println!("- No garbage collection overhead");

    println!();
}

fn ownership_with_custom_types() {
    println!("9. Ownership with Custom Types:");
    println!("==============================\n");

    println!("CUSTOM STRUCTS - Owned vs Borrowed Fields:");
    println!("=========================================");

    let person = Person {
        name: String::from("Alice"), // Owned field
        age: 30,                     // Copy field
    };
    println!("Person: {:?}", person);

    println!("\nMOVING STRUCT FIELDS:");
    println!("=====================");

    let name = person.name; // Move the owned field
    println!("name: '{}'", name);
    // println!("person: {:?}", person);  // person.name is no longer valid

    println!("\nCOPY STRUCT FIELDS:");
    println!("===================");

    let age = person.age; // Copy the Copy field
    println!("age: {}", age);
    println!("person.age is still valid: {}", person.age);

    println!("\nCUSTOM TYPES WITH REFERENCES:");
    println!("=============================");

    let text = String::from("hello world");
    let word = first_word(&text);
    println!("text: '{}', first word: '{}'", text, word);
    println!("word is a reference to part of text");

    println!("\nOWNERSHIP IN ENUMS:");
    println!("===================");

    #[derive(Debug)]
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Copy data
        Write(String),              // Owned data
        ChangeColor(i32, i32, i32), // Copy data
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("Messages: {:?}, {:?}, {:?}, {:?}", msg1, msg2, msg3, msg4);

    println!();
}

fn advanced_borrowing_patterns() {
    println!("10. Advanced Borrowing Patterns:");
    println!("================================\n");

    println!("BORROWING WITH LIFETIMES:");
    println!("=========================");

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("Longest string: '{}'", result);

    println!("\nBORROWING WITH STRUCTS:");
    println!("=======================");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: '{}'", i.part);

    println!("\nBORROWING WITH ITERATORS:");
    println!("=========================");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);
    println!("numbers is still valid: {:?}", numbers);

    println!("\nBORROWING WITH CLOSURES:");
    println!("=======================");

    let mut list = vec![1, 2, 3];
    println!("Before closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After closure: {:?}", list);

    println!("\nADVANCED BORROWING RULES:");
    println!("=========================");
    println!("1. References must always be valid");
    println!("2. You can't have data races");
    println!("3. You can't have use-after-free");
    println!("4. The compiler enforces these rules");

    println!();
}

// Helper functions

fn takes_ownership(some_string: String) {
    println!("takes_ownership: '{}'", some_string);
    println!("some_string owns the data");
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
    println!("some_integer is copied, not moved");
} // some_integer goes out of scope, but nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    println!("gives_ownership: creating '{}'", some_string);
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("takes_and_gives_back: received '{}'", a_string);
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    println!("calculate_length: borrowing '{}'", s);
    s.len()
} // s goes out of scope, but because it does not have ownership of what it refers to, nothing happens

fn change(some_string: &mut String) {
    println!("change: mutably borrowing '{}'", some_string);
    some_string.push_str(", world");
    println!("change: modified to '{}'", some_string);
}

fn dangle() -> &'static str {
    println!("dangle: returning static string");
    "hello" // Return a string literal (static lifetime)
}

fn first_word(s: &str) -> &str {
    println!("first_word: finding first word in '{}'", s);
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Main function to run all ownership examples
fn main() {
    ownership();
}

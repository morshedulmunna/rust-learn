# Match.rs - Pattern Matching

## Overview

The `match.rs` file demonstrates Rust's powerful pattern matching capabilities using the `match` expression, which is exhaustive and type-safe.

## Code Analysis

```rust
fn match_example() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_with_multiple_patterns() {
    let x = 5;
    match x {
        1 | 2 | 3 => println!("one, two, or three"),
        _ => println!("anything"),
    }
}

fn match_with_range() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn match_with_binding() {
    let x = 5;
    match x {
        x => println!("x is {x}"),
    }
}

fn match_with_guard() {
    let x = 5;
    match x {
        x if x % 2 == 0 => println!("x is even"),
        x if x % 2 != 0 => println!("x is odd"),
        _ => println!("x is not a number"),
    }
}
```

## Key Concepts

### 1. Basic Match Expression

```rust
let x = 5;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

**Characteristics:**

- Exhaustive: must cover all possible cases
- `_` is the catch-all pattern (default case)
- Each arm is separated by `,`
- Last arm doesn't need a comma
- Arms are evaluated in order

### 2. Multiple Patterns

```rust
let x = 5;
match x {
    1 | 2 | 3 => println!("one, two, or three"),
    _ => println!("anything"),
}
```

**Key Points:**

- Use `|` (pipe) to match multiple patterns
- All patterns in the same arm must have the same type
- Useful for grouping similar cases

### 3. Range Patterns

```rust
let x = 5;
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

**Key Points:**

- `1..=5` is inclusive range (includes 5)
- `1..5` is exclusive range (excludes 5)
- Can use with numeric types and characters
- Useful for checking value ranges

### 4. Pattern Binding

```rust
let x = 5;
match x {
    x => println!("x is {x}"),
}
```

**Key Points:**

- Binds the matched value to a variable
- Variable name can be the same as the original
- Useful for extracting values from complex patterns

### 5. Guards

```rust
let x = 5;
match x {
    x if x % 2 == 0 => println!("x is even"),
    x if x % 2 != 0 => println!("x is odd"),
    _ => println!("x is not a number"),
}
```

**Key Points:**

- Use `if` condition after pattern
- Additional filtering beyond pattern matching
- Can use bound variables in guard conditions
- Guards are evaluated in order

## Advanced Patterns

### 1. Destructuring

```rust
// Tuple destructuring
let point = (3, -7);
match point {
    (0, y) => println!("On y-axis at {}", y),
    (x, 0) => println!("On x-axis at {}", x),
    (x, y) => println!("At ({}, {})", x, y),
}

// Struct destructuring
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 0, y: 7 };
match point {
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

### 2. Enum Matching

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Move { x: 1, y: 2 };
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
}
```

### 3. Reference Patterns

```rust
let reference = &4;
match reference {
    &val => println!("Got a value via destructuring: {}", val),
}

// Or use ref pattern
let value = 5;
match value {
    ref r => println!("Got a reference to a value: {}", r),
}
```

### 4. Multiple Guards

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    Some(n) if n < 5 => println!("Less than 5: {}", n),
    Some(n) if n > 5 => println!("Greater than 5: {}", n),
    Some(_) => println!("Other value"),
    None => println!("No value"),
}
```

## Common Patterns

### 1. Option Handling

```rust
let some_value = Some(5);
match some_value {
    Some(value) => println!("Got value: {}", value),
    None => println!("No value"),
}

// With guards
let some_number = Some(4);
match some_number {
    Some(x) if x < 5 => println!("Less than 5: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### 2. Result Handling

```rust
let result: Result<i32, &str> = Ok(5);
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

### 3. Character Matching

```rust
let character = 'c';
match character {
    'a'..='j' => println!("Early ASCII letter"),
    'k'..='z' => println!("Late ASCII letter"),
    _ => println!("Something else"),
}
```

### 4. Array Matching

```rust
let arr = [1, 2, 3];
match arr {
    [first, second, third] => println!("Array: [{}, {}, {}]", first, second, third),
    [first, ..] => println!("Array starts with: {}", first),
    [] => println!("Empty array"),
}
```

## Best Practices

### 1. Exhaustiveness

```rust
// Good: Covers all cases
match value {
    1 => "one",
    2 => "two",
    _ => "other",
}

// Bad: Not exhaustive (will not compile)
match value {
    1 => "one",
    2 => "two",
    // Missing catch-all
}
```

### 2. Order Matters

```rust
// Good: Specific patterns first
match value {
    1 => "one",
    2 => "two",
    x if x > 10 => "large",
    _ => "other",
}

// Bad: Catch-all before specific patterns
match value {
    1 => "one",
    _ => "other", // This will catch everything!
    2 => "two",   // Unreachable
}
```

### 3. Use Guards for Complex Logic

```rust
// Good: Use guards for complex conditions
match value {
    x if x % 2 == 0 && x > 10 => "large even",
    x if x % 2 == 0 => "even",
    x if x > 10 => "large odd",
    _ => "small odd",
}
```

## Common Mistakes

### ❌ Non-exhaustive match

```rust
// Bad: Will not compile
let x = 5;
match x {
    1 => println!("one"),
    2 => println!("two"),
    // Missing cases for 3, 4, 5, etc.
}
```

### ❌ Unreachable patterns

```rust
// Bad: Unreachable pattern
match x {
    1 => println!("one"),
    _ => println!("anything"),
    2 => println!("two"), // Unreachable
}
```

### ❌ Wrong pattern syntax

```rust
// Bad: Wrong range syntax
match x {
    1..5 => println!("range"), // Should be 1..=5 for inclusive
    _ => println!("other"),
}
```

### ✅ Correct patterns

```rust
// Good: Exhaustive and well-ordered
match x {
    1 => println!("one"),
    2 => println!("two"),
    x if x > 10 => println!("large"),
    _ => println!("other"),
}
```

## Performance Considerations

### 1. Pattern Order

```rust
// Good: Most common patterns first
match value {
    "common" => "frequent case",
    "less_common" => "less frequent",
    _ => "rare case",
}
```

### 2. Guard Efficiency

```rust
// Good: Simple guards first
match value {
    x if x < 5 => "small",
    x if x < 10 => "medium",
    x if x < 100 => "large",
    _ => "huge",
}
```

## Exercises

1. **Number Classifier**: Create a match expression that classifies numbers into categories
2. **Enum Processor**: Create an enum and write match expressions to handle all variants
3. **Pattern Extractor**: Use pattern binding to extract values from complex structures
4. **Guard Master**: Write match expressions using various guard conditions
5. **Range Matcher**: Create patterns that match different ranges of values

## Related Concepts

- **Enums**: Using match with custom enum types
- **Option and Result**: Error handling with pattern matching
- **Destructuring**: Extracting values from complex types
- **Control Flow**: How match fits into program flow

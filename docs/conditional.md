# Conditional.rs - Conditional Statements

## Overview

The `conditional.rs` file demonstrates how to use conditional statements (`if`, `else if`, `else`) and conditional expressions in Rust.

## Code Analysis

```rust
fn conditional() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");
}
```

## Key Concepts

### 1. Basic If-Else Statement

```rust
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

**Characteristics:**

- Condition must be a `bool` type
- Parentheses around condition are optional
- Curly braces are required for blocks
- `else` block is optional

### 2. Multiple Conditions with Else If

```rust
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

**Key Points:**

- Conditions are evaluated in order
- First true condition executes
- `else` handles all remaining cases
- Only one block executes

### 3. If as Expression

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

**Characteristics:**

- `if` can be used as an expression
- All branches must return the same type
- No semicolon after the last expression
- Useful for conditional assignment

### 4. Loop with Break Value

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

**Key Points:**

- `loop` creates an infinite loop
- `break` exits the loop
- `break` can return a value
- The value becomes the result of the loop expression

## Advanced Patterns

### 1. Nested Conditions

```rust
let x = 5;
let y = 10;

if x > 0 {
    if y > 0 {
        println!("Both x and y are positive");
    } else {
        println!("x is positive, y is not");
    }
} else {
    println!("x is not positive");
}
```

### 2. Compound Conditions

```rust
let age = 25;
let has_permission = true;

if age >= 18 && has_permission {
    println!("Access granted");
} else {
    println!("Access denied");
}
```

### 3. Pattern Matching in Conditions

```rust
let value = Some(5);

if let Some(x) = value {
    println!("Value is: {}", x);
} else {
    println!("No value");
}
```

### 4. Conditional Assignment with Multiple Branches

```rust
let score = 85;
let grade = if score >= 90 {
    "A"
} else if score >= 80 {
    "B"
} else if score >= 70 {
    "C"
} else if score >= 60 {
    "D"
} else {
    "F"
};
println!("Grade: {}", grade);
```

## Best Practices

### 1. Condition Clarity

```rust
// Good: Clear condition
if user.is_authenticated() && user.has_permission("admin") {
    // ...
}

// Bad: Complex condition
if user != None && user.unwrap().auth == true && user.unwrap().perms.contains("admin") {
    // ...
}
```

### 2. Early Returns

```rust
// Good: Early return pattern
fn process_user(user: Option<User>) -> Result<(), Error> {
    if user.is_none() {
        return Err(Error::UserNotFound);
    }

    let user = user.unwrap();
    if !user.is_authenticated() {
        return Err(Error::NotAuthenticated);
    }

    // Process user...
    Ok(())
}
```

### 3. Guard Clauses

```rust
// Good: Guard clauses
fn calculate_discount(price: f64, is_member: bool, age: u32) -> f64 {
    if price <= 0.0 {
        return 0.0;
    }

    if !is_member {
        return 0.0;
    }

    if age < 18 {
        return price * 0.1; // 10% discount for minors
    }

    price * 0.2 // 20% discount for adult members
}
```

## Common Patterns

### 1. Range Checking

```rust
let value = 42;
if value >= 0 && value <= 100 {
    println!("Value is in valid range");
} else {
    println!("Value is out of range");
}
```

### 2. Null/None Checking

```rust
let option_value: Option<i32> = Some(5);

if let Some(value) = option_value {
    println!("Value is: {}", value);
} else {
    println!("No value provided");
}
```

### 3. Error Handling

```rust
let result = some_operation();
if result.is_ok() {
    let value = result.unwrap();
    println!("Success: {}", value);
} else {
    println!("Error: {}", result.unwrap_err());
}
```

### 4. State Machine

```rust
enum State {
    Initial,
    Processing,
    Complete,
    Error,
}

let mut state = State::Initial;

match state {
    State::Initial => {
        // Start processing
        state = State::Processing;
    }
    State::Processing => {
        // Continue processing
        state = State::Complete;
    }
    State::Complete => {
        println!("Processing complete");
    }
    State::Error => {
        println!("An error occurred");
    }
}
```

## Common Mistakes

### ❌ Using assignment instead of comparison

```rust
// Bad: This assigns 5 to x, not compares
if x = 5 {
    println!("x is 5");
}
```

### ❌ Missing else branch with different types

```rust
// Bad: Different types in branches
let result = if condition { 5 } else { "six" };
```

### ❌ Unnecessary parentheses

```rust
// Bad: Unnecessary parentheses
if (x > 5) {
    println!("x is greater than 5");
}
```

### ✅ Correct patterns

```rust
// Good: Proper comparison
if x == 5 {
    println!("x is 5");
}

// Good: Same types in branches
let result = if condition { 5 } else { 6 };

// Good: Clean condition
if x > 5 {
    println!("x is greater than 5");
}
```

## Performance Considerations

### 1. Condition Order

```rust
// Good: Most likely condition first
if user.is_authenticated() {
    // Common case
} else if user.is_guest() {
    // Less common
} else {
    // Rare case
}
```

### 2. Short-circuit Evaluation

```rust
// Good: Short-circuit prevents expensive operation
if user.is_authenticated() && expensive_permission_check() {
    // Only runs expensive check if user is authenticated
}
```

## Exercises

1. **Grade Calculator**: Create a function that assigns letter grades based on numeric scores
2. **Age Classifier**: Write a program that categorizes people by age groups
3. **Number Analyzer**: Create a function that determines if a number is positive, negative, or zero
4. **Permission Checker**: Implement a system that checks multiple permissions
5. **State Machine**: Build a simple state machine using conditional statements

## Related Concepts

- **Boolean Logic**: Understanding `&&`, `||`, `!` operators
- **Pattern Matching**: Using `match` expressions
- **Error Handling**: Using `Result` and `Option` types
- **Control Flow**: Understanding program flow

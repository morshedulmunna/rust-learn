# Enum.rs - Enumerations

## Overview

The `enum.rs` file demonstrates how to define and use enumerations (enums) in Rust, which are a way to define a type that can have one of several possible values.

## Code Analysis

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_example() {
    let msg = Message::Move { x: 1, y: 2 };
    println!("msg is {msg}");

    let msg = Message::Write(String::from("hello"));
    println!("msg is {msg}");

    let msg = Message::ChangeColor(0, 0, 0);
    println!("msg is {msg}");

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(s) => println!("Write: {s}"),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: ({r}, {g}, {b})"),
        _ => println!("Not a message"),
    }

    let msg = Message::Move { x: 1, y: 2 };
    if let Message::Move { x, y } = msg {
        println!("Move to ({x}, {y})");
    } else {
        println!("Not a move message");
    }
}
```

## Key Concepts

### 1. Basic Enum Definition

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

**Characteristics:**

- Each variant can hold different types and amounts of data
- `Quit` - Unit variant (no data)
- `Move` - Struct-like variant with named fields
- `Write` - Tuple-like variant with one value
- `ChangeColor` - Tuple-like variant with multiple values

### 2. Creating Enum Instances

```rust
// Unit variant
let quit_msg = Message::Quit;

// Struct-like variant
let move_msg = Message::Move { x: 1, y: 2 };

// Tuple-like variants
let write_msg = Message::Write(String::from("hello"));
let color_msg = Message::ChangeColor(255, 0, 0);
```

### 3. Pattern Matching with Enums

```rust
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({x}, {y})"),
    Message::Write(s) => println!("Write: {s}"),
    Message::ChangeColor(r, g, b) => println!("ChangeColor: ({r}, {g}, {b})"),
}
```

**Key Points:**

- Match must be exhaustive (cover all variants)
- Can destructure data from variants
- Use `_` for catch-all pattern

### 4. If Let Pattern

```rust
if let Message::Move { x, y } = msg {
    println!("Move to ({x}, {y})");
} else {
    println!("Not a move message");
}
```

**Key Points:**

- Useful when you only care about one variant
- More concise than full match for single cases
- Can include else clause for other cases

## Advanced Enum Patterns

### 1. Enums with Methods

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to ({}, {}, {})", r, g, b),
        }
    }

    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}
```

### 2. Nested Enums

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

enum Drawing {
    Shape(Shape),
    ColoredShape(Shape, Color),
    Text(String),
}
```

### 3. Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Custom generic enum
enum Container<T> {
    Empty,
    Single(T),
    Multiple(Vec<T>),
}
```

## Common Enum Patterns

### 1. Option-like Enums

```rust
enum MaybeNumber {
    Just(i32),
    Nothing,
}

fn divide(a: i32, b: i32) -> MaybeNumber {
    if b == 0 {
        MaybeNumber::Nothing
    } else {
        MaybeNumber::Just(a / b)
    }
}

// Usage
match divide(10, 2) {
    MaybeNumber::Just(result) => println!("Result: {}", result),
    MaybeNumber::Nothing => println!("Cannot divide by zero"),
}
```

### 2. State Machines

```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { id: u32 },
    Error { message: String },
}

fn handle_connection(state: &mut ConnectionState) {
    match state {
        ConnectionState::Disconnected => {
            println!("Attempting to connect...");
            *state = ConnectionState::Connecting;
        }
        ConnectionState::Connecting => {
            println!("Connection established");
            *state = ConnectionState::Connected { id: 12345 };
        }
        ConnectionState::Connected { id } => {
            println!("Connected with ID: {}", id);
        }
        ConnectionState::Error { message } => {
            println!("Connection error: {}", message);
            *state = ConnectionState::Disconnected;
        }
    }
}
```

### 3. Command Pattern

```rust
enum Command {
    Move { x: i32, y: i32 },
    Rotate { angle: f64 },
    Scale { factor: f64 },
    Draw { shape: String },
}

fn execute_command(cmd: Command) {
    match cmd {
        Command::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Command::Rotate { angle } => println!("Rotating by {} degrees", angle),
        Command::Scale { factor } => println!("Scaling by factor {}", factor),
        Command::Draw { shape } => println!("Drawing {}", shape),
    }
}
```

### 4. Error Handling

```rust
enum DatabaseError {
    ConnectionFailed { reason: String },
    QueryFailed { sql: String, error: String },
    NotFound { table: String, id: u32 },
    PermissionDenied { user: String },
}

fn handle_database_error(error: DatabaseError) {
    match error {
        DatabaseError::ConnectionFailed { reason } => {
            eprintln!("Connection failed: {}", reason);
        }
        DatabaseError::QueryFailed { sql, error } => {
            eprintln!("Query '{}' failed: {}", sql, error);
        }
        DatabaseError::NotFound { table, id } => {
            eprintln!("Record with ID {} not found in table {}", id, table);
        }
        DatabaseError::PermissionDenied { user } => {
            eprintln!("User {} does not have permission", user);
        }
    }
}
```

## Best Practices

### 1. Use Descriptive Names

```rust
// Good: Clear and descriptive
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

// Bad: Unclear
enum Method {
    A,
    B,
    C,
    D,
}
```

### 2. Group Related Data

```rust
// Good: Related data grouped together
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

// Bad: Separate enums for similar concepts
enum Circle {
    Radius(f64),
}

enum Rectangle {
    Width(f64),
    Height(f64),
}
```

### 3. Use Methods for Common Operations

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

impl Status {
    fn is_active(&self) -> bool {
        matches!(self, Status::Active)
    }

    fn can_transition_to(&self, new_status: &Status) -> bool {
        match (self, new_status) {
            (Status::Pending, Status::Active) => true,
            (Status::Active, Status::Inactive) => true,
            _ => false,
        }
    }
}
```

## Common Mistakes

### ❌ Forgetting to handle all variants

```rust
// Bad: Non-exhaustive match
let msg = Message::Quit;
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move"),
    // Missing Write and ChangeColor variants
}
```

### ❌ Using wrong variant syntax

```rust
// Bad: Wrong syntax for struct-like variant
let msg = Message::Move(1, 2); // Should be Message::Move { x: 1, y: 2 }

// Bad: Wrong syntax for tuple-like variant
let msg = Message::Write { text: "hello" }; // Should be Message::Write("hello".to_string())
```

### ❌ Not using pattern matching effectively

```rust
// Bad: Manual checking
if msg == Message::Quit {
    println!("Quit");
} else if msg == Message::Move { x: 1, y: 2 } {
    println!("Move to (1, 2)");
}

// Good: Use pattern matching
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    _ => println!("Other message"),
}
```

### ✅ Correct patterns

```rust
// Good: Exhaustive pattern matching
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
}

// Good: Using if let for single cases
if let Message::Move { x, y } = msg {
    println!("Move to ({}, {})", x, y);
}
```

## Performance Considerations

### 1. Memory Layout

```rust
// Enums are sized to hold the largest variant
enum SmallEnum {
    A, // 0 bytes of data
    B, // 0 bytes of data
}

enum LargeEnum {
    A,                    // 0 bytes of data
    B(Vec<i32>),         // 24 bytes of data (on 64-bit)
}

// SmallEnum: 1 byte (discriminant)
// LargeEnum: 24 bytes (size of Vec + discriminant)
```

### 2. Boxing for Large Variants

```rust
// Good: Use Box for large variants to reduce enum size
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(Box<String>), // Box large strings
    ChangeColor(i32, i32, i32),
}
```

## Exercises

1. **Simple Enum**: Create an enum for days of the week and write functions to work with it
2. **State Machine**: Implement a simple state machine using enums
3. **Error Types**: Create custom error types using enums
4. **Command Parser**: Build a command parser that uses enums to represent different commands
5. **Game States**: Create enums to represent different states in a simple game

## Related Concepts

- **Pattern Matching**: Using match with enums
- **Structs**: Similar to struct-like enum variants
- **Generics**: Creating generic enums
- **Traits**: Implementing traits for enums

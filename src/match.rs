/// match

fn match_example() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// match with multiple patterns

fn match_with_multiple_patterns() {
    let x = 5;
    match x {
        1 | 2 | 3 => println!("one, two, or three"),
        _ => println!("anything"),
    }
}

/// match with range

fn match_with_range() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

/// match with binding

fn match_with_binding() {
    let x = 5;
    match x {
        x => println!("x is {x}"),
    }
}

/// match with guard

fn match_with_guard() {
    let x = 5;
    match x {
        x if x % 2 == 0 => println!("x is even"),
        x if x % 2 != 0 => println!("x is odd"),
        _ => println!("x is not a number"),
    }
}

/// match with multiple arms
fn match_with_multiple_arms() {
    let x = 5;
    match x {
        1 | 2 | 3 => println!("one, two, or three"),
        _ => println!("anything"),
    }
}

///looping rust

fn loop_example() {
    loop {
        println!("again!");
    }
}

fn while_example() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// Additional loop patterns

fn loop_with_break() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
        println!("Counter: {}", counter);
    }
    println!("Loop finished at counter: {}", counter);
}

fn loop_with_break_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return a value from the loop
        }
    };
    println!("Result: {}", result);
}

fn loop_with_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // Break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn while_let_example() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_with_enumerate() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn for_with_reference() {
    let v = vec![100, 32, 57];
    for i in &v {
        // Using reference to avoid moving
        println!("{}", i);
    }
    println!("Vector is still available: {:?}", v);
}

fn for_with_mut_reference() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // Mutable reference to modify values
        *i += 50;
    }
    println!("Modified vector: {:?}", v);
}

fn for_with_range_inclusive() {
    for number in 1..=5 {
        // Inclusive range (includes 5)
        println!("{}", number);
    }
}

fn for_with_step_by() {
    for number in (0..10).step_by(2) {
        // Step by 2
        println!("{}", number);
    }
}

fn for_with_filter() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter().filter(|&&x| x % 2 == 0) {
        println!("Even number: {}", number);
    }
}

fn for_with_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    for doubled in numbers.iter().map(|x| x * 2) {
        println!("Doubled: {}", doubled);
    }
}

fn for_with_zip() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
}

fn for_with_chain() {
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];

    for number in first.iter().chain(second.iter()) {
        println!("{}", number);
    }
}

fn for_with_take() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter().take(3) {
        // Only take first 3
        println!("{}", number);
    }
}

fn for_with_skip() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter().skip(3) {
        // Skip first 3
        println!("{}", number);
    }
}

fn for_with_rev() {
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter().rev() {
        // Reverse iteration
        println!("{}", number);
    }
}

fn for_with_cycle() {
    let colors = vec!["red", "green", "blue"];
    for (i, color) in colors.iter().cycle().take(7).enumerate() {
        println!("Item {}: {}", i, color);
    }
}

fn for_with_windows() {
    let numbers = vec![1, 2, 3, 4, 5];
    for window in numbers.windows(3) {
        // Sliding window of size 3
        println!("Window: {:?}", window);
    }
}

fn for_with_chunks() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for chunk in numbers.chunks(3) {
        // Split into chunks of size 3
        println!("Chunk: {:?}", chunk);
    }
}

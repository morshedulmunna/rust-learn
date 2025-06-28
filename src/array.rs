/// array

fn array_example() {
    let a = [1, 2, 3, 4, 5];
    println!("a is {a:?}");
}

fn array_with_type() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a is {a:?}");
}

fn array_with_default_value() {
    let a: [i32; 5] = [3; 5];
    a.push(6);
    a.pop();
    a.len();
    a.is_empty();
    a.contains(&3);
    a.iter().sum();
    a.iter().product();
    a.iter().max();
    a.iter().min();
    a.iter().count();
    a.iter().nth(2);
    a.iter().position(|x| x == &3);
    a.iter().rposition(|x| x == &3);
    a.iter().rev().collect::<Vec<&i32>>();
    a.iter().skip(2).collect::<Vec<&i32>>();
    a.iter().take(2).collect::<Vec<&i32>>();
    a.iter().filter(|x| x % 2 == 0).collect::<Vec<&i32>>();
    a.iter().filter(|x| x % 2 != 0).collect::<Vec<&i32>>();

    a.iter().map(|x| x * 2).collect::<Vec<i32>>();

    println!("a is {a:?}");
}

fn array_with_index() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first is {first}, second is {second}");
}

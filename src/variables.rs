/// variables

fn variables() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y + 2;
        println!("y is {y}");
    }
    println!("y is {y}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is {spaces}");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("guess is {guess}");

    let mut x = String::from("hello");
    x.push_str(", world");
    println!("x is {x}");
}

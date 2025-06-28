/// enum

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

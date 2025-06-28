fn main() {
    command_line_args();
}

fn command_line_args() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    if args.len() > 1 {
        println!("First argument: {}", args[1]);
    } else {
        println!("No arguments provided");
    }
}

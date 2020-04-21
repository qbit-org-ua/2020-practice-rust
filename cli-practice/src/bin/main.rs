fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);

    if args.len() < 3 {
        println!("Not enough arguments");
        return;
    }
    let first_number: i64 = args[1].parse().unwrap();
    let second_number: i64 = args[2].parse().unwrap();
    println!(
        "{} + {} = {}",
        first_number,
        second_number,
        first_number + second_number
    );
    /*
    match args[1].as_str() {
        "--help" => println!("Help: main --help"),
        "-p" | "--print" => println!("Print"),
        "--ask" => println!("Ask"),
        first_argument => println!("Unknown command: {}", first_argument),
    }
    */
}

use std::env::args;
use std::io;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "read" => read(),
            "run" => run(),
            "test" => test(),
            _ => println!("Invalid argument. You can only use [read, run]"),
        }
    } else {
        doc()
    }
}

fn doc() {
    println!("Hi! Welcome to Mathaly. Don't try to do something impossible for math because it just won't work.");
    println!("Arguments: [read, run]");
    println!("  - read: Read and parse a provided file.");
    println!("  - run: Run and parse a provided expression as input (stdin).");
}

fn run() {
    println!(
        "Welcome to the Mathaly runner!\n|-> To run any math expression, just type them below!"
    );
    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => println!("Failed to parse current expr: {}", input),
        }
        parse(input)
    }
}

fn read() {}

fn parse(expr: String) {
    let splitted: Vec<&str> = expr.trim().trim().split("").collect();
    let mut overtrimmed: Vec<&str> = Vec::new();
    for i in splitted {
        if i == "" {
            continue;
        } else {
            overtrimmed.push(i)
        }
    }
    println!("{:?}", overtrimmed)
}

fn test() {
    
}

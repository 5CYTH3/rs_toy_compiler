mod lexer;
mod parser;

use std::env::{args, Args};
use std::io;

fn main() {
    let mut args: Args = args();
    let first_arg: String = match args.nth(1) {
        Some(val) => val,
        None => String::from(""),
    };

    if first_arg != "" {
        match first_arg.as_str() {
            "read" => read(),
            "run" => run(),
            "test" => todo!(),
            _ => println!("Invalid argument. You can only use [read, run, test]"),
        }
    } else {
        doc()
    }
}

fn doc() {
    println!("Hi! Welcome to Mathaly. Don't try to do something impossible for math because it just won't work.");
    println!("Arguments: [read, run, test]");
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
    }
}

fn read() {
    use lexer::*;

    let lex = lexer::Lexer::new();
}

use std::env::{args, Args};
use std::io;

mod compiler;
mod errors;
mod log;
mod parse;

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
    println!("Hi! Welcome to Karm. Don't try to do something impossible for math because it just won't work.");
    println!("Arguments: [read, run, test]");
    println!("  - read: Read and parse a provided file.");
    println!("  - run: Run and parse a provided expression as input (stdin).");
}

fn run() {
    println!("Welcome to the Karm runner!\n|-> To run any math expression, just type them below!");
    let mut parser = Parser::new();

    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let ast = parser.parse(input);
                println!("{:?}", ast)
            }

            Err(_) => println!("Failed to parse current expr: {:?}", input),
        }
    }
}

use compiler::*;
use parse::*;

fn read() {
    // TODO: Create logging system to log all messages and halt the program (like when there is a syntax error).
    let mut parser = Parser::new();
    let program: &str = r#"hello := 55;"#;
    let ast = parser.parse(program.to_owned());

    println!("{}", ast);
    /*
       let mut compiler = Compiler::new(ast.clone());
       compiler.compile()
    */
}

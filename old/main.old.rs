mod tokenizer;

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
            "test" => test(),
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
    let expr: &str = "15 + 22";
    println!("{:?}", parse(expr))
}

fn parse(expr: &str) -> Vec<&str> {
    let splitted: Vec<&str> = expr.trim().split("").collect();
    let mut overtrimmed: Vec<&str> = Vec::new();
    for i in splitted {
        if i == "" || i == " " {
            continue;
        } else {
            overtrimmed.push(i)
        }
    }
    return overtrimmed;
}

fn test() {
    // 28 - 10 * 2 = 8
    let more_complex_expr: i64 = Expr::new(BinaryExpr {
        op: Token::MIN,
        left: Expr::new(NumericLiteral(28)),
        right: Expr::new(BinaryExpr {
            op: Token::MUL,
            left: Expr::new(NumericLiteral(10)),
            right: Expr::new(NumericLiteral(2)),
        }),
    });
}

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
    /*
    let expr: &str = "15 + 22";
    let re = Regex::new(r"\d+").unwrap();
    let sliced_expr: Vec<&str> = re.split(expr).collect();
    println!("{:?}", sliced_expr);
    */

    use tokenizer::*;
    use ExprType::*;

    // 18 + 16 = 34
    let expression: i64 = Expr::new(BinaryExpression {
        op: Operator::PLUS,
        left: Expr::new(NumericLiteral { val: 18 }),
        right: Expr::new(NumericLiteral { val: 16 }),
    });

    // 28 - 10 * 2 = -5
    let more_complex_expr: i64 = Expr::new(BinaryExpression {
        op: Operator::MIN,
        left: Expr::new(NumericLiteral { val: 28 }),
        right: Expr::new(BinaryExpression {
            op: Operator::MUL,
            left: Expr::new(NumericLiteral { val: 10 }),
            right: Expr::new(NumericLiteral { val: 2 }),
        }),
    });
    println!("{:?}", expression);
    println!("{:?}", more_complex_expr);
}

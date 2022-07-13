use crate::parser::literals::{Literal, NumericLiteral};

pub mod literals;
mod parser_tests;
use crate::lexer::*;

pub struct Parser {
    string: String,
    l: Lexer,
    lookahead: Token,
}

use std::error::Error;
impl Parser {
    pub fn new() -> Self {
        Parser {
            string: String::from(""),
            l: Lexer::new(),
            lookahead: Token::DEFAULT,
        }
    }

    pub fn parse(&mut self, string: &str) -> Literal {
        self.string = String::from(string);
        self.l.init(String::from(string));

        self.lookahead = self.l.get_next_token();

        Parser::program(self)
    }

    fn program(&self) -> Literal {
        Literal::Numeric(self.numeric_literal())
    }

    fn numeric_literal(&self) -> NumericLiteral {
        NumericLiteral::new(self.string.parse().unwrap())
    }

    fn eat(&self) -> Result<Token, Box<dyn Error>> {
        let t = &self.lookahead;
        if !assert_eq!(t, Token::DEFAULT) {
            Ok(Token::DEFAULT)
        } else {
            Err()
        }
    }
}

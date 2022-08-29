use crate::parser::literals::{Literal, NumericLiteral};

pub mod literals;
mod parser_tests;
use crate::lexer::*;

pub struct Parser {
    string: String,
    l: Lexer,
    lookahead: Token,
}

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

    fn eat(mut self) -> Token {
        let mut t = self.lookahead;
        t = self.l.get_next_token();

        return t;
    }
}

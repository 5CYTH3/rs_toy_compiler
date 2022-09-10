pub mod lexer;
pub mod token;

use core::panic;

use lexer::Lexer;
use token::Token;
use token::TokenType;

pub struct Parser {
    program: String,
    lexer: Lexer,
    lookahead: Option<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            program: String::from(""),
            lexer: Lexer::new(),
            lookahead: Some(Token::new(TokenType::String, "".to_owned())),
        }
    }

    // parse
    pub fn parse(&mut self, program: String) -> Token {
        self.program = program.clone();
        self.lexer.init(program);
        self.lookahead = self.lexer.get_next_token();

        println!("LOOKAHEAD VAL : {:?}", self.lookahead);

        return self.program();
    }

    // Main entry point of everything
    fn program(&mut self) -> Token {
        return self.literal();
    }

    fn literal(&mut self) -> Token {
        match self.lookahead.clone() {
            Some(token) => match token.r#type {
                TokenType::Integers => return self.numeric_literal(),
                TokenType::String => return self.numeric_literal(),

                _ => panic!("NOT COVERED"),
            },
            None => panic!("Not covered"),
        }
    }

    fn numeric_literal(&mut self) -> Token {
        let t = self.eat(TokenType::Integers);
        return Token::new(TokenType::Integers, t.val);
    }

    fn string_literal(&mut self) -> Token {
        let t = self.eat(TokenType::String);
        return Token::new(TokenType::String, t.val);
    }

    fn eat(&mut self, tt: TokenType) -> Token {
        let t = self.lookahead.clone();
        if t.clone().is_none() {
            panic!("SyntaxError!    -> Expected: {} and got: None", tt)
        }

        // ! Peut etre pb ici
        let ttype = t.clone().unwrap().r#type;

        if ttype != tt {
            panic!("UnexpectedToken!    -> Expected: {} and got: {}", tt, ttype)
        }

        let new_lookahead = self.lexer.get_next_token();
        self.lookahead = new_lookahead;

        return t.unwrap();
    }
}

pub mod lexer;
pub mod token;

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
        self.lexer.init(program.clone());
        self.lookahead = self.lexer.get_next_token();
        return self.program();
    }

    // Main entry point of everything
    fn program(&mut self) -> Token {
        return self.numeric_literal();
    }

    fn numeric_literal(&mut self) -> Token {
        let t = self.eat(TokenType::Int);
        return Token::new(TokenType::Int, t.val);
    }

    fn eat(&mut self, tt: TokenType) -> Token {
        let t = self.lookahead.clone();
        if t.clone().is_none() {
            panic!("SyntaxError!    -> Expected: {} and got: None", tt)
        }

        let ttype = t.clone().unwrap().r#type;

        if ttype == tt {
            panic!("UnexpectedToken!    -> Expected: {} and got: {}", tt, ttype)
        }

        let new_lookahead = self.lexer.get_next_token();
        self.lookahead = new_lookahead;

        return t.unwrap();
    }
}

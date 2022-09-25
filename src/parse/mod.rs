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
    pub fn parse(&mut self, program: String) -> Vec<Token> {
        self.program = program.clone();
        self.lexer.init(program);
        self.lookahead = self.lexer.get_next_token();

        println!("LOOKAHEAD VAL : {:?}", self.lookahead);

        return self.program();
    }

    // Main entry point of everything
    fn program(&mut self) -> Vec<Token> {
        return self.statement_list();
    }

    // TODO: Why not convert those things into a struct?
    fn statement_list(&self) -> Vec<Token> {
        let statement_list: Vec<Token> = vec![self.statement()];

        return statement_list;
    }

    fn statement(&self) -> Token {}

    fn literal(&mut self) -> Token {
        match self.lookahead.clone() {
            Some(token) => match token.r#type {
                TokenType::Integers => return self.numeric_literal(),
                TokenType::String => return self.string_literal(),

                _ => panic!("NOT COVERED"),
            },
            None => panic!("Not covered"),
        }
    }

    fn numeric_literal(&mut self) -> Token {
        let eaten_token = self.eat(TokenType::Integers);
        return Token::new(TokenType::Integers, eaten_token.val);
    }

    fn string_literal(&mut self) -> Token {
        let eaten_token = self.eat(TokenType::String);
        return Token::new(TokenType::String, eaten_token.val);
    }

    fn eat(&mut self, targetted_token_type: TokenType) -> Token {
        let t: Token = match self.lookahead.clone() {
            Some(val) => val,
            None => panic!(
                "SyntaxError!    -> Expected: {} and got: None",
                targetted_token_type
            ),
        };

        // ! Peut etre pb ici
        let token_type: TokenType = match t {
            Token { val, r#type } => r#type,
        };

        if token_type != targetted_token_type {
            panic!(
                "UnexpectedToken!    -> Expected: {} and got: {}",
                token_type, targetted_token_type
            )
        }

        let new_lookahead = self.lexer.get_next_token();
        self.lookahead = new_lookahead;

        return t;
    }
}

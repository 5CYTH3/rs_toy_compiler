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

//TODO: Convert all those functions to Struct or Enums because it is useless as fuck to have that much function
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

        return self.program();
    }

    // Main entry point of everything
    fn program(&mut self) -> Vec<Token> {
        return self.statement_list();
    }

    fn statement_list(&mut self) -> Vec<Token> {
        let mut statement_list: Vec<Token> = vec![self.statement()];

        while self.lookahead != None {
            statement_list.push(self.statement());
        }

        return statement_list;
    }

    fn statement(&mut self) -> Token {
        match self.lookahead {
            Some(val) => match val.r#type {
                TokenType::LBracket => return self.block_statement(),
                _ => return self.expr_statement(),
            },
            None => panic!("Lookahead empty"),
        }
    }

    fn block_statement(&mut self) -> Token {
        self.eat(TokenType::LBracket);
        let body: Vec<Token> = if self.lookahead.unwrap().r#type != TokenType::RBracket {
            self.statement_list()
        } else {
            vec![]
        };
        self.eat(TokenType::RBracket);
    }

    fn expr_statement(&mut self) -> Token {
        let expr = self.expr();
        self.eat(TokenType::SemiColon);

        return expr;
    }

    fn expr(&mut self) -> Token {
        return self.literal();
    }

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
        let token_type: TokenType = match t.clone() {
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

#[cfg(test)]
mod parser_test {

    use super::*;

    #[test]
    fn test_parsing() {
        let mut parser = Parser::new();
        let program: &str = "\"25\";";
        let ast = parser.parse(program.to_owned());

        assert_eq!(
            ast,
            vec![Token {
                r#type: TokenType::String,
                val: String::from("\"25\"")
            }]
        )
    }
}

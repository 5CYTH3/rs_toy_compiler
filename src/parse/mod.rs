pub mod lexer;
pub mod token;

use core::panic;

use lexer::Lexer;
use token::Token;
use token::TokenType;

// Migrate all Vec<Token> to statements and StatementList.
#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(StatementList),
    Expr(Token),
}
pub type StatementList = Vec<Statement>;

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
    pub fn parse(&mut self, program: String) -> Vec<Statement> {
        self.program = program.clone();
        self.lexer.init(program);
        self.lookahead = self.lexer.get_next_token();

        return self.program();
    }

    // Main entry point of everything
    fn program(&mut self) -> StatementList {
        return self.statement_list();
    }

    fn statement_list(&mut self) -> StatementList {
        let mut statement_list: Vec<Statement> = vec![self.statement()];

        while self.lookahead != None {
            statement_list.push(self.statement());
        }

        return statement_list;
    }

    fn statement(&mut self) -> Statement {
        match self.lookahead.clone() {
            Some(val) => match val.r#type {
                TokenType::LBracket => return self.block_statement(),
                _ => return self.expr_statement(),
            },
            None => panic!("Lookahead empty"),
        }
    }

    // WHat is this method supposed to return? Is it body?
    fn block_statement(&mut self) -> Statement {
        let lookahead = self.lookahead.clone().unwrap().r#type;
        self.eat(TokenType::LBracket);
        let body: Vec<Statement> = if lookahead != TokenType::RBracket {
            self.statement_list()
        } else {
            vec![]
        };
        self.eat(TokenType::RBracket);
        return Statement::Block(body);
    }

    fn expr_statement(&mut self) -> Statement {
        let expr = self.expr();
        self.eat(TokenType::SemiColon);

        return expr;
    }

    fn expr(&mut self) -> Statement {
        return Statement::Expr(self.literal());
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
            vec![Statement::Expr(Token {
                r#type: TokenType::String,
                val: String::from("\"25\"")
            })]
        )
    }
}

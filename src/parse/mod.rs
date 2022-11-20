pub mod ast;
pub mod lexer;
pub mod token;

use core::panic;

use lexer::Lexer;
use token::Token;
use token::TokenType;

use ast::{Expr, Statement, StatementList};

pub struct Parser {
    program: String,
    lexer: Lexer,
    lookahead: Option<Token>,
}

/*
    Parser ;
    Main entrypoint of the whole code.
*/
//TODO: Convert all those functions to Struct or Enums because it is useless as fuck to have that much function
impl Parser {
    pub fn new() -> Self {
        Parser {
            program: String::from(""),
            lexer: Lexer::new(),
            lookahead: Some(Token::new(TokenType::String, "".to_owned())),
        }
    }

    /*
    Parse ;
    Returns a Vector of Statements (StatementList), that constitute the tokenized code.
    */
    pub fn parse(&mut self, program: String) -> StatementList {
        // Init the Parser with values.
        self.program = program.clone();
        self.lexer.init(program);
        self.lookahead = self.lexer.get_next_token();

        return self.program();
    }

    fn program(&mut self) -> StatementList {
        return self.statement_list(None);
    }

    /*
    StatementList ;
    Returns a Vector of Statements containing all tokens between a LeftBracket and a RightBracket.
    */
    fn statement_list(&mut self, stop_lookahead: Option<TokenType>) -> StatementList {
        let mut statement_list: StatementList = StatementList(vec![self.statement()]);

        while self.lookahead.clone() != None
            && self.lookahead.clone().unwrap().r#type != stop_lookahead.clone().unwrap()
        {
            statement_list.0.push(self.statement());
        }

        return statement_list;
    }

    /*
    Statement ;
    Returns a "Statement", either a BlockStatement ({}) or a ExpressionStatement (Literal) depending on the next token.
    */
    fn statement(&mut self) -> Statement {
        match self.lookahead.clone() {
            Some(val) => match val.r#type {
                TokenType::LBracket => return self.block_statement(),
                _ => return self.expr_statement(),
            },
            None => panic!("Lookahead empty"),
        }
    }

    /*
    BlockStatement ;
    Returns a StatementList if the next token (lookahead) isn't a Right Bracket.
    Eats the trailing Right Bracket.
    */
    fn block_statement(&mut self) -> Statement {
        let lookahead = self.lookahead.clone().unwrap().r#type;
        self.eat(TokenType::LBracket);
        let body: StatementList = if lookahead != TokenType::RBracket {
            self.statement_list(Some(TokenType::RBracket))
        } else {
            StatementList(vec![])
        };
        self.eat(TokenType::RBracket);
        return Statement::Block(body);
    }

    /*
    ExpressionStatement ;
    Returns an expression and deletes the trailing semicolon.
    */
    fn expr_statement(&mut self) -> Statement {
        let expr = self.expr();
        self.eat(TokenType::SemiColon);

        return expr;
    }

    fn expr(&mut self) -> Statement {
        return self.assignment_expr();
    }

    fn assignment_expr(&mut self) -> Statement {
        let left = self.additive_expr();
        if self.lookahead.clone().unwrap().r#type != TokenType::Assign {
            return left;
        }

        self.eat(TokenType::Assign);
        return Statement::Expr(Expr::Assignment {
            left: Box::new(self.check_valid_assignment_target(left)),
            right: Box::new(self.assignment_expr()),
        });
    }

    // TODO: Implement this function.
    fn check_valid_assignment_target(&mut self, statement: Statement) -> Statement {
        match statement {
            _ => panic!("INVALID LHS ASSIGNMENT EXPR."),
        }
    }

    fn lhs_expr(&mut self) -> Token {
        return self.identifier();
    }

    fn identifier(&mut self) -> Token {
        let val = self.eat(TokenType::Identifier).val;
        return Token {
            r#type: TokenType::Assign,
            val,
        };
    }

    fn primary_expr(&mut self) -> Statement {
        match self.lookahead.clone().unwrap().r#type {
            TokenType::LParen => self.paren_expr(),
            _ => Statement::Expr(Expr::Primary(self.literal())),
        }
    }

    fn multiplicative_expr(&mut self) -> Statement {
        self.binary_expr("multiplicative_expr", TokenType::Mul)
    }

    fn additive_expr(&mut self) -> Statement {
        self.binary_expr("additive_expr", TokenType::Plus)
    }

    fn binary_expr(&mut self, expr_type: &str, token_type: TokenType) -> Statement {
        let mut left = match expr_type {
            "additive_expr" => self.multiplicative_expr(),
            "multiplicative_expr" => self.primary_expr(),
            _ => panic!("Not covered case. it's either a additive or multiplicative expr."),
        };

        while self.lookahead.clone().unwrap().r#type == token_type {
            let op = self.eat(token_type.clone());

            let right = match expr_type {
                "additive_expr" => self.multiplicative_expr(),
                "multiplicative_expr" => self.primary_expr(),
                _ => panic!("Not covered case. it's either a additive or multiplicative expr."),
            };

            left = Statement::Expr(Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        return left;
    }

    fn paren_expr(&mut self) -> Statement {
        self.eat(TokenType::LParen);
        let expr = self.expr();
        self.eat(TokenType::RParen);
        return expr;
    }

    fn literal(&mut self) -> Token {
        match self.lookahead.clone() {
            Some(token) => match token.r#type {
                TokenType::Integers => return self.numeric_literal(),
                TokenType::String => return self.string_literal(),
                _ => panic!("NOT COVERED, {}", token),
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
            StatementList(vec![Statement::Expr(Expr::Primary(Token {
                r#type: TokenType::String,
                val: String::from("\"25\"")
            }))])
        )
    }
}

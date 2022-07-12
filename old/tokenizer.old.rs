pub enum Token {
    PLUS,
    MIN,
    DIV,
    MUL,
    EQUAL,
    LPAREN,
    RPAREN,
}

pub struct Identifier(String);
pub struct Parameter(String);
pub struct ParamExpr(Parameter, Token, ExprResult);

pub struct Lexer {}

impl Lexer {
    pub fn from_str(s: &str) -> Self {
        todo!()
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        todo!();
    }
}

type ExprResult = i64;

pub struct Func {
    id: Identifier,
    param: Parameter,
    operation: ParamExpr,
}

pub enum ExprType {
    BinaryExpr {
        op: Token,
        left: ExprResult,
        right: ExprResult,
    },
    AssignExpr {
        op: Token,
        left: Identifier,
        right: ExprResult,
    },
    NumericLiteral(i64),
}

// https://youtu.be/Ra_Fk7JFMoo?t=6012

pub struct Expr {
    pub expr_type: ExprType,
}

impl Func {
    pub fn new(id: Identifier, param: Parameter, operation: ParamExpr) -> Func {
        return Func {
            id,
            param,
            operation,
        };
    }
}

impl ExprType {
    pub fn run(&self) -> ExprResult {
        match self {
            ExprType::BinaryExpr { op, left, right } => match op {
                Token::PLUS => return left + right,
                Token::MIN => return left - right,
                Token::MUL => return left * right,
                Token::DIV => return left / right,
                _ => return 0,
            },
            ExprType::NumericLiteral(val) => return *val,
            ExprType::AssignExpr { op, left, right } => todo!(),
            _ => return 0,
        }
    }
}

// TODO: Instead of unpacking each Expr, why not returning a final expr and get the final value of it with a function?
// TODO: Use YOUR type instead of primitive types. Returning i64 is dumb as hell.

impl Expr {
    pub fn new(expr_type: ExprType) -> ExprResult {
        expr_type.run()
    }
}

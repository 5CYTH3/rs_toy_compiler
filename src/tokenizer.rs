use either::*;

pub enum ExprType {
    BinaryExpression { op: Operator, left: i64, right: i64 },
    NumericLiteral { val: i64 },
    StringLiteral { val: String },
}

pub enum Operator {
    PLUS,
    MIN,
    DIV,
    MUL,
    EQUAL,
}

pub struct Expr {
    pub expr_type: ExprType,
}

enum ExprResult {
    A(i64),
    B(String),
}

impl ExprResult {
    pub fn get() {
        match *self {
            ExprResult::A(ref data) => data,
            ExprResult::B(ref data) => data
        }
    }
}

impl Expr {
    pub fn new(expr_type: ExprType) -> ExprResult {
        match expr_type {
            ExprType::BinaryExpression { op, left, right } => match op {
                Operator::PLUS => return ExprResult::A(left + right),
                Operator::MIN => return ExprResult::A(left - right),
                Operator::MUL => return ExprResult::A(left * right),
                Operator::DIV => return ExprResult::A(left / right),
                Operator::EQUAL => todo!(),
            },
            ExprType::NumericLiteral { val } => return ExprResult::A(val),
            ExprType::StringLiteral { val } => return ExprResult::B(val),
            _ => return ExprResult::A(0),
        }
    }
}

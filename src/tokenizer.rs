pub enum ExprType {
    BinaryExpression { op: Operator, left: i64, right: i64 },
    NumericLiteral { val: i64 },
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

impl Expr {
    pub fn new(expr_type: ExprType) -> i64 {
        match expr_type {
            ExprType::BinaryExpression { op, left, right } => match op {
                Operator::PLUS => return left + right,
                Operator::MIN => return left - right,
                Operator::MUL => return left * right,
                Operator::DIV => return left / right,
                Operator::EQUAL => todo!(),
            },
            ExprType::NumericLiteral { val } => return val,
        }
        return 0;
    }
}

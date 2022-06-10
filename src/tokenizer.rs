pub enum ExprType {
    BinaryExpression { op: Operator, left: i64, right: i64 },
    NumericLiteral { val: i64 },
    AssignExpression { op: Operator, left: Identifier(String), right: i64 }
}

pub struct Func {
    id: Identifier(String),
    param: Parameter(String),
    operation: ParamExpr(String, Expr)
}

impl Func {
    pub fn new(id: Identifier, param: Parameter, operation: ParamExpr) -> Func {
        return Func { id, param, operation };
    }
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
            _ => return 0,
        }
    }
}

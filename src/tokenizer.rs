pub enum ExprType {
    BinaryExpression { op: Operator, left: Box<Expr>, right: Box<Expr> },
    NumericLiteral { val: i64 },
    AssignExpression { op: Operator, left: Identifier(String), right: Box<Expr> }
}

// https://youtu.be/Ra_Fk7JFMoo?t=6012
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

// TODO: Instead of unpacking each Expr, why not returning a final expr and get the final value of it with a function?
// TODO: Use YOUR type instead of primitive types. Returning i64 is dumb as hell.
// TODO: 
impl Expr {
    pub fn new(expr_type: ExprType) -> Expr {
        match expr_type {
            ExprType::BinaryExpression { op, left, right } => match op {
                Operator::PLUS => return ExprType::NumericLiteral { val: left + right },
                Operator::MIN => return ExprType::NumericLiteral { left - right },
                Operator::MUL => return ExprType::NumericLiteral { left * right },
                Operator::DIV => return ExprType::NumericLiteral { left / right },
                Operator::EQUAL => todo!(),
            },
            ExprType::NumericLiteral { val } => return ExprType::NumericLiteral { val },
            _ => return 0,
        }
    }

    pub fn to_int(&self) -> i64 {
        
    }
}

use crate::parse::token::Token;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Block(StatementList),
    Expr(Expr),
}

// ! I think I kinda broke everything. There's some weird errors like "HEY BRO RECURSIVE DATA"
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Primary(Token),
    Binary {
        op: Token,
        left: Box<Statement>,
        right: Box<Statement>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Expr(expr) => write!(f, "({})", expr),
            Statement::Block(list) => {
                write!(f, "Block {{ {} }}", list)
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Primary(token) => write!(f, "Primary({})", token),
            Expr::Binary { op, left, right } => {
                write!(f, "Binary {{ {} {} {} }}", left, op, right)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementList(pub Vec<Statement>);

impl fmt::Display for StatementList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |res, statement| {
            res.and_then(|_| write!(f, "Program: \n{}", *statement))
        })
    }
}

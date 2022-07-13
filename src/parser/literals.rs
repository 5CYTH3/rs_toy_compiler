pub enum Literal {
    Numeric(NumericLiteral),
    String(StringLiteral),
}

use std::fmt;
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = "";
        match self {
            Literal::Numeric(num) => write!(f, "NumericLiteral(value={})", num.value),
            Literal::String(str) => write!(f, "StringLiteral(value=\"{}\")", str.value),
        }
    }
}

pub struct NumericLiteral {
    pub value: i32,
}

impl NumericLiteral {
    pub fn new(value: i32) -> Self {
        NumericLiteral { value }
    }
}

pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        StringLiteral {
            value: String::from(value),
        }
    }
}

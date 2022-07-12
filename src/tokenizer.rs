use std::fmt;

enum Token {}

pub trait Literal {}

#[derive(Debug)]
pub struct NumericLiteral {
    val: i32,
}

impl Literal for NumericLiteral {}

impl NumericLiteral {
    pub fn new(val: i32) -> NumericLiteral {
        return NumericLiteral { val };
    }
}

impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.val)
    }
}

#[derive(Debug)]
pub struct StringLiteral {
    val: String,
}

impl Literal for StringLiteral {}

impl StringLiteral {
    pub fn new(val: String) -> StringLiteral {
        return StringLiteral { val };
    }
}

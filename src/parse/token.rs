use std::fmt;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub val: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DT \"{}\" with value {}", self.r#type, self.val)
    }
}

impl Token {
    pub fn new(t_type: TokenType, val: String) -> Self {
        Token {
            r#type: t_type,
            val,
        }
    }
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TokenType {
    // Primitive types
    Integers,
    Natural,
    String,

    // Bin Op
    Plus,      // +
    Min,       // -
    Mul,       // *
    Div,       // /
    Less,      // <
    Greater,   // >
    EqLess,    // <=
    EqGreater, // >=
    Percent,   // %
}

impl TokenType {
    pub fn guard_precedence(&self) -> Option<u8> {
        match self {
            /*
            Self::VbarVbar => Some(1),

            Self::AmperAmper => Some(2),

            Self::EqualEqual | Self::NotEqual => Some(3),
            */
            Self::Less | Self::EqLess | Self::EqGreater | Self::Greater => Some(4),
            _ => None,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_token = match self {
            TokenType::Integers => "Int",
            TokenType::String => "String",
            TokenType::Plus => "+",
            TokenType::Min => "-",
            TokenType::Mul => "*",
            TokenType::Div => "/",
            TokenType::Less => "<",
            TokenType::Greater => ">",
            TokenType::EqLess => "<=",
            TokenType::EqGreater => ">=",
            TokenType::Percent => "%",
            _ => "!!!uninmplemented!!!",
        };
        write!(f, "{}", fmt_token)
    }
}

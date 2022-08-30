use std::fmt;

pub enum Token {
    // Primitive types
    Int { val: String },
    String { val: String },

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

impl Token {
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_token = match self {
            Token::Int { val } => val,
            Token::String { val } => val,
            Token::Plus => "+",
            Token::Min => "-",
            Token::Mul => "*",
            Token::Div => "/",
            Token::Less => "<",
            Token::Greater => ">",
            Token::EqLess => "<=",
            Token::EqGreater => ">=",
            Token::Percent => "%",
            _ => "!!!uninmplemented!!!",
        };
        write!(f, "{}", fmt_token)
    }
}

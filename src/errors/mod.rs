use std::{error::Error, fmt};

use crate::parse::token::TokenType;

#[derive(Debug)]
pub struct SyntaxError(TokenType, TokenType);

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Syntax Error Detected! Expected : \"{}\", got \"{}\"",
            self.0, self.1
        )
    }
}

impl Error for SyntaxError {}

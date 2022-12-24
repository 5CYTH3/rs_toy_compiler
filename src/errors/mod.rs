use std::{error::Error, fmt};

use crate::parse::token::TokenType;

#[derive(Debug)]
pub enum KarmError {
    SyntaxError(TokenType, TokenType)
}

impl fmt::Display for KarmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KarmError::SyntaxError(left, right) => write!(
                f,
                "Syntax Error Detected! Expected : \"{}\", got \"{}\"",
                left, right
            )
        }
    }

}
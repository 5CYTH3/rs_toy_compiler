// use regex::Regex;
use crate::parse::token::{Token, TokenType};
use regex::Regex;

pub struct Lexer {
    string: String,
    cursor: i32,
}

fn test() {
    // let pubspec: Vec<(&str, Token)> = vec![("", Token::Int), ()];
}

impl Lexer {
    pub fn new() -> Self {
        return Lexer {
            string: String::from(""),
            cursor: 0,
        };
    }
    pub fn init(&mut self, string: String) {
        self.string = string;
        self.cursor = 0;
    }

    fn has_more_token(&self) -> bool {
        return self.cursor < self.string.len().try_into().unwrap();
    }

    pub fn get_next_token(&self) -> Option<Token> {
        if !self.has_more_token() {
            panic!("No more tokens ;D")
        }

        // Sliced string
        let s_str = &self.string[..self.cursor as usize];
        // "number" rule for regex
        let r_num = Regex::new(r"^\d+").unwrap();

        // "string" rule for regex
        // let r_str = Regex::new(r"").unwrap();

        if r_num.is_match(s_str) {
            return Some(Token::new(TokenType::Int, s_str.to_owned()));
        }

        return None;
    }
}

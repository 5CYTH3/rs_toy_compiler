// use regex::Regex;
use crate::parse::token::{Token, TokenType};
use regex::Regex;

pub struct Lexer {
    string: String,
    cursor: usize,
}

fn test() {
    // let pubspec: Vec<(&str, Token)> = vec![("", Token::Int), ()];
}

impl Lexer {
    pub fn new() -> Self {
        return Lexer {
            string: String::from(""),
            cursor: 1,
        };
    }
    pub fn init(&mut self, string: String) {
        self.string = string;
        self.cursor = 0;
    }

    fn has_more_token(&self) -> bool {
        return self.cursor < self.string.len().try_into().unwrap();
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_more_token() {
            return None;
        }

        // Sliced string
        let s_str = &self.string[self.cursor..];
    
        // "number" rule for regex
        let r_num = Regex::new(r"^\d+").unwrap();

        // "string" rule for regex
        let r_str = Regex::new(r#"^\\"[^\\"]*\\""#).unwrap();

        match r_num.captures(s_str) {
            Some(caps) => {
                self.cursor += caps.get(0).unwrap().as_str().len();
                return Some(Token::new(TokenType::Int, caps.get(0).unwrap().as_str().to_string()));
            },
            None => ()
        }

        match r_str.captures(s_str) {
            Some(caps) => {
                self.cursor += caps.get(0).unwrap().as_str().len();
                return Some(Token::new(TokenType::Int, caps.get(0).unwrap().as_str().to_string()));
            },
            None => ()
        }

        
    }
}

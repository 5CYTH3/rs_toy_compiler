// use regex::Regex;
use crate::parse::token::{Token, TokenType};
use regex::Regex;

pub struct Lexer {
    string: String,
    cursor: usize,
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

    fn match_token(&mut self, regexp: (&str, TokenType), ctx: &str) {}

    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_more_token() {
            return None;
        }

        let regex_set: Vec<(&str, Option<TokenType>)> = Vec::from([
            (r"^\d+", Some(TokenType::Integers)),     // Integers
            (r#"^"[^"]*""#, Some(TokenType::String)), // String
            (r"^\s+", None),                          // Whitespace
            (r"^;", Some(TokenType::SemiColon)),
            (r"^\{", Some(TokenType::LBracket)),
            (r"^\}", Some(TokenType::RBracket)),
        ]);

        // Sliced string
        let s_str = &self.string[self.cursor..];

        // ! Wrap this into a function but it doesn't want to work.
        // "cannot borrow `*self` as mutable because it is also borrowed as immutable mutable borrow occurs here"
        for r_s in regex_set {
            match Regex::new(r_s.0).unwrap().captures(s_str) {
                Some(caps) => {
                    let capture = caps.get(0).unwrap().as_str();
                    self.cursor += capture.len();
                    match r_s.1 {
                        Some(token_type) => {
                            return Some(Token::new(token_type, capture.to_string()));
                        }
                        None => return self.get_next_token(), // _ => panic!("Unimplemented. Error occured when resolving token type."),
                    }
                }
                None => continue,
            }
        }
        return None;
    }
}

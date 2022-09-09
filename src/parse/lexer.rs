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

    fn match_token(&mut self, regexp: (&str, TokenType), ctx: &str) {}

    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_more_token() {
            return None;
        }

        // ! Change TokenType to Option<TokenType> to allow None return
        let regex_set: Vec<(&str, TokenType)> = Vec::from([
            (r"^\d+", TokenType::Integers),
            (r#"^"[^"]*""#, TokenType::String),
        ]);

        // Sliced string
        let s_str = &self.string[self.cursor..];

        // ! Wrap this into a function but it doesn't want to work.
        // "cannot borrow `*self` as mutable because it is also borrowed as immutable mutable borrow occurs here"
        for r_s in regex_set {
            match Regex::new(r_s.0).unwrap().captures(s_str) {
                Some(caps) => {
                    self.cursor += caps.get(0).unwrap().as_str().len();
                    match r_s.1 {
                        TokenType::Integers => {
                            return Some(Token::new(
                                TokenType::Integers,
                                caps.get(0).unwrap().as_str().to_string(),
                            ));
                        }
                        TokenType::String => {
                            return Some(Token::new(
                                TokenType::String,
                                caps.get(0).unwrap().as_str().to_string(),
                            ));
                        }
                        _ => panic!("Unimplemented. Error occured when resolving token type."),
                    }
                }
                None => (),
            }
        }

        return None;
    }
}

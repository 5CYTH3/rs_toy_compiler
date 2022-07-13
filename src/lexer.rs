use regex::Regex;

pub enum Token {
    DEFAULT,
}

pub struct Lexer {
    string: String,
    cursor: i32,
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

    pub fn get_next_token(&self) -> Token {
        if !self.has_more_token() {
            return Token::DEFAULT;
        }

        let s_str = &self.string[..self.cursor as usize];
        let re = Regex::new(r"^\d+").unwrap();

        if re.is_match(s_str) {
            return Token::DEFAULT;
        } else {
            return Token::DEFAULT;
        }
    }
}

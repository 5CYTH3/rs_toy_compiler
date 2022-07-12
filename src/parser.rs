use crate::tokenizer::*;
use std::boxed::Box;

#[derive(Debug)]
pub struct Parser {
    string: String,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            string: "".to_string(),
        };
    }

    pub fn parse(&mut self, string: String) {
        self.string = string
    }

    pub fn program(&self) -> Box<dyn Literal> {
        return Box::new(Parser::numeric_literal(self));
    }

    pub fn numeric_literal(&self) -> NumericLiteral {
        return NumericLiteral::new(self.string.parse().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_numeric_literal() {
        let mut inst_parser: Parser = Parser::new();
        assert_eq!(
            inst_parser.parse("323".to_string()),
            NumericLiteral { val: 323 }
        )
    }
}

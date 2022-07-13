#[cfg(test)]
mod parser_tests {
    use super::super::Parser;
    use crate::parser::literals::{Literal, NumericLiteral};

    #[test]
    pub fn test_parsing_integer() {
        let mut parser_impl = Parser::new();
        let code = "52";

        let result = parser_impl.parse(code);

        match result {
            Literal::Numeric(num) => assert_eq!(num.value, 52),
            default => panic!("NumericLiteral expected, got {}", default),
        }
    }
}

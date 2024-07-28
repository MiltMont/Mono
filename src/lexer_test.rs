#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[derive(Debug)]
    struct TestType {
        expected_type: TokenType,
        expected_literal: String,
    }

    impl TestType {
        fn new(expected_type: TokenType, expected_literal: String) -> Self {
            Self {
                expected_type,
                expected_literal,
            }
        }
    }
    #[test]
    fn test_next_token() {
        let input = "let testing = 5;".to_string();

        let tests: Vec<TestType> = vec![
            TestType::new(TokenType::LET, "let".to_string()),
            TestType::new(TokenType::IDENT, "testing".to_string()),
            TestType::new(TokenType::ASSIGN, "=".to_string()),
            TestType::new(TokenType::INT, "5".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::EOF, " ".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        for test in tests {
            let token = lexer.next_token();

            if token.typ != test.expected_type {
                panic!(
                    "Wrong token type, expected {:?} but got {:?}",
                    test.expected_type, token.typ
                )
            }

            if token.literal != test.expected_literal {
                panic!(
                    "Wrong literal, expected {} but got {}",
                    test.expected_literal, token.literal
                )
            }
        }
    }
}

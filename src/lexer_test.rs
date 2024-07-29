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
        let input = "let testing = 5;
        let add = fn(x,y) {
            x + y;
        };

        !-/*5;

        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        };

        10 == 10;
        10 != 9;
        "
        .to_string();

        let tests: Vec<TestType> = vec![
            TestType::new(TokenType::LET, "let".to_string()),
            TestType::new(TokenType::IDENT, "testing".to_string()),
            TestType::new(TokenType::ASSIGN, "=".to_string()),
            TestType::new(TokenType::INT, "5".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::LET, "let".to_string()),
            TestType::new(TokenType::IDENT, "add".to_string()),
            TestType::new(TokenType::ASSIGN, "=".to_string()),
            TestType::new(TokenType::FUNCTION, "fn".to_string()),
            TestType::new(TokenType::LPAREN, "(".to_string()),
            TestType::new(TokenType::IDENT, "x".to_string()),
            TestType::new(TokenType::COMMA, ",".to_string()),
            TestType::new(TokenType::IDENT, "y".to_string()),
            TestType::new(TokenType::RPAREN, ")".to_string()),
            TestType::new(TokenType::LBRACE, "{".to_string()),
            TestType::new(TokenType::IDENT, "x".to_string()),
            TestType::new(TokenType::PLUS, "+".to_string()),
            TestType::new(TokenType::IDENT, "y".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::RBRACE, "}".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::BANG, "!".to_string()),
            TestType::new(TokenType::MINUS, "-".to_string()),
            TestType::new(TokenType::SLASH, "/".to_string()),
            TestType::new(TokenType::ASTERISK, "*".to_string()),
            TestType::new(TokenType::INT, "5".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::INT, "5".to_string()),
            TestType::new(TokenType::LT, "<".to_string()),
            TestType::new(TokenType::INT, "10".to_string()),
            TestType::new(TokenType::GT, ">".to_string()),
            TestType::new(TokenType::INT, "5".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::IF, "if".to_string()),
            TestType::new(TokenType::LPAREN, "(".to_string()),
            TestType::new(TokenType::INT, "5".to_string()),
            TestType::new(TokenType::LT, "<".to_string()),
            TestType::new(TokenType::INT, "10".to_string()),
            TestType::new(TokenType::RPAREN, ")".to_string()),
            TestType::new(TokenType::LBRACE, "{".to_string()),
            TestType::new(TokenType::RETURN, "return".to_string()),
            TestType::new(TokenType::TRUE, "true".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::RBRACE, "}".to_string()),
            TestType::new(TokenType::ELSE, "else".to_string()),
            TestType::new(TokenType::LBRACE, "{".to_string()),
            TestType::new(TokenType::RETURN, "return".to_string()),
            TestType::new(TokenType::FALSE, "false".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::RBRACE, "}".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::INT, "10".to_string()),
            TestType::new(TokenType::EQ, "==".to_string()),
            TestType::new(TokenType::INT, "10".to_string()),
            TestType::new(TokenType::SEMICOLON, ";".to_string()),
            TestType::new(TokenType::INT, "10".to_string()),
            TestType::new(TokenType::NEQ, "!=".to_string()),
            TestType::new(TokenType::INT, "9".to_string()),
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

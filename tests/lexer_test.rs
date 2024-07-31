#[cfg(test)]
mod tests {
    use mono::lexer::Lexer;
    use mono::token::TokenType;

    #[derive(Debug)]
    struct TestType {
        expected_type: TokenType,
        expected_literal: String,
    }

    impl TestType {
        fn new(expected_type: TokenType, expected_literal: &str) -> Self {
            Self {
                expected_type,
                expected_literal: expected_literal.to_string(),
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
        10 != 9"
            .to_string();

        let tests: Vec<TestType> = vec![
            TestType::new(TokenType::LET, "let"),
            TestType::new(TokenType::IDENT, "testing"),
            TestType::new(TokenType::ASSIGN, "="),
            TestType::new(TokenType::INT, "5"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::LET, "let"),
            TestType::new(TokenType::IDENT, "add"),
            TestType::new(TokenType::ASSIGN, "="),
            TestType::new(TokenType::FUNCTION, "fn"),
            TestType::new(TokenType::LPAREN, "("),
            TestType::new(TokenType::IDENT, "x"),
            TestType::new(TokenType::COMMA, ","),
            TestType::new(TokenType::IDENT, "y"),
            TestType::new(TokenType::RPAREN, ")"),
            TestType::new(TokenType::LBRACE, "{"),
            TestType::new(TokenType::IDENT, "x"),
            TestType::new(TokenType::PLUS, "+"),
            TestType::new(TokenType::IDENT, "y"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::RBRACE, "}"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::BANG, "!"),
            TestType::new(TokenType::MINUS, "-"),
            TestType::new(TokenType::SLASH, "/"),
            TestType::new(TokenType::ASTERISK, "*"),
            TestType::new(TokenType::INT, "5"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::INT, "5"),
            TestType::new(TokenType::LT, "<"),
            TestType::new(TokenType::INT, "10"),
            TestType::new(TokenType::GT, ">"),
            TestType::new(TokenType::INT, "5"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::IF, "if"),
            TestType::new(TokenType::LPAREN, "("),
            TestType::new(TokenType::INT, "5"),
            TestType::new(TokenType::LT, "<"),
            TestType::new(TokenType::INT, "10"),
            TestType::new(TokenType::RPAREN, ")"),
            TestType::new(TokenType::LBRACE, "{"),
            TestType::new(TokenType::RETURN, "return"),
            TestType::new(TokenType::TRUE, "true"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::RBRACE, "}"),
            TestType::new(TokenType::ELSE, "else"),
            TestType::new(TokenType::LBRACE, "{"),
            TestType::new(TokenType::RETURN, "return"),
            TestType::new(TokenType::FALSE, "false"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::RBRACE, "}"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::INT, "10"),
            TestType::new(TokenType::EQ, "=="),
            TestType::new(TokenType::INT, "10"),
            TestType::new(TokenType::SEMICOLON, ";"),
            TestType::new(TokenType::INT, "10"),
            TestType::new(TokenType::NEQ, "!="),
            TestType::new(TokenType::INT, "9"),
            TestType::new(TokenType::EOF, " "),
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

#[cfg(test)]
mod tests {

    use crate::{
        ast::{Node, StatementVariant},
        lexer::Lexer,
        parser::Parser,
    };

    struct TestType {
        expected_identifier: String,
    }

    impl TestType {
        fn new(expected_identifier: &str) -> Self {
            Self {
                expected_identifier: expected_identifier.to_string(),
            }
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "
    let x = 5;
    let y = 10;
    let testing = 234;
            "
        .to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);

        if program.statements.len() == 0 {
            panic!("parse_program returned an empty vector.")
        }
        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. Got {}",
                program.statements.len()
            )
        }

        let tests: Vec<TestType> = vec![
            TestType::new("x"),
            TestType::new("y"),
            TestType::new("testing"),
        ];

        for (idx, test) in tests.iter().enumerate() {
            let statement = &program.statements[idx];

            if !test_let_statement(statement, test.expected_identifier.clone()) {
                break;
            }
        }
    }

    fn test_let_statement(statement: &StatementVariant, name: String) -> bool {
        if statement.token_literal() != "let" {
            println!("Token literal not eq, got {}", statement.token_literal());
            return false;
        }

        match statement {
            StatementVariant::Let(s) => {
                if s.name.value != name {
                    dbg!(
                        "statement.name.value is not {}, got {}",
                        name,
                        s.name.value.clone()
                    );
                    return false;
                }

                if s.name.token_literal() != name {
                    dbg!("statement.name  is not {}, got {:?}", name, &s.name);
                    return false;
                }
            }
        }

        true
    }

    fn check_parser_errors(parser: Parser) {
        let errors = parser.errors().clone();

        if errors.len() == 0 {
            return;
        }

        eprint!("\nParser has {} errors.\n", errors.len());

        for message in errors {
            eprint!("\nParser error: {:?}", message);
        }

        panic!();
    }
}

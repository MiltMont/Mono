#[cfg(test)]
mod tests {

    use core::panic;

    use mono::{
        ast::{ExpressionVariants, Node, StatementVariant},
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
            _ => {}
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

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 15;
        "
        .to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. Got {:?}",
                program.statements.leak()
            );
        }

        for statement in program.statements {
            match statement {
                StatementVariant::Return(s) => {
                    if s.token_literal() != "return" {
                        panic!(
                            "statement.token_literal is not 'return', got {}",
                            s.token_literal()
                        );
                    }
                }
                _ => {
                    eprint!("statement is not ReturnStatement, got {:?}", statement);
                }
            }
        }
    }

    #[test]
    fn test_identifier_expressions() {
        let input = "foobar;".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(parser);

        if program.statements.len() != 1 {
            panic!(
                "Program has not enough statements: {}",
                program.statements.len()
            );
        }

        if let StatementVariant::Expression(exp) = &program.statements[0] {
            if let Some(expr_variant) = &exp.expression {
                if let ExpressionVariants::Ident(identifier) = expr_variant {
                    if identifier.value != "foobar" {
                        panic!("ident.value not foobar, got {}", identifier.value);
                    }

                    if identifier.token_literal() != "foobar" {
                        panic!(
                            "ident.token_literal() not foobar, got {}",
                            identifier.token_literal()
                        )
                    }
                }
            } else {
                panic!("exp is not Identifier, got {:?}", exp.expression);
            }
        } else {
            panic!(
                "program.statements[0] is not an ExpressionStatement, got {:?}",
                program.statements[0]
            );
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(parser);

        if program.statements.len() != 1 {
            panic!(
                "Program has not enough statements: {}",
                program.statements.len()
            );
        }

        if let StatementVariant::Expression(expr_stmt) = &program.statements[0] {
            if let Some(exp_var) = &expr_stmt.expression {
                if let ExpressionVariants::Integer(int_lit) = exp_var {
                    if int_lit.value != 5 {
                        panic!("Literal value not 5, got {}", int_lit.value);
                    }

                    if int_lit.token_literal() != "5" {
                        panic!(
                            "int_lit.token_literal not 5, got {}",
                            int_lit.token_literal()
                        );
                    }
                } else {
                    panic!("Expression is not IntegerLiteral, got {:?}", exp_var)
                }
            } else {
                panic!("No expression!, got {:?}", &expr_stmt.expression);
            }
        } else {
            panic!(
                "program.statements[0] is not an ExpressionStatement. Got {:?}",
                program.statements[0],
            );
        }
    }
}

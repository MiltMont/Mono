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

    #[derive(Debug, Clone)]
    struct PrefixTest {
        input: String,
        operator: String,
        integer_value: i64,
    }

    impl PrefixTest {
        fn new(input: &str, operator: &str, integer_value: i64) -> Self {
            Self {
                input: input.to_string(),
                operator: operator.to_string(),
                integer_value,
            }
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let tests: Vec<PrefixTest> = vec![
            PrefixTest::new("!5;", "!", 5),
            PrefixTest::new("-15;", "-", 15),
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program: mono::ast::Program = parser.parse_program();

            check_parser_errors(parser);

            if program.statements.len() != 1 {
                panic!(
                    "Program has not enough statements: {}",
                    program.statements.len()
                );
            }

            if let StatementVariant::Expression(expr_stmt) = &program.statements[0] {
                if let Some(exp_variant) = &expr_stmt.expression {
                    if let ExpressionVariants::Prefix(pre_expr) = exp_variant {
                        if pre_expr.operator != test.operator {
                            panic!(
                                "pre_expr operator is not {}, got {}",
                                test.operator, pre_expr.operator
                            );
                        }

                        if !test_integer_literal(*pre_expr.right.clone(), test.integer_value) {
                            break;
                        }
                    } else {
                        panic!("pre_expr is not a PrefixExpression, got {:?}", exp_variant);
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

    fn test_integer_literal(integer_literal: ExpressionVariants, value: i64) -> bool {
        if let ExpressionVariants::Integer(int_var) = integer_literal {
            if int_var.value != value {
                eprint!("int_var.value not {}, got {}", value, int_var.value);
                return false;
            }

            if int_var.token_literal() != format!("{}", value) {
                eprint!(
                    "int_var.token_literal() not {}, got {}",
                    value,
                    int_var.token_literal()
                );
                return false;
            }
        } else {
            panic!(
                "Integer_literal is not IntegerLiteral, got {:?}",
                integer_literal
            );
        }
        true
    }

    struct InfixTest {
        input: String,
        left_value: i64,
        operator: String,
        right_value: i64,
    }

    impl InfixTest {
        fn new(input: &str, left_value: i64, operator: &str, right_value: i64) -> Self {
            Self {
                input: input.to_string(),
                left_value,
                operator: operator.to_string(),
                right_value,
            }
        }
    }

    #[test]

    fn test_parsing_infix_expressions() {
        let tests: Vec<InfixTest> = vec![
            InfixTest::new("5+5;", 5, "+", 5),
            InfixTest::new("5-5;", 5, "-", 5),
            InfixTest::new("5*5;", 5, "*", 5),
            InfixTest::new("5/5;", 5, "/", 5),
            InfixTest::new("5>5;", 5, ">", 5),
            InfixTest::new("5<5;", 5, "<", 5),
            InfixTest::new("5 == 5;", 5, "==", 5),
            InfixTest::new("5 != 5;", 5, "!=", 5),
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program: mono::ast::Program = parser.parse_program();

            check_parser_errors(parser);

            if program.statements.len() != 1 {
                panic!(
                    "Program has not enough statements: {}",
                    program.statements.len()
                );
            }

            if let StatementVariant::Expression(expr_stmt) = &program.statements[0] {
                if let Some(exp_var) = &expr_stmt.expression {
                    if let ExpressionVariants::Infix(inf_exp) = exp_var {
                        if !test_integer_literal(*inf_exp.left.clone(), test.left_value) {
                            return;
                        }

                        if inf_exp.operator != test.operator {
                            panic!(
                                "inf_exp.operator is not {:?}, got {:?}",
                                test.operator, inf_exp.operator
                            );
                        }

                        if !test_integer_literal(*inf_exp.right.clone(), test.right_value) {
                            return;
                        }
                    } else {
                        panic!("exp_var is not an InfixExpression, got {:?}", exp_var);
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
}

#[cfg(test)]
mod tests {

    use core::panic;
    use std::vec;

    use mono::{
        ast::{ExpressionVariants, Node, Program, StatementVariant},
        lexer::Lexer,
        parser::Parser,
    };

    fn create_parse_program(input: &str) -> Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(parser);

        program
    }

    #[test]
    fn test_let_statements() {
        #[derive(Debug)]
        struct Test {
            input: String,
            expected_identifier: String,
            expected_value: Expected,
        }

        impl Test {
            fn new(input: &str, expected_identifier: &str, expected_value: Expected) -> Self {
                Self {
                    input: input.to_string(),
                    expected_identifier: expected_identifier.to_string(),
                    expected_value: expected_value,
                }
            }
        }

        let tests: Vec<Test> = vec![
            Test::new("let x = 5;", "x", Expected::Int(5)),
            Test::new("let y = 4;", "y", Expected::Int(4)),
            Test::new(
                "let foobar = y;",
                "foobar",
                Expected::String(String::from("y")),
            ),
            Test::new("let boolean = true;", "boolean", Expected::Boolean(true)),
            Test::new("let barfoo = false;", "barfoo", Expected::Boolean(false)),
        ];

        for test in tests.iter() {
            let program = create_parse_program(&test.input);

            if program.statements.len() != 1 {
                panic!(
                    "program.statements does not contain 1 statement. Got {}",
                    program.statements.len()
                );
            }

            let statement = &program.statements[0];

            if !test_let_statement(statement, &test.expected_identifier) {
                return;
            }

            if let StatementVariant::Let(let_stmt) = statement {
                if let Some(exp_var) = &let_stmt.value {
                    if !test_literal_expression(exp_var.clone(), test.expected_value.clone()) {
                        panic!();
                    }
                } else {
                    panic!(
                        "NO EXPRESSION FOUND AT {:?}, coming from {:?}",
                        let_stmt, &test
                    );
                }
            }
        }
    }

    fn test_let_statement(statement: &StatementVariant, name: &str) -> bool {
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
        if let StatementVariant::Let(let_statement) = statement {
            if let_statement.name.value != name {
                eprintln!(
                    "let_statement.name is not {}, got {}",
                    name, let_statement.name.value
                );
                return false;
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

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 15;
        ";

        let program = create_parse_program(input);

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
        let input = "foobar;";

        let program = create_parse_program(input);

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
    fn test_boolean_expression() {
        struct Test {
            input: String,
            expected_boolean_value: bool,
        }

        let tests: Vec<Test> = vec![
            Test {
                input: String::from("true;"),
                expected_boolean_value: true,
            },
            Test {
                input: String::from("false;"),
                expected_boolean_value: false,
            },
        ];

        for test in tests {
            let program = create_parse_program(&test.input);

            if program.statements.len() != 1 {
                panic!(
                    "
                    Program does not contain 1 statement, got {}",
                    program.statements.len()
                );
            }

            for statement in program.statements {
                if let StatementVariant::Expression(exp) = statement {
                    if let Some(expression_variant) = &exp.expression {
                        if let ExpressionVariants::Boolean(boolean) = expression_variant {
                            if boolean.value != test.expected_boolean_value {
                                panic!("boolean.value is not true, got {}", boolean.value);
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let program = create_parse_program(input);

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
            let program = create_parse_program(&test.input);

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

    fn test_identifier(expression: ExpressionVariants, value: &str) -> bool {
        if let ExpressionVariants::Ident(identifier) = expression {
            if identifier.value != value {
                eprintln!("identifier.value not {}, got {}", value, identifier.value);
                return false;
            }

            if identifier.token_literal() != value {
                eprintln!(
                    "identifier.token_literal not {} got {}",
                    value,
                    identifier.token_literal()
                );
                return false;
            }

            true
        } else {
            eprintln!("expression no Identifier, got {:?}", expression);
            false
        }
    }

    fn test_integer_literal(integer_literal: ExpressionVariants, value: i64) -> bool {
        if let ExpressionVariants::Integer(int_var) = integer_literal {
            if int_var.value != value {
                eprintln!("int_var.value not {}, got {}", value, int_var.value);
                return false;
            }

            if int_var.token_literal() != format!("{}", value) {
                eprintln!(
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
        left_value: Expected,
        operator: String,
        right_value: Expected,
    }

    impl InfixTest {
        fn new(input: &str, left_value: Expected, operator: &str, right_value: Expected) -> Self {
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
            InfixTest::new("5+5;", Expected::Int(5), "+", Expected::Int(5)),
            InfixTest::new("5-5;", Expected::Int(5), "-", Expected::Int(5)),
            InfixTest::new("5*5;", Expected::Int(5), "*", Expected::Int(5)),
            InfixTest::new("5/5;", Expected::Int(5), "/", Expected::Int(5)),
            InfixTest::new("5>5;", Expected::Int(5), ">", Expected::Int(5)),
            InfixTest::new("5<5;", Expected::Int(5), "<", Expected::Int(5)),
            InfixTest::new("5 == 5;", Expected::Int(5), "==", Expected::Int(5)),
            InfixTest::new("5 != 5;", Expected::Int(5), "!=", Expected::Int(5)),
        ];

        for test in tests {
            let program = create_parse_program(&test.input);

            if program.statements.len() != 1 {
                panic!(
                    "Program has not enough statements: {}",
                    program.statements.len()
                );
            }

            if let StatementVariant::Expression(expr_stmt) = &program.statements[0] {
                if let Some(exp_var) = &expr_stmt.expression {
                    if let ExpressionVariants::Infix(inf_exp) = exp_var {
                        if let Expected::Int(val) = test.left_value {
                            if !test_integer_literal(*inf_exp.left.clone(), val) {
                                return;
                            }
                        }

                        if inf_exp.operator != test.operator {
                            panic!(
                                "inf_exp.operator is not {:?}, got {:?}",
                                test.operator, inf_exp.operator
                            );
                        }
                        if let Expected::Int(val) = test.right_value {
                            if !test_integer_literal(*inf_exp.right.clone(), val) {
                                return;
                            }
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

    #[derive(Debug, Clone)]
    struct OpPrecedenceTest {
        input: String,
        expected: String,
    }

    impl OpPrecedenceTest {
        fn new(input: &str, expected: &str) -> Self {
            Self {
                input: input.to_string(),
                expected: expected.to_string(),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests: Vec<OpPrecedenceTest> = vec![
            OpPrecedenceTest::new("-a * b", "((-a) * b)"),
            OpPrecedenceTest::new("a + b + c", "((a + b) + c)"),
            OpPrecedenceTest::new("a * b * c", "((a * b) * c)"),
            OpPrecedenceTest::new("a * b / c", "((a * b) / c)"),
            OpPrecedenceTest::new("a + b / c", "(a + (b / c))"),
            OpPrecedenceTest::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            OpPrecedenceTest::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            OpPrecedenceTest::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            OpPrecedenceTest::new("true", "true"),
            OpPrecedenceTest::new("false", "false"),
            OpPrecedenceTest::new("3 > 5 == false", "((3 > 5) == false)"),
            OpPrecedenceTest::new("3< 5 == true", "((3 < 5) == true)"),
        ];

        for test in tests {
            let program = create_parse_program(&test.input);

            let actual = program.string();
            if actual != test.expected {
                panic!("Expected {}, got {}", test.expected, actual);
            }
        }
    }

    #[derive(Debug, Clone)]
    enum Expected {
        Int(i64),
        String(String),
        Boolean(bool),
    }

    fn test_literal_expression(expression: ExpressionVariants, expected: Expected) -> bool {
        match expected {
            Expected::Int(v) => test_integer_literal(expression, v),
            Expected::String(v) => test_identifier(expression, &v),
            Expected::Boolean(v) => test_boolean_literal(expression, v),
        }
    }

    fn test_boolean_literal(expression: ExpressionVariants, value: bool) -> bool {
        if let ExpressionVariants::Boolean(bool_exp) = expression {
            if bool_exp.value != value {
                eprintln!("boolean.value is not {}, got {}", value, bool_exp.value);
                return false;
            }

            if bool_exp.token_literal() != format!("{}", value) {
                eprintln!(
                    "boolean.token_literal() not {}, got {}",
                    format!("{}", value),
                    bool_exp.token_literal()
                );
                return false;
            }

            true
        } else {
            eprintln!("Expression is not a Boolean, got {:?}", expression);
            false
        }
    }

    fn test_infix_expression(
        expression: ExpressionVariants,
        left: Expected,
        operator: &str,
        right: Expected,
    ) -> bool {
        if let ExpressionVariants::Infix(inf_exp) = expression {
            if !test_literal_expression(ExpressionVariants::Infix(inf_exp.clone()), left) {
                return false;
            }

            if inf_exp.operator != operator {
                eprintln!(
                    "inf_exp.operator is not {}, got {}",
                    operator, inf_exp.operator
                );
                return false;
            }

            if !test_literal_expression(*inf_exp.right, right) {
                return false;
            }

            return true;
        } else {
            eprintln!("expression is not InfixExpression, got {:?}", expression);
            return false;
        }
    }
}

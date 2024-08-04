use std::{collections::HashMap, os::linux::raw::stat};

use crate::{
    ast::{
        Boolean, ExpressionStatement, ExpressionVariant, ExpressionVariants, Identifier,
        InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program, ReturnStatement,
        StatementVariant,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::ILLEGAL, ' '),
            peek_token: Token::new(TokenType::ILLEGAL, ' '),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.next_token();
        parser.next_token();

        // Register prefix parse functions
        parser.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        parser.register_prefix(TokenType::INT, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
        // Boolean parse functions
        parser.register_prefix(TokenType::TRUE, Parser::parse_boolean);
        parser.register_prefix(TokenType::FALSE, Parser::parse_boolean);

        // Register infix parse functions
        parser.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        parser.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NEQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LT, Parser::parse_infix_expression);
        parser.register_infix(TokenType::GT, Parser::parse_infix_expression);

        parser
    }

    /////////////////////
    // Parsing functions.
    /////////////////////

    fn parse_boolean(&mut self) -> Option<ExpressionVariants> {
        Some(ExpressionVariants::Boolean(Boolean {
            token: self.current_token.clone(),
            value: self.current_token_is(TokenType::TRUE),
        }))
    }

    fn parse_identifier(&mut self) -> Option<ExpressionVariants> {
        Some(ExpressionVariants::Ident(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while !self.current_token_is(TokenType::EOF) {
            let statement = self.parse_statement();

            if let Some(statement_variant) = statement {
                program.statements.push(statement_variant);
            }

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<StatementVariant> {
        match self.current_token.typ {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            // We try to parse expression statement by default.
            _ => self.parse_expression_statements(),
        }
    }

    fn parse_expression_statements(&mut self) -> Option<StatementVariant> {
        let mut statement = ExpressionStatement {
            token: self.current_token.clone(),
            expression: None,
        };

        statement.expression = self.parse_expression(Precedence::LOWEST.index());

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(StatementVariant::Expression(statement))
    }

    fn parse_expression(&mut self, precedence: usize) -> Option<ExpressionVariants> {
        if !self.prefix_parse_fns.contains_key(&self.current_token.typ) {
            self.no_prefix_parse_fn_error(self.current_token.typ);
            return None;
        } else {
            let mut left_exp = self.prefix_parse_fns[&self.current_token.typ](self);

            while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_preference() {
                if !self.infix_parse_fns.contains_key(&self.peek_token.typ) {
                    return left_exp;
                } else {
                    let infix = self.infix_parse_fns[&self.peek_token.typ];
                    self.next_token();
                    left_exp = infix(self, left_exp?);
                }
            }

            left_exp
        }
    }

    fn parse_infix_expression(&mut self, left: ExpressionVariants) -> Option<ExpressionVariants> {
        // TODO: Fix this, its ugly.
        let mut expression = InfixExpression {
            token: self.current_token.clone(),
            operator: self.current_token.literal.clone(),
            left: Box::new(left),
            right: Box::new(ExpressionVariants::Ident(Identifier {
                token: Token::new(TokenType::ASTERISK, ';'),
                value: "none".to_string(),
            })),
        };

        let precedence = self.current_precedence();

        self.next_token();
        if let Some(exp) = self.parse_expression(precedence) {
            expression.right = Box::new(exp);
        }

        Some(ExpressionVariants::Infix(expression))
    }

    fn parse_prefix_expression(&mut self) -> Option<ExpressionVariants> {
        let mut expression = PrefixExpression {
            // TODO: Refactor using a constructor
            token: self.current_token.clone(),
            operator: self.current_token.literal.clone(),
            right: Box::new(ExpressionVariants::Ident(Identifier {
                token: Token::new(TokenType::ASTERISK, ';'),
                value: "none".to_string(),
            })),
        };

        self.next_token();

        //expression.right = self.parse_expression(Precedence::PREFIX);
        if let Some(ex) = self.parse_expression(Precedence::PREFIX.index()) {
            expression.right = Box::new(ex)
        }
        Some(ExpressionVariants::Prefix(expression))
    }

    fn parse_return_statement(&mut self) -> Option<StatementVariant> {
        let statement = ReturnStatement {
            token: self.current_token.clone(),
            return_value: None,
        };

        self.next_token();

        // We are skipping expressions until we encounter a semicolon
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(StatementVariant::Return(statement))
    }

    fn parse_let_statement(&mut self) -> Option<StatementVariant> {
        let mut statement = LetStatement {
            token: self.current_token.clone(),
            name: Identifier {
                token: self.current_token.clone(),
                value: String::new(),
            },
            value: None,
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        statement.name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        self.next_token();

        // Current token is now <expr> in
        // let <ident> = <expr>
        statement.value = self.parse_expression(Precedence::LOWEST.index());

        // We are skipping the expressions
        // until we encounter a semicolon
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(StatementVariant::Let(statement))
    }

    fn parse_integer_literal(&mut self) -> Option<ExpressionVariants> {
        let mut literal = IntegerLiteral {
            token: self.current_token.clone(),
            value: 0,
        };

        // Parse the string as an integer
        // Handle parsing error instead of default to 0!
        let value = self.current_token.literal.parse::<i64>();

        match value {
            Ok(v) => {
                literal.value = v;

                return Some(ExpressionVariants::Integer(literal));
            }
            Err(e) => {
                eprintln!("Parse error: {}, found {}", e, self.current_token.literal);
                return None;
            }
        }
    }

    /////////////////////
    // Error functions.
    /////////////////////

    pub fn errors(&self) -> Vec<String> {
        //TODO: This is not optimal...
        self.errors.clone()
    }

    fn peek_error(&mut self, token: TokenType) {
        let message = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token, self.peek_token.typ
        );
        self.errors.push(message);
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "No prefix parse function for {:?} found",
            token_type,
        ));
    }

    //////////////////////
    // Utility functions.
    //////////////////////

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.typ == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.typ == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type.clone());
            false
        }
    }

    fn peek_preference(&self) -> usize {
        if let Some(precedence) = precedences().get(&self.peek_token.typ) {
            precedence.index()
        } else {
            Precedence::LOWEST.index()
        }
    }

    fn current_precedence(&self) -> usize {
        if let Some(precedence) = precedences().get(&self.current_token.typ) {
            precedence.index()
        } else {
            Precedence::LOWEST.index()
        }
    }

    /////////////////////////////////////////////
    // These methods add entries to the hashmaps
    /////////////////////////////////////////////

    fn register_prefix(&mut self, token_type: TokenType, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: TokenType, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, function);
    }
}

type PrefixParseFn = fn(&mut Parser) -> ExpressionVariant;
type InfixParseFn = fn(&mut Parser, ExpressionVariants) -> ExpressionVariant;

#[derive(Clone, Copy, Debug)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > OR <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

impl Precedence {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub fn precedences() -> HashMap<TokenType, Precedence> {
    HashMap::from([
        (TokenType::EQ, Precedence::EQUALS),
        (TokenType::NEQ, Precedence::EQUALS),
        (TokenType::LT, Precedence::LESSGREATER),
        (TokenType::GT, Precedence::LESSGREATER),
        (TokenType::PLUS, Precedence::SUM),
        (TokenType::MINUS, Precedence::SUM),
        (TokenType::SLASH, Precedence::PRODUCT),
        (TokenType::ASTERISK, Precedence::PRODUCT),
    ])
}

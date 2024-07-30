use std::collections::HashMap;

use crate::{
    ast::{
        ExpressionStatement, ExpressionVariant, Identifier, LetStatement, Program, ReturnStatement,
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

        parser
    }

    pub fn errors(&self) -> Vec<String> {
        // This is not optimal...
        self.errors.clone()
    }

    fn peek_error(&mut self, token: TokenType) {
        let message = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token, self.peek_token.typ
        );
        self.errors.push(message);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while !self.current_token_is(TokenType::EOF) {
            let statement = self.parse_statement();

            match statement {
                Some(s) => program.statements.push(s),
                None => {}
            }
            //program.statements.push(statement);
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

        statement.expression = self.parse_expression();

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(StatementVariant::Expression(statement))
    }

    fn parse_expression(&self) -> Option<Identifier> {
        todo!()
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
                value: self.current_token.literal.clone(),
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

        // We are skipping the expressions
        // until we encounter a semicolon
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(StatementVariant::Let(statement))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, token: TokenType) -> bool {
        self.current_token.typ == token
    }

    fn peek_token_is(&self, token: TokenType) -> bool {
        self.peek_token.typ == token
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            self.peek_error(token.clone());
            false
        }
    }

    // These methods add entries to the hashmaps
    fn register_prefix(&mut self, token_type: TokenType, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: TokenType, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, function);
    }
}

type PrefixParseFn = fn() -> ExpressionVariant;
type InfixParseFn = fn(ExpressionVariant) -> ExpressionVariant;

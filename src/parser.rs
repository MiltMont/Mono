use std::fmt::format;

use crate::{
    ast::{Identifier, LetStatement, Program, StatementVariant},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::ILLEGAL, ' '),
            peek_token: Token::new(TokenType::ILLEGAL, ' '),
            errors: Vec::new(),
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
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<StatementVariant> {
        let mut statement = LetStatement {
            token: self.current_token.clone(),
            name: Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            },
            value: (),
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
}

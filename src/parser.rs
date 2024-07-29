use crate::{
    ast::{ExpressionVariant, Identifier, LetStatement, Program, Statement, StatementVariant},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::ILLEGAL, ' '),
            peek_token: Token::new(TokenType::ILLEGAL, ' '),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        let eof = Token::new(TokenType::EOF, '0');

        while self.current_token != eof {
            let statement = self.parse_statement();

            program.statements.push(statement);
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> StatementVariant {
        match self.current_token.typ {
            TokenType::LET => self.parse_let_statement(),
            TokenType::ILLEGAL => todo!(),
            TokenType::EOF => todo!(),
            TokenType::IDENT => todo!(),
            TokenType::INT => todo!(),
            TokenType::ASSIGN => todo!(),
            TokenType::PLUS => todo!(),
            TokenType::MINUS => todo!(),
            TokenType::BANG => todo!(),
            TokenType::ASTERISK => todo!(),
            TokenType::SLASH => todo!(),
            TokenType::LT => todo!(),
            TokenType::GT => todo!(),
            TokenType::COMMA => todo!(),
            TokenType::SEMICOLON => todo!(),
            TokenType::LPAREN => todo!(),
            TokenType::RPAREN => todo!(),
            TokenType::LBRACE => todo!(),
            TokenType::RBRACE => todo!(),
            TokenType::FUNCTION => todo!(),

            TokenType::TRUE => todo!(),
            TokenType::FALSE => todo!(),
            TokenType::IF => todo!(),
            TokenType::ELSE => todo!(),
            TokenType::RETURN => todo!(),
            TokenType::EQ => todo!(),
            TokenType::NEQ => todo!(),
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

        todo!()
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
            false
        }
    }
}

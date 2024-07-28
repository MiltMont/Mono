use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: ' ' as char,
        };
        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }

        self.position = self.read_position;

        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut token: Token = Token::new(TokenType::LET, ' ');

        self.skip_whitespace();

        dbg!(&token);
        match self.ch {
            '=' => token = Token::new(TokenType::ASSIGN, self.ch),
            ';' => token = Token::new(TokenType::SEMICOLON, self.ch),
            '(' => token = Token::new(TokenType::LPAREN, self.ch),
            ')' => token = Token::new(TokenType::RPAREN, self.ch),
            ',' => token = Token::new(TokenType::COMMA, self.ch),
            '+' => token = Token::new(TokenType::PLUS, self.ch),
            '{' => token = Token::new(TokenType::LBRACE, self.ch),
            '}' => token = Token::new(TokenType::RBRACE, self.ch),
            '0' => token = Token::new(TokenType::EOF, ' '),
            _ => {
                if is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.typ = TokenType::serialize(&token.literal);
                    return token;
                } else if is_digit(self.ch) {
                    token.literal = self.read_number();
                    token.typ = TokenType::INT;
                    return token;
                }
            }
        }

        self.read_char();

        token
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}

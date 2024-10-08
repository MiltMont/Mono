use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: ' ' as char,
        };
        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let mut token: Token = Token::new(TokenType::ILLEGAL, ' ');

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token.typ = TokenType::EQ;
                    token.literal = String::from("==");
                } else {
                    token = Token::new(TokenType::ASSIGN, self.ch);
                }
            }
            ';' => token = Token::new(TokenType::SEMICOLON, self.ch),
            '(' => token = Token::new(TokenType::LPAREN, self.ch),
            ')' => token = Token::new(TokenType::RPAREN, self.ch),
            ',' => token = Token::new(TokenType::COMMA, self.ch),
            '+' => token = Token::new(TokenType::PLUS, self.ch),
            '-' => token = Token::new(TokenType::MINUS, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token.typ = TokenType::NEQ;
                    token.literal = String::from("!=");
                } else {
                    token = Token::new(TokenType::BANG, self.ch)
                }
            }
            '/' => token = Token::new(TokenType::SLASH, self.ch),
            '*' => token = Token::new(TokenType::ASTERISK, self.ch),
            '<' => token = Token::new(TokenType::LT, self.ch),
            '>' => token = Token::new(TokenType::GT, self.ch),
            '{' => token = Token::new(TokenType::LBRACE, self.ch),
            '}' => token = Token::new(TokenType::RBRACE, self.ch),
            '\0' => token = Token::new(TokenType::EOF, ' '),
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

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }

        self.position = self.read_position;

        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '0'
        } else {
            self.input.as_bytes()[self.read_position] as char
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}

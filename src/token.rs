#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(typ: TokenType, ch: char) -> Self {
        Self {
            typ,
            literal: ch.to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers and literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

    EQ,
    NEQ,
}

impl TokenType {
    // TODO: Refactor this using a hashmap.
    pub fn serialize(string: &String) -> TokenType {
        let value = match string.as_str() {
            "let" => Self::LET,
            "fn" => Self::FUNCTION,
            "true" => Self::TRUE,
            "false" => Self::FALSE,
            "if" => Self::IF,
            "else" => Self::ELSE,
            "return" => Self::RETURN,
            _ => Self::IDENT,
        };

        value
    }
}

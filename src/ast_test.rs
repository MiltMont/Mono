#[cfg(test)]
mod tests {
    use crate::{
        ast::{Identifier, LetStatement, Node, Program, StatementVariant},
        token::{Token, TokenType},
    };

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![StatementVariant::Let(LetStatement {
                token: Token {
                    typ: TokenType::LET,
                    literal: "let".to_string(),
                },
                name: Identifier {
                    token: Token {
                        typ: TokenType::IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: Some(Identifier {
                    token: Token {
                        typ: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                }),
            })],
        };

        if program.string() != "let myVar = anotherVar;" {
            panic!("Found: {} expected let myVar =;", program.string());
        }
    }
}

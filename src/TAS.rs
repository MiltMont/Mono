use crate::token::{Token, TokenType};



trait Node {
    fn token_literal(&self) -> String {};
}

impl TokenLiteral for Node {
    fn token_literal(&self) -> String {
        todo!()
    }
}

pub trait TokenLiteral {
    fn token_literal(&self) -> String;
}

pub struct Statement {
    node: Node,
}

impl TokenLiteral for Statement {
    fn token_literal(&self) -> String {
        todo!()
    }
}

struct Expression {
    node: Node,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl TokenLiteral for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }
}

struct Identifier {
    Token: Token,
    Value: String,
}

impl Identifier {
    fn expression_node(&self) {}
}

impl TokenLiteral for Identifier {
    fn token_literal(&self) -> String {
        // TODO: Review the performance of this block.
        self.Token.literal.clone()
    }
}

struct LetStatement {
    token: Token,
    identifier: Identifier,
    value: Expression,
}

impl LetStatement {
    fn statement_node(&self) {}
}

impl TokenLiteral for LetStatement {
    fn token_literal(&self) -> String {
        // TODO: Review the performance of this block.
        self.token.literal.clone()
    }
}

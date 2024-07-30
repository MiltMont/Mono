use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement {
    fn statement_node(&self);
}

#[derive(Debug)]
pub enum StatementVariant {
    Let(LetStatement),
}

// Statements are nodes.
impl Node for StatementVariant {
    fn token_literal(&self) -> String {
        match self {
            StatementVariant::Let(_) => "let".to_string(),
        }
    }
}

pub trait Expression {
    fn expression_node(&self);
}

pub type ExpressionVariant = ();

impl Expression for ExpressionVariant {
    fn expression_node(&self) {
        todo!()
    }
}

// Expressions are node.
impl Node for dyn Expression {
    fn token_literal(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct Program {
    // This is a vector of objects implementing the Statement trait.
    pub statements: Vec<StatementVariant>,
}

// The program is the root node.
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            " ".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: ExpressionVariant,
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

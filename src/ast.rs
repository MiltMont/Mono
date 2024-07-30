use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;

    fn string(&self) -> String;
}

pub trait Statement {
    fn statement_node(&self);
}

// Improve this!
#[derive(Debug)]
pub enum StatementVariant {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

// Statements are nodes.
impl Node for StatementVariant {
    fn token_literal(&self) -> String {
        match self {
            StatementVariant::Let(_) => "let".to_string(),
            StatementVariant::Return(_) => "return".to_string(),
            StatementVariant::Expression(_) => todo!(),
        }
    }

    fn string(&self) -> String {
        match self {
            StatementVariant::Let(s) => s.string(),
            StatementVariant::Return(s) => s.string(),
            StatementVariant::Expression(s) => s.string(),
        }
    }
}

pub trait Expression {
    fn expression_node(&self);
}

pub type ExpressionVariant = Option<Identifier>;

/*
impl Expression for ExpressionVariant {
    fn expression_node(&self) {
        todo!()
    }
}
*/

// Expressions are node.
impl Node for dyn Expression {
    fn token_literal(&self) -> String {
        todo!()
    }

    fn string(&self) -> String {
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

    fn string(&self) -> String {
        let mut out = String::from("");
        for statement in &self.statements {
            out.push_str(&statement.string());
        }

        out
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

    fn string(&self) -> String {
        self.value.clone()
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

    fn string(&self) -> String {
        let mut out = String::from("");

        out.push_str(&self.token_literal());
        out.push(' ');
        out.push_str(&self.name.string());
        out.push_str(" = ");

        match &self.value {
            Some(ident) => out.push_str(&ident.string()),
            None => (),
        }

        out.push(';');

        out
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: ExpressionVariant,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::from("");

        out.push_str(&self.token_literal());
        out.push_str(" ");

        match &self.return_value {
            Some(ident) => out.push_str(&ident.string()),
            None => (),
        }

        out.push(';');

        out
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: ExpressionVariant,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        match &self.expression {
            Some(exp) => exp.string(),
            None => "".to_string(),
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }
}

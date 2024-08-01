use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;

    fn string(&self) -> String;
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

///////////////////////
// Statements   ///////
///////////////////////
pub trait Statement: Node {
    fn statement_node(&self);
}
// TODO: How can I do this better?
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

        if let Some(expr_var) = &self.value {
            if let ExpressionVariants::Ident(ident) = expr_var {
                out.push_str(&ident.string());
            }
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

        if let Some(expr_var) = &self.return_value {
            if let ExpressionVariants::Ident(ident) = expr_var {
                out.push_str(&ident.string());
            }
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

///////////////////////
// Expressions  ///////
///////////////////////

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<ExpressionVariants>,
    pub operator: String,
    pub right: Box<ExpressionVariants>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<ExpressionVariants>, //?
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token_literal()
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionVariants {
    Ident(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

impl Node for ExpressionVariants {
    fn token_literal(&self) -> String {
        todo!()
    }

    fn string(&self) -> String {
        match self {
            ExpressionVariants::Ident(ident) => ident.string(),
            ExpressionVariants::Integer(int_lit) => int_lit.string(),
            ExpressionVariants::Prefix(pe) => pe.string(),
            ExpressionVariants::Infix(ie) => ie.string(),
        }
    }
}

pub type ExpressionVariant = Option<ExpressionVariants>;

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token, // the { token
    pub statements: Vec<StatementVariant>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        for statement in self.statements.iter() {
            out.push_str(&statement.string());
        }

        out
    }
}

#[derive(Debug, Clone)]
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

        out.push_str(&format!(
            "{} {} = ",
            &self.token_literal(),
            &self.name.string()
        ));

        if let Some(expr_var) = &self.value {
            if let ExpressionVariants::Ident(ident) = expr_var {
                out.push_str(&ident.string());
            }
        }

        out.push(';');

        out
    }
}

#[derive(Debug, Clone)]
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

        out.push_str(&format!("{} ", &self.token_literal()));

        if let Some(expr_var) = &self.return_value {
            if let ExpressionVariants::Ident(ident) = expr_var {
                out.push_str(&ident.string());
            }
        }
        out.push(';');

        out
    }
}

#[derive(Debug, Clone)]
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
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Expression for Boolean {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<ExpressionVariants>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::from(format!(
            "if{} {}",
            self.condition.string(),
            self.consequence.string()
        ));

        if let Some(alternative) = &self.alternative {
            out.push_str(&format!("else {}", alternative.string()));
        }

        out
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionVariants {
    Ident(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Boolean(Boolean),
    If(IfExpression),
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
            ExpressionVariants::Boolean(b) => b.string(),
            ExpressionVariants::If(ie) => ie.string(),
        }
    }
}

pub type ExpressionVariant = Option<ExpressionVariants>;

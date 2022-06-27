use std::rc::Rc;

use downcast_rs::{impl_downcast, Downcast};

use crate::token::Token;
use core::fmt::Debug;

pub trait Node: Downcast {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}
impl_downcast!(Node);

/* ----------------------------------------------- */
pub trait Statement: Node {
    fn statement_node(&self) -> bool;
}
impl_downcast!(Statement);

impl Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let r = write!(f, "Statement -> {}", self.token_literal());
        r
    }
}

/* ----------------------------------------------- */
pub trait Expression: Node {
    fn expression_node(&self) -> bool;
}
impl_downcast!(Expression);

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let r = write!(f, "Expression -> {}", self.token_literal());
        r
    }
}

/* ----------------------------------------------- */
#[derive(Debug)]
pub struct Identifier {
    pub token: Rc<Token>,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) -> bool {
        println!("Identifier statement_node");
        true
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

/* ----------------------------------------------- */
pub struct IntegerLiteral {
    pub token: Rc<Token>,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();
        buf.push_str(format!("{}", self.value).as_str());
        buf
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
pub struct PrefixExpression {
    pub token: Rc<Token>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();
        buf.push('(');
        buf.push_str(self.operator.as_str());
        buf.push_str(self.right.string().as_str());
        buf.push(')');
        buf
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
pub struct InfixExpression {
    pub token: Rc<Token>,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();
        buf.push('(');
        buf.push_str(self.left.string().as_str());
        buf.push(' ');
        buf.push_str(self.operator.as_str());
        buf.push(' ');
        buf.push_str(self.right.string().as_str());
        buf.push(')');
        buf
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
pub struct IfExpression {
    pub token: Rc<Token>,
    pub condition: Box<dyn Expression>,
    pub consequence: Box<BlockStatement>,
    pub alternative: Option<Box<BlockStatement>>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();

        buf.push_str("if");
        buf.push_str(self.condition.string().as_str());
        buf.push(' ');
        buf.push_str(self.consequence.string().as_str());

        if let Some(alt) = &self.alternative {
            buf.push_str("else ");
            buf.push_str(alt.string().as_str());
        }
        buf
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
pub struct FunctionLiteral {
    pub token: Rc<Token>,
    pub parameters: Vec<Identifier>,
    pub body: Box<BlockStatement>,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|p| p.string())
            .collect::<Vec<String>>()
            .join(", ");

        let mut buf = String::new();
        buf.push_str(self.token_literal().as_str());
        buf.push('(');
        buf.push_str(&params);
        buf.push(')');
        buf.push_str(self.body.string().as_str());
        buf
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
#[derive(Debug)]
pub struct LetStatement {
    pub token: Rc<Token>,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();

        buf.push_str(&(self.token_literal() + " "));
        buf.push_str(&(self.name.string()));
        buf.push_str(" = ");

        if let Some(val) = &self.value {
            buf.push_str(&(val.string()))
        }

        buf.push(';');

        buf
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) -> bool {
        println!("let statement statement_node");
        true
    }
}

/* ----------------------------------------------- */
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Rc<Token>,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();

        buf.push_str(&(self.token_literal() + " "));

        if let Some(ret_val) = &self.return_value {
            buf.push_str(&(ret_val.string()))
        }

        buf.push(';');

        buf
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) -> bool {
        println!("return statement statement_node");
        true
    }
}

/* ----------------------------------------------- */
pub struct ExpressionStatement {
    pub token: Rc<Token>,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();
        if let Some(expr) = &self.expression {
            buf.push_str(&(expr.string()))
        }
        buf
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) -> bool {
        println!("expression statement statement_node");
        true
    }
}

/* ----------------------------------------------- */
pub struct BlockStatement {
    pub token: Rc<Token>,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut buf = String::new();
        for stmt in &self.statements {
            buf.push_str(stmt.string().as_str());
        }
        buf
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
pub struct Boolean {
    pub token: Rc<Token>,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Expression for Boolean {
    fn expression_node(&self) -> bool {
        true
    }
}

/* ----------------------------------------------- */
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }

    fn string(&self) -> String {
        let mut buf = String::new();
        for statement in &self.statements {
            buf.push_str(&statement.string());
        }
        buf
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Identifier, LetStatement, Program};
    use crate::{
        ast::Node,
        token::{Token, IDENT, LET},
    };
    use std::rc::Rc;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Rc::new(Token {
                    token_type: LET,
                    literal: String::from("let"),
                }),
                name: Identifier {
                    token: Rc::new(Token {
                        token_type: IDENT,
                        literal: String::from("myVar"),
                    }),
                    value: String::from("myVar"),
                },
                value: Some(Box::new(Identifier {
                    token: Rc::new(Token {
                        token_type: IDENT,
                        literal: String::from("anotherVar"),
                    }),
                    value: String::from("anotherVar"),
                })),
            })],
        };
        assert_eq!(program.string(), "let myVar = anotherVar;");
    }
}

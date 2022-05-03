use std::rc::Rc;

use downcast_rs::{impl_downcast, Downcast};

use crate::token::Token;

pub trait Node: Downcast {
    fn token_literal(&self) -> String;
}
impl_downcast!(Node);

pub trait Statement: Node {
    fn statement_node(&self) -> bool;
}
impl_downcast!(Statement);

pub trait Expression: Node {
    fn expression_node(&self) -> bool;
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

use core::fmt::Debug;
impl Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let r = write!(f, "Statement -> {}", self.token_literal());
        r
    }
}

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
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Rc<Token>,
    pub name: Identifier,
    // TODO: 後で実装
    // pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) -> bool {
        println!("let statement statement_node");
        true
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

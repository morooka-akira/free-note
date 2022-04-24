use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self) -> bool;
}

trait Expression: Node {
    fn expression_node(&self) -> bool;
}

struct Identifier {
    token: Token,
    value: String,
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Identifier {
    fn expression_node(&self) {
        println!("Identifier statement_node");
    }

    fn token_literale(&self) -> String {
        self.token.literal.to_string()
    }
}

impl LetStatement {
    fn statement_node(&self) {
        println!("let statement statement_node");
    }

    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

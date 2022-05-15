use std::rc::Rc;

use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType, ASSIGN, EOF, IDENT, LET, RETURN, SEMICOLON},
};

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    cur_token: Rc<Token>,
    peek_token: Rc<Token>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let cur_token = Rc::new(lexer.next_token());
        let peek_token = Rc::new(lexer.next_token());
        Parser {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.cur_token = Rc::clone(&self.peek_token);
        self.peek_token = Rc::new(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();

        while !self.cur_token_is(EOF) {
            let stmt = self.parse_statement();
            match stmt {
                Some(stmt) => program.statements.push(stmt),
                None => println!("Statement not found"),
            }
            self.next_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("{}", self.cur_token.token_type);
        match self.cur_token.token_type {
            LET => self.parse_let_statement(),
            RETURN => self.parse_return_statement(),
            _ => {
                println!("token_type unknown");
                None
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = Rc::clone(&self.cur_token);

        if !self.expect_peek(IDENT) {
            return None;
        }

        let name = Identifier {
            token: Rc::clone(&self.cur_token),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(ASSIGN) {
            return None;
        }
        // TODO: とりあえずセミコロンまで値を読み飛ばす
        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(LetStatement {
            token,
            name,
            value: None,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = Rc::clone(&self.cur_token);

        self.next_token();

        // TODO: とりあえずセミコロンまで値を読み飛ばす
        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement {
            token,
            return_value: None,
        }))
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn cur_token_is(&mut self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{LetStatement, Node, ReturnStatement},
        lexer::Lexer,
    };

    #[test]
    fn test_let_statement() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

        let mut l = Lexer::new(input);
        let mut p = super::Parser::new(&mut l);

        let program = p.parse_program();
        if program.is_err() {
            panic!("parse_program() returned an error");
        }
        check_parser_errors(&p);
        let program = program.unwrap();
        let statement_len = program.statements.len();
        if statement_len != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                statement_len
            );
        }
        let expected_identifier = vec!["x", "y", "foobar"];
        for (i, expected) in expected_identifier.iter().enumerate().take(statement_len) {
            let st = &program.statements[i];
            let let_st = (*st).downcast_ref::<LetStatement>().unwrap();
            // TODO: 値のテストは後で書く
            assert_eq!(
                let_st.name.value,
                expected.to_string(),
                "let_st.name.value not correct"
            );
            assert_eq!(
                let_st.name.token_literal(),
                expected.to_string(),
                "let_st.name.token_literal not correct"
            );
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
            return = 5;
            return 10;
            return = 993322;
        "#;

        let mut l = Lexer::new(input);
        let mut p = super::Parser::new(&mut l);

        let program = p.parse_program();
        if program.is_err() {
            panic!("parse_program() returned an error");
        }
        check_parser_errors(&p);
        let program = program.unwrap();
        let statement_len = program.statements.len();
        if statement_len != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                statement_len
            );
        }
        for st in program.statements.iter() {
            let return_st = (*st).downcast_ref::<ReturnStatement>().unwrap();
            assert_eq!(
                return_st.token_literal(),
                "return",
                "token_literal not correct"
            );
        }
    }

    fn check_parser_errors(p: &super::Parser) {
        if p.errors().is_empty() {
            return;
        }
        for error in p.errors() {
            print!("error -> {}", error);
        }
        panic!("parser has {} errors", p.errors().len());
    }
}

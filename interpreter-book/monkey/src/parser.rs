use std::rc::Rc;

use crate::{ast::Program, lexer::Lexer, token::Token};

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    cur_token: Rc<Token>,
    peek_token: Rc<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let cur_token = Rc::new(lexer.next_token());
        let peek_token = Rc::new(lexer.next_token());
        Parser {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = Rc::clone(&self.peek_token);
        self.peek_token = Rc::new(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        Err(String::from("Nothing"))
    }
}

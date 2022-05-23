use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType, ASSIGN, BANG, EOF, IDENT, INT, LET, MINUS, RETURN, SEMICOLON},
};

// トークンタイプが前置で出現した時
type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
// トークンタイプが中置で出現した時
type InfixParseFn = fn(dyn Expression) -> Result<Box<dyn Expression>, String>;

enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,

    cur_token: Rc<Token>,
    peek_token: Rc<Token>,

    errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let cur_token = Rc::new(lexer.next_token());
        let peek_token = Rc::new(lexer.next_token());
        let mut parser = Parser {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.register_prefix(IDENT, Parser::parse_identifier);
        parser.register_prefix(INT, Parser::parse_integer_literal);
        parser.register_prefix(BANG, Parser::parse_prefix_expression);
        parser.register_prefix(MINUS, Parser::parse_prefix_expression);
        parser
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
            _ => self.parse_expression_statement(),
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

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = Rc::clone(&self.cur_token);

        let expression = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ExpressionStatement { token, expression }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        if let Some(prefix) = self.prefix_parse_fns.get(&self.cur_token.token_type) {
            prefix(self)
        } else {
            self.no_prefix_parse_fn_error(self.cur_token.token_type);
            None
        }
    }

    fn parse_identifier(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: parser.cur_token.clone(),
            value: parser.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let token = parser.cur_token.clone();
        match token.literal.parse::<i64>() {
            Ok(value) => Some(Box::new(IntegerLiteral { token, value })),
            Err(err) => {
                parser.errors.push(format!("{}", err));
                None
            }
        }
    }

    fn parse_prefix_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let token = Rc::clone(&parser.cur_token);
        let operator = token.literal.as_str();

        parser.next_token();

        match parser.parse_expression(Precedence::PREFIX) {
            Some(exp) => Some(Box::new(PrefixExpression {
                token: Rc::clone(&token),
                operator: operator.to_string(),
                right: exp,
            })),
            None => {
                parser
                    .errors
                    .push("no prefix parse function for this token".to_string());
                None
            }
        }
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

    fn register_prefix(&mut self, token_type: TokenType, parse_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, parse_fn);
    }

    fn register_infix(&mut self, token_type: TokenType, parse_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, parse_fn);
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {} found", token_type);
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Node,
            PrefixExpression, ReturnStatement,
        },
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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let mut l = Lexer::new(input);
        let mut p = super::Parser::new(&mut l);

        let program = p.parse_program();
        if program.is_err() {
            panic!("parse_program() returned an error");
        }
        check_parser_errors(&p);

        let program = program.unwrap();
        let statement_len = program.statements.len();
        if statement_len != 1 {
            panic!(
                "program.statements does not contain 1 statements. got={}",
                statement_len
            );
        }
        let st = &program.statements[0];
        let exp_st = (*st).downcast_ref::<ExpressionStatement>().unwrap();
        if let Some(exp) = &exp_st.expression {
            let ident = exp.downcast_ref::<Identifier>().unwrap();
            assert_eq!(ident.value, "foobar");
            assert_eq!(ident.token_literal(), "foobar");
        } else {
            panic!("exp_st.expression is None");
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let mut l = Lexer::new(input);
        let mut p = super::Parser::new(&mut l);

        let program = p.parse_program();
        if program.is_err() {
            panic!("parse_program() returned an error");
        }
        check_parser_errors(&p);

        let program = program.unwrap();
        let statement_len = program.statements.len();
        if statement_len != 1 {
            panic!(
                "program.statements does not contain 1 statements. got={}",
                statement_len
            );
        }
        let st = &program.statements[0];
        let exp_st = (*st).downcast_ref::<ExpressionStatement>().unwrap();
        if let Some(int_literal) = &exp_st.expression {
            let literal = int_literal.downcast_ref::<IntegerLiteral>().unwrap();
            assert_eq!(literal.value, 5);
        } else {
            panic!("exp_st.expression is None");
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, operator, integer_value) in prefix_tests {
            let mut l = Lexer::new(input);
            let mut p = super::Parser::new(&mut l);

            let program = p.parse_program();
            if program.is_err() {
                panic!("parse_program() returned an error");
            }
            check_parser_errors(&p);

            let program = program.unwrap();
            let statement_len = program.statements.len();
            if statement_len != 1 {
                panic!(
                    "program.statements does not contain 1 statements. got={}",
                    statement_len
                );
            }
            let st = &program.statements[0];
            let exp_st = (*st).downcast_ref::<ExpressionStatement>().unwrap();
            if let Some(exp) = &exp_st.expression {
                let prefix_exp = exp.downcast_ref::<PrefixExpression>().unwrap();
                assert_eq!(prefix_exp.operator, operator);
                test_integer_literal(prefix_exp.right.as_ref(), integer_value);
            } else {
                panic!("exp_st.expression is None");
            }
        }

        fn test_integer_literal(i_literal: &dyn Expression, value: i64) {
            if let Some(literal) = i_literal.downcast_ref::<IntegerLiteral>() {
                assert_eq!(literal.value, value);
            } else {
                panic!("i_literal is not IntegerLiteral");
            }
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

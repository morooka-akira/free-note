use once_cell::sync::Lazy;
use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{
        BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
        Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression,
        Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{
        Token, TokenType, ASSIGN, ASTERISK, BANG, COMMA, ELSE, EOF, EQ, FALSE, FUNCTION, GT, IDENT,
        IF, INT, LBRACE, LET, LPAREN, LT, MINUS, NOT_EQ, PLUS, RBRACE, RETURN, RPAREN, SEMICOLON,
        SLASH, TRUE,
    },
};

// トークンタイプが前置で出現した時
type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
// トークンタイプが中置で出現した時
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

#[derive(Clone, Copy)]
enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

static PRECEDENCES: Lazy<HashMap<TokenType, Precedence>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(EQ, Precedence::EQUALS);
    m.insert(NOT_EQ, Precedence::EQUALS);
    m.insert(LT, Precedence::LESSGREATER);
    m.insert(GT, Precedence::LESSGREATER);
    m.insert(PLUS, Precedence::SUM);
    m.insert(MINUS, Precedence::SUM);
    m.insert(SLASH, Precedence::PRODUCT);
    m.insert(ASTERISK, Precedence::PRODUCT);
    m.insert(LPAREN, Precedence::CALL);
    m
});

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
        parser.register_prefix(TRUE, Parser::parse_boolean);
        parser.register_prefix(FALSE, Parser::parse_boolean);
        parser.register_prefix(LPAREN, Parser::parse_grouped_expression);
        parser.register_prefix(IF, Parser::parse_if_expression);
        parser.register_prefix(FUNCTION, Parser::parse_function_literal);

        parser.register_infix(PLUS, Parser::parse_infix_expression);
        parser.register_infix(MINUS, Parser::parse_infix_expression);
        parser.register_infix(SLASH, Parser::parse_infix_expression);
        parser.register_infix(ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(EQ, Parser::parse_infix_expression);
        parser.register_infix(NOT_EQ, Parser::parse_infix_expression);
        parser.register_infix(LT, Parser::parse_infix_expression);
        parser.register_infix(GT, Parser::parse_infix_expression);
        parser.register_infix(LPAREN, Parser::parse_call_expression);
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
        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(LetStatement { token, name, value }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = Rc::clone(&self.cur_token);

        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement {
            token,
            return_value: value,
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
        let prefix = self.prefix_parse_fns.get(&self.cur_token.token_type);
        if prefix.is_none() {
            self.no_prefix_parse_fn_error(self.cur_token.token_type);
            return None;
        }
        let mut left_exp = prefix.unwrap()(self);
        while !self.peek_token_is(SEMICOLON)
            && (precedence as i32) < (self.peek_precedence() as i32)
        {
            let infix = self.infix_parse_fns.get(self.peek_token.token_type);
            if let Some(infix) = infix {
                // NOTE: next_tokenが上手く呼べないので暫定でinline化している
                self.cur_token = Rc::clone(&self.peek_token);
                self.peek_token = Rc::new(self.lexer.next_token());
                left_exp = infix(self, left_exp.unwrap());
            } else {
                return left_exp;
            }
        }
        left_exp
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

    fn parse_boolean(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        Some(Box::new(Boolean {
            token: parser.cur_token.clone(),
            value: parser.cur_token_is(TRUE),
        }))
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

    fn parse_grouped_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        parser.next_token();

        let exp = parser.parse_expression(Precedence::LOWEST);
        if parser.expect_peek(RPAREN) {
            exp
        } else {
            None
        }
    }

    fn parse_infix_expression(
        parser: &mut Parser,
        left: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let token = Rc::clone(&parser.cur_token);
        let operator = token.literal.as_str();
        let precedence = parser.cur_precedence();

        parser.next_token();

        match parser.parse_expression(precedence) {
            Some(right) => Some(Box::new(InfixExpression {
                token: Rc::clone(&token),
                operator: operator.to_string(),
                right,
                left,
            })),
            None => {
                parser
                    .errors
                    .push("no prefix parse function for this token".to_string());
                None
            }
        }
    }

    fn parse_if_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let token = Rc::clone(&parser.cur_token);

        if !parser.expect_peek(LPAREN) {
            return None;
        }

        parser.next_token();
        let condition = parser.parse_expression(Precedence::LOWEST);

        if !parser.expect_peek(RPAREN) {
            return None;
        }

        if !parser.expect_peek(LBRACE) {
            return None;
        }

        let consequence = parser.parse_block_statement();

        let alternative = if parser.peek_token_is(ELSE) {
            parser.next_token();

            if !parser.expect_peek(LBRACE) {
                return None;
            }

            parser.parse_block_statement()
        } else {
            None
        };

        Some(Box::new(IfExpression {
            token,
            condition: condition.unwrap(),
            consequence: consequence.unwrap(),
            alternative,
        }))
    }

    fn parse_function_literal(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let token = Rc::clone(&parser.cur_token);

        if !parser.expect_peek(LPAREN) {
            return None;
        }

        let parameters = parser.parse_function_parameters();

        if !parser.expect_peek(LBRACE) {
            return None;
        }

        let body = parser.parse_block_statement();

        Some(Box::new(FunctionLiteral {
            token,
            parameters: parameters.unwrap(),
            body: body.unwrap(),
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(RPAREN) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();
        let ident = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        identifiers.push(ident);

        while self.peek_token_is(COMMA) {
            // ,はスキップ
            self.next_token();
            self.next_token();
            let ident = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            identifiers.push(ident);
        }

        if !self.expect_peek(RPAREN) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(
        parser: &mut Parser,
        left: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let token = Rc::clone(&parser.cur_token);
        let arguments = parser.parse_call_arguments().unwrap();
        Some(Box::new(CallExpression {
            token,
            function: left,
            arguments,
        }))
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Box<dyn Expression>>> {
        let mut args = Vec::new();
        if self.peek_token_is(RPAREN) {
            self.next_token();
            return Some(args);
        }
        self.next_token();
        args.push(self.parse_expression(Precedence::LOWEST)?);
        while self.peek_token_is(COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::LOWEST)?);
        }
        if !self.expect_peek(RPAREN) {
            return None;
        }
        Some(args)
    }

    fn parse_block_statement(&mut self) -> Option<Box<BlockStatement>> {
        let token = Rc::clone(&self.cur_token);
        let mut statements: Vec<Box<dyn Statement>> = vec![];

        self.next_token();

        while !self.cur_token_is(RBRACE) && !self.peek_token_is(EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Some(Box::new(BlockStatement { token, statements }))
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

    fn peek_precedence(&self) -> Precedence {
        *PRECEDENCES
            .get(&self.peek_token.token_type)
            .unwrap_or(&Precedence::LOWEST)
    }

    fn cur_precedence(&mut self) -> Precedence {
        *PRECEDENCES
            .get(&self.cur_token.token_type)
            .unwrap_or(&Precedence::LOWEST)
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
    use std::any::Any;

    use crate::{
        ast::{
            Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral, Identifier,
            IfExpression, InfixExpression, IntegerLiteral, LetStatement, Node, PrefixExpression,
            ReturnStatement,
        },
        lexer::Lexer,
    };

    #[test]
    fn test_let_statement() {
        let input: Vec<(&str, &str, &dyn Any)> = vec![
            ("let x = 5;", "x", &5),
            ("let y = true;", "y", &true),
            ("let foobar = y;", "foobar", &"y"),
        ];

        for (input, expected_ident, expected_value) in input {
            let mut l = Lexer::new(input);
            let mut parser = super::Parser::new(&mut l);
            let program = parser.parse_program();
            let program = program.unwrap();

            let st = &program.statements[0];
            let let_st = (*st).downcast_ref::<LetStatement>().unwrap();
            assert_eq!(
                let_st.name.value, expected_ident,
                "let_st.name.value not correct"
            );
            assert_eq!(
                &let_st.name.token_literal(),
                expected_ident,
                "let_st.name.token_literal not correct"
            );
            let val = let_st.value.as_ref().unwrap();
            test_literal_expression(val.as_ref(), expected_value);
        }
    }

    #[test]
    fn test_return_statement() {
        let input: Vec<(&str, &dyn Any)> = vec![
            ("return 5", &5),
            ("return 10", &10),
            ("return 993322", &993322),
        ];

        for (input, expected_value) in input {
            let mut l = Lexer::new(input);
            let mut p = super::Parser::new(&mut l);

            let program = p.parse_program();
            if program.is_err() {
                panic!("parse_program() returned an error");
            }
            check_parser_errors(&p);
            let program = program.unwrap();

            let st = &program.statements[0];
            let return_st = (*st).downcast_ref::<ReturnStatement>().unwrap();

            assert_eq!(
                return_st.token_literal(),
                "return",
                "token_literal not correct"
            );

            let val = return_st.return_value.as_ref().unwrap();
            test_literal_expression(val.as_ref(), expected_value);
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
    fn test_boolean_expression() {
        let input = "true;";

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
        if let Some(boolean) = &exp_st.expression {
            let literal = boolean.downcast_ref::<Boolean>().unwrap();
            assert!(literal.value);
        } else {
            panic!("exp_st.expression is None");
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests: Vec<(&str, &str, &dyn Any)> = vec![
            ("!5;", "!", &5_i64),
            ("-15;", "-", &15_i64),
            ("!true;", "!", &true),
            ("!false;", "!", &false),
        ];

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
                test_literal_expression(prefix_exp.right.as_ref(), integer_value);
            } else {
                panic!("exp_st.expression is None");
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests: Vec<(&str, &dyn Any, &str, &dyn Any)> = vec![
            ("5 + 5;", &5_i64, "+", &5_i64),
            ("5 - 5;", &5_i64, "-", &5_i64),
            ("5 * 5;", &5_i64, "*", &5_i64),
            ("5 / 5;", &5_i64, "/", &5_i64),
            ("5 > 5;", &5_i64, ">", &5_i64),
            ("5 < 5;", &5_i64, "<", &5_i64),
            ("5 == 5;", &5_i64, "==", &5_i64),
            ("5 != 5;", &5_i64, "!=", &5_i64),
            ("true == true;", &true, "==", &true),
            ("true != false;", &true, "!=", &false),
            ("false == false;", &false, "==", &false),
        ];

        for (input, left_value, operator, right_value) in infix_tests {
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
                test_infix_expression(exp.as_ref(), left_value, operator, right_value);
            } else {
                panic!("exp_st.expression is None");
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";

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
            let if_exp = (*exp).downcast_ref::<IfExpression>().unwrap();
            test_infix_expression(
                if_exp.condition.as_ref(),
                &"x".to_string(),
                "<",
                &"y".to_string(),
            );
            if if_exp.consequence.statements.len() != 1 {
                panic!(
                    "consequence.statements does not contain 1 statements. got={}",
                    if_exp.consequence.statements.len()
                );
            }
            let st = &if_exp.consequence.statements[0];
            let exp_st = (*st).downcast_ref::<ExpressionStatement>().unwrap();
            if let Some(exp) = &exp_st.expression {
                test_identifier(exp.as_ref(), "x");
            }
            if if_exp.alternative.is_some() {
                panic!("if_exp.alternative is not None");
            }
        } else {
            panic!("exp_st.expression is None");
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";

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
            let if_exp = (*exp).downcast_ref::<IfExpression>().unwrap();
            test_infix_expression(
                if_exp.condition.as_ref(),
                &"x".to_string(),
                "<",
                &"y".to_string(),
            );
            if if_exp.consequence.statements.len() != 1 {
                panic!(
                    "consequence.statements does not contain 1 statements. got={}",
                    if_exp.consequence.statements.len()
                );
            }
            let con_st = &if_exp.consequence.statements[0];
            let exp_st = (*con_st).downcast_ref::<ExpressionStatement>().unwrap();
            if let Some(exp) = &exp_st.expression {
                test_identifier(exp.as_ref(), "x");
            }

            let alt_st = &if_exp.alternative.as_ref().unwrap().statements[0];
            let exp_st = (*alt_st).downcast_ref::<ExpressionStatement>().unwrap();
            if let Some(exp) = &exp_st.expression {
                test_identifier(exp.as_ref(), "y");
            }
        } else {
            panic!("exp_st.expression is None");
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
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
            let fun_exp = (*exp).downcast_ref::<FunctionLiteral>().unwrap();
            if fun_exp.parameters.len() != 2 {
                panic!(
                    "fun_exp.parameters does not contain 2 parameters. got={}",
                    fun_exp.parameters.len()
                );
            }
            test_literal_expression(&fun_exp.parameters[0], &"x");
            test_literal_expression(&fun_exp.parameters[1], &"y");
            let st = &fun_exp.body.as_ref().statements[0];
            let body_stmt = (*st).downcast_ref::<ExpressionStatement>().unwrap();
            let ex = body_stmt.expression.as_ref().unwrap();
            test_infix_expression(ex.as_ref(), &"x", "+", &"y");
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5)";
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
            let call_exp = (*exp).downcast_ref::<CallExpression>().unwrap();
            test_identifier(call_exp.function.as_ref(), "add");

            let arg_len = call_exp.arguments.len();
            if arg_len != 3 {
                panic!(
                    "call_exp.arguments does not contain 3 arguments. got={}",
                    arg_len
                );
            }
            test_literal_expression(call_exp.arguments[0].as_ref(), &1_i64);
            test_infix_expression(call_exp.arguments[1].as_ref(), &2_i64, "*", &3_i64);
            test_infix_expression(call_exp.arguments[2].as_ref(), &4_i64, "+", &5_i64);
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        let input: Vec<(&str, Vec<&str>)> = vec![
            ("fn() {}", vec![]),
            ("fn(x) {}", vec!["x"]),
            ("fn(x, y) {}", vec!["x", "y"]),
        ];

        for (input, expected) in input {
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
                let fun_exp = (*exp).downcast_ref::<FunctionLiteral>().unwrap();
                if fun_exp.parameters.len() != expected.len() {
                    panic!(
                        "fun_exp.parameters does not contain {} parameters. got={}",
                        expected.len(),
                        fun_exp.parameters.len()
                    );
                }
                for (i, param) in fun_exp.parameters.iter().enumerate() {
                    test_literal_expression(param, &expected[i]);
                }
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; - 5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 > 4 != 3 < 4", "((5 > 4) != (3 < 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];
        for (input, expected) in tests {
            let mut l = Lexer::new(input);
            let mut p = super::Parser::new(&mut l);
            let program = p.parse_program();
            if program.is_err() {
                panic!("parse_program() returned an error");
            }
            check_parser_errors(&p);
            let program = program.unwrap();
            assert_eq!(program.string(), expected);
        }
    }

    fn test_infix_expression(
        exp: &dyn Expression,
        left: &dyn Any,
        operator: &str,
        right: &dyn Any,
    ) {
        let op_exp = exp.downcast_ref::<InfixExpression>().unwrap();
        test_literal_expression(op_exp.left.as_ref(), left);
        assert_eq!(op_exp.operator, operator);
        test_literal_expression(op_exp.right.as_ref(), right);
    }

    fn test_literal_expression(exp: &dyn Expression, expected: &dyn Any) {
        if let Some(v) = expected.downcast_ref::<i64>() {
            test_integer_literal(exp, *v);
        } else if let Some(v) = expected.downcast_ref::<i32>() {
            test_integer_literal(exp, *v as i64);
        } else if let Some(v) = expected.downcast_ref::<String>() {
            test_identifier(exp, v);
        } else if let Some(v) = expected.downcast_ref::<&str>() {
            test_identifier(exp, v);
        } else if let Some(v) = expected.downcast_ref::<bool>() {
            test_boolean_literal(exp, *v);
        } else {
            panic!("invalid expected : {:?}.", expected);
        }
    }

    fn test_identifier(exp: &dyn Expression, value: &str) {
        let ident = (*exp).downcast_ref::<Identifier>().unwrap();
        if ident.value != value {
            panic!("ident.value is not {}. got={}", value, ident.value);
        }
        if ident.token_literal() != value {
            panic!(
                "ident.token_literal is not {}. got={}",
                value,
                ident.token_literal()
            );
        }
    }

    fn test_integer_literal(i_literal: &dyn Expression, value: i64) {
        if let Some(literal) = i_literal.downcast_ref::<IntegerLiteral>() {
            assert_eq!(literal.value, value);
        } else {
            panic!("i_literal is not IntegerLiteral");
        }
    }

    fn test_boolean_literal(exp: &dyn Expression, value: bool) {
        if let Some(literal) = exp.downcast_ref::<Boolean>() {
            assert_eq!(literal.value, value);
        } else {
            panic!("exp is not Boolean");
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

use std::{any::type_name, rc::Rc};

use crate::{
    ast::{ExpressionStatement, IntegerLiteral, Node, Program, Statement},
    lexer::{self, Lexer},
    object::{Integer, Object},
    parser::Parser,
    token::{Token, INT},
};

fn type_of<T>(_: T) -> String {
    let test = unsafe { type_name::<T>() };
    println!("{}", test);

    let a = std::any::type_name::<T>();
    return a.to_string();
}

pub fn eval(node: &dyn Node) -> Box<dyn Object> {
    if let Some(program) = node.downcast_ref::<Program>() {
        return eval_statement(&program.statements);
    }
    if let Some(exp_statement) = node.downcast_ref::<ExpressionStatement>() {
        let exp = exp_statement.expression.as_ref().unwrap().as_ref();
        return eval(exp);
    }
    if let Some(integer_literal) = node.downcast_ref::<IntegerLiteral>() {
        return Box::new(Integer {
            value: integer_literal.value,
        });
    }
    panic!("Unknown node type: {}", type_of(node));
}

fn eval_statement(statements: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    let mut result = None;
    for statement in statements {
        result = Some(eval(statement.as_ref()));
    }
    result.unwrap()
}

#[test]
fn test_type_of() {
    let token = Rc::new(Token {
        token_type: INT,
        literal: "10".to_string(),
    });
    let integer_literal = IntegerLiteral { token, value: 10 };
    assert_eq!(type_of(integer_literal), "monkey::ast::IntegerLiteral");
}

#[test]
fn test_evaluator_integer_expression() {
    let input = vec![("5", 5), ("10", 10)];
    for (input, expected) in input {
        let evaluated = test_eval(input);
        test_integer_object(evaluated.as_ref(), expected);
    }
}

fn test_eval(input: &str) -> Box<dyn Object> {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    let program = program.expect("Program failed to parse");

    eval(&program)
}

fn test_integer_object(obj: &dyn Object, expected: i64) {
    let integer = obj.downcast_ref::<Integer>().unwrap();
    assert_eq!(integer.value, expected);
}

use crate::{
    ast::{
        Boolean as BooleanAst, ExpressionStatement, IntegerLiteral, Node, PrefixExpression,
        Program, Statement,
    },
    object::{Integer, Object, BOOLEAN_OBJ, FALSE, NULL, NULL_OBJ, TRUE},
};

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
    if let Some(exp) = node.downcast_ref::<PrefixExpression>() {
        let right = eval(exp.right.as_ref());
        return eval_prefix_expression(&exp.operator, right.as_ref());
    }
    if let Some(boolean_literal) = node.downcast_ref::<BooleanAst>() {
        return native_bool_to_boolean_object(boolean_literal.value);
    }
    panic!("Unknown node type:");
}

fn eval_statement(statements: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    let mut result = None;
    for statement in statements {
        result = Some(eval(statement.as_ref()));
    }
    result.unwrap()
}

fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
    Box::new(if input { TRUE } else { FALSE })
}

fn eval_prefix_expression(operator: &str, right: &dyn Object) -> Box<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Box::new(NULL),
    }
}

fn eval_bang_operator_expression(right: &dyn Object) -> Box<dyn Object> {
    match right.obj_type().as_str() {
        NULL_OBJ => Box::new(TRUE),
        BOOLEAN_OBJ => native_bool_to_boolean_object(!right.inspect().parse::<bool>().unwrap()),
        _ => Box::new(FALSE),
    }
}

fn eval_minus_prefix_operator_expression(right: &dyn Object) -> Box<dyn Object> {
    match right.downcast_ref::<Integer>() {
        Some(int) => Box::new(Integer { value: -int.value }),
        _ => Box::new(NULL),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, object::Boolean, parser::Parser};

    #[test]
    fn test_eval_integer_expression() {
        let input = vec![("5", 5), ("10", 10), ("-5", -5), ("-10", -10)];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_integer_object(evaluated.as_ref(), expected);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let input = vec![("true", true), ("false", false)];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated.as_ref(), expected);
        }
    }

    #[test]
    fn test_bang_operator() {
        let input = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated.as_ref(), expected);
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

    fn test_boolean_object(obj: &dyn Object, expected: bool) {
        let bool_val = obj.downcast_ref::<Boolean>().unwrap();
        assert_eq!(bool_val.value, expected);
    }
}

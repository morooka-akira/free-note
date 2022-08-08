use crate::{
    ast::{
        BlockStatement, Boolean as BooleanAst, ExpressionStatement, IfExpression, InfixExpression,
        IntegerLiteral, Node, PrefixExpression, Program, Statement,
    },
    object::{Boolean, Integer, Object, BOOLEAN_OBJ, FALSE, INTEGER_OBJ, NULL, NULL_OBJ, TRUE},
};

pub fn eval(node: &dyn Node) -> Box<dyn Object> {
    if let Some(program) = node.downcast_ref::<BlockStatement>() {
        return eval_statement(&program.statements);
    }
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
    if let Some(infix_expression) = node.downcast_ref::<InfixExpression>() {
        return eval_infix_expression(
            infix_expression.operator.as_ref(),
            eval(infix_expression.left.as_ref()).as_ref(),
            eval(infix_expression.right.as_ref()).as_ref(),
        );
    }
    if let Some(if_expression) = node.downcast_ref::<IfExpression>() {
        return eval_if_expression(if_expression);
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

fn eval_infix_expression(operator: &str, left: &dyn Object, right: &dyn Object) -> Box<dyn Object> {
    if left.obj_type() == INTEGER_OBJ && right.obj_type() == INTEGER_OBJ {
        eval_integer_infix_expression(operator, left, right)
    } else if operator == "==" {
        let left_bool = left.downcast_ref::<Boolean>().unwrap().value;
        let right_bool = right.downcast_ref::<Boolean>().unwrap().value;
        native_bool_to_boolean_object(left_bool == right_bool)
    } else if operator == "!=" {
        let left_bool = left.downcast_ref::<Boolean>().unwrap().value;
        let right_bool = right.downcast_ref::<Boolean>().unwrap().value;
        native_bool_to_boolean_object(left_bool != right_bool)
    } else {
        Box::new(NULL)
    }
}

fn eval_integer_infix_expression(
    operator: &str,
    left: &dyn Object,
    right: &dyn Object,
) -> Box<dyn Object> {
    let left_value = left.downcast_ref::<Integer>().unwrap();
    let right_value = right.downcast_ref::<Integer>().unwrap();
    match operator {
        "+" => Box::new(Integer {
            value: left_value.value + right_value.value,
        }),
        "-" => Box::new(Integer {
            value: left_value.value - right_value.value,
        }),
        "*" => Box::new(Integer {
            value: left_value.value * right_value.value,
        }),
        "/" => Box::new(Integer {
            value: left_value.value / right_value.value,
        }),
        "<" => native_bool_to_boolean_object(left_value.value < right_value.value),
        ">" => native_bool_to_boolean_object(left_value.value > right_value.value),
        "==" => native_bool_to_boolean_object(left_value.value == right_value.value),
        "!=" => native_bool_to_boolean_object(left_value.value != right_value.value),
        _ => Box::new(NULL),
    }
}

fn eval_if_expression(if_expression: &IfExpression) -> Box<dyn Object> {
    let condition = eval(if_expression.condition.as_ref());

    if is_truthy(condition.as_ref()) {
        eval(if_expression.consequence.as_ref())
    } else if let Some(block) = &if_expression.alternative {
        eval(block.as_ref())
    } else {
        Box::new(NULL)
    }
}

fn is_truthy(obj: &dyn Object) -> bool {
    if obj.obj_type() == NULL_OBJ {
        return false;
    }
    if obj.obj_type() == BOOLEAN_OBJ {
        return obj.downcast_ref::<Boolean>().unwrap().value;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, object::Boolean, parser::Parser};

    #[test]
    fn test_eval_integer_expression() {
        let input = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];
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
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated.as_ref(), expected);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let int_test_input = vec![
            ("if (true) { 10 }", 10),
            ("if (1) { 10 }", 10),
            ("if (1 < 2) { 10 }", 10),
            ("if (1 > 2) { 10 } else { 20 }", 20),
            ("if (1 < 2) { 10 } else { 20 }", 10),
        ];
        for (input, expected) in int_test_input {
            let evaluated = test_eval(input);
            test_integer_object(evaluated.as_ref(), expected);
        }
        let int_test_input = vec!["if (false) { 10 }", "if (1 > 2) { 10 }"];
        for input in int_test_input {
            let evaluated = test_eval(input);
            test_null_object(evaluated.as_ref());
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

    fn test_null_object(obj: &dyn Object) {
        assert_eq!(obj.inspect(), NULL.inspect());
    }
}

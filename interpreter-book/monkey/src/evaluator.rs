use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        BlockStatement, Boolean as BooleanAst, CallExpression, Expression, ExpressionStatement,
        FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement,
        Node, PrefixExpression, Program, ReturnStatement, StringLiteral,
    },
    object::{
        Boolean, Environment, Error, Function, Integer, Object, ReturnValue, StringObj,
        BOOLEAN_OBJ, ERROR_OBJ, FALSE, INTEGER_OBJ, NULL, NULL_OBJ, RETURN_VALUE_OBJ, TRUE,
    },
};

pub fn eval(node: &dyn Node, env: Rc<RefCell<Environment>>) -> Rc<dyn Object> {
    if let Some(block) = node.downcast_ref::<BlockStatement>() {
        return eval_block_statement(block, env);
    }
    if let Some(program) = node.downcast_ref::<Program>() {
        return eval_program(program, env);
    }
    if let Some(exp_statement) = node.downcast_ref::<ExpressionStatement>() {
        let exp = exp_statement.expression.as_ref().unwrap().as_ref();
        return eval(exp, env);
    }
    if let Some(integer_literal) = node.downcast_ref::<IntegerLiteral>() {
        return Rc::new(Integer {
            value: integer_literal.value,
        });
    }
    if let Some(integer_literal) = node.downcast_ref::<Identifier>() {
        return eval_identifier(integer_literal, env);
    }
    if let Some(exp) = node.downcast_ref::<PrefixExpression>() {
        let right = eval(exp.right.as_ref(), env);
        if is_error(right.as_ref()) {
            return right;
        }
        return eval_prefix_expression(&exp.operator, right.as_ref());
    }
    if let Some(boolean_literal) = node.downcast_ref::<BooleanAst>() {
        return native_bool_to_boolean_object(boolean_literal.value);
    }
    if let Some(infix_expression) = node.downcast_ref::<InfixExpression>() {
        let left = eval(infix_expression.left.as_ref(), Rc::clone(&env));
        if is_error(left.as_ref()) {
            return left;
        }
        let right = eval(infix_expression.right.as_ref(), env);
        if is_error(right.as_ref()) {
            return right;
        }
        return eval_infix_expression(
            infix_expression.operator.as_ref(),
            left.as_ref(),
            right.as_ref(),
        );
    }
    if let Some(return_statement) = node.downcast_ref::<ReturnStatement>() {
        let return_val = return_statement.return_value.as_ref();
        let value = eval(return_val.unwrap().as_ref(), env);
        if is_error(value.as_ref()) {
            return value;
        }
        return Rc::new(ReturnValue { value });
    }
    if let Some(if_expression) = node.downcast_ref::<IfExpression>() {
        return eval_if_expression(if_expression, env);
    }
    if let Some(let_statement) = node.downcast_ref::<LetStatement>() {
        let let_val = let_statement.value.as_ref();
        let value = eval(let_val.unwrap().as_ref(), Rc::clone(&env));
        if is_error(value.as_ref()) {
            return value;
        }
        let mut env = env.borrow_mut();
        env.set(&let_statement.name.value, Rc::clone(&value));
        return value;
    }

    if let Some(func) = node.downcast_ref::<FunctionLiteral>() {
        return Rc::new(Function {
            parameters: func.parameters.iter().map(Rc::clone).collect(),
            body: Rc::clone(&func.body),
            env,
        });
    }

    if let Some(call) = node.downcast_ref::<CallExpression>() {
        let func = eval(call.function.as_ref(), Rc::clone(&env));
        if is_error(func.as_ref()) {
            return func;
        }
        let args = call.arguments.iter().map(Rc::clone).collect();
        let args = eval_expressions(args, env);
        if args.len() == 1 && is_error(args[0].as_ref()) {
            return Rc::clone(&args[0]);
        }
        return apply_function(func, args);
    }

    if let Some(str) = node.downcast_ref::<StringLiteral>() {
        return Rc::new(StringObj {
            value: str.value.clone(),
        });
    }

    new_error(&format!("eval: Unknown node type {}", node.token_literal()))
}

fn apply_function(object: Rc<dyn Object>, args: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
    if let Some(func) = object.as_ref().downcast_ref::<Function>() {
        let extended_env = extend_function_env(func, args);
        let evaluated = eval(func.body.as_ref(), extended_env);
        unwrap_return_value(evaluated)
    } else {
        new_error(&format!("not a function: {}", object.obj_type()))
    }
}

fn extend_function_env(func: &Function, args: Vec<Rc<dyn Object>>) -> Rc<RefCell<Environment>> {
    let mut env = Environment::new_enclosed_environment(Rc::clone(&func.env));
    for (i, param) in func.parameters.iter().enumerate() {
        env.set(&param.value, Rc::clone(&args[i]));
    }
    Rc::new(RefCell::new(env))
}

fn unwrap_return_value(object: Rc<dyn Object>) -> Rc<dyn Object> {
    if let Some(return_val) = object.downcast_ref::<ReturnValue>() {
        Rc::clone(&return_val.value)
    } else {
        object
    }
}

fn eval_expressions(
    exps: Vec<Rc<dyn Expression>>,
    env: Rc<RefCell<Environment>>,
) -> Vec<Rc<dyn Object>> {
    let mut result: Vec<Rc<dyn Object>> = vec![];
    for exp in exps {
        let evaluated = eval(exp.as_ref(), Rc::clone(&env));
        if is_error(evaluated.as_ref()) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    result
}

fn eval_program(program: &Program, env: Rc<RefCell<Environment>>) -> Rc<dyn Object> {
    let mut result = None;
    for statement in &program.statements {
        let e = eval(statement.as_ref(), Rc::clone(&env));
        if is_error(e.as_ref()) {
            return e;
        }
        if let Some(return_value) = e.downcast_ref::<ReturnValue>() {
            return return_value.value.clone();
        }
        result = Some(e);
    }
    result.unwrap()
}

fn eval_block_statement(block: &BlockStatement, env: Rc<RefCell<Environment>>) -> Rc<dyn Object> {
    let mut result = None;
    for statement in &block.statements {
        let e = eval(statement.as_ref(), Rc::clone(&env));
        if is_error(e.as_ref()) {
            return e;
        }
        if e.as_ref().obj_type() == RETURN_VALUE_OBJ {
            return e;
        }
        result = Some(e);
    }
    result.unwrap()
}

fn native_bool_to_boolean_object(input: bool) -> Rc<dyn Object> {
    Rc::new(if input { TRUE } else { FALSE })
}

fn eval_prefix_expression(operator: &str, right: &dyn Object) -> Rc<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Rc::new(NULL),
    }
}

fn eval_bang_operator_expression(right: &dyn Object) -> Rc<dyn Object> {
    match right.obj_type().as_str() {
        NULL_OBJ => Rc::new(TRUE),
        BOOLEAN_OBJ => native_bool_to_boolean_object(!right.inspect().parse::<bool>().unwrap()),
        _ => Rc::new(FALSE),
    }
}

fn eval_minus_prefix_operator_expression(right: &dyn Object) -> Rc<dyn Object> {
    match right.downcast_ref::<Integer>() {
        Some(int) => Rc::new(Integer { value: -int.value }),
        _ => new_error(&format!("unknown operator: -{}", right.obj_type())),
    }
}

fn eval_infix_expression(operator: &str, left: &dyn Object, right: &dyn Object) -> Rc<dyn Object> {
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
    } else if left.obj_type() != right.obj_type() {
        new_error(&format!(
            "type mismatch: {} + {}",
            left.obj_type(),
            right.obj_type()
        ))
    } else {
        new_error(&format!(
            "unknown operator: {} + {}",
            left.obj_type(),
            right.obj_type()
        ))
    }
}

fn eval_integer_infix_expression(
    operator: &str,
    left: &dyn Object,
    right: &dyn Object,
) -> Rc<dyn Object> {
    let left_value = left.downcast_ref::<Integer>().unwrap();
    let right_value = right.downcast_ref::<Integer>().unwrap();
    match operator {
        "+" => Rc::new(Integer {
            value: left_value.value + right_value.value,
        }),
        "-" => Rc::new(Integer {
            value: left_value.value - right_value.value,
        }),
        "*" => Rc::new(Integer {
            value: left_value.value * right_value.value,
        }),
        "/" => Rc::new(Integer {
            value: left_value.value / right_value.value,
        }),
        "<" => native_bool_to_boolean_object(left_value.value < right_value.value),
        ">" => native_bool_to_boolean_object(left_value.value > right_value.value),
        "==" => native_bool_to_boolean_object(left_value.value == right_value.value),
        "!=" => native_bool_to_boolean_object(left_value.value != right_value.value),
        _ => new_error("unknown operator"),
    }
}

fn eval_if_expression(
    if_expression: &IfExpression,
    env: Rc<RefCell<Environment>>,
) -> Rc<dyn Object> {
    let condition = eval(if_expression.condition.as_ref(), Rc::clone(&env));
    if is_error(condition.as_ref()) {
        return condition;
    }
    if is_truthy(condition.as_ref()) {
        eval(if_expression.consequence.as_ref(), env)
    } else if let Some(block) = &if_expression.alternative {
        eval(block.as_ref(), env)
    } else {
        Rc::new(NULL)
    }
}

fn eval_identifier(identifier: &Identifier, env: Rc<RefCell<Environment>>) -> Rc<dyn Object> {
    let env = env.borrow();
    if let Some(val) = env.get(&identifier.value) {
        val
    } else {
        Rc::new(Error {
            message: format!("identifier not found: {}", identifier.value),
        })
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

fn new_error(message: &str) -> Rc<dyn Object> {
    Rc::new(Error {
        message: String::from(message),
    })
}

fn is_error(obj: &dyn Object) -> bool {
    obj.obj_type() == ERROR_OBJ
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexer::Lexer,
        object::{Boolean, Error, Function, StringObj},
        parser::Parser,
    };

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

    #[test]
    fn test_return_statements() {
        let input = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                r#"
            if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }

                return 1;
            }
            "#,
                10,
            ),
        ];

        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_integer_object(evaluated.as_ref(), expected);
        }
    }

    #[test]
    fn test_error_handling() {
        let input = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                r#"
                if (10 > 1) {
                  if (10 > 1) {
                    return true + false;
                  }
                return 1;
                }
                "#,
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
        ];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            if let Some(error) = evaluated.downcast_ref::<Error>() {
                assert_eq!(error.message, expected);
            } else {
                panic!(
                    "no error object returned {}: {}",
                    evaluated.obj_type(),
                    evaluated.inspect()
                );
            }
        }
    }

    #[test]
    fn test_let_statement() {
        let input = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_integer_object(evaluated.as_ref(), expected);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";

        let evaluated = test_eval(input);
        if let Some(func) = evaluated.downcast_ref::<Function>() {
            if func.parameters.len() != 1 {
                panic!(
                    "function has wrong parameters. Parameters={:?}",
                    func.parameters,
                );
            }
            if "x" != func.parameters.first().unwrap().string() {
                panic!("parameters is not 'x'. got={:?}", func.parameters.first(),);
            }
            assert_eq!(func.body.string(), "(x + 2)");
        } else {
            panic!(
                "object is not Function. got = {} ( {})",
                evaluated.obj_type(),
                evaluated.inspect()
            );
        }
    }

    #[test]
    fn test_function_application() {
        let input = vec![
            ("let identity = fn (x) { x; }; identity(5);", 5),
            ("let identity = fn (x) { return x; }; identity(5);", 5),
            ("let double = fn (x) { x * 2; }; double(5);", 10),
            ("let add = fn (x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn (x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn (x) { x; }(5);", 5),
        ];
        for (input, expected) in input {
            let evaluated = test_eval(input);
            test_integer_object(evaluated.as_ref(), expected);
        }
    }

    #[test]
    fn test_closures() {
        let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };

        let addTwo = newAdder(2);
        addTwo(2);
        "#;
        let evaluated = test_eval(input);
        test_integer_object(evaluated.as_ref(), 4);
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World!\"";
        let evaluated = test_eval(input);

        if let Some(str) = evaluated.downcast_ref::<StringObj>() {
            assert_eq!(str.value, "Hello World!");
        } else {
            panic!("object is not String. got={}", evaluated.inspect());
        }
    }

    fn test_eval(input: &str) -> Rc<dyn Object> {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        let program = program.expect("Program failed to parse");
        let env = Rc::new(RefCell::new(Environment::default()));
        eval(&program, env)
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

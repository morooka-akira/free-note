use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use crate::{evaluator, lexer::Lexer, object::Environment, parser::Parser};

const MONKEY_FACE: &str = r#"
            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-""""""""-./, -' /
   `'-' /_   ^ ^   _\ '-'`
       |  \._   _./  |
        \   \ '~' /   /
        | \_  `-`  _/ |
        \.' `-._,-' .'/
        / `-._,-'_,-' \
"#;

pub fn start() {
    let env = Rc::new(RefCell::new(Environment::default()));
    loop {
        let mut buf = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");

        let mut l = Lexer::new(buf.as_str());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        if !p.errors.is_empty() {
            print_parser_errors(&p.errors);
            continue;
        }
        if let Ok(program) = program {
            let evaluated = evaluator::eval(&program, Rc::clone(&env));
            println!("{}", evaluated.inspect());
        } else {
            println!("Program is not valid.");
        }
    }
}

fn print_parser_errors(errors: &Vec<String>) {
    println!("{}", MONKEY_FACE);
    println!("Woops! We ran int some monkey business here!");
    println!(" parser errors");

    for error in errors {
        println!("\t{}", error);
    }
}

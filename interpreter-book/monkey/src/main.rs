#![feature(trait_upcasting)]
use users::{get_current_uid, get_user_by_uid};
mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

fn main() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!(
        "Hello, {}! This is the Monkey programming language!",
        user.name().to_string_lossy()
    );
    println!("Feel free to Type in commands",);
    repl::start();
}

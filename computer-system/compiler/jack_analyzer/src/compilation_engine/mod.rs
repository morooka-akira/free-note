use crate::jack_tokenizer::{Keyword, Token, TokenType, Tokenizer};

pub fn compile(tokenizer: &mut Tokenizer) {
    compile_class(tokenizer)
}

fn compile_class(tokenizer: &mut Tokenizer) {
    if !tokenizer.has_more_tokens() {
        panic!("class not found")
    }
    let token = tokenizer.current().unwrap();
    if token.keyword() != Keyword::Class {
        panic!("class not found")
    }
    println!("<class>");
    // class name
    if tokenizer.advance().is_some() {
        println!("{}", get_xml(tokenizer.current().unwrap()));
    }
    // symbol {
    if tokenizer.advance().is_some() {
        println!("{}", get_xml(tokenizer.current().unwrap()));
    }
    tokenizer.advance();
    while tokenizer.has_more_tokens() {
        let token = tokenizer.current().unwrap();
        if TokenType::Keyword != token.token_type {
            panic!("keyword not found")
        }
        match token.keyword() {
            Keyword::Field => println!("it is Field"),
            Keyword::Method => println!("it is Method"),
            Keyword::Static => println!("it is Static"),
            Keyword::Function => println!("it is Static"),
            _ => println!("unknown Keyword"),
        }
        tokenizer.advance();
    }
    println!("</class>");
}

fn compile_subroutine(tokenizer: &mut Tokenizer) {
    println!("<subroutineDec>");
    println!("</subroutineDec>");
}

fn get_xml(token: &Token) -> String {
    match token.token_type {
        TokenType::Keyword => format!("<keyword> {} </keyword>", token.raw),
        TokenType::Identifier => format!("<identifier> {} </identifier>", token.raw),
        TokenType::Symbol => format!("<symbol> {} </symbol>", token.raw),
        TokenType::IntConst => format!("<integerConstant> {} </integerConstant>", token.raw),
        TokenType::StringConst => format!("<stringConstant> {} </stringConstant>", token.raw),
        _ => "unknown xml".to_string(),
    }
}

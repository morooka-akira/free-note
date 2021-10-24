use crate::jack_tokenizer::{Token, TokenType};

pub fn compile(tokens: Vec<Token>) {
    println!("Compiling {:?}", tokens);
    for token in tokens {
        print_xml(&token)
    }
}

fn print_xml(token: &Token) {
    let xml = match token.token_type {
        TokenType::Keyword => format!("<keyword> {} </keyword>", token.raw),
        TokenType::Identifier => format!("<identifier> {} </identifier>", token.raw),
        TokenType::Symbol => format!("<symbol> {} </symbol>", token.raw),
        TokenType::IntConst => format!("<integerConstant> {} </integerConstant>", token.raw),
        TokenType::StringConst => format!("<stringConstant> {} </stringConstant>", token.raw),
        _ => "no".to_string(),
    };
    println!("xml {:?}", xml);
}

use std::io::{self, Write};

use crate::{lexer::Lexer, token::EOF};

pub fn start() {
    loop {
        let mut buf = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");

        let mut l = Lexer::new(buf.as_str());

        loop {
            let tok = l.next_token();
            match tok.token_type {
                EOF => break,
                _ => println!("{:?}", tok),
            }
        }
    }
}

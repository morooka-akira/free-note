use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Token {
    pub raw: String,
}

impl Token {
    pub fn new(raw: String) -> Token {
        Token { raw: raw }
    }
}

pub fn tokenize(file: &File) {
    println!("start analyze");
    let mut tokens: Vec<Token> = vec![];
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let line = trim_comment(&l);
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut a: Vec<char> = vec![];
            for c in line.chars() {
                if c == ' ' {
                    tokenize_id(&mut a);
                    continue;
                }
                let c_str = c.to_string();
                if is_symbol(&c_str) {
                    tokens.push(Token::new(c_str));
                    tokenize_id(&mut a);
                    continue;
                }
                a.push(c);
                let str: String = a.iter().collect();
                if is_keyword(&str) {
                    tokens.push(Token::new(str));
                    a.clear();
                    continue;
                }
                //println!("c: {:?}", a);
            }
            // let tokens = line.trim().split(' ');
            // for t in tokens {
            // 	let n = t.split(';');
            // 	// println!("{:?}", n);
            // 	for t in n {
            // 		println!("{:?}", t);
            // 	}
            // }
        }
    }
}

fn tokenize_id(chars: &mut Vec<char>) {
    if chars.is_empty() {
        return;
    }
    let is_num = chars.iter().all(|c| c >= &'0' && c <= &'9');
    println!("num: {:?}, is_num: {}", chars, is_num);
    chars.clear();
}

pub fn trim_comment(code: &str) -> String {
    let trim = Regex::new(r"//.+").unwrap().replace_all(code, "");
    let trim = Regex::new(r"/\*.*\*/").unwrap().replace_all(&trim, "");
    let trim = Regex::new(r"/\*.*").unwrap().replace_all(&trim, "");
    let trim = Regex::new(r".*\*/").unwrap().replace_all(&trim, "");
    return trim.to_string();
}

// fn is_alpha();
// fn is_alnum();
// fn is_digit();

fn is_keyword(token: &str) -> bool {
    [
        "class",
        "constructor",
        "function",
        "method",
        "field",
        "static",
        "var",
        "int",
        "char",
        "boolean",
        "void",
        "true",
        "false",
        "null",
        "this",
        "let",
        "do",
        "if",
        "else",
        "while",
        "return",
    ]
    .contains(&token)
}

fn is_symbol(token: &str) -> bool {
    [
        "{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<", ">", "=",
        "~",
    ]
    .contains(&token)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod trim_comment {
        use super::*;
        #[test]
        fn test_not_include() {
            let query = "  var int i;";
            let res = trim_comment(&query);
            assert_eq!(res, query)
        }

        #[test]
        fn test_slash() {
            let query = "  var int i; // comment";
            let res = trim_comment(&query);
            assert_eq!(res, "  var int i; ")
        }

        #[test]
        fn test_asterisk_full() {
            let query = "  var int i; /* comment */";
            let res = trim_comment(&query);
            assert_eq!(res, "  var int i; ")
        }

        #[test]
        fn test_asterisk_front() {
            let query = "  var int i; /* comment";
            let res = trim_comment(&query);
            assert_eq!(res, "  var int i; ")
        }

        #[test]
        fn test_asterisk_back() {
            let query = " comment*/ var int i;";
            let res = trim_comment(&query);
            assert_eq!(res, " var int i;")
        }
    }
}

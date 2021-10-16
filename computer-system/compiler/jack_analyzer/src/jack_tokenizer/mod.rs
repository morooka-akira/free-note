use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str;

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

pub fn tokenize3(file: &File) {
    println!("start analyze");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let line = trim_comment(&l);
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let line_bytes = line.as_bytes();
            let mut i: usize = 0;
            while i < line_bytes.len() {
                // 文字の判定
                if line_bytes[i] == b'"' {
                    // ”の次文字から見るため + 1
                    let mut p = i + 1;
                    if p >= line_bytes.len() {
                        return;
                    }
                    while p < line_bytes.len() && line_bytes[p] != b'"' {
                        p += 1;
                    }
                    let string_const = str::from_utf8(&line_bytes[(i + 1)..p]).unwrap();
                    println!("string_const: {}", string_const);
                    // ”の次文字から見るため + 1
                    i = p + 1;
                    continue;
                }
                // 文字の判定
                if line_bytes[i].is_ascii_alphanumeric() {
                    let mut p = i;
                    while line_bytes[p].is_ascii_alphanumeric() {
                        p += 1;
                    }
                    // ここからキーワードを除外する
                    let str = str::from_utf8(&line_bytes[i..p]).unwrap();
                    println!("{}", str);
                    i = p;
                    continue;
                }
                i += 1;
            }
        }
    }
}

pub fn tokenize2(file: &File) {
    println!("start analyze");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("read error");
    println!("num: {:?}", contents);
    let mut i: usize = 0;
    let mut buf = contents.as_bytes();
    while i < buf.len() {
        if buf[i] == b'/' && !(i + 1 >= buf.len()) && buf[i + 1] == b'/' {
            println!("break line {}", buf[i]);
        }
        i += 1;
    }
}

fn tokenize_id(chars: &mut Vec<char>) {
    if chars.is_empty() {
        return;
    }
    let is_num = chars.iter().all(|c| c >= &'0' && c <= &'9');
    let is_alpha = chars.iter().all(|c| c >= &'0' && c <= &'9');
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

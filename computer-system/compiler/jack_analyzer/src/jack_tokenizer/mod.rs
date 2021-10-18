use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::str;

#[derive(Debug, Clone)]
struct Token {
    pub raw: String,
}

impl Token {
    pub fn new(raw: String) -> Token {
        Token { raw: raw }
    }
}

pub fn tokenize3(file: &File) {
    println!("start analyze");
    let mut tokens: Vec<Token> = vec![];
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
                // 文字列
                if line_bytes[i] == b'"' {
                    let token = obtain_string_const(&line_bytes, i);
                    println!("{:?}", token);
                    // ダブルクォーテーションの数は除外して加算
                    i += token.raw.len() + 2;
                    continue;
                }
                // 文字の判定
                if line_bytes[i].is_ascii_alphanumeric() {
                    let mut p = i;
                    while line_bytes[p].is_ascii_alphanumeric() {
                        p += 1;
                    }
                    let str = str::from_utf8(&line_bytes[i..p]).unwrap();
                    if is_num(&str) {
                        let token = Token::new(str.to_string());
                        println!("num: {:?}", token);
                        i += token.raw.len();
                        continue;
                    }
                    if is_keyword(str) {
                        let token = Token::new(str.to_string());
                        println!("keyword: {:?}", token);
                        i += token.raw.len();
                        continue;
                    }
                    let token = Token::new(str.to_string());
                    println!("identifier: {:?}", token);
                    i = p;
                    continue;
                }
                if is_symbol(&line_bytes[i]) {
                    let token = obtain_symbol(&line_bytes, i);
                    i += token.raw.len();
                    println!("symbol: {:?}", token);
                    continue;
                }
                i += 1;
            }
        }
    }
}

fn obtain_string_const(bytes: &[u8], index: usize) -> Token {
    let start = index + 1;
    let mut p = start;
    if p >= bytes.len() {
        panic!("end quotation not found");
    }
    while p < bytes.len() && bytes[p] != b'"' {
        p += 1;
    }
    let str = str::from_utf8(&bytes[start..p]).unwrap();
    Token::new(str.to_string())
}

fn obtain_symbol(bytes: &[u8], index: usize) -> Token {
    let str = str::from_utf8(&bytes[index..index + 1]).unwrap();
    Token::new(str.to_string())
}

pub fn trim_comment(code: &str) -> String {
    let trim = Regex::new(r"//.+").unwrap().replace_all(code, "");
    let trim = Regex::new(r"/\*.*\*/").unwrap().replace_all(&trim, "");
    let trim = Regex::new(r"/\*.*").unwrap().replace_all(&trim, "");
    let trim = Regex::new(r".*\*/").unwrap().replace_all(&trim, "");
    return trim.to_string();
}

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

fn is_num(str: &str) -> bool {
    let r: Result<i32, ParseIntError> = str.parse();
    r.is_ok()
}

fn is_symbol(byte: &u8) -> bool {
    [
        b'{', b'}', b'(', b')', b'[', b']', b'.', b',', b';', b'+', b'-', b'*', b'/', b'&', b'|',
        b'<', b'>', b'=', b'~',
    ]
    .contains(byte)
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

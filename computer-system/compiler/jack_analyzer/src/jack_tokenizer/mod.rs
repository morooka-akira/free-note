use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub raw: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(raw: String, token_type: TokenType) -> Token {
        Token {
            raw: raw,
            token_type,
        }
    }

    pub fn keyword(&self) -> Keyword {
        match self.raw.as_str() {
            "class" => Keyword::Class,
            "method" => Keyword::Method,
            "class" => Keyword::Class,
            "method" => Keyword::Method,
            "function" => Keyword::Function,
            "constructor" => Keyword::Constructor,
            "int" => Keyword::Int,
            "boolean" => Keyword::Boolean,
            "char" => Keyword::Char,
            "void" => Keyword::Void,
            "var" => Keyword::Var,
            "static" => Keyword::Static,
            "field" => Keyword::Field,
            "let" => Keyword::Let,
            "do" => Keyword::Do,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "return" => Keyword::Return,
            "true" => Keyword::True,
            "false" => Keyword::False,
            "null" => Keyword::Null,
            "this" => Keyword::This,
            _ => Keyword::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
    index: usize,
}

impl Tokenizer {
    pub fn new(tokens: Vec<Token>) -> Tokenizer {
        Tokenizer {
            tokens: tokens,
            index: 0,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.tokens.len() > self.index
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index)
    }

    pub fn current(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}

pub fn tokenize(file: &File) -> Tokenizer {
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
                    // ダブルクォーテーションの数は除外して加算
                    i += token.raw.len() + 2;
                    tokens.push(token);
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
                        let token = Token::new(str.to_string(), TokenType::IntConst);
                        i += token.raw.len();
                        tokens.push(token);
                        continue;
                    }
                    if is_keyword(str) {
                        let token = Token::new(str.to_string(), TokenType::Keyword);
                        i += token.raw.len();
                        tokens.push(token);
                        continue;
                    }
                    let token = Token::new(str.to_string(), TokenType::Identifier);
                    i = p;
                    tokens.push(token);
                    continue;
                }
                if is_symbol(&line_bytes[i]) {
                    let token = obtain_symbol(&line_bytes, i);
                    i += token.raw.len();
                    tokens.push(token);
                    continue;
                }
                i += 1;
            }
        }
    }
    return Tokenizer::new(tokens);
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
    Token::new(str.to_string(), TokenType::StringConst)
}

fn obtain_symbol(bytes: &[u8], index: usize) -> Token {
    let str = str::from_utf8(&bytes[index..index + 1]).unwrap();
    Token::new(str.to_string(), TokenType::Symbol)
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

    mod token {
        use super::*;

        #[test]
        fn test_keyword() {
            let t = Token::new("class".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Class);
            let t = Token::new("method".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Method);
            let t = Token::new("function".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Function);
            let t = Token::new("constructor".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Constructor);
            let t = Token::new("int".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Int);
            let t = Token::new("boolean".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Boolean);
            let t = Token::new("char".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Char);
            let t = Token::new("void".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Void);
            let t = Token::new("var".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Var);
            let t = Token::new("static".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Static);
            let t = Token::new("field".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Field);
            let t = Token::new("let".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Let);
            let t = Token::new("do".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Do);
            let t = Token::new("if".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::If);
            let t = Token::new("else".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Else);
            let t = Token::new("while".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::While);
            let t = Token::new("return".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Return);
            let t = Token::new("true".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::True);
            let t = Token::new("false".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::False);
            let t = Token::new("null".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::Null);
            let t = Token::new("this".to_string(), TokenType::Keyword);
            assert_eq!(t.keyword(), Keyword::This);
        }
    }

    mod tokenizer {
        use super::*;

        mod has_more_tokens {
            use super::*;

            #[test]
            fn return_true_if_more_than_one() {
                let tokens = vec![Token::new("do".to_string(), TokenType::Keyword)];
                let target = Tokenizer::new(tokens);
                assert_eq!(target.has_more_tokens(), true);
            }

            #[test]
            fn return_false_if_empty() {
                assert_eq!(Tokenizer::new(vec![]).has_more_tokens(), false);
            }
        }

        mod current {
            use super::*;

            #[test]
            fn return_first_token() {
                let tokens = vec![Token::new("do".to_string(), TokenType::Keyword)];
                let mut target = Tokenizer::new(tokens);
                assert_eq!(target.current().is_some(), true);
                assert_eq!(target.current().unwrap().raw, "do");
            }

            #[test]
            fn return_none() {
                let tokens = vec![];
                let mut target = Tokenizer::new(tokens);
                assert_eq!(target.current().is_none(), true);
            }
        }

        mod advance {
            use super::*;

            #[test]
            fn return_next_token() {
                let tokens = vec![
                    Token::new("do".to_string(), TokenType::Keyword),
                    Token::new("var".to_string(), TokenType::Keyword),
                ];
                let mut target = Tokenizer::new(tokens);
                assert_eq!(target.advance().is_some(), true);
                assert_eq!(target.current().unwrap().raw, "var");
            }

            #[test]
            fn return_none() {
                let tokens = vec![Token::new("do".to_string(), TokenType::Keyword)];
                let mut target = Tokenizer::new(tokens);
                assert_eq!(target.advance().is_none(), true);
            }
        }
    }
}

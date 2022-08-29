use crate::token::{
    lookup_ident, Token, TokenType, ASSIGN, ASTERISK, BANG, COMMA, EOF, EQ, GT, ILLEGAL, INT,
    LBRACE, LPAREN, LT, MINUS, NOT_EQ, PLUS, RBRACE, RPAREN, SEMICOLON, SLASH,
};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token {
                        token_type: EQ,
                        literal: ch.to_string() + &self.ch.to_string(),
                    }
                } else {
                    new_token(ASSIGN, self.ch)
                }
            }
            ';' => new_token(SEMICOLON, self.ch),
            '(' => new_token(LPAREN, self.ch),
            ')' => new_token(RPAREN, self.ch),
            ',' => new_token(COMMA, self.ch),
            '+' => new_token(PLUS, self.ch),
            '-' => new_token(MINUS, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token {
                        token_type: NOT_EQ,
                        literal: ch.to_string() + &self.ch.to_string(),
                    }
                } else {
                    new_token(BANG, self.ch)
                }
            }
            '/' => new_token(SLASH, self.ch),
            '*' => new_token(ASTERISK, self.ch),
            '<' => new_token(LT, self.ch),
            '>' => new_token(GT, self.ch),
            '{' => new_token(LBRACE, self.ch),
            '}' => new_token(RBRACE, self.ch),
            '\0' => new_token(EOF, '\0'),
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return Token {
                        token_type: lookup_ident(&ident),
                        literal: ident,
                    };
                } else if is_digit(self.ch) {
                    let ident = self.read_number();
                    return Token {
                        token_type: INT,
                        literal: ident,
                    };
                } else {
                    println!("Unexpected character: {}", self.ch);
                    return new_token(ILLEGAL, self.ch);
                }
            }
        };
        self.read_char();
        token
    }
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use crate::token::{ELSE, FALSE, FUNCTION, IDENT, IF, LET, RETURN, TRUE};

    use super::*;
    struct NextTokenTest {
        expected_type: TokenType,
        expected_literal: &'static str,
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = [
            NextTokenTest {
                expected_type: ASSIGN,
                expected_literal: "=",
            },
            NextTokenTest {
                expected_type: PLUS,
                expected_literal: "+",
            },
        ];

        let mut l = Lexer::new(input);

        for t in tests.iter() {
            let tok = l.next_token();
            assert_eq!(t.expected_type, tok.token_type);
            assert_eq!(t.expected_literal, tok.literal);
        }
    }

    #[test]
    fn test_next_token2() {
        let input = r#"
            let five = 5;
            let ten = 10;
        
            let add = fn(x, y) {
              x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;
        let tests = [
            NextTokenTest {
                expected_type: LET,
                expected_literal: "let",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "five",
            },
            NextTokenTest {
                expected_type: ASSIGN,
                expected_literal: "=",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "5",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: LET,
                expected_literal: "let",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "ten",
            },
            NextTokenTest {
                expected_type: ASSIGN,
                expected_literal: "=",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "10",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: LET,
                expected_literal: "let",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "add",
            },
            NextTokenTest {
                expected_type: ASSIGN,
                expected_literal: "=",
            },
            NextTokenTest {
                expected_type: FUNCTION,
                expected_literal: "fn",
            },
            NextTokenTest {
                expected_type: LPAREN,
                expected_literal: "(",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "x",
            },
            NextTokenTest {
                expected_type: COMMA,
                expected_literal: ",",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "y",
            },
            NextTokenTest {
                expected_type: RPAREN,
                expected_literal: ")",
            },
            NextTokenTest {
                expected_type: LBRACE,
                expected_literal: "{",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "x",
            },
            NextTokenTest {
                expected_type: PLUS,
                expected_literal: "+",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "y",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: RBRACE,
                expected_literal: "}",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: LET,
                expected_literal: "let",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "result",
            },
            NextTokenTest {
                expected_type: ASSIGN,
                expected_literal: "=",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "add",
            },
            NextTokenTest {
                expected_type: LPAREN,
                expected_literal: "(",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "five",
            },
            NextTokenTest {
                expected_type: COMMA,
                expected_literal: ",",
            },
            NextTokenTest {
                expected_type: IDENT,
                expected_literal: "ten",
            },
            NextTokenTest {
                expected_type: RPAREN,
                expected_literal: ")",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: BANG,
                expected_literal: "!",
            },
            NextTokenTest {
                expected_type: MINUS,
                expected_literal: "-",
            },
            NextTokenTest {
                expected_type: SLASH,
                expected_literal: "/",
            },
            NextTokenTest {
                expected_type: ASTERISK,
                expected_literal: "*",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "5",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "5",
            },
            NextTokenTest {
                expected_type: LT,
                expected_literal: "<",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "10",
            },
            NextTokenTest {
                expected_type: GT,
                expected_literal: ">",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "5",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: IF,
                expected_literal: "if",
            },
            NextTokenTest {
                expected_type: LPAREN,
                expected_literal: "(",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "5",
            },
            NextTokenTest {
                expected_type: LT,
                expected_literal: "<",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "10",
            },
            NextTokenTest {
                expected_type: RPAREN,
                expected_literal: ")",
            },
            NextTokenTest {
                expected_type: LBRACE,
                expected_literal: "{",
            },
            NextTokenTest {
                expected_type: RETURN,
                expected_literal: "return",
            },
            NextTokenTest {
                expected_type: TRUE,
                expected_literal: "true",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: RBRACE,
                expected_literal: "}",
            },
            NextTokenTest {
                expected_type: ELSE,
                expected_literal: "else",
            },
            NextTokenTest {
                expected_type: LBRACE,
                expected_literal: "{",
            },
            NextTokenTest {
                expected_type: RETURN,
                expected_literal: "return",
            },
            NextTokenTest {
                expected_type: FALSE,
                expected_literal: "false",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: RBRACE,
                expected_literal: "}",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "10",
            },
            NextTokenTest {
                expected_type: EQ,
                expected_literal: "==",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "10",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "10",
            },
            NextTokenTest {
                expected_type: NOT_EQ,
                expected_literal: "!=",
            },
            NextTokenTest {
                expected_type: INT,
                expected_literal: "9",
            },
            NextTokenTest {
                expected_type: SEMICOLON,
                expected_literal: ";",
            },
            NextTokenTest {
                expected_type: EOF,
                expected_literal: "\0",
            },
        ];

        let mut l = Lexer::new(input);

        for t in tests.iter() {
            let tok = l.next_token();
            assert_eq!(t.expected_type, tok.token_type);
            assert_eq!(t.expected_literal, tok.literal);
        }
    }

    #[test]
    fn test_is_letter_ok() {
        let input = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ_";
        for c in input.chars() {
            is_letter(c);
            assert!(is_letter(c));
        }
    }

    #[test]
    fn test_is_letter_ng() {
        let input = "1234567890!@#$%^&*()-+={}[]|\\:;\"'<>,.?/";
        for c in input.chars() {
            is_letter(c);
            assert!(!is_letter(c));
        }
    }
}

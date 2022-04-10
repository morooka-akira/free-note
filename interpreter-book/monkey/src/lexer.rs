use crate::token::{
    Token, TokenType, ASSIGN, COMMA, EOF, FUNCTION, IDENT, ILLEGAL, INT, LBRACE, LET, LPAREN, PLUS,
    RBRACE, RPAREN, SEMICOLON,
};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
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

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => new_token(ASSIGN, self.ch),
            'l' => new_token(SEMICOLON, self.ch),
            '(' => new_token(LPAREN, self.ch),
            ')' => new_token(RPAREN, self.ch),
            ',' => new_token(COMMA, self.ch),
            '+' => new_token(PLUS, self.ch),
            '{' => new_token(LBRACE, self.ch),
            '}' => new_token(RBRACE, self.ch),
            '\0' => new_token(EOF, '\0'),
            _ => {
                if is_letter(self.ch) {
                    Token {
                        token_type: IDENT,
                        literal: self.read_identifier(),
                    }
                } else {
                    return new_token(ILLEGAL, self.ch);
                }
            }
        };
        self.read_char();
        token
    }
}

fn new_token(tokenType: TokenType, ch: char) -> Token {
    Token {
        token_type: tokenType,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {

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

        let mut l = Lexer::new(input.to_string());

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
                expected_type: EOF,
                expected_literal: "",
            },
        ];

        let mut l = Lexer::new(input.to_string());

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

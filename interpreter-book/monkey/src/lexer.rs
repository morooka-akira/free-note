use crate::token::{Token, TokenType, ASSIGN, PLUS};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        }
    }

    pub fn next_token(&self) -> Token {
        todo!()
    }
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

        let l = Lexer::new(input.to_string());

        for t in tests.iter() {
            let tok = l.next_token();
            assert_eq!(t.expected_type, tok.token_type);
            assert_eq!(t.expected_literal, tok.literal);
        }
    }
}

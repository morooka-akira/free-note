use crate::jack_tokenizer::{Keyword, Token, TokenType, Tokenizer};

struct CompileEngine<'a> {
    tokenizer: &'a mut Tokenizer,
    output: &'a mut Vec<String>,
}

impl<'a> CompileEngine<'a> {
    fn new(tokenizer: &'a mut Tokenizer, output: &'a mut Vec<String>) -> CompileEngine<'a> {
        CompileEngine {
            tokenizer: tokenizer,
            output: output,
        }
    }

    // letStatement | ifStatement | whileStatement | doStatement | returnStatement
    fn compile_statements(&mut self) {
        self.output.push("<statements>".to_string());
        loop {
            if self.tokenizer.current().unwrap().raw == "}" {
                break;
            }
            match self.tokenizer.current() {
                Some(token) => match token.token_type {
                    TokenType::Keyword => match token.keyword() {
                        Keyword::Let => self.compile_let(),
                        Keyword::If => self.compile_if(),
                        Keyword::While => self.compile_while(),
                        Keyword::Do => self.compile_do(),
                        Keyword::Return => self.compile_return(),
                        _ => panic!("unexpected keyword: {:?}", token),
                    },
                    _ => panic!("unexpected token: {:?}", token),
                },
                None => {}
            }
        }
        self.output.push("</statements>".to_string());
    }

    // 'while' '(' expression ')' '{' statements '}'
    fn compile_while(&mut self) {
        self.output.push("<whileStatement>".to_string());
        // while
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // (
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // expression
        self.compile_expression();
        // )
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // {
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // statements
        self.compile_statements();
        // }
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        self.output.push("</whileStatement>".to_string());
    }

    // 'let' varName ('[' expression ']')? '=' expression ';'
    fn compile_let(&mut self) {
        self.output.push("<letStatement>".to_string());
        // let
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // varName
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        match self.tokenizer.current() {
            Some(token) => {
                if token.raw == "[" {
                    // '[' expression ']'
                    // [
                    self.output.push(get_xml(token));
                    self.tokenizer.advance();
                    self.compile_expression();
                    // ]
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                }
            }
            None => {}
        }
        // =
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        self.compile_expression();
        // :
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        self.output.push("</letStatement>".to_string());
    }

    fn compile_if(&mut self) {
        self.output.push("<ifStatement>".to_string());
        // if
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // (
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // expression
        self.compile_expression();
        // )
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // {
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // statements
        self.compile_statements();
        // }
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();

        match self.tokenizer.current() {
            Some(token) => {
                if token.raw == "else" {
                    // else '{' statements '}'
                    // else
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                    // {
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                    // statements
                    self.compile_statements();
                    // }
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                }
            }
            None => {}
        }

        self.output.push("</ifStatement>".to_string());
    }

    // 'do' subroutineCall ';'
    fn compile_do(&mut self) {
        self.output.push("<doStatement>".to_string());
        // do
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        self.compile_subroutine_call();
        // ;
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        self.output.push("</doStatement>".to_string());
    }

    // return expression? ;
    fn compile_return(&mut self) {
        self.output.push("<returnStatement>".to_string());
        // return
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        // expression?
        match self.tokenizer.current() {
            Some(token) => {
                if token.raw != ";" {
                    self.compile_expression();
                }
            }
            None => {}
        }
        // ;
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        self.output.push("</returnStatement>".to_string());
    }

    // (expression (',' expression))
    fn compile_expression_list(&mut self) {
        self.output.push("<expressionList>".to_string());
        self.compile_expression();
        loop {
            match self.tokenizer.current() {
                Some(token) => {
                    // ,がついている場合は再帰で自分を呼ぶ
                    if token.raw == "," {
                        self.output.push(get_xml(token));
                        self.tokenizer.advance();
                        self.compile_expression();
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }
        self.output.push("</expressionList>".to_string());
    }

    // term (op term) *
    fn compile_expression(&mut self) {
        self.output.push("<expression>".to_string());
        self.compile_term();
        loop {
            if let Some(token) = self.tokenizer.current() {
                if token.is_operator() {
                    self.output.push(get_xml(token));
                    self.tokenizer.advance();
                    self.compile_term();
                    let t = self.tokenizer.current().unwrap();
                    println!("[debug] {:?}", t);
                    break;
                } else {
                    let t = self.tokenizer.current().unwrap();
                    println!("[debug] not op {:?}", t);
                    break;
                }
            } else {
                println!("[debug] Not token");
                break;
            }
        }
        self.output.push("</expression>".to_string());
    }

    fn compile_term(&mut self) {
        self.output.push("<term>".to_string());
        // unaryOp term
        if let Some(token) = self.tokenizer.current() {
            if token.raw == "-" || token.raw == "~" {
                self.output.push(get_xml(token));
                self.tokenizer.advance();
                self.compile_term();
                self.output.push("</term>".to_string());
                return;
            }
            if token.raw == "(" {
                self.output.push(get_xml(token));
                self.tokenizer.advance();
                self.compile_expression();
                // )を出力
                self.output.push(get_xml(self.tokenizer.current().unwrap()));
                self.tokenizer.advance();
                self.output.push("</term>".to_string());
                return;
            }
        }

        // integerConstant | stringConstant | keywordConstant | varName
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();

        if let Some(token) = self.tokenizer.current() {
            match token.raw.as_str() {
                // [ expression ]
                "[" => {
                    // [
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    // expression
                    self.tokenizer.advance();
                    self.compile_expression();
                    // ]
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                }
                // subroutineName ( expressionList )
                "(" => {
                    // (
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    // expression list
                    self.tokenizer.advance();
                    self.compile_expression_list();
                    // )
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                }
                // (className | varName) . subroutineName (expressionList)
                "." => {
                    // .
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                    // subroutine name
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    // (
                    self.tokenizer.advance();
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    // expression list
                    self.tokenizer.advance();
                    self.compile_expression_list();
                    // )
                    self.output.push(get_xml(self.tokenizer.current().unwrap()));
                    self.tokenizer.advance();
                }
                _ => {
                    let xml = get_xml(self.tokenizer.current().unwrap());
                    println!("next term: {}", xml);
                }
            }
        }
        self.output.push("</term>".to_string());
    }

    fn compile_subroutine_call(&mut self) {
        // subroutine Name | (className | varName)
        self.output.push(get_xml(self.tokenizer.current().unwrap()));
        self.tokenizer.advance();
        match self.tokenizer.current() {
            Some(token) => {
                match token.raw.as_str() {
                    // subroutineName ( expressionList )
                    "(" => {
                        // (
                        self.output.push(get_xml(self.tokenizer.current().unwrap()));
                        // expression list
                        self.tokenizer.advance();
                        self.compile_expression_list();
                        // )
                        self.output.push(get_xml(self.tokenizer.current().unwrap()));
                        self.tokenizer.advance();
                    }
                    // (className | varName) . subroutineName (expressionList)
                    "." => {
                        // .
                        self.output.push(get_xml(self.tokenizer.current().unwrap()));
                        self.tokenizer.advance();
                        // subroutine name
                        self.output.push(get_xml(self.tokenizer.current().unwrap()));
                        // (
                        self.tokenizer.advance();
                        self.output.push(get_xml(self.tokenizer.current().unwrap()));
                        // expression list
                        self.tokenizer.advance();
                        self.compile_expression_list();
                        // )
                        self.output.push(get_xml(self.tokenizer.current().unwrap()));
                        self.tokenizer.advance();
                    }
                    _ => {
                        let xml = get_xml(self.tokenizer.current().unwrap());
                        println!("next term: {}", xml);
                    }
                }
            }
            None => {}
        }
    }
}

pub fn compile(tokenizer: &mut Tokenizer) {
    let mut output: Vec<String> = vec![];
    let engine = CompileEngine::new(tokenizer, &mut output);
    // compile_class(tokenizer, &mut output);
}

// fn compile_class(tokenizer: &mut Tokenizer, output: &mut Vec<String>) {
//     if !tokenizer.has_more_tokens() {
//         panic!("class not found")
//     }
//     let token = tokenizer.current().unwrap();
//     if token.keyword() != Keyword::Class {
//         panic!("class not found")
//     }
//     output.push("<class>".to_string());
//     // class name
//     if tokenizer.advance().is_some() {
//         output.push(get_xml(tokenizer.current().unwrap()));
//     }
//     // symbol {
//     if tokenizer.advance().is_some() {
//         output.push(get_xml(tokenizer.current().unwrap()));
//     }
//     tokenizer.advance();

//     while tokenizer.has_more_tokens() {
//         let token = tokenizer.current().unwrap();
//         if TokenType::Keyword != token.token_type {
//             panic!("keyword not found")
//         }
//         match token.keyword() {
//             Keyword::Field => println!("it is Field"),
//             Keyword::Method | Keyword::Function | Keyword::Constructor => {
//                 compile_subroutine(tokenizer, output);
//             }
//             Keyword::Static => println!("it is Static"),
//             _ => println!("unknown Keyword"),
//         }
//         tokenizer.advance();
//     }
//     output.push("</class>".to_string());
// }

// fn compile_subroutine(tokenizer: &mut Tokenizer, output: &mut Vec<String>) {
//     output.push("<subroutineDec>".to_string());

//     // function or method or constructor
//     let token = tokenizer.current().unwrap();
//     output.push(get_xml(token));

//     if token.keyword() == Keyword::Function {
//         // void | type
//         if tokenizer.advance().is_some() {
//             output.push(get_xml(tokenizer.current().unwrap()));
//         }
//         // function name
//         if tokenizer.advance().is_some() {
//             output.push(get_xml(tokenizer.current().unwrap()));
//         }
//         // (
//         if tokenizer.advance().is_some() {
//             output.push(get_xml(tokenizer.current().unwrap()));
//         }
//         // parameter
//         compile_parameter_list(tokenizer, output);
//         // )
//         output.push(get_xml(tokenizer.current().unwrap()));
//         compile_subroutine_body(tokenizer);
//     }
//     output.push("</subroutineDec>".to_string());
// }

fn compile_parameter_list(tokenizer: &mut Tokenizer, output: &mut Vec<String>) {
    output.push("<parameterList>".to_string());
    while tokenizer.advance().unwrap().raw != ")" {
        let token = tokenizer.current().unwrap();
        output.push(get_xml(token));
    }
    output.push("</parameterList>".to_string());
}

// fn compile_subroutine_body(tokenizer: &mut Tokenizer) {
//     // output.push("</subroutineBody>".to_string());
//     // {
//     if let Some(token) = tokenizer.advance() {
//         println!("{}", get_xml(token));
//     }
//     tokenizer.advance();
//     while tokenizer.has_more_tokens() {
//         if let Some(token) = tokenizer.current() {
//             if TokenType::Keyword != token.token_type {
//                 panic!("keyword not found")
//             }
//             match token.keyword() {
//                 Keyword::Var => compile_var_dec(tokenizer),
//                 _ => compile_statements(tokenizer),
//             }
//         }
//         tokenizer.advance();
//     }
//     // }
//     if let Some(token) = tokenizer.advance() {
//         println!("{}", get_xml(token));
//     }
//     println!("</subroutineBody>");
// }

fn compile_var_dec(tokenizer: &mut Tokenizer) {
    println!("<varDec>");
    // var
    println!("{}", get_xml(tokenizer.current().unwrap()));
    // type
    if let Some(token) = tokenizer.advance() {
        println!("{}", get_xml(token));
    }
    // var name | ,
    while tokenizer.advance().unwrap().raw != ";" {
        let token = tokenizer.current().unwrap();
        println!("{}", get_xml(token));
    }
    // ;
    println!("{}", get_xml(tokenizer.current().unwrap()));
    println!("</varDec>");
}

// fn compile_statements(tokenizer: &mut Tokenizer) {
//     println!("<statements>");
//     while tokenizer.has_more_tokens() {
//         if let Some(token) = tokenizer.current() {
//             if TokenType::Keyword != token.token_type {
//                 panic!("keyword not found")
//             }
//             match token.keyword() {
//                 Keyword::Let => compile_let(tokenizer),
//                 _ => println!("other words"),
//             }
//         }
//         tokenizer.advance();
//     }
//     println!("</statements>");
// }

// fn compile_let(tokenizer: &mut Tokenizer) {
//     println!("<letStatement>");
//     // let
//     println!("{}", get_xml(tokenizer.current().unwrap()));
//     // var name
//     if let Some(token) = tokenizer.advance() {
//         println!("{}", get_xml(token));
//     }
//     // [] がある場合は添字の処理
//     if let Some(token) = tokenizer.advance() {
//         if token.raw == "[" {
//             println!("{}", get_xml(token));
//             tokenizer.advance();
//         }
//     }
//     println!("</letStatement>");
// }

fn get_xml(token: &Token) -> String {
    let fix_token = xml_encode(&token.raw);
    match token.token_type {
        TokenType::Keyword => format!("<keyword> {} </keyword>", fix_token),
        TokenType::Identifier => format!("<identifier> {} </identifier>", fix_token),
        TokenType::Symbol => format!("<symbol> {} </symbol>", fix_token),
        TokenType::IntConst => format!("<integerConstant> {} </integerConstant>", fix_token),
        TokenType::StringConst => format!("<stringConstant> {} </stringConstant>", fix_token),
        _ => "unknown xml".to_string(),
    }
}

fn xml_encode(str: &str) -> &str {
    match str {
        "<" => "&lt;",
        _ => str,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod compile_engine {
        use super::*;

        mod compile_expression_list {
            use super::*;

            // #[test]
            // fn test_compile_expression_list() {
            //     // do func(x, y, z) みたいな引数を想定
            //     let mut tokenizer = Tokenizer::new(vec![
            //         Token::new("x".to_string(), TokenType::Identifier),
            //         Token::new(",".to_string(), TokenType::Symbol),
            //         Token::new("y".to_string(), TokenType::Identifier),
            //         Token::new(",".to_string(), TokenType::Symbol),
            //         Token::new("z".to_string(), TokenType::Identifier),
            //     ]);

            //     let mut output: Vec<String> = vec![];
            //     let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
            //     engine.compile_expression_list();

            //     assert_eq!(
            //         output,
            //         [
            //             "<expressionList>",
            //             "<identifier> x </identifier>",
            //             "<symbol> , </symbol>",
            //             "<identifier> y </identifier>",
            //             "<symbol> , </symbol>",
            //             "<identifier> z </identifier>",
            //             "</expressionList>"
            //         ]
            //     )
            // }
        }

        mod compile_if {
            use super::*;

            #[test]
            fn test_normal() {
                // if (i) {
                //     let s = i;
                //     let s = j;
                //     let a[i] = j;
                // }
                // else {
                //     let i = i;
                //     let j = j;
                //     let i = i | j;
                // }
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("if".to_string(), TokenType::Keyword),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("s".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("s".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("a".to_string(), TokenType::Identifier),
                    Token::new("[".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("]".to_string(), TokenType::Symbol),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                    Token::new("else".to_string(), TokenType::Keyword),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("|".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_if();

                assert_eq!(
                    output,
                    [
                        "<ifStatement>",
                        "<keyword> if </keyword>",
                        "<symbol> ( </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ) </symbol>",
                        "<symbol> { </symbol>",
                        "<statements>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> s </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> s </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> j </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> a </identifier>",
                        "<symbol> [ </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ] </symbol>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> j </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "<keyword> else </keyword>",
                        "<symbol> { </symbol>",
                        "<statements>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> i </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> j </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> j </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> i </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "<symbol> | </symbol>",
                        "<term>",
                        "<identifier> j </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</ifStatement>",
                    ]
                )
            }
        }

        mod compile_while {
            use super::*;

            #[test]
            fn test_normal() {
                // while (i < length) {
                //     let sum = sum + a[i];
                //     let i = i + 1;
                // }
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("while".to_string(), TokenType::Keyword),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("<".to_string(), TokenType::Symbol),
                    Token::new("length".to_string(), TokenType::Identifier),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("sum".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("sum".to_string(), TokenType::Identifier),
                    Token::new("+".to_string(), TokenType::Symbol),
                    Token::new("a".to_string(), TokenType::Identifier),
                    Token::new("[".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("]".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("+".to_string(), TokenType::Symbol),
                    Token::new("1".to_string(), TokenType::IntConst),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_while();

                assert_eq!(
                    output,
                    [
                        "<whileStatement>",
                        "<keyword> while </keyword>",
                        "<symbol> ( </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "<symbol> &lt; </symbol>",
                        "<term>",
                        "<identifier> length </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ) </symbol>",
                        "<symbol> { </symbol>",
                        "<statements>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> sum </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> sum </identifier>",
                        "</term>",
                        "<symbol> + </symbol>",
                        "<term>",
                        "<identifier> a </identifier>",
                        "<symbol> [ </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ] </symbol>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> i </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "<symbol> + </symbol>",
                        "<term>",
                        "<integerConstant> 1 </integerConstant>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</whileStatement>",
                    ]
                )
            }
        }

        mod compile_let {
            use super::*;

            #[test]
            fn test_normal() {
                // let size = Asize;
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("size".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("Asize".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_let();

                assert_eq!(
                    output,
                    [
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> size </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> Asize </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                    ]
                )
            }

            #[test]
            fn test_with_brackets() {
                // let size = Asize;
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("a".to_string(), TokenType::Identifier),
                    Token::new("[".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("]".to_string(), TokenType::Symbol),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("Keyboard".to_string(), TokenType::Identifier),
                    Token::new(".".to_string(), TokenType::Symbol),
                    Token::new("readInt".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(
                        "ENTER THE NEXT NUMBER: ".to_string(),
                        TokenType::StringConst,
                    ),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_let();

                assert_eq!(
                    output,
                    [
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> a </identifier>",
                        "<symbol> [ </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ] </symbol>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> Keyboard </identifier>",
                        "<symbol> . </symbol>",
                        "<identifier> readInt </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "<expression>",
                        "<term>",
                        "<stringConstant> ENTER THE NEXT NUMBER:  </stringConstant>",
                        "</term>",
                        "</expression>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                    ]
                )
            }
        }

        mod compile_do {
            use super::*;

            #[test]
            fn test_with_var_name() {
                // do Screen.drawRectangle(x, y, x, y);
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("do".to_string(), TokenType::Keyword),
                    Token::new("Screen".to_string(), TokenType::Identifier),
                    Token::new(".".to_string(), TokenType::Symbol),
                    Token::new("drawRectangle".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("x".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("y".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("x".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("y".to_string(), TokenType::Identifier),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_do();
                println!("{:?}", output);

                assert_eq!(
                    output,
                    [
                        "<doStatement>",
                        "<keyword> do </keyword>",
                        "<identifier> Screen </identifier>",
                        "<symbol> . </symbol>",
                        "<identifier> drawRectangle </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "<expression>",
                        "<term>",
                        "<identifier> x </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> , </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> y </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> , </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> x </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> , </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> y </identifier>",
                        "</term>",
                        "</expression>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "<symbol> ; </symbol>",
                        "</doStatement>",
                    ]
                )
            }
        }

        mod compile_return {
            use super::*;

            #[test]
            fn test_only_return() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_return();

                assert_eq!(
                    output,
                    [
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                    ]
                )
            }

            #[test]
            fn test_with_expression() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new("this".to_string(), TokenType::Keyword),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_return();

                assert_eq!(
                    output,
                    [
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<expression>",
                        "<term>",
                        "<keyword> this </keyword>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                    ]
                )
            }
        }

        mod compile_term {
            use super::*;

            #[test]
            fn test_integer_constant() {
                let mut tokenizer =
                    Tokenizer::new(vec![Token::new("0".to_string(), TokenType::IntConst)]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<integerConstant> 0 </integerConstant>",
                        "</term>",
                    ]
                )
            }

            #[test]
            fn test_string_constant() {
                let mut tokenizer = Tokenizer::new(vec![Token::new(
                    "ENTER THE NEXT NUMBER:".to_string(),
                    TokenType::StringConst,
                )]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<stringConstant> ENTER THE NEXT NUMBER: </stringConstant>",
                        "</term>",
                    ]
                )
            }

            #[test]
            fn test_keyword_constant() {
                let mut tokenizer =
                    Tokenizer::new(vec![Token::new("this".to_string(), TokenType::Keyword)]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(output, ["<term>", "<keyword> this </keyword>", "</term>",])
            }

            #[test]
            fn test_var_name() {
                let mut tokenizer =
                    Tokenizer::new(vec![Token::new("x".to_string(), TokenType::Identifier)]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    ["<term>", "<identifier> x </identifier>", "</term>",]
                )
            }

            #[test]
            // verName [ expression ]
            fn test_var_name_expression() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("a".to_string(), TokenType::Identifier),
                    Token::new("[".to_string(), TokenType::Symbol),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new("]".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<identifier> a </identifier>",
                        "<symbol> [ </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> i </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ] </symbol>",
                        "</term>",
                    ]
                )
            }

            // subroutineCall: “subroutineName '(' expressionList ')”
            #[test]
            fn test_subroutine_call1() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("func".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("-".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(")".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<identifier> func </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "<expression>",
                        "<term>",
                        "<symbol> - </symbol>",
                        "<term>",
                        "<identifier> j </identifier>",
                        "</term>",
                        "</term>",
                        "</expression>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "</term>",
                    ]
                )
            }

            // subroutineCall : “(className | varName) '.' subroutineName '(' expressionList ')”
            #[test]
            fn test_subroutine_call2() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("Keyboard".to_string(), TokenType::Identifier),
                    Token::new(".".to_string(), TokenType::Symbol),
                    Token::new("readInt".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(
                        "ENTER THE NEXT NUMBER: ".to_string(),
                        TokenType::StringConst,
                    ),
                    Token::new(")".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<identifier> Keyboard </identifier>",
                        "<symbol> . </symbol>",
                        "<identifier> readInt </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "<expression>",
                        "<term>",
                        "<stringConstant> ENTER THE NEXT NUMBER:  </stringConstant>",
                        "</term>",
                        "</expression>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "</term>"
                    ]
                )
            }

            // ( expression )
            #[test]
            fn test_expression() {
                // ((y + size) < 254)
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("y".to_string(), TokenType::Identifier),
                    Token::new("+".to_string(), TokenType::Symbol),
                    Token::new("size".to_string(), TokenType::Identifier),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("<".to_string(), TokenType::Symbol),
                    Token::new("254".to_string(), TokenType::IntConst),
                    Token::new(")".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<symbol> ( </symbol>",
                        "<expression>",
                        "<term>",
                        "<symbol> ( </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> y </identifier>",
                        "</term>",
                        "<symbol> + </symbol>",
                        "<term>",
                        "<identifier> size </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ) </symbol>",
                        "</term>",
                        "<symbol> &lt; </symbol>", // <
                        "<term>",
                        "<integerConstant> 254 </integerConstant>",
                        "</term>",
                        "</expression>",
                        "<symbol> ) </symbol>",
                        "</term>",
                    ]
                )
            }

            // unaryOp term
            #[test]
            fn test_unary_op() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("-".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_term();

                assert_eq!(
                    output,
                    [
                        "<term>",
                        "<symbol> - </symbol>",
                        "<term>",
                        "<identifier> j </identifier>",
                        "</term>",
                        "</term>",
                    ]
                )
            }
        }
    }
}

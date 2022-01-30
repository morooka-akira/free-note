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

    fn compile_class(&mut self) {
        self.output.push("<class>".to_string());
        // class
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // className
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // {
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();

        loop {
            if self.tokenizer.current().unwrap().raw == "}" {
                break;
            }
            match self.tokenizer.current() {
                Some(token) => match token.token_type {
                    TokenType::Keyword => match token.keyword() {
                        Keyword::Static | Keyword::Field => self.compile_class_var_dec(),
                        Keyword::Constructor | Keyword::Function | Keyword::Method => {
                            self.compile_subroutine_dec()
                        }
                        _ => panic!("compile_class: unexpected keyword: {:?}", token),
                    },
                    _ => panic!("compile_class: unexpected token type: {:?}", token),
                },
                None => {}
            }
        }
        // }
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</class>".to_string());
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

    // ('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody
    fn compile_subroutine_dec(&mut self) {
        self.output.push("<subroutineDec>".to_string());
        // ('constructor' | 'function' | 'method')
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // ('void' | type)
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // subroutineName
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // (
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // parameterList
        self.compile_parameter_list();
        // )
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // subrountineBody
        self.compile_subroutine_body();
        self.output.push("</subroutineDec>".to_string());
    }

    fn compile_parameter_list(&mut self) {
        self.output.push("<parameterList>".to_string());
        loop {
            match self.tokenizer.current() {
                Some(token) => {
                    if token.raw == ")" {
                        break;
                    }
                }
                None => break,
            }
            self.output.push(self.tokenizer.current().unwrap().to_xml());
            self.tokenizer.advance();
        }
        self.output.push("</parameterList>".to_string());
    }

    // '{' varDec* statements '}'
    fn compile_subroutine_body(&mut self) {
        self.output.push("<subroutineBody>".to_string());
        // {
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();

        loop {
            match self.tokenizer.current().unwrap().keyword() {
                Keyword::Var => {
                    self.compile_var_dec();
                }
                _ => break,
            }
        }
        self.compile_statements();

        // }
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</subroutineBody>".to_string());
    }

    // ('static' | 'field') varName (',' varName)* ';'
    fn compile_class_var_dec(&mut self) {
        self.output.push("<classVarDec>".to_string());
        // static | field
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // type
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // varName
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        loop {
            if self.tokenizer.current().unwrap().raw == ";" {
                break;
            }
            self.output.push(self.tokenizer.current().unwrap().to_xml());
            self.tokenizer.advance();
        }
        // ;
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</classVarDec>".to_string());
    }

    // 'var' type varName (',' varName)* ';'
    fn compile_var_dec(&mut self) {
        self.output.push("<varDec>".to_string());
        // var
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // type
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // varName
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        loop {
            if self.tokenizer.current().unwrap().raw == ";" {
                break;
            }
            self.output.push(self.tokenizer.current().unwrap().to_xml());
            self.tokenizer.advance();
        }
        // ;
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</varDec>".to_string());
    }

    // 'while' '(' expression ')' '{' statements '}'
    fn compile_while(&mut self) {
        self.output.push("<whileStatement>".to_string());
        // while
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // (
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // expression
        self.compile_expression();
        // )
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // {
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // statements
        self.compile_statements();
        // }
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</whileStatement>".to_string());
    }

    // 'let' varName ('[' expression ']')? '=' expression ';'
    fn compile_let(&mut self) {
        self.output.push("<letStatement>".to_string());
        // let
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // varName
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        match self.tokenizer.current() {
            Some(token) => {
                if token.raw == "[" {
                    // '[' expression ']'
                    // [
                    self.output.push(token.to_xml());
                    self.tokenizer.advance();
                    self.compile_expression();
                    // ]
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                }
            }
            None => {}
        }
        // =
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.compile_expression();
        // :
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</letStatement>".to_string());
    }

    fn compile_if(&mut self) {
        self.output.push("<ifStatement>".to_string());
        // if
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // (
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // expression
        self.compile_expression();
        // )
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // {
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        // statements
        self.compile_statements();
        // }
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();

        match self.tokenizer.current() {
            Some(token) => {
                if token.raw == "else" {
                    // else '{' statements '}'
                    // else
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                    // {
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                    // statements
                    self.compile_statements();
                    // }
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
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
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.compile_subroutine_call();
        // ;
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</doStatement>".to_string());
    }

    // return expression? ;
    fn compile_return(&mut self) {
        self.output.push("<returnStatement>".to_string());
        // return
        self.output.push(self.tokenizer.current().unwrap().to_xml());
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
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        self.output.push("</returnStatement>".to_string());
    }

    // (expression (',' expression))
    fn compile_expression_list(&mut self) {
        self.output.push("<expressionList>".to_string());
        loop {
            match self.tokenizer.current() {
                Some(token) => {
                    if token.raw == ")" {
                        break;
                    }
                    if token.raw == "," {
                        self.output.push(token.to_xml());
                        self.tokenizer.advance();
                    }
                    self.compile_expression();
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
                    self.output.push(token.to_xml());
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
                self.output.push(token.to_xml());
                self.tokenizer.advance();
                self.compile_term();
                self.output.push("</term>".to_string());
                return;
            }
            if token.raw == "(" {
                self.output.push(token.to_xml());
                self.tokenizer.advance();
                self.compile_expression();
                // )を出力
                self.output.push(self.tokenizer.current().unwrap().to_xml());
                self.tokenizer.advance();
                self.output.push("</term>".to_string());
                return;
            }
        }

        // integerConstant | stringConstant | keywordConstant | varName
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();

        if let Some(token) = self.tokenizer.current() {
            match token.raw.as_str() {
                // [ expression ]
                "[" => {
                    // [
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    // expression
                    self.tokenizer.advance();
                    self.compile_expression();
                    // ]
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                }
                // subroutineName ( expressionList )
                "(" => {
                    // (
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    // expression list
                    self.tokenizer.advance();
                    self.compile_expression_list();
                    // )
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                }
                // (className | varName) . subroutineName (expressionList)
                "." => {
                    // .
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                    // subroutine name
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    // (
                    self.tokenizer.advance();
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    // expression list
                    self.tokenizer.advance();
                    self.compile_expression_list();
                    // )
                    self.output.push(self.tokenizer.current().unwrap().to_xml());
                    self.tokenizer.advance();
                }
                _ => {
                    let xml = self.tokenizer.current().unwrap().to_xml();
                    println!("next term: {}", xml);
                }
            }
        }
        self.output.push("</term>".to_string());
    }

    fn compile_subroutine_call(&mut self) {
        // subroutine Name | (className | varName)
        self.output.push(self.tokenizer.current().unwrap().to_xml());
        self.tokenizer.advance();
        match self.tokenizer.current() {
            Some(token) => {
                match token.raw.as_str() {
                    // subroutineName ( expressionList )
                    "(" => {
                        // (
                        self.output.push(self.tokenizer.current().unwrap().to_xml());
                        // expression list
                        self.tokenizer.advance();
                        self.compile_expression_list();
                        // )
                        self.output.push(self.tokenizer.current().unwrap().to_xml());
                        self.tokenizer.advance();
                    }
                    // (className | varName) . subroutineName (expressionList)
                    "." => {
                        // .
                        self.output.push(self.tokenizer.current().unwrap().to_xml());
                        self.tokenizer.advance();
                        // subroutine name
                        self.output.push(self.tokenizer.current().unwrap().to_xml());
                        // (
                        self.tokenizer.advance();
                        self.output.push(self.tokenizer.current().unwrap().to_xml());
                        // expression list
                        self.tokenizer.advance();
                        self.compile_expression_list();
                        // )
                        self.output.push(self.tokenizer.current().unwrap().to_xml());
                        self.tokenizer.advance();
                    }
                    _ => {
                        let xml = self.tokenizer.current().unwrap().to_xml();
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
    let mut engine = CompileEngine::new(tokenizer, &mut output);
    engine.compile_class();
}

#[cfg(test)]
mod tests {
    use super::*;
    mod compile_engine {
        use super::*;

        mod test_compile_class {
            use super::*;

            #[test]
            fn test_compile_class() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("class".to_string(), TokenType::Keyword),
                    Token::new("Main".to_string(), TokenType::Identifier),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("static".to_string(), TokenType::Keyword),
                    Token::new("boolean".to_string(), TokenType::Keyword),
                    Token::new("test".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("function".to_string(), TokenType::Keyword),
                    Token::new("void".to_string(), TokenType::Keyword),
                    Token::new("main".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("SquareGame".to_string(), TokenType::Identifier),
                    Token::new("game".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("game".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("game".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("do".to_string(), TokenType::Keyword),
                    Token::new("game".to_string(), TokenType::Identifier),
                    Token::new(".".to_string(), TokenType::Symbol),
                    Token::new("run".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("do".to_string(), TokenType::Keyword),
                    Token::new("game".to_string(), TokenType::Identifier),
                    Token::new(".".to_string(), TokenType::Symbol),
                    Token::new("dispose".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                    Token::new("function".to_string(), TokenType::Keyword),
                    Token::new("void".to_string(), TokenType::Keyword),
                    Token::new("test".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("String".to_string(), TokenType::Identifier),
                    Token::new("s".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("Array".to_string(), TokenType::Identifier),
                    Token::new("a".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
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
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                ]);
                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_class();

                assert_eq!(
                    output,
                    [
                        "<class>",
                        "<keyword> class </keyword>",
                        "<identifier> Main </identifier>",
                        "<symbol> { </symbol>",
                        "<classVarDec>",
                        "<keyword> static </keyword>",
                        "<keyword> boolean </keyword>",
                        "<identifier> test </identifier>",
                        "<symbol> ; </symbol>",
                        "</classVarDec>",
                        "<subroutineDec>",
                        "<keyword> function </keyword>",
                        "<keyword> void </keyword>",
                        "<identifier> main </identifier>",
                        "<symbol> ( </symbol>",
                        "<parameterList>",
                        "</parameterList>",
                        "<symbol> ) </symbol>",
                        "<subroutineBody>",
                        "<symbol> { </symbol>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<identifier> SquareGame </identifier>",
                        "<identifier> game </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<statements>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> game </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> game </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<doStatement>",
                        "<keyword> do </keyword>",
                        "<identifier> game </identifier>",
                        "<symbol> . </symbol>",
                        "<identifier> run </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "<symbol> ; </symbol>",
                        "</doStatement>",
                        "<doStatement>",
                        "<keyword> do </keyword>",
                        "<identifier> game </identifier>",
                        "<symbol> . </symbol>",
                        "<identifier> dispose </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "<symbol> ; </symbol>",
                        "</doStatement>",
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</subroutineBody>",
                        "</subroutineDec>",
                        "<subroutineDec>",
                        "<keyword> function </keyword>",
                        "<keyword> void </keyword>",
                        "<identifier> test </identifier>",
                        "<symbol> ( </symbol>",
                        "<parameterList>",
                        "</parameterList>",
                        "<symbol> ) </symbol>",
                        "<subroutineBody>",
                        "<symbol> { </symbol>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<keyword> int </keyword>",
                        "<identifier> i </identifier>",
                        "<symbol> , </symbol>",
                        "<identifier> j </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<identifier> String </identifier>",
                        "<identifier> s </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<identifier> Array </identifier>",
                        "<identifier> a </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<statements>",
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
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</subroutineBody>",
                        "</subroutineDec>",
                        "<symbol> } </symbol>",
                        "</class>",
                    ]
                )
            }
        }

        mod compile_expression_list {
            use super::*;

            #[test]
            fn test_compile_expression_list() {
                // do func(x, y, z) みたいな引数を想定
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("x".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("y".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("z".to_string(), TokenType::Identifier),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_expression_list();

                assert_eq!(
                    output,
                    [
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
                        "<identifier> z </identifier>",
                        "</term>",
                        "</expression>",
                        "</expressionList>",
                    ]
                )
            }

            #[test]
            fn test_compile_empty() {
                let mut tokenizer = Tokenizer::new(vec![]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_expression_list();

                assert_eq!(output, ["<expressionList>", "</expressionList>",])
            }
        }

        mod compile_subroutine_dec {
            use super::*;

            #[test]
            fn test_function() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("function".to_string(), TokenType::Keyword),
                    Token::new("void".to_string(), TokenType::Keyword),
                    Token::new("test".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("String".to_string(), TokenType::Identifier),
                    Token::new("s".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("Array".to_string(), TokenType::Identifier),
                    Token::new("a".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
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
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_subroutine_dec();

                assert_eq!(
                    output,
                    [
                        "<subroutineDec>",
                        "<keyword> function </keyword>",
                        "<keyword> void </keyword>",
                        "<identifier> test </identifier>",
                        "<symbol> ( </symbol>",
                        "<parameterList>",
                        "</parameterList>",
                        "<symbol> ) </symbol>",
                        "<subroutineBody>",
                        "<symbol> { </symbol>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<keyword> int </keyword>",
                        "<identifier> i </identifier>",
                        "<symbol> , </symbol>",
                        "<identifier> j </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<identifier> String </identifier>",
                        "<identifier> s </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<identifier> Array </identifier>",
                        "<identifier> a </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>",
                        "<statements>",
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
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</subroutineBody>",
                        "</subroutineDec>",
                    ]
                )
            }

            #[test]
            fn test_constructor() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("constructor".to_string(), TokenType::Keyword),
                    Token::new("Square".to_string(), TokenType::Identifier),
                    Token::new("new".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("Ax".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("Ay".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("Asize".to_string(), TokenType::Identifier),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("x".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("Ax".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("y".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("Ay".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("let".to_string(), TokenType::Keyword),
                    Token::new("size".to_string(), TokenType::Identifier),
                    Token::new("=".to_string(), TokenType::Symbol),
                    Token::new("Asize".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("do".to_string(), TokenType::Keyword),
                    Token::new("draw".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new("x".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_subroutine_dec();

                assert_eq!(
                    output,
                    [
                        "<subroutineDec>",
                        "<keyword> constructor </keyword>",
                        "<identifier> Square </identifier>",
                        "<identifier> new </identifier>",
                        "<symbol> ( </symbol>",
                        "<parameterList>",
                        "<keyword> int </keyword>",
                        "<identifier> Ax </identifier>",
                        "<symbol> , </symbol>",
                        "<keyword> int </keyword>",
                        "<identifier> Ay </identifier>",
                        "<symbol> , </symbol>",
                        "<keyword> int </keyword>",
                        "<identifier> Asize </identifier>",
                        "</parameterList>",
                        "<symbol> ) </symbol>",
                        "<subroutineBody>",
                        "<symbol> { </symbol>",
                        "<statements>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> x </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> Ax </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
                        "<letStatement>",
                        "<keyword> let </keyword>",
                        "<identifier> y </identifier>",
                        "<symbol> = </symbol>",
                        "<expression>",
                        "<term>",
                        "<identifier> Ay </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</letStatement>",
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
                        "<doStatement>",
                        "<keyword> do </keyword>",
                        "<identifier> draw </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "<symbol> ; </symbol>",
                        "</doStatement>",
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<expression>",
                        "<term>",
                        "<identifier> x </identifier>",
                        "</term>",
                        "</expression>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</subroutineBody>",
                        "</subroutineDec>",
                    ]
                )
            }
        }

        mod compile_parameter_list {
            use super::*;

            #[test]
            fn test_parameter_list() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("Ax".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("Ay".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("Asize".to_string(), TokenType::Identifier),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_parameter_list();

                assert_eq!(
                    output,
                    [
                        "<parameterList>",
                        "<keyword> int </keyword>",
                        "<identifier> Ax </identifier>",
                        "<symbol> , </symbol>",
                        "<keyword> int </keyword>",
                        "<identifier> Ay </identifier>",
                        "<symbol> , </symbol>",
                        "<keyword> int </keyword>",
                        "<identifier> Asize </identifier>",
                        "</parameterList>",
                    ]
                )
            }
        }

        mod compile_subroutine_body {
            use super::*;

            #[test]
            fn test_subroutine_body() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("{".to_string(), TokenType::Symbol),
                    Token::new("do".to_string(), TokenType::Keyword),
                    Token::new("Memory".to_string(), TokenType::Identifier),
                    Token::new(".".to_string(), TokenType::Symbol),
                    Token::new("deAlloc".to_string(), TokenType::Identifier),
                    Token::new("(".to_string(), TokenType::Symbol),
                    Token::new("this".to_string(), TokenType::Keyword),
                    Token::new(")".to_string(), TokenType::Symbol),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("return".to_string(), TokenType::Keyword),
                    Token::new(";".to_string(), TokenType::Symbol),
                    Token::new("}".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_subroutine_body();

                assert_eq!(
                    output,
                    [
                        "<subroutineBody>",
                        "<symbol> { </symbol>",
                        "<statements>",
                        "<doStatement>",
                        "<keyword> do </keyword>",
                        "<identifier> Memory </identifier>",
                        "<symbol> . </symbol>",
                        "<identifier> deAlloc </identifier>",
                        "<symbol> ( </symbol>",
                        "<expressionList>",
                        "<expression>",
                        "<term>",
                        "<keyword> this </keyword>",
                        "</term>",
                        "</expression>",
                        "</expressionList>",
                        "<symbol> ) </symbol>",
                        "<symbol> ; </symbol>",
                        "</doStatement>",
                        "<returnStatement>",
                        "<keyword> return </keyword>",
                        "<symbol> ; </symbol>",
                        "</returnStatement>",
                        "</statements>",
                        "<symbol> } </symbol>",
                        "</subroutineBody>",
                    ]
                )
            }
        }

        mod compile_class_var_dec {
            use super::*;

            #[test]
            fn test_field() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("field".to_string(), TokenType::Keyword),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("x".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("y".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_class_var_dec();

                assert_eq!(
                    output,
                    [
                        "<classVarDec>",
                        "<keyword> field </keyword>",
                        "<keyword> int </keyword>",
                        "<identifier> x </identifier>",
                        "<symbol> , </symbol>",
                        "<identifier> y </identifier>",
                        "<symbol> ; </symbol>",
                        "</classVarDec>",
                    ]
                )
            }
        }

        mod compile_var_dec {
            use super::*;

            #[test]
            fn test_normal() {
                let mut tokenizer = Tokenizer::new(vec![
                    Token::new("var".to_string(), TokenType::Keyword),
                    Token::new("int".to_string(), TokenType::Keyword),
                    Token::new("i".to_string(), TokenType::Identifier),
                    Token::new(",".to_string(), TokenType::Symbol),
                    Token::new("j".to_string(), TokenType::Identifier),
                    Token::new(";".to_string(), TokenType::Symbol),
                ]);

                let mut output: Vec<String> = vec![];
                let mut engine = CompileEngine::new(&mut tokenizer, &mut output);
                engine.compile_var_dec();

                assert_eq!(
                    output,
                    [
                        "<varDec>",
                        "<keyword> var </keyword>",
                        "<keyword> int </keyword>",
                        "<identifier> i </identifier>",
                        "<symbol> , </symbol>",
                        "<identifier> j </identifier>",
                        "<symbol> ; </symbol>",
                        "</varDec>"
                    ]
                )
            }
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

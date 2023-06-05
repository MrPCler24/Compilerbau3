
use crate::lexer::{C1Lexer, C1Token};
use crate::ParseResult;
use std::error::Error;
use std::ops::{Deref, DerefMut};

pub struct C1Parser<'a>(C1Lexer<'a>);
// Implement Deref and DerefMut to enable the direct use of the lexer's methods
impl<'a> Deref for C1Parser<'a> {
    type Target = C1Lexer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for C1Parser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> C1Parser<'a> {
    pub fn parse(text: &str) -> ParseResult {
        let mut parser = Self::initialize_parser(text);
        parser.program()
    }

    fn initialize_parser(text: &str) -> C1Parser {
        C1Parser(C1Lexer::new(text))
    }

    /// program ::= ( functiondefinition )* <EOF>
    fn program(&mut self) -> ParseResult {
        while self.current_token().is_some() {
            self.function_definition()?;
        }
        Ok(())
    }

    // TODO: implement remaining grammar

    //type <ID> "(" ")" "{" statementlist "}"
    fn function_definition(&mut self) -> ParseResult {
        self.return_type()?;
        self.check_and_eat_token(&C1Token::Identifier,"\"ID\" Expected")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis,"\"(\" Expected")?;
        self.check_and_eat_token(&C1Token::RightParenthesis,"\")\" Expected")?;
        self.check_and_eat_token(&C1Token::LeftBrace, "\"{\" Expected")?;
        self.statement_list()?;
        self.check_and_eat_token(&C1Token::RightBrace, "\"}\" Expected")?;
        Ok(())
    }

    //<ID> "(" ")"
    fn function_call(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::Identifier,"\"ID\" Expected")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis,"\"(\" Expected")?;
        self.check_and_eat_token(&C1Token::RightParenthesis,"\")\" Expected")?;
        Ok(())
    }

    //( block )*
    fn statement_list(&mut self) -> ParseResult {     
        
        while self.any_match_current(&[C1Token::LeftBrace, C1Token::KwIf, C1Token::KwReturn, C1Token::KwPrintf,C1Token::Identifier]){
            self.block()?;   
        }
        Ok(())
        
    }

    //"{" statementlist "}" | statement
    fn block(&mut self) -> ParseResult {
        if self.current_matches(&C1Token::LeftBrace){
            self.eat();
            self.statement_list()?;
            self.check_and_eat_token(&C1Token::RightBrace, "\"}\" Expected")?;
        }
        else {
            self.statement()?;
        }  
        Ok(())
    }

    //ifstatement | returnstatement ";" | printf ";" | statassignment ";" | functioncall ";"
    fn statement(&mut self) -> ParseResult {
        if self.current_matches(&C1Token::KwIf){

            self.if_statement()?;
            Ok(()) 
        } else
        if self.current_matches(&C1Token::KwReturn){
           
            self.return_statement()?;
            self.check_and_eat_token(&C1Token::Semicolon, "\";\" Expected")?;
            Ok(()) 
        } else
        if self.current_matches(&C1Token::KwPrintf){
            
            self.printf()?;
            self.check_and_eat_token(&C1Token::Semicolon, "\";\" Expected")?;
            Ok(()) 
        } else
        if self.current_matches(&C1Token::Identifier)&&self.next_matches(&C1Token::Assign){
            
            self.stat_assignment()?;
            self.check_and_eat_token(&C1Token::Semicolon, "\";\" Expected")?;
            Ok(())
        }
        else 
        if self.current_matches(&C1Token::Identifier)&&self.next_matches(&C1Token::LeftParenthesis){
           
            self.function_call()?;
            self.check_and_eat_token(&C1Token::Semicolon, "\";\" Expected")?;
            Ok(())
              
        }else{
            Err(String::from("Invalid Statement found"))
        }
       
            
        


    }

    //<KW_IF> "(" assignment ")" block
    fn if_statement(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::KwIf, "\"If\" Expected")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis, "\"(\" Expected")?;
        self.assignment()?;
        self.check_and_eat_token(&C1Token::RightParenthesis, "\")\" Expected")?;
        self.block()?;
        Ok(())
    }

    //<KW_RETURN> ( assignment )?
    fn return_statement(&mut self) -> ParseResult {

        self.check_and_eat_token(&C1Token::KwReturn, "\"Return\" Expected")?;
        if self.any_match_current(&[C1Token::Identifier, C1Token::Minus, C1Token::ConstInt, C1Token::ConstFloat, C1Token::ConstBoolean, C1Token::LeftParenthesis]){
            self.assignment()?;
        }
        Ok(())
        
    }

    //<KW_PRINTF> "(" assignment ")"
    fn printf(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::KwPrintf, "\"Print\" Expected")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis, "\"(\" Expected")?;    
        self.assignment()?;
        self.check_and_eat_token(&C1Token::RightParenthesis, "\")\" Expected")?;
        Ok(())
    }
    
    //<KW_BOOLEAN> | <KW_FLOAT> | <KW_INT> | <KW_VOID>
    fn return_type(&mut self) -> ParseResult {
        self.any_match_and_eat(&[C1Token::KwBoolean,C1Token::KwFloat, C1Token::KwInt, C1Token::KwVoid],  &String::from("Correct \"Type\" Expected"))?;        
        Ok(())
    }

    //<ID> "=" assignment
    fn stat_assignment(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::Identifier, "\"ID\" Expected")?;
        self.check_and_eat_token(&C1Token::Assign, "\"=\" Expected")?;    
        self.assignment()?;
        Ok(())
    }
    
    //( ( <ID> "=" assignment ) | expr )
    fn assignment(&mut self) -> ParseResult {
        if self.current_matches(&C1Token::Identifier){
           if self.next_matches(&C1Token::Assign){
            self.eat();
            self.eat();
            self.assignment()?;

            }
            else{
                self.expr()?;
            }
        }
         else{
            self.expr()?;
        }
        Ok(())        
        
    }

    

    //simpexpr ( ( "==" | "!=" | "<=" | ">=" | "<" | ">" ) simpexpr )?
    fn expr(&mut self) -> ParseResult {
        self.simpexpr()?;
        if self.any_match_current(&[C1Token::Equal, C1Token::NotEqual, C1Token::LessEqual, C1Token::GreaterEqual, C1Token::Less, C1Token::Greater]){
            self.eat();
            self.simpexpr()?;
        } 
        Ok(())
    }

    //( "-" )? term ( ( "+" | "-" | "||" ) term )*
    fn simpexpr(&mut self) -> ParseResult {
        if self.current_matches(&C1Token::Minus){
            self.eat();
        }
        self.term()?;

        while self.any_match_current(&[C1Token::Plus, C1Token::Minus, C1Token::Or]) {
            self.eat();
            self.term()?;
        }    
        Ok(())
    }

    //factor ( ( "*" | "/" | "&&" ) factor )*
    fn term(&mut self) -> ParseResult {
        self.factor()?;

        while self.any_match_current(&[C1Token::Asterisk, C1Token::Slash, C1Token::And]) {
            self.eat();
            self.factor()?;
        }    
        Ok(())

    }

    //<CONST_INT> | <CONST_FLOAT> | <CONST_BOOLEAN> | functioncall | <ID> | "(" assignment ")"
    fn factor(&mut self) -> ParseResult {
        if self.any_match_current(&[C1Token::ConstInt, C1Token::ConstFloat, C1Token::ConstBoolean]){
            self.eat();
        }else 
        if self.current_matches(&C1Token::Identifier){

            if self.next_matches(&C1Token::LeftParenthesis){
                self.function_call()?;

            }else{
                self.eat();
            }

        }else{
            self.check_and_eat_token(&C1Token::LeftParenthesis, "\"(\" Expected")?;
            self.assignment()?;
            self.check_and_eat_token(&C1Token::RightParenthesis, "\")\" Expected")?;
        }
        Ok(())
    }

    /// Check whether the current token is equal to the given token. If yes, consume it, otherwise
    /// return an error with the given error message
    fn check_and_eat_token(&mut self, token: &C1Token, error_message: &str) -> ParseResult {
        if self.current_matches(token) {
            self.eat();
            Ok(())
        } else {
            Err(String::from(error_message))
        }
    }

    /// For each token in the given slice, check whether the token is equal to the current token,
    /// consume the current token, and check the next token in the slice against the next token
    /// provided by the lexer.
    fn check_and_eat_tokens(&mut self, token: &[C1Token], error_message: &str) -> ParseResult {
        match token
            .iter()
            .map(|t| self.check_and_eat_token(t, error_message))
            .filter(ParseResult::is_err)
            .last()
        {
            None => Ok(()),
            Some(err) => err,
        }
    }

    /// Check whether the given token matches the current token
    fn current_matches(&self, token: &C1Token) -> bool {
        match &self.current_token() {
            None => false,
            Some(current) => current == token,
        }
    }

    /// Check whether the given token matches the next token
    fn next_matches(&self, token: &C1Token) -> bool {
        match &self.peek_token() {
            None => false,
            Some(next) => next == token,
        }
    }

    /// Check whether any of the tokens matches the current token.
    fn any_match_current(&self, token: &[C1Token]) -> bool {
        token.iter().any(|t| self.current_matches(t))
    }

    /// Check whether any of the tokens matches the current token, then consume it
    fn any_match_and_eat(&mut self, token: &[C1Token], error_message: &String) -> ParseResult {
        if token
            .iter()
            .any(|t| self.check_and_eat_token(t, "").is_ok())
        {
            Ok(())
        } else {
            Err(String::from(error_message))
        }
    }

    fn error_message_current(&self, reason: &'static str) -> String {
        match self.current_token() {
            None => format!("{}. Reached EOF", reason),
            Some(_) => format!(
                "{} at line {:?} with text: '{}'",
                reason,
                self.current_line_number().unwrap(),
                self.current_text().unwrap()
            ),
        }
    }

    fn error_message_peek(&mut self, reason: &'static str) -> String {
        match self.peek_token() {
            None => format!("{}. Reached EOF", reason),
            Some(_) => format!(
                "{} at line {:?} with text: '{}'",
                reason,
                self.peek_line_number().unwrap(),
                self.peek_text().unwrap()
            ),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::{C1Parser, ParseResult};

    fn call_method<'a, F>(parse_method: F, text: &'static str) -> ParseResult
    where
        F: Fn(&mut C1Parser<'a>) -> ParseResult,
    {
        let mut parser = C1Parser::initialize_parser(text);
        if let Err(message) = parse_method(&mut parser) {
            eprintln!("Parse Error: {}", message);
            Err(message)
        } else {
            Ok(())
        }
    }

    #[test]
    fn parse_empty_program() {
        let result = C1Parser::parse("");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("   ");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("// This is a valid comment!");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("/* This is a valid comment!\nIn two lines!*/\n");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("  \n ");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn fail_invalid_program() {
        let result = C1Parser::parse("  bool  ");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("x = 0;");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("// A valid comment\nInvalid line.");
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn valid_function() {
        let result = C1Parser::parse("  void foo() {}  ");
        assert!(result.is_ok());

        let result = C1Parser::parse("int bar() {return 0;}");
        assert!(result.is_ok());

        let result = C1Parser::parse(
            "float calc() {\n\
        x = 1.0;
        y = 2.2;
        return x + y;
        \n\
        }",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn fail_invalid_function() {
        let result = C1Parser::parse("  void foo()) {}  ");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("const bar() {return 0;}");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse(
            "int bar() {
                                                          return 0;
                                                     int foo() {}",
        );
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse(
            "float calc(int invalid) {\n\
        x = 1.0;
        y = 2.2;
        return x + y;
        \n\
        }",
        );
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn valid_function_call() {
        assert!(call_method(C1Parser::function_call, "foo()").is_ok());
        assert!(call_method(C1Parser::function_call, "foo( )").is_ok());
        assert!(call_method(C1Parser::function_call, "bar23( )").is_ok());
    }

    #[test]
    fn fail_invalid_function_call() {
        assert!(call_method(C1Parser::function_call, "foo)").is_err());
        assert!(call_method(C1Parser::function_call, "foo").is_err());
        assert!(call_method(C1Parser::function_call, "foo{ )").is_err());
        assert!(call_method(C1Parser::function_call, "bar _foo( )").is_err());
    }

    #[test]
    fn valid_statement_list() {
        assert!(call_method(C1Parser::statement_list, "x = 4;").is_ok());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
            int y = 2.1;"
        )
            .is_ok());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
            {\
            foo();\n\
            }"
        )
            .is_ok());
        assert!(call_method(C1Parser::statement_list, "{x = 4;}\ny = 1;\nfoo();\n{}").is_ok());
    }

    #[test]
    fn fail_invalid_statement_list() {
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4\n\
        y = 2.1;"
        )
        .is_err());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
        {\
        foo();"
        )
        .is_err());
        assert!(call_method(C1Parser::statement_list, "{x = 4;\ny = 1;\nfoo;\n{}").is_err());
    }

    #[test]
    fn valid_if_statement() {
        assert!(call_method(C1Parser::if_statement, "if(x == 1) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(x == y) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(z) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(true) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(false) {}").is_ok());
    }

    #[test]
    fn fail_invalid_if_statement() {
        assert!(call_method(C1Parser::if_statement, "if(x == ) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if( == y) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if(> z) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if( {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if(false) }").is_err());
    }

    #[test]
    fn valid_return_statement() {
        assert!(call_method(C1Parser::return_statement, "return x").is_ok());
        assert!(call_method(C1Parser::return_statement, "return 1").is_ok());
        assert!(call_method(C1Parser::return_statement, "return").is_ok());
    }

    #[test]
    fn fail_invalid_return_statement() {
        assert!(call_method(C1Parser::return_statement, "1").is_err());
    }

    #[test]
    fn valid_printf_statement() {
        assert!(call_method(C1Parser::printf, " printf(a+b)").is_ok());
        assert!(call_method(C1Parser::printf, "printf( 1)").is_ok());
        assert!(call_method(C1Parser::printf, "printf(a - c)").is_ok());
    }

    #[test]
    fn fail_invalid_printf_statement() {
        assert!(call_method(C1Parser::printf, "printf( ").is_err());
        assert!(call_method(C1Parser::printf, "printf(printf)").is_err());
        assert!(call_method(C1Parser::printf, "Printf()").is_err());
    }

    #[test]
    fn valid_return_type() {
        assert!(call_method(C1Parser::return_type, "void").is_ok());
        assert!(call_method(C1Parser::return_type, "bool").is_ok());
        assert!(call_method(C1Parser::return_type, "int").is_ok());
        assert!(call_method(C1Parser::return_type, "float").is_ok());
    }

    #[test]
    fn valid_assignment() {
        assert!(call_method(C1Parser::assignment, "x = y").is_ok());
        assert!(call_method(C1Parser::assignment, "x =y").is_ok());
        assert!(call_method(C1Parser::assignment, "1 + 2").is_ok());
    }

    #[test]
    fn valid_stat_assignment() {
        assert!(call_method(C1Parser::stat_assignment, "x = y").is_ok());
        assert!(call_method(C1Parser::stat_assignment, "x =y").is_ok());
        assert!(call_method(C1Parser::stat_assignment, "x =y + t").is_ok());
    }

    #[test]
    fn valid_factor() {
        assert!(call_method(C1Parser::factor, "4").is_ok());
        assert!(call_method(C1Parser::factor, "1.2").is_ok());
        assert!(call_method(C1Parser::factor, "true").is_ok());
        assert!(call_method(C1Parser::factor, "foo()").is_ok());
        assert!(call_method(C1Parser::factor, "x").is_ok());
        assert!(call_method(C1Parser::factor, "(x + y)").is_ok());
    }

    #[test]
    fn fail_invalid_factor() {
        assert!(call_method(C1Parser::factor, "if").is_err());
        assert!(call_method(C1Parser::factor, "(4").is_err());
        assert!(call_method(C1Parser::factor, "bool").is_err());
    }

    #[test]
    fn multiple_functions() {
        assert!(call_method(
            C1Parser::program,
            "void main() { hello();}\nfloat bar() {return 1.0;}"
        )
        .is_ok());
    }
}

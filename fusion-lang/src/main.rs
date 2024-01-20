use crate::ast::Ast;
use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;

mod ast;

fn main() {
    let input : &str = "7+";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);
    let mut ast = Ast::new();
    let mut parser = Parser::from_tokens(
        tokens
    );
    while let Some(stmt) = parser.next_statement() {
        ast.add_statement(stmt);
    }
    ast.visualize();
}

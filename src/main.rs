mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let source = "1 + 2 * (3 - 4)";
    println!("Rynix v0.1.0 — parsing: {}", source);

    let mut lex = Lexer::new(source);
    let tokens = match lex.tokenize() {
        Ok(tokens) => tokens,
        Err(message) => {
            println!("Lex error: {}", message);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => println!("AST: {:#?}", expr),
        Err(message) => println!("Parse error: {}", message),
    }
}
mod interpreter;
mod lexer;
mod parser;

use interpreter::eval::eval;
use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let source = "1 + 2 * (3 - 4)";
    println!("Rynix v0.1.0 — evaluating: {}", source);

    let mut lex = Lexer::new(source);
    let tokens = match lex.tokenize() {
        Ok(tokens) => tokens,
        Err(message) => {
            println!("Lex error: {}", message);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(expr) => expr,
        Err(message) => {
            println!("Parse error: {}", message);
            return;
        }
    };

    match eval(&expr) {
        Ok(result) => println!("Result: {}", result),
        Err(message) => println!("Runtime error: {}", message),
    }
}
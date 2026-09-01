mod interpreter;
mod lexer;
mod parser;

use interpreter::eval::run;
use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let source = "let x = 5\nx = x + 10\nx * 2";
    println!("Rynix v0.1.0 — running program:\n{}\n", source);

    let mut lex = Lexer::new(source);
    let tokens = match lex.tokenize() {
        Ok(tokens) => tokens,
        Err(message) => {
            println!("Lex error: {}", message);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(message) => {
            println!("Parse error: {}", message);
            return;
        }
    };

    match run(&program) {
        Ok(Some(value)) => println!("Result: {}", value),
        Ok(None) => println!("(program produced no value)"),
        Err(message) => println!("Runtime error: {}", message),
    }
}
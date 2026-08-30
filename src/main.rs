mod lexer;

use lexer::lexer::Lexer;

fn main() {
    let source = "let count = 42 ~ a friendly comment ~";
    println!("Rynix v0.1.0 — tokenizing: {}", source);

    let mut lex = Lexer::new(source);
    match lex.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(message) => {
            println!("Lex error: {}", message);
        }
    }
}
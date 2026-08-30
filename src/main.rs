mod lexer;

use lexer::lexer::Lexer;

fn main() {
    let source = "1 + 2 * (3 - 4)";
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
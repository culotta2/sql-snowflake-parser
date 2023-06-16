mod lexer;
use lexer::lexer::{Lexer, Token};

fn main() {
    // Load in file
    let input = std::fs::read_to_string("input.sql").expect("Failed to read file");
    let lexer = Lexer::new(input.into());
    let tokens: Vec<Token> = lexer.get_tokens().into_iter().flatten().collect();

    for token in tokens {
        println!("{:?}", token);
    }
}

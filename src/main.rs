mod lexer;
mod parser;

use crate::parser::parser::Parser;

fn main() {
   let parser = Parser::new("input.sql".into()); 

   for token in parser.tokens {
      println!("{:?}", token);
   }
}

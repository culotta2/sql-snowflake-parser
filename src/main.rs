mod lexer;
mod parser;

// use parser::parser::Column;

use crate::parser::parser::Parser;


fn main() {
    let parser = Parser::new("input.sql".into()); 

    for tok in parser.tokens {
        println!("{:?}", tok);
    }

    // // let mut returned_cols = parser.get_selected_columns();
    // for col in returned_cols.iter() {
    //     println!("{:?}", col);
    // }

}

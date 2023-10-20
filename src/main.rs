mod lexer;
mod parser;

// use parser::parser::Column;

use crate::parser::parser::Parser;


fn main() {
    let parser = Parser::new("input.sql".into()); 

    let _returned_cols = &parser.get_selected_columns();

}

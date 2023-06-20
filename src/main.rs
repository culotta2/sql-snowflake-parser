mod lexer;
mod parser;

use parser::parser::Column;

use crate::parser::parser::Parser;


fn main() {
    let parser = Parser::new("small.sql".into()); 

    let columns = vec![
        Column::new("car".to_string()),
        Column::new("total_price".to_string()),
        Column::new("total_count".to_string()),
    ];

    let returned_cols = &parser.get_selected_columns()[0];

    for (column, expected_column) in returned_cols.iter().zip(columns.iter()) {
        println!("Expected: {:?}, Actual: {:?}", expected_column, column);
        assert_eq!(expected_column, column);
    }
}

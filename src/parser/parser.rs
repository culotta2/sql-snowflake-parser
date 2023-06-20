// use anyhow::Result;

use crate::lexer::lexer::{DMLKeyword, DDLKeyword, Lexer, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    name: String,
    // table: Option<String>,
    // alias: Option<String>,
}

impl Column {
    pub fn new(name: String) -> Self {
        Column {
            name,
        }
    }

    // pub fn set_table(&mut self, table: String) {
    //     self.table = Some(table);
    // }
    //
    // pub fn set_alias(&mut self, alias: String) {
    //     self.table = Some(alias);
    // }
}

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
     pub fn new(file_path: String) -> Self {
        let input = std::fs::read_to_string(file_path).expect("Failed to read file");
        let lexer = Lexer::new(input.into());
        let tokens: Vec<Token> = lexer.get_tokens().into_iter().flatten().collect();

        Parser { tokens }
     }

     pub fn get_selected_columns(&self) -> Vec<Vec<Column>> {
         let mut all_columns = Vec::new();
         let mut current_columns = Vec::new();
         let mut current_column: Option<Column> = None;

         let mut in_select = false;
         let mut in_function = false;
         let mut paren_count = 0;

         for token in &self.tokens {
             match (in_select, in_function, token) {
                 (false, _, &Token::DML(DMLKeyword::Select)) => {
                     in_select = true;
                 },
                 (true, _,  &Token::From) => {
                     if let Some(ref column) = current_column {
                         current_columns.push(column.clone());
                     }
                     all_columns.push(current_columns.clone());
                     current_columns.clear();
                     in_select = false;
                 },
                 (true, _, &Token::ColumnFunction(_)) => {
                     in_function = true;
                 },
                 (true, true, &Token::OpenParen) => {
                     paren_count += 1;
                 }
                 (true, true, &Token::CloseParen) => {
                     paren_count -= 1;
                     if paren_count == 0 {
                         in_function = false;
                     }
                 },
                 (true, false, &Token::Ident(ref name)) => {
                     current_column = Some(Column::new(name.clone()));
                 },
                 (true, false, &Token::Comma) => {
                     if let Some(ref column) = current_column {
                         current_columns.push(column.clone());
                     }
                 },
                 _ => (),
             }
         }

        return all_columns;
     }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Parser, Column};

    #[test]
    fn assert_finds_all_columns() -> Result<()> {
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

        Ok(())
    }
}

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

struct ParserState {
    in_select: bool,
    in_function: bool,
    paren_count: usize,
    all_columns: Vec<Vec<Column>>,
    current_columns: Vec<Column>,
    current_column: Option<Column>,
}

impl ParserState {
    fn new() -> Self {
        ParserState {
            in_select: false,
            in_function: false,
            paren_count: 0,
            all_columns: Vec::new(),
            current_columns: Vec::new(),
            current_column: None,
        }
    }

    fn enter_select(&mut self) {
        self.in_select = true;
    }

    fn exit_select(&mut self) {
        self.in_select = false;
        if let Some(column) = self.current_column.take() {
            self.current_columns.push(column);
        }
        if !self.current_columns.is_empty() {
            self.all_columns.push(std::mem::take(&mut self.current_columns));
        }
    }

    fn enter_function(&mut self) {
        self.in_function = true;
    }

    fn exit_function(&mut self) {
        self.in_function = false;
    }

    fn open_paren(&mut self) {
        if self.in_function {
            self.paren_count += 1;
        }
    }

    fn close_paren(&mut self) {
        if self.in_function {
            self.paren_count -= 1;
            if self.paren_count == 0 {
                self.exit_function();
            }
        }
    }

    fn add_column(&mut self, name: String) {
        if self.in_select && !self.in_function {
            self.current_column = Some(Column::new(name));
        }
    }

    fn add_comma(&mut self) {
        if let Some(column) = self.current_column.take() {
            self.current_columns.push(column);
        }
    }
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
         let mut state = ParserState::new();

         for token in &self.tokens {
             match token {
                 Token::DML(DMLKeyword::Select) => {
                     state.enter_select();
                 },
                 Token::From => {
                     state.exit_select();
                 },
                 Token::ColumnFunction(_) => {
                     state.enter_function();
                 },
                 Token::OpenParen => {
                     state.open_paren();
                 },
                 Token::CloseParen => {
                     state.close_paren();
                 },
                 Token::Ident(name) => {
                     state.add_column(name.clone());
                 },
                 Token::Comma => {
                     state.add_comma();
                 },
                 _ => {},
             }
         }

         state.all_columns
     }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Parser, Column};

    #[test]
    fn assert_finds_all_columns() -> Result<()> {
        let parser = Parser::new("input.sql".into()); 

        // let columns = vec![
        //     Column::new("id".to_string()),
        //     Column::new("name".to_string()),
        //     Column::new("name".to_string()),
        //     Column::new("name".to_string()),
        //     Column::new("name".to_string()),
        //     Column::new("name".to_string()),
        // ];

        let returned_cols = &parser.get_selected_columns()[0];

        for column in returned_cols.iter() {
            println!("{:?}", column);
        }


        // for (column, expected_column) in returned_cols.iter().zip(columns.iter()) {
        //     println!("Expected: {:?}, Actual: {:?}", expected_column, column);
        //     // assert_eq!(expected_column, column);
        // }

        None.unwrap()

    }
}

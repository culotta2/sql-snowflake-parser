// use anyhow::Result;

use crate::lexer::lexer::{Lexer, Token};
use crate::lexer::lexer::{DMLKeyword, DDLKeyword};

struct Column {
    name: String,
    table: Option<String>,
    alias: Option<String>,
}

impl Column {
    pub fn new(name: String) -> Self {
        Column {
            name, table: None, alias: None,
        }
    }

    pub fn set_table(&mut self, table: String) {
        self.table = Some(table);
    }

    pub fn set_alias(&mut self, alias: String) {
        self.table = Some(alias);
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
         let mut in_select = false;
         let mut in_function = false;
         let mut paren_count = 0;

         for token in &self.tokens {
             match (in_select, in_function, token) {
                 (false, _, &Token::DML(DMLKeyword::Select)) => {
                     in_select = true;
                 },
                 (true, _,  &Token::From) => {
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
                 }
             }
         }

        return vec![vec![Column::new("id".to_string())]]; }
}

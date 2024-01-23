// use anyhow::Result;

use crate::lexer::lexer::{DDLKeyword, DMLKeyword, Lexer, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    name: String,
}

impl Column {
    pub fn new(name: String) -> Self {
        Column { name }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedColumns {
    table: String,
    cte: bool,
    columns: Vec<Column>,
}

impl SelectedColumns {
    pub fn new(table: String, columns: Vec<Column>, cte: bool) -> Self {
        SelectedColumns {
            table,
            columns,
            cte,
        }
    }
}

struct ParserState {
    in_select: bool,
    in_function: bool,
    in_object_creation: bool,
    in_cte: bool,
    paren_count: usize,
    function_paren_count: usize,
    current_columns: Vec<Column>,
}

impl ParserState {
    fn new() -> Self {
        ParserState {
            in_select: false,
            in_function: false,
            in_object_creation: false,
            in_cte: false,
            paren_count: 0,
            function_paren_count: 0,
            current_columns: Vec::new(),
        }
    }

    fn enter_select(&mut self) {
        self.in_select = true;
        self.current_columns.clear();
    }

    fn exit_select(&mut self) {
        self.in_select = false;
    }

    fn enter_function(&mut self) {
        self.in_function = true;
    }

    fn exit_function(&mut self) {
        self.in_function = false;
    }

    fn open_paren(&mut self) {
        self.paren_count += 1;
        if self.in_function {
            self.function_paren_count += 1;
        }
    }

    fn close_paren(&mut self) {
        self.paren_count -= 1;
        if self.in_function {
            self.function_paren_count -= 1;
            if self.function_paren_count == 0 {
                self.exit_function();
            }
        }
    }

    fn add_column(&mut self, name: String) {
        if self.in_select && !self.in_function {
            self.current_columns.push(Column::new(name));
        }
    }

    fn enter_object_creation(&mut self) {
        self.in_object_creation = true;
    }

    fn enter_cte(&mut self) {
        self.in_cte = true;
    }

    fn update_selected_columns(
        &mut self,
        selected_columns: &mut Vec<SelectedColumns>,
        table_name: &Option<String>,
    ) {
        if let Some(table_name) = table_name {
            selected_columns.push(SelectedColumns::new(
                table_name.to_owned(),
                self.current_columns.clone(),
                self.in_cte,
            ));
        } else {
            selected_columns.push(SelectedColumns::new(
                "".to_string(),
                self.current_columns.clone(),
                false,
            ));
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

    pub fn get_selected_columns(&self) -> Vec<SelectedColumns> {
        let mut state = ParserState::new();
        let mut selected_columns = Vec::new();
        let mut current_table: Option<String> = None;
        let mut current_cte: Option<String> = None;
        let mut tokens_iter = self.tokens.iter().peekable();

        while let Some(token) = tokens_iter.next() {
            let next_token = tokens_iter.peek().unwrap_or(&&Token::EOF);

            match token {
                Token::EOF => {
                    break;
                }
                Token::Ident(col)
                    if state.in_select && !state.in_function && next_token != &&Token::Period =>
                {
                    state.add_column(col.clone());
                }
                Token::Comma
                    if state.in_cte && state.function_paren_count == 0 && !state.in_select =>
                {
                    current_cte = if let Token::Ident(s) = next_token {
                        Some(s.clone())
                    } else {
                        None
                    };
                }
                Token::Table | Token::View => {
                    if let Token::Ident(table_name) = next_token {
                        current_table = Some(table_name.clone());
                    }
                }
                Token::DDL(DDLKeyword::With) => {
                    if let Token::Ident(table_name) = next_token {
                        current_cte = Some(table_name.clone());
                        state.enter_cte()
                    }
                }
                Token::DDL(DDLKeyword::Create) => {
                    state.enter_object_creation();
                }
                Token::DML(DMLKeyword::Select) => {
                    state.enter_select();
                }
                Token::From => {
                    if current_cte.is_some() {
                        state.update_selected_columns(&mut selected_columns, &current_cte.clone());
                        current_cte = None;
                    } else {
                        state.in_cte = false;
                        state.update_selected_columns(&mut selected_columns, &current_table);
                    }
                    state.exit_select();
                }
                Token::ColumnFunction(_) | Token::Over => {
                    state.enter_function();
                }
                Token::OpenParen => {
                    state.open_paren();
                }
                Token::CloseParen => {
                    state.close_paren();
                }
                _ => {}
            }
        }

        selected_columns
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Column, Parser, SelectedColumns};

    #[test]
    fn assert_finds_all_columns() -> Result<()> {
        let parser = Parser::new("scripts/input.sql".into());

        let columns = {
            vec![
                SelectedColumns::new(
                    "EMPLOYEES".to_string(),
                    vec![
                        Column::new("ID".to_string()),
                        Column::new("NAME".to_string()),
                    ],
                    true,
                ),
                SelectedColumns::new(
                    "SALARIES".to_string(),
                    vec![
                        Column::new("ID".to_string()),
                        Column::new("SALARY_RANK".to_string()),
                    ],
                    true,
                ),
                SelectedColumns::new(
                    "COMBINED_TABLE".to_string(),
                    vec![
                        Column::new("ID".to_string()),
                        Column::new("NAME".to_string()),
                        Column::new("SALARY_RANK".to_string()),
                    ],
                    false,
                ),
            ]
        };

        let returned_cols = &parser.get_selected_columns();

        for (column, expected_column) in returned_cols.iter().zip(columns.iter()) {
            println!("Expected: {:?}, Actual: {:?}", expected_column, column);
            assert_eq!(expected_column, column);
        }

        Ok(())
    }
}

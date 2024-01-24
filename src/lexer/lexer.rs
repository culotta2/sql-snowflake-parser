use anyhow::Result;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Ident(String),

    // Literals
    Int(i64),
    Float(f64),
    Bool(bool),
    // Date(String),
    Varchar(String),
    Null,

    // Data Language
    DDL(DDLKeyword),
    DML(DMLKeyword),

    // Data Types
    DataType(DataType),

    // Join Types
    JoinType(JoinType),

    // Functions
    ColumnFunction(Function),

    // Logical
    Logical(Logical),

    // Operators
    Assign,
    Asterisk,
    Minus,
    Modulo,
    Plus,
    Slash,

    // Delimiters
    OpenParen,
    CloseParen,
    Comma,
    Period,
    Semicolon,
    Colon,
    Dollar,
    DollarDelimiter,
    SingleQuote,
    DoubleQuote,
    ExclamationPoint,

    // Comments
    BlockComment(String),
    InlineComment(String),

    // Comparisons
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    NotEqual,

    // Keywords
    All,
    As,
    Between,
    By,
    Caller,
    Case,
    Distinct,
    Except,
    Execute,
    From,
    Function,
    Group,
    Having,
    Join,
    Language,
    Like,
    Limit,
    On,
    Order,
    Over,
    Procedure,
    Return,
    Returns,
    Set,
    Temporary,
    Top,
    Union,
    When,
    Where,

    // Objects
    View,
    Materialized,
    Table,

    // End of file
    EOF,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum DDLKeyword {
    Alter,
    Create,
    Drop,
    Rename,
    Replace,
    Truncate,
    With,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum DMLKeyword {
    Call,
    Delete,
    Insert,
    Select,
    Update,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Avg,
    Cast,
    Concat,
    Count,
    Max,
    Min,
    Sum,

    DenseRank,
    Lag,
    Lead,
    Rank,
    RowNumber,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Outer,
    Natural,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Logical {
    And,
    Or,
    Not,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Number,
    Int,
    BigInt,
    SmallInt,
    TinyInt,
    ByteInt,
    Float,   // can be called with FLOAT, FLOAT4, FLOAT8, DOUBLE, DOUBLE PRECISION, REAL
    Varchar, // can be called with STRING, TEXT, NVARCHAR, NVARCHAR2, CHAR VARYING, NCHAR VARYING
    Char,    // can be called with CHAR, CHARACTER, NCHAR
    Binary,  // can be called with BINARY, VARBINARY
    Boolean,
    // Date, Time, TimestampLTZ, TimestampTZ, TimestampNTZ - TODO: Add date/time types
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };

        lexer.read_char();
        return lexer;
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            b'+' => Token::Plus,
            b'%' => Token::Modulo,
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            b'-' => {
                if self.peek_char() == b'-' {
                    Token::InlineComment(self.read_inline_comment())
                } else {
                    Token::Minus
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::ExclamationPoint
                }
            }
            b'*' => Token::Asterisk,
            b'=' => Token::Equal,
            b'/' => {
                if self.peek_char() == b'*' {
                    Token::BlockComment(self.read_block_comment())
                } else if self.peek_char() == b'/' {
                    Token::InlineComment(self.read_inline_comment())
                } else {
                    Token::Slash
                }
            }

            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b',' => Token::Comma,
            b'.' => Token::Period,
            b';' => Token::Semicolon,
            b'$' => {
                if self.peek_char() == b'$' {
                    self.read_char();
                    Token::DollarDelimiter
                } else {
                    Token::Dollar
                }
            }
            b':' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Assign
                } else {
                    Token::Colon
                }
            }
            b'\'' => Token::Varchar(self.read_varchar()),
            b'"' => Token::DoubleQuote,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                let lower_ident = ident.clone().to_lowercase();
                let ident_type = self.string_to_token(&lower_ident);
                return match ident_type {
                    Some(ident_type) => Ok(ident_type),
                    None => Ok(Token::Ident(ident)),
                };
            }
            b'0'..=b'9' => {
                let potential_number = self.read_number();
                let ret_tok = match &potential_number.chars().filter(|ch| ch == &'.').count() {
                    0 => Token::Int(potential_number.parse().expect("Expected an int value")),
                    1 => Token::Float(potential_number.parse().expect("Expected an float value")),
                    _ => Token::Illegal,
                };

                return Ok(ret_tok);
            }
            0 => Token::EOF,
            _ => todo!(),
        };

        self.read_char();
        return Ok(token);
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[start..self.position]).to_string();
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[start..self.position]).to_string();
    }

    fn read_inline_comment(&mut self) -> String {
        let start = self.position;
        while self.ch != b'\n' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[start..self.position]).to_string();
    }

    fn read_block_comment(&mut self) -> String {
        let start = self.position;
        loop {
            if self.ch == b'*' && self.peek_char() == b'/' {
                self.read_char();
                self.read_char();
                break;
            }

            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[start..self.position]).to_string();
    }

    fn string_to_token(&self, ident: &str) -> Option<Token> {
        let returned_tok = match ident {
            // DDL
            "alter" => Some(Token::DDL(DDLKeyword::Alter)),
            "create" => Some(Token::DDL(DDLKeyword::Create)),
            "drop" => Some(Token::DDL(DDLKeyword::Drop)),
            "replace" => Some(Token::DDL(DDLKeyword::Replace)),
            "rename" => Some(Token::DDL(DDLKeyword::Rename)),
            "truncate" => Some(Token::DDL(DDLKeyword::Truncate)),
            "with" => Some(Token::DDL(DDLKeyword::With)),

            // DML
            "call" => Some(Token::DML(DMLKeyword::Call)),
            "delete" => Some(Token::DML(DMLKeyword::Delete)),
            "insert" => Some(Token::DML(DMLKeyword::Insert)),
            "select" => Some(Token::DML(DMLKeyword::Select)),
            "update" => Some(Token::DML(DMLKeyword::Update)),

            // Column Functions
            "avg" => Some(Token::ColumnFunction(Function::Avg)),
            "cast" => Some(Token::ColumnFunction(Function::Cast)),
            "concat" => Some(Token::ColumnFunction(Function::Concat)),
            "count" => Some(Token::ColumnFunction(Function::Count)),
            "dense_rank" => Some(Token::ColumnFunction(Function::DenseRank)),
            "lag" => Some(Token::ColumnFunction(Function::Lag)),
            "lead" => Some(Token::ColumnFunction(Function::Lead)),
            "max" => Some(Token::ColumnFunction(Function::Max)),
            "min" => Some(Token::ColumnFunction(Function::Min)),
            "rank" => Some(Token::ColumnFunction(Function::Rank)),
            "row_number" => Some(Token::ColumnFunction(Function::RowNumber)),
            "sum" => Some(Token::ColumnFunction(Function::Sum)),

            // Keywords
            "all" => Some(Token::All),
            "as" => Some(Token::As),
            "between" => Some(Token::Between),
            "by" => Some(Token::By),
            "caller" => Some(Token::Caller),
            "case" => Some(Token::Case),
            "distinct" => Some(Token::Distinct),
            "except" => Some(Token::Except),
            "execute" => Some(Token::Execute),
            "from" => Some(Token::From),
            "function" => Some(Token::Function),
            "group" => Some(Token::Group),
            "having" => Some(Token::Having),
            "join" => Some(Token::Join),
            "language" => Some(Token::Language),
            "like" => Some(Token::Like),
            "limit" => Some(Token::Limit),
            "on" => Some(Token::On),
            "order" => Some(Token::Order),
            "over" => Some(Token::Over),
            "procedure" => Some(Token::Procedure),
            "return" => Some(Token::Return),
            "returns" => Some(Token::Returns),
            "set" => Some(Token::Set),
            "temp" | "temporary" => Some(Token::Temporary),
            "top" => Some(Token::Top),
            "union" => Some(Token::Union),
            "when" => Some(Token::When),
            "where" => Some(Token::Where),

            // Objects
            "materialized" => Some(Token::View),
            "table" => Some(Token::Table),
            "view" => Some(Token::View),

            // Booleans
            "true" => Some(Token::Bool(true)),
            "false" => Some(Token::Bool(false)),

            // Join types
            "inner" => Some(Token::JoinType(JoinType::Inner)),
            "left" => Some(Token::JoinType(JoinType::Left)),
            "right" => Some(Token::JoinType(JoinType::Right)),
            "outer" => Some(Token::JoinType(JoinType::Outer)),
            "natural" => Some(Token::JoinType(JoinType::Natural)),

            // Logicals
            "and" => Some(Token::Logical(Logical::And)),
            "or" => Some(Token::Logical(Logical::Or)),
            "not" => Some(Token::Logical(Logical::Not)),

            // Data types
            "int" | "integer" => Some(Token::DataType(DataType::Int)),
            "bigint" => Some(Token::DataType(DataType::BigInt)),
            "smallint" => Some(Token::DataType(DataType::SmallInt)),
            "tinyint" => Some(Token::DataType(DataType::TinyInt)),
            "byteint" => Some(Token::DataType(DataType::ByteInt)),
            "number" => Some(Token::DataType(DataType::Number)),
            "float" | "float4" | "float8" | "double" | "real" => {
                Some(Token::DataType(DataType::Float))
            }
            "varchar" | "string" | "text" | "nvarchar" | "nvarchar2" => {
                Some(Token::DataType(DataType::Varchar))
            }
            "char" | "character" | "nchar" => Some(Token::DataType(DataType::Char)),
            "binary" | "varbinary" => Some(Token::DataType(DataType::Binary)),
            "boolean" => Some(Token::DataType(DataType::Boolean)),

            // Null
            "null" => Some(Token::Null),

            _ => None,
        };

        return returned_tok;
    }

    fn read_varchar(&mut self) -> String {
        let start = self.position;
        self.read_char();
        while self.ch != b'\'' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[start..=self.position]).to_string();
    }

    fn peek_char(&self) -> u8 {
        if let Some(c) = self.input.get(self.read_position) {
            return *c;
        } else {
            return 0;
        }
    }

    pub fn get_tokens(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut next_token = self.next_token()?;
        while next_token != Token::EOF {
            tokens.push(next_token);
            next_token = self.next_token()?;
        }

        return Ok(tokens);
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{DDLKeyword, DMLKeyword, DataType, Function, Lexer, Logical, Token};

    #[test]
    fn assert_basic_string_match() -> Result<()> {
        let input = r#"
-- Create a temporary table
CREATE TEMPORARY TABLE temp_table (
  id INT,
  name VARCHAR,
  age INT
);

-- Create a stored procedure
CREATE OR REPLACE PROCEDURE my_stored_procedure()
RETURNS VARCHAR
LANGUAGE SQL
AS
$$
  -- SQL code goes here
  -- ...
  RETURN 'Stored procedure executed successfully';
$$;

-- Create a function
CREATE OR REPLACE FUNCTION my_function(arg INT)
RETURNS TABLE (id INT, name VARCHAR)
LANGUAGE SQL
AS
$$
  -- SQL code goes here
  -- ...
  RETURN 'Stored procedure executed successfully';
$$;

-- Use a CTE to query data
WITH cte AS (
  SELECT id, name, salary
  FROM temp_table
  WHERE age > 30
)
SELECT name, sum(salary)
FROM cte
group by name
;

-- Call the stored procedure
CALL my_stored_procedure();

-- Call the function
SELECT *
FROM TABLE(my_function(123));
            "#;

        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::InlineComment("-- Create a temporary table".to_string()),
            Token::DDL(DDLKeyword::Create),
            Token::Temporary,
            Token::Table,
            Token::Ident("temp_table".to_string()),
            Token::OpenParen,
            Token::Ident("id".to_string()),
            Token::DataType(DataType::Int),
            Token::Comma,
            Token::Ident("name".to_string()),
            Token::DataType(DataType::Varchar),
            Token::Comma,
            Token::Ident("age".to_string()),
            Token::DataType(DataType::Int),
            Token::CloseParen,
            Token::Semicolon,
            Token::InlineComment("-- Create a stored procedure".to_string()),
            Token::DDL(DDLKeyword::Create),
            Token::Logical(Logical::Or),
            Token::DDL(DDLKeyword::Replace),
            Token::Procedure,
            Token::Ident("my_stored_procedure".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Returns,
            Token::DataType(DataType::Varchar),
            Token::Language,
            Token::Ident("SQL".to_string()),
            Token::As,
            Token::DollarDelimiter,
            Token::InlineComment("-- SQL code goes here".to_string()),
            Token::InlineComment("-- ...".to_string()),
            Token::Return,
            Token::Varchar("'Stored procedure executed successfully'".to_string()),
            Token::Semicolon,
            Token::DollarDelimiter,
            Token::Semicolon,
            Token::InlineComment("-- Create a function".to_string()),
            Token::DDL(DDLKeyword::Create),
            Token::Logical(Logical::Or),
            Token::DDL(DDLKeyword::Replace),
            Token::Function,
            Token::Ident("my_function".to_string()),
            Token::OpenParen,
            Token::Ident("arg".to_string()),
            Token::DataType(DataType::Int),
            Token::CloseParen,
            Token::Returns,
            Token::Table,
            Token::OpenParen,
            Token::Ident("id".to_string()),
            Token::DataType(DataType::Int),
            Token::Comma,
            Token::Ident("name".to_string()),
            Token::DataType(DataType::Varchar),
            Token::CloseParen,
            Token::Language,
            Token::Ident("SQL".to_string()),
            Token::As,
            Token::DollarDelimiter,
            Token::InlineComment("-- SQL code goes here".to_string()),
            Token::InlineComment("-- ...".to_string()),
            Token::Return,
            Token::Varchar("'Stored procedure executed successfully'".to_string()),
            Token::Semicolon,
            Token::DollarDelimiter,
            Token::Semicolon,
            Token::InlineComment("-- Use a CTE to query data".to_string()),
            Token::DDL(DDLKeyword::With),
            Token::Ident("cte".to_string()),
            Token::As,
            Token::OpenParen,
            Token::DML(DMLKeyword::Select),
            Token::Ident("id".to_string()),
            Token::Comma,
            Token::Ident("name".to_string()),
            Token::Comma,
            Token::Ident("salary".to_string()),
            Token::From,
            Token::Ident("temp_table".to_string()),
            Token::Where,
            Token::Ident("age".to_string()),
            Token::GreaterThan,
            Token::Int(30),
            Token::CloseParen,
            Token::DML(DMLKeyword::Select),
            Token::Ident("name".to_string()),
            Token::Comma,
            Token::ColumnFunction(Function::Sum),
            Token::OpenParen,
            Token::Ident("salary".to_string()),
            Token::CloseParen,
            Token::From,
            Token::Ident("cte".to_string()),
            Token::Group,
            Token::By,
            Token::Ident("name".to_string()),
            Token::Semicolon,
            Token::InlineComment("-- Call the stored procedure".to_string()),
            Token::DML(DMLKeyword::Call),
            Token::Ident("my_stored_procedure".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Semicolon,
            Token::InlineComment("-- Call the function".to_string()),
            Token::DML(DMLKeyword::Select),
            Token::Asterisk,
            Token::From,
            Token::Table,
            Token::OpenParen,
            Token::Ident("my_function".to_string()),
            Token::OpenParen,
            Token::Int(123),
            Token::CloseParen,
            Token::CloseParen,
            Token::Semicolon,
            Token::EOF,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, recieved: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        Ok(())
    }
}

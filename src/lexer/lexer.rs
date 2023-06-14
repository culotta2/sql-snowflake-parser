use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
    // All,
    As,
    Between,
    By,
    Caller,
    Case,
    Distinct,
    Except,
    // Execute,
    From,
    Function,
    Group,
    Having,
    Join,
    // Language,
    Limit,
    On,
    Order,
    Over,
    Procedure,
    Return,
    Set,
    Top,
    Union,
    When,
    Where,
    With,

    // End of file
    EOF,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum DDLKeyword {
    Alter,
    Create,
    Drop,
    Rename,
    Replace,
    Truncate,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum DMLKeyword {
    Call,
    Delete,
    Insert,
    Select,
    Update,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Outer,
    Natural,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Logical {
    And,
    Or,
    Not,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Self {
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
            },
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            },
            b'-' => {
                if self.peek_char() == b'-' {
                    Token::InlineComment(self.read_inline_comment())
                } else {
                    Token::Minus
                }
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::ExclamationPoint
                }
            },
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
            },

            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b',' => Token::Comma,
            b'.' => Token::Period,
            b';' => Token::Semicolon,
            b'$' => Token::Dollar,
            b':' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Assign
                } else {
                    Token::Colon
                }
            },
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
            },
            b'0'..=b'9' => {
                let potential_number = self.read_number();
                let ret_tok = match &potential_number
                    .chars()
                    .filter(|ch| ch == &'.')
                    .count() 
                {
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
        while self.ch.is_ascii_whitespace(){
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

    fn string_to_token(&self, ident: &str) -> Option<Token>{
        let returned_tok = match ident {
            // DDL
            "alter" => Some(Token::DDL(DDLKeyword::Alter)),
            "create" => Some(Token::DDL(DDLKeyword::Create)),
            "drop" => Some(Token::DDL(DDLKeyword::Drop)),
            "replace" => Some(Token::DDL(DDLKeyword::Replace)),
            "rename" => Some(Token::DDL(DDLKeyword::Rename)),
            "truncate" => Some(Token::DDL(DDLKeyword::Truncate)),

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
            "max" => Some(Token::ColumnFunction(Function::Max)),
            "min" => Some(Token::ColumnFunction(Function::Min)),
            "sum" => Some(Token::ColumnFunction(Function::Sum)),

            // Window Functions
            "dense_rank" => Some(Token::ColumnFunction(Function::DenseRank)),
            "lag" => Some(Token::ColumnFunction(Function::Lag)),
            "lead" => Some(Token::ColumnFunction(Function::Lead)),
            "rank" => Some(Token::ColumnFunction(Function::Rank)),
            "row_number" => Some(Token::ColumnFunction(Function::RowNumber)),

            // Keywords
            "as" => Some(Token::As),
            "between" => Some(Token::Between),
            "by" => Some(Token::By),
            "case" => Some(Token::Case),
            "caller" => Some(Token::Caller),
            "distinct" => Some(Token::Distinct),
            "except" => Some(Token::Except),
            "from" => Some(Token::From),
            "function" => Some(Token::Function),
            "group" => Some(Token::Group),
            "having" => Some(Token::Having),
            "join" => Some(Token::Join),
            "limit" => Some(Token::Limit),
            "on" => Some(Token::On),
            "order" => Some(Token::Order),
            "over" => Some(Token::Over),
            "procedure" => Some(Token::Procedure),
            "return" => Some(Token::Return),
            "set" => Some(Token::Set),
            "top" => Some(Token::Top),
            "union" => Some(Token::Union),
            "when" => Some(Token::When),
            "where" => Some(Token::Where),
            "with" => Some(Token::With),

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
            "or" => Some(Token::Logical(Logical::And)),
            "not" => Some(Token::Logical(Logical::And)),

            _ => None,
        };

        return returned_tok;

    }

    fn read_varchar(&mut self) -> String {
        let start = self.position;
        println!("{:?}", self.ch as char);
        self.read_char();
        while self.ch != b'\'' {
            self.read_char();
            println!("{}", self.ch as char);
        }
        return String::from_utf8_lossy(&self.input[start..=self.position]).to_string();
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }
}
    
#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Token, Lexer, Function, DDLKeyword, DMLKeyword, Logical} ;

    #[test]
    fn assert_basic_string_match() -> Result<()> {
        let input = r#"
with my_table (
    name,
    age,
    year,
    salary
) as (
    select distinct
        emp.name,
        emp.age,
        info.year,
        info.salary
    from employees emp
        join information as info
            on emp.emp_id = info.emp_id
    where emp.employed = TRUE
    and emp.salary > 50000.00
)
;
            "#;

        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::With,
            Token::Ident("my_table".to_string()),
            Token::OpenParen,
            Token::Ident("name".to_string()),
            Token::Comma,
            Token::Ident("age".to_string()),
            Token::Comma,
            Token::Ident("year".to_string()),
            Token::Comma,
            Token::Ident("salary".to_string()),
            Token::CloseParen,
            Token::As,
            Token::OpenParen,
            Token::DML(DMLKeyword::Select),
            Token::Distinct,
            Token::Ident("emp".to_string()),
            Token::Period,
            Token::Ident("name".to_string()),
            Token::Comma,
            Token::Ident("emp".to_string()),
            Token::Period,
            Token::Ident("age".to_string()),
            Token::Comma,
            Token::Ident("info".to_string()),
            Token::Period,
            Token::Ident("year".to_string()),
            Token::Comma,
            Token::Ident("info".to_string()),
            Token::Period,
            Token::Ident("salary".to_string()),
            Token::From,
            Token::Ident("employees".to_string()),
            Token::Ident("emp".to_string()),
            Token::Join,
            Token::Ident("information".to_string()),
            Token::As,
            Token::Ident("info".to_string()),
            Token::On,
            Token::Ident("emp".to_string()),
            Token::Period,
            Token::Ident("emp_id".to_string()),
            Token::Equal,
            Token::Ident("info".to_string()),
            Token::Period,
            Token::Ident("emp_id".to_string()),
            Token::Where,
            Token::Ident("emp".to_string()),
            Token::Period,
            Token::Ident("employed".to_string()),
            Token::Equal,
            Token::Bool(true),
            Token::Logical(Logical::And),
            Token::Ident("emp".to_string()),
            Token::Period,
            Token::Ident("salary".to_string()),
            Token::GreaterThan,
            Token::Float(50000.00),
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

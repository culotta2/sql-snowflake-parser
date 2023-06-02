use anyhow::Result;

pub enum Token {
    Ident(String),

    // Literals
    Int(i64),
    Float(f64),
    // Bool(bool),
    // Date(String),
    // Varchar(String),
    Null,

    // DDL
    // DDL(DDLKeyword),

    // ColumnFunction(Function),

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,

    // Delimiters
    OpenParen,
    CloseParen,
    Comma,
    Period,
    Semicolon,
    Colon,
    Dollar,

    // Comparisons
    // Equal,
    // GreaterThan,
    // GreaterThanEqual,
    // LessThan,
    // LessThanEqual,
    // NotEqual,

    // Logical
    // And,
    // Not,
    // Or,

    // Keywords
    // All,
    // As,
    // Assign, // :=
    // Between,
    // By,
    // Caller,
    // Case,
    // Distinct,
    // Except,
    // Execute,
    // From,
    // Function,
    // Group,
    // Having,
    // Join,
    // Language,
    // Limit,
    // On,
    // Order,
    // Procedure,
    // Return,
    // Select,
    // Set,
    // Top,
    // Union,
    // When,
    // Where,
    // With,

    // Join Types
    // Inner,
    // Left,
    // Right,
    // Outer,

    EOF,
}

pub enum DDLKeyword {
    Alter,
    Create,
    Delete,
    Drop,
    Insert,
    Replace,
    Truncate,
    Update,
}

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
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,

            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b',' => Token::Comma,
            b'.' => Token::Period,
            b';' => Token::Semicolon,
            b'$' => Token::Dollar,
            b':' => todo!(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                let lower_ident = ident.clone().to_lowercase();
                let ident_type = match lower_ident.as_str() {
                    "null" => Token::Null,
                    _ => Token::Ident(ident),
                };
                return Ok(ident_type);
            },
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

    fn read_int(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[start..self.position]).to_string();
    }
}
    

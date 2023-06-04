use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
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
    Minus, // TODO: Also check for comments inline comments --
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

    // Comments
    BlockComment(String),
    InlineComment(String),


    // Comparisons
    Equal,
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
    Assign, // :=
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
            b'-' => {
                if self.peek_char() == b'-' {
                    Token::InlineComment(self.read_inline_comment())
                } else {
                    Token::Minus
                }
            }
            b'*' => Token::Asterisk,
            b'=' => Token::Equal,
            b'/' => Token::Slash,

            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b',' => Token::Comma,
            b'.' => Token::Period,
            b';' => Token::Semicolon,
            b'$' => Token::Dollar,
            b':' => {
                if self.peek_char() == b'=' {
                    Token::Assign
                } else {
                    Token::Colon
                }
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                let lower_ident = ident.clone().to_lowercase();
                let ident_type = match lower_ident.as_str() {
                    "null" => Token::Null,
                    _ => Token::Ident(ident),
                };
                return Ok(ident_type);
            },
            b'0'..=b'9' => {
                let potential_number = self.read_number();
                let ret_tok = match &potential_number
                    .chars()
                    .filter(|ch| ch == &'.')
                    .count() 
                {
                    0 => Token::Int(potential_number.parse().unwrap()),
                    1 => Token::Float(potential_number.parse().unwrap()),
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

    use super::{Token, Lexer};

    #[test]
    fn assert_basic_string_match() -> Result<()> {
        let input = r#"
            SET x = 2; -- This is x, it is cool
            SELECT * FROM test;
            -- Not
            -- Soueu
            --------oeuaoeuou-aoeu-eo-au-eaou-eou-u-ae--
            "#;

        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Ident("SET".into()),
            Token::Ident("x".into()),
            Token::Equal,
            Token::Int(2),
            Token::Semicolon,
            Token::InlineComment("-- This is x, it is cool".into()),
            Token::Ident("SELECT".into()),
            Token::Asterisk,
            Token::Ident("FROM".into()),
            Token::Ident("test".into()),
            Token::Semicolon,
            Token::InlineComment("-- Not".into()),
            Token::InlineComment("-- Soueu".into()),
            Token::InlineComment("--------oeuaoeuou-aoeu-eo-au-eaou-eou-u-ae--".into()),
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, recieved: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        Ok(())
    }
}

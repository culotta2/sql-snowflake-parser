use crate::lexer::lexer;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum AstNode {
    Program(Vec<AstNode>),
    Statement(Vec<AstNode>),
    Expression(lexer::Token),
}

#[allow(dead_code)]
impl AstNode {
    fn add_child(&mut self, child: AstNode) {
        match self {
            AstNode::Program(node) | AstNode::Statement(node) => node.push(child),
            AstNode::Expression(_) => todo!("Not sure what to do for an expression"),
        }
    }
}

#[allow(dead_code)]
fn build_ast(tokens: Vec<lexer::Token>) -> AstNode {
    let mut program_node = AstNode::Program(vec![]);
    let mut statement_node = AstNode::Statement(vec![]);

    for token in tokens {
        let ast_node = match token {
            lexer::Token::DML(lexer::DMLKeyword::Select) => AstNode::Expression(token),
            lexer::Token::Ident(_) | lexer::Token::Varchar(_) => AstNode::Expression(token),
            _ => AstNode::Expression(token), // TODO: Currently treating all other tokens as
                                             // expressions
        };

        statement_node.add_child(ast_node.clone());
    }

    program_node.add_child(statement_node);
    program_node
}

#[allow(dead_code)]
enum Clause {
    Select,
    From,
    Where,
    // Having,
    // Qualify,
    GroupBy,
    // OrderBy,
    // Limit,
    // Join,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_query() {
        // Query: SELECT date, temperate FROM table WHERE date = '2022-01-01';
        let tokens = vec![
            lexer::Token::DML(lexer::DMLKeyword::Select),
            lexer::Token::Ident("date".into()),
            lexer::Token::Comma,
            lexer::Token::Ident("temperature".into()),
            lexer::Token::From,
            lexer::Token::Ident("table".into()),
            lexer::Token::Where,
            lexer::Token::Ident("date".into()),
            lexer::Token::Equal,
            lexer::Token::Varchar("2022-01-01".into()),
            lexer::Token::Semicolon,
        ];

        let ast_expected = AstNode::Program(vec![AstNode::Statement(vec![
            AstNode::Expression(lexer::Token::DML(lexer::DMLKeyword::Select)),
            AstNode::Expression(lexer::Token::Ident("date".into())),
            AstNode::Expression(lexer::Token::Comma),
            AstNode::Expression(lexer::Token::Ident("temperature".into())),
            AstNode::Expression(lexer::Token::From),
            AstNode::Expression(lexer::Token::Ident("table".into())),
            AstNode::Expression(lexer::Token::Where),
            AstNode::Expression(lexer::Token::Ident("date".into())),
            AstNode::Expression(lexer::Token::Equal),
            AstNode::Expression(lexer::Token::Varchar("2022-01-01".into())),
            AstNode::Expression(lexer::Token::Semicolon),
        ])]);

        let ast_actual = build_ast(tokens);

        assert_eq!(ast_expected, ast_actual);
    }
}

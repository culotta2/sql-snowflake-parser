use crate::lexer::lexer;

// TODO: Replace the syntax type with an enum
// once all the syntax types are defined
//
// enum SyntaxType {
//     Program,
//     Select,
// }

struct SyntaxNode {}

impl SyntaxNode {
    fn new(tokens: Vec<lexer::Token>) -> Self {
        SyntaxNode {}
    }
    
    pub fn parse(lexer: lexer::Lexer) -> Vec<lexer::Token> {
        return lexer.get_tokens().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer;
    use crate::ast::ast::SyntaxNode;

    #[test]
    fn test_get_tokens() {
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
        let lexer = lexer::Lexer::new(input.into());
        let x = SyntaxNode::parse(lexer);
        for val in x {
            println!("{:?}", val);
        }
        assert!(false);
    }
}

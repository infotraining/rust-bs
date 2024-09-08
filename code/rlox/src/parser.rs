use crate::scanner::{Token, TokenType};


#[derive(Debug)]
pub enum Expression{
    Number(f64),
    Binary(Box<Expression>, TokenType, Box<Expression>),
    Unary(TokenType, Box<Expression>),
    Grouping(Box<Expression>),
}

fn print_ast(expression: &Expression) -> String {
    match expression {
        Expression::Number(n) => n.to_string(),
        
        Expression::Binary(left, operator, right) => {
            format!("({:?} {} {})", operator, print_ast(left), print_ast(right))
        }
        
        Expression::Unary(operator, right) => {
            format!("({:?} {})", operator, print_ast(right))
        }
        
        Expression::Grouping(expression) => {
            format!("(group {})", print_ast(expression))
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn simple_binary_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = Expression::Binary(
            Box::new(Expression::Number(1.0)),
            TokenType::Plus,
            Box::new(Expression::Number(2.0)),
        );
 
        assert_eq!(format!("{:?}", expression), "Binary(Number(1.0), Plus, Number(2.0))");
        
        assert_eq!(print_ast(&expression), "(Plus 1 2)");
    }

    fn simple_unary_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = Expression::Unary(
            TokenType::Minus,
            Box::new(Expression::Number(1.0)),
        );
 
        assert_eq!(format!("{:?}", expression), "Unary(Minus, Number(1.0))");
        
        assert_eq!(print_ast(&expression), "(Minus 1)");
    }

    fn simple_grouping_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = 
            Expression::Binary(
                Box::new(Expression::Grouping(
                    Box::new(Expression::Binary(Box::new(Expression::Number(1.0)), TokenType::Plus, Box::new(Expression::Number(2.0))))
                )),
                TokenType::Star,
                Box::new(Expression::Grouping(
                    Box::new(Expression::Binary(Box::new(Expression::Number(1.0)), TokenType::Plus, Box::new(Expression::Number(2.0))))
                ))
            );
        assert_eq!(format!("{:?}", expression), "Binary(Grouping(Binary(Number(1.0), Plus, Number(2.0))), Star, Grouping(Binary(Number(1.0), Plus, Number(2.0)))");
        
        assert_eq!(print_ast(&expression), "(Star (group (Plus 1 2)) (group (Plus 1 2)))");
    }
}
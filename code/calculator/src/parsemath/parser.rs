use crate::parsemath::ast::Expression;
use crate::parsemath::token::Token;
use crate::parsemath::tokenizer::{Tokenizer, TokenizingError};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ParserError {
    UnableToParse,
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
}

impl Parser {
    pub fn new(expression: &str) -> Result<Self, ParserError> {
        let tokenizer = Tokenizer::new(expression);
        let tokens = tokenizer.collect::<Result<Vec<Token>, TokenizingError>>();

        match tokens {
            Ok(tokens) => Ok(Parser {
                tokens,
                current_token_index: 0,
            }),
            Err(_) => Err(ParserError::UnableToParse),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        let expression = self.expression();

        Ok(expression)
    }

    fn expression(&mut self) -> Expression {
        let expression = self.term();

        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.consume();
                    let right = self.factor();
                    expression = Expression::Add(Box::new(expression), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.consume();
                    let right = self.factor();
                    expression = Expression::Subtract(Box::new(expression), Box::new(right));
                }
                _ => break,
            }
        }

        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();

        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.consume();
                    let right = self.unary();
                    expression = Expression::Multiply(Box::new(expression), Box::new(right));
                }
                Some(Token::Slash) => {
                    self.consume();
                    let right = self.unary();
                    expression = Expression::Divide(Box::new(expression), Box::new(right));
                }
                _ => break,
            }
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if let Some(Token::Minus) = self.peek() {
            self.consume();

            let right = self.unary();
            return Expression::Negate(Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        let expression = match self.next() {
            Some(Token::Number(n)) => Expression::Number(n),
            _ => panic!(),
        };

        //self.advance();

        expression
    }

    // fn match_token(&mut self, expected: Token) -> bool {
    //     if self.check(expected) {
    //         self.advance();
    //         return true;
    //     }
    //     false
    // }

    // fn check(&mut self, expected: Token) -> bool {
    //     if self.is_at_end() {
    //         return false;
    //     }
    //
    //     return self.peek() == expected;
    // }

    fn consume(&mut self) {
        if !self.is_at_end() {
            self.current_token_index += 1;
        }
    }

    // fn advance(&mut self) -> Token {
    //     if !self.is_at_end() {
    //         self.current_token_index += 1;
    //     }
    //
    //     return self.previous();
    // }

    fn is_at_end(&self) -> bool {
        self.current_token_index >= self.tokens.len()
    }

    fn peek(&self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }
        Some(self.tokens[self.current_token_index].clone())
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.current_token_index += 1;
        return token;
    }

    // fn previous(&self) -> Token {
    //     self.tokens[self.current_token_index - 1].clone()
    // }
}

#[cfg(test)]
mod tests {
    use crate::parsemath::ast::Expression;
    use crate::parsemath::parser::Parser;
    use rstest::*;

    #[rstest]
    fn parse_to_ast() {
        let expression = "1";
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, Expression::Number(1.0));
    }

    #[rstest]
    #[case::expr_1_times_2(
        "1 * 2",
        Expression::Multiply(Box::new(Expression::Number(1.0)), Box::new(Expression::Number(2.0)))
    )]
    #[case::expr_1_times_2_times_3(
        "1 * 2 * 3",
        Expression::Multiply(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )),
            Box::new(Expression::Number(3.0))
        )
    )]
    fn parse_to_ast_multiplication(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    fn parse_to_ast_division() {
        let expression = "1 / 2";
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Expression::Divide(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[rstest]
    #[case::expr_1_plus_2(
        "1 + 2",
        Expression::Add(Box::new(Expression::Number(1.0)), Box::new(Expression::Number(2.0)))
    )]
    fn parse_to_ast_addition(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_1_minus_2(
        "1 - 2",
        Expression::Subtract(Box::new(Expression::Number(1.0)), Box::new(Expression::Number(2.0)))
    )]
    fn parse_to_ast_subtraction(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_1_plus_2_minus_3(
        "1 + 2 - 3",
        Expression::Subtract(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )),
            Box::new(Expression::Number(3.0))
        )
    )]
    #[case::expr_2_times_4_plus_6_div_2(
        "2 * 4 + 6 / 2",
        Expression::Add(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(4.0))
            )),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(6.0)),
                Box::new(Expression::Number(2.0))
            ))
        )
    )]
    fn parse_to_ast_complex_expression(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_negate_1("-1", Expression::Negate(Box::new(Expression::Number(1.0))))]
    #[case::expr_negate_1_plus_2(
        "-1 + 2",
        Expression::Add(
            Box::new(Expression::Negate(Box::new(Expression::Number(1.0)))),
            Box::new(Expression::Number(2.0))
        )
    )]
    fn parse_negate_expression(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }
}

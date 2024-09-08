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

fn evaluate_numeric_expression(expression: &Expression) -> f64 {
    match expression {
        Expression::Number(n) => *n,
        
        Expression::Binary(left, operator, right) => {
            let left = evaluate_numeric_expression(left);
            let right = evaluate_numeric_expression(right);
            match operator {
                TokenType::Plus => left + right,
                TokenType::Minus => left - right,
                TokenType::Star => left * right,
                TokenType::Slash => left / right,
                _ => panic!("Unknown operator: {:?}", operator),
            }
        }
        
        Expression::Unary(operator, right) => {
            let right = evaluate_numeric_expression(right);
            match operator {
                TokenType::Minus => -right,                
                _ => panic!("Unknown operator: {:?}", operator),
            }
        }
        
        Expression::Grouping(expression) => {
            evaluate_numeric_expression(expression)
        }
    }
}

struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut scanner = crate::scanner::Scanner::new(source);
        let mut scanned_tokens: Vec<Token<'a>> = scanner.scan_tokens().unwrap();
        
        Self {
            source: source,
            tokens: scanned_tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expression {
        return self.equality();
    }

    fn equality(&mut self) -> Expression {
        let mut expression = self.comparison();

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().token_type;
            let right = self.comparison();
            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        return expression;
    }

    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();

        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().token_type;
            let right = self.term();
            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        return expression;
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().token_type;
            let right = self.factor();
            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        return expression;
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().token_type;
            let right = self.unary();
            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        return expression;
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(&[TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous().token_type;
            let right = self.unary();
            return Expression::Unary(operator, Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        // if self.match_token(&[TokenType::False]) {
        //     return Expression::Number(0.0);
        // }
        // if self.match_token(&[TokenType::True]) {
        //     return Expression::Number(1.0);
        // }
        // if self.match_token(&[TokenType::Nil]) {
        //     return Expression::Number(0.0);
        // }

        if self.match_token(&[TokenType::Number]) {
            return Expression::Number(self.previous().lexeme.parse().unwrap());
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expression = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expression::Grouping(Box::new(expression));
        }

        panic!("Expression expected. Line: {}", self.peek().line);
    }

    fn consume(&mut self, token_type: TokenType, message: &'static str) {
        if !self.check(&token_type) {
            panic!("ERROR: {}", message);
        }
        self.advance();
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    /// Returns true if current token is of the given type
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *token_type;
    }

    /// Consumes the current token and returns it
    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    /// Returns true if we are at the end of lexems - EOF token
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    /// Returns the current token without consuming it
    fn peek(&self) -> &Token<'a> {
        return &self.tokens[self.current];
    }

    /// Returns the previous token
    fn previous(&self) -> &Token<'a> {
        return &self.tokens[self.current - 1];
    }

    pub fn parse(&mut self) -> Expression {
        println!("Tokens: {:#?}", self.tokens);
        return self.expression();
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

    #[test]
    fn parse_simple_expression_1() {
        let source = "1 + 2 * 3";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Binary(Number(1.0), Plus, Binary(Number(2.0), Star, Number(3.0)))");
        assert_eq!(print_ast(&expression), "(Plus 1 (Star 2 3))");
    }

    #[test]
    fn parse_simple_expression_2() {
        let source = "-((1 + 2) * (4 - 2))";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Unary(Minus, Grouping(Binary(Grouping(Binary(Number(1.0), Plus, Number(2.0))), Star, Grouping(Binary(Number(4.0), Minus, Number(2.0))))))");
        assert_eq!(print_ast(&expression), "(Minus (group (Star (group (Plus 1 2)) (group (Minus 4 2)))))");
    }

    #[test]
    fn parse_simple_comparison_expression() {
        let source = "1 + 2 > 3 * 4";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Binary(Binary(Number(1.0), Plus, Number(2.0)), Greater, Binary(Number(3.0), Star, Number(4.0)))");
        assert_eq!(print_ast(&expression), "(Greater (Plus 1 2) (Star 3 4))");
    }

    #[test]
    #[should_panic(expected = "Expression expected. Line: 1")]
    fn parsing_code_with_incorrect_syntax() {
        let source = "1 + 2 *";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Binary(Number(1.0), Plus, Binary(Number(2.0), Star, Number(0.0))");
        assert_eq!(print_ast(&expression), "(Plus 1 (Star 2 0))");
    }

    #[test]
    fn parse_and_evaluate_numeric_expression() {
        let source = "-((1 + 2) * (4 - 2))";
        let mut parser = Parser::new(source);
        let expression = parser.parse();
        let result = evaluate_numeric_expression(&expression);

        assert_eq!(result, -6.0);
    }

}
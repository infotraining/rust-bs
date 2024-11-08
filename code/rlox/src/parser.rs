use crate::scanner::{Token, TokenType};
use crate::ast::{Expression, Statement, Value};


pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut scanner = crate::scanner::Scanner::new(source);
        let scanned_tokens: Vec<Token<'a>> = scanner.scan_tokens().unwrap();

        Self {
            source: source,
            tokens: scanned_tokens,
            current: 0,
        }
    }

    fn statement(&mut self) -> Statement {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Statement {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Statement::PrintStmt(expression)
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Statement::ExpressionStmt(expression)
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

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
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
            return Expression::Literal(Value::Number(self.previous().lexeme.parse().unwrap()));
        }

        if self.match_token(&[TokenType::String]) {
            return Expression::Literal(Value::String(self.previous().lexeme.to_string()));
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

    pub fn parse_source(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        return statements;
    }
}

#[cfg(test)]
mod parser_tests {
    pub struct AstPrinter {
        pub result: String,
    }
    
    impl AstPrinter {
        pub fn new() -> Self {
            Self {
                result: String::new(),
            }
        }
    }
    
    impl ExpressionVisitor for AstPrinter {
        type VisitResult = AstResult<()>;
    
        fn visit_literal(&mut self, value: &Value) -> Self::VisitResult {
            let _literal_value = match value {
                Value::Number(n) => self.result.push_str(&n.to_string()),
                Value::String(s) => self.result.push_str(&s),
                Value::Boolean(b) => self.result.push_str(&b.to_string()),
                Value::Nil => self.result.push_str("nil"),
            };
            Ok(())
        }
    
        fn visit_binary(&mut self, left: &Expression, operator: &TokenType, right: &Expression) -> Self::VisitResult {
            self.result.push_str("(");
            self.result.push_str(&format!("{:?}", operator));
            self.result.push_str(" ");
            let _ = left.accept(self);
            self.result.push_str(" ");
            let _ = right.accept(self);
            self.result.push_str(")");
            Ok(())
        }
    
        fn visit_unary(&mut self, operator: &TokenType, right: &Expression) -> Self::VisitResult {
            self.result.push_str("(");
            self.result.push_str(&format!("{:?}", operator));
            self.result.push_str(" ");
            let _ = right.accept(self);
            self.result.push_str(")");
            Ok(())
        }
    
        fn visit_grouping(&mut self, expression: &Expression) -> Self::VisitResult {
            self.result.push_str("(group ");
            let _ = expression.accept(self);
            self.result.push_str(")");
            Ok(())
        }
    }
    
    fn print_ast(expression: &Expression) -> String {
        let mut printer = AstPrinter::new();
        
        let _ = expression.accept(&mut printer);
        return printer.result;
    }
    
    fn evaluate_numeric_expression(expression: &Expression) -> f64 {
        match expression {
            Expression::Literal(value) => match value {
                Value::Number(n) => *n,
                _ => panic!("Expected number, got {:?}", value),
            },
    
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
    
            Expression::Grouping(expression) => evaluate_numeric_expression(expression),
        }
    }

    use crate::ast::{AstResult, ExpressionVisitor};

    use super::*;

    #[test]
    fn simple_binary_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = Expression::Binary(
            Box::new(Expression::Literal(Value::Number(1.0))),
            TokenType::Plus,
            Box::new(Expression::Literal(Value::Number(2.0))),
        );

        assert_eq!(
            format!("{:?}", expression),
            "Binary(Literal(Number(1.00)), Plus, Literal(Number(2.00)))"
        );

        assert_eq!(print_ast(&expression), "(Plus 1 2)");
    }

    #[test]
    fn test_ast_printer() {
        let expression = Expression::Binary(
            Box::new(Expression::Literal(Value::Number(1.0))),
            TokenType::Plus,
            Box::new(Expression::Literal(Value::Number(2.0))),
        );

        let mut printer = AstPrinter::new();
        
        let _ = expression.accept(&mut printer);

        assert_eq!(printer.result, "(Plus 1 2)");
    }

    #[test]
    fn simple_unary_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = Expression::Unary(TokenType::Minus, Box::new(Expression::Literal(Value::Number(1.0))));

        assert_eq!(format!("{:?}", expression), "Unary(Minus, Literal(Number(1.00)))");

        assert_eq!(print_ast(&expression), "(Minus 1)");
    }

    #[test]
    fn simple_grouping_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = Expression::Binary(
            Box::new(Expression::Grouping(Box::new(Expression::Binary(
                Box::new(Expression::Literal(Value::Number(1.0))),
                TokenType::Plus,
                Box::new(Expression::Literal(Value::Number(2.0))),
            )))),
            TokenType::Star,
            Box::new(Expression::Grouping(Box::new(Expression::Binary(
                Box::new(Expression::Literal(Value::Number(1.0))),
                TokenType::Plus,
                Box::new(Expression::Literal(Value::Number(2.0))),
            )))),
        );
        assert_eq!(format!("{:?}", expression), "Binary(Grouping(Binary(Literal(Number(1.00)), Plus, Literal(Number(2.00)))), Star, Grouping(Binary(Literal(Number(1.00)), Plus, Literal(Number(2.00)))))");

        let mut printer = AstPrinter::new();        
        
        let _ = expression.accept(&mut printer);
        
        assert_eq!(
            printer.result,
            "(Star (group (Plus 1 2)) (group (Plus 1 2)))"
        );

        assert_eq!(
            print_ast(&expression),
            "(Star (group (Plus 1 2)) (group (Plus 1 2)))"
        );
    }

    #[test]
    fn parse_simple_expression_1() {
        let source = "1 + 2 * 3";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(
            format!("{:?}", expression),
            "Binary(Literal(Number(1.00)), Plus, Binary(Literal(Number(2.00)), Star, Literal(Number(3.00))))"
        );
        assert_eq!(print_ast(&expression), "(Plus 1 (Star 2 3))");
    }

    #[test]
    fn parse_simple_expression_2() {
        let source = "-((1 + 2) * (4 - 2))";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Unary(Minus, Grouping(Binary(Grouping(Binary(Literal(Number(1.00)), Plus, Literal(Number(2.00)))), Star, Grouping(Binary(Literal(Number(4.00)), Minus, Literal(Number(2.00)))))))");
        assert_eq!(
            print_ast(&expression),
            "(Minus (group (Star (group (Plus 1 2)) (group (Minus 4 2)))))"
        );
    }

    #[test]
    fn parse_simple_comparison_expression() {
        let source = "1 + 2 > 3 * 4";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Binary(Binary(Literal(Number(1.00)), Plus, Literal(Number(2.00))), Greater, Binary(Literal(Number(3.00)), Star, Literal(Number(4.00))))");
        assert_eq!(print_ast(&expression), "(Greater (Plus 1 2) (Star 3 4))");
    }

    #[test]
    #[should_panic(expected = "Expression expected. Line: 1")]
    fn parsing_code_with_incorrect_syntax() {
        let source = "1 + 2 *";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(
            format!("{:?}", expression),
            "Binary(Literal(Number(1.0)), Plus, Binary(Literal(Number(2.00)), Star, Literal(Number(0.00)))"
        );
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

    #[test]
    fn parse_string_literal()
    {
        let source = r#""Hello""#;
        let mut parser = Parser::new(source);
        let expression: Expression = parser.parse();

        let expected_expression = Expression::Literal(Value::String("Hello".to_string()));

        assert_eq!(expression, expected_expression);
    }

    #[test]
    fn parse_print_string_literal()
    {
        let source = r#"print "Hello, World!";"#;
        let mut parser = Parser::new(source);
        let statements = parser.parse_source();

        let expected_statements = vec![
            Statement::PrintStmt(Expression::Literal(Value::String("Hello, World!".to_string())))
        ];

        assert_eq!(statements, expected_statements);
    }

    #[test]
    fn parse_multiple_statements() {
        let source = r#"print 1; print 2; print "Hello";"#;
        let mut parser = Parser::new(source);
        let statements = parser.parse_source();

        let expected_statements = vec![
            Statement::PrintStmt(Expression::Literal(Value::Number(1.0))),
            Statement::PrintStmt(Expression::Literal(Value::Number(2.0))),
            Statement::PrintStmt(Expression::Literal(Value::String("Hello".to_string()))),
        ];

        assert_eq!(statements, expected_statements);
    }
}

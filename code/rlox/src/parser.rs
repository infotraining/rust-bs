use crate::scanner::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => { Some(*n) }
            _ => { None }
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => { Some(*b) }
            _ => { None }
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => { Some(s) }
            _ => { None }
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Binary(Box<Expression>, TokenType, Box<Expression>),
    Unary(TokenType, Box<Expression>),
    Grouping(Box<Expression>),
}

pub trait AstVisitor {
    fn visit_literal(&mut self, value: &Value) -> Option<Value>;
    fn visit_binary(&mut self, left: &Expression, operator: &TokenType, right: &Expression) -> Option<Value>;
    fn visit_unary(&mut self, operator: &TokenType, expression: &Expression) -> Option<Value>;
    fn visit_grouping(&mut self, expression: &Expression) -> Option<Value>;
}

pub fn accept_visitor<V: AstVisitor>(visitor: &mut V, expression: &Expression) -> Option<Value> {
    match expression {
        Expression::Literal(value) => { visitor.visit_literal(value) }

        Expression::Binary(left, operator, right) => {
            visitor.visit_binary(left, operator, right)
        }

        Expression::Unary(operator, right) => {
            visitor.visit_unary(operator, right)
        }

        Expression::Grouping(expression) => {
            visitor.visit_grouping(expression)
        }
    }
}

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

impl AstVisitor for AstPrinter {
    fn visit_literal(&mut self, value: &Value) -> Option<Value> {
        match value {
            Value::Number(n) => self.result.push_str(&n.to_string()),
            Value::String(s) => self.result.push_str(&s),
            Value::Boolean(b) => self.result.push_str(&b.to_string()),
            Value::Nil => self.result.push_str("nil"),
        };
        None
    }

    fn visit_binary(&mut self, left: &Expression, operator: &TokenType, right: &Expression) -> Option<Value> {
        self.result.push_str("(");
        self.result.push_str(&format!("{:?}", operator));
        self.result.push_str(" ");
        accept_visitor(self, left);
        self.result.push_str(" ");
        accept_visitor(self, right);
        self.result.push_str(")");
        None
    }

    fn visit_unary(&mut self, operator: &TokenType, right: &Expression) -> Option<Value> {
        self.result.push_str("(");
        self.result.push_str(&format!("{:?}", operator));
        self.result.push_str(" ");
        accept_visitor(self, right);
        self.result.push_str(")");
        None
    }

    fn visit_grouping(&mut self, expression: &Expression) -> Option<Value> {
        self.result.push_str("(group ");
        accept_visitor(self, expression);
        self.result.push_str(")");
        None
    }
}

fn print_ast(expression: &Expression) -> String {
    let mut printer = AstPrinter::new();
    accept_visitor(&mut printer, expression);
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

struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut scanner = crate::scanner::Scanner::new(source);
        let scanned_tokens: Vec<Token<'a>> = scanner.scan_tokens().unwrap();

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
            Box::new(Expression::Literal(Value::Number(1.0))),
            TokenType::Plus,
            Box::new(Expression::Literal(Value::Number(2.0))),
        );

        assert_eq!(
            format!("{:?}", expression),
            "Binary(Literal(Number(1.0)), Plus, Literal(Number(2.0)))"
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
        accept_visitor(&mut printer, &expression);

        assert_eq!(printer.result, "(Plus 1 2)");
    }

    #[test]
    fn simple_unary_expression() {
        use crate::parser::{Expression, TokenType};
        let expression = Expression::Unary(TokenType::Minus, Box::new(Expression::Literal(Value::Number(1.0))));

        assert_eq!(format!("{:?}", expression), "Unary(Minus, Literal(Number(1.0)))");

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
        assert_eq!(format!("{:?}", expression), "Binary(Grouping(Binary(Literal(Number(1.0)), Plus, Literal(Number(2.0)))), Star, Grouping(Binary(Literal(Number(1.0)), Plus, Literal(Number(2.0)))))");

        let mut printer = AstPrinter::new();
        accept_visitor(&mut printer, &expression);
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
            "Binary(Literal(Number(1.0)), Plus, Binary(Literal(Number(2.0)), Star, Literal(Number(3.0))))"
        );
        assert_eq!(print_ast(&expression), "(Plus 1 (Star 2 3))");
    }

    #[test]
    fn parse_simple_expression_2() {
        let source = "-((1 + 2) * (4 - 2))";
        let mut parser = Parser::new(source);
        let expression = parser.parse();

        assert_eq!(format!("{:?}", expression), "Unary(Minus, Grouping(Binary(Grouping(Binary(Literal(Number(1.0)), Plus, Literal(Number(2.0)))), Star, Grouping(Binary(Literal(Number(4.0)), Minus, Literal(Number(2.0)))))))");
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

        assert_eq!(format!("{:?}", expression), "Binary(Binary(Literal(Number(1.0)), Plus, Literal(Number(2.0))), Greater, Binary(Literal(Number(3.0)), Star, Literal(Number(4.0))))");
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
            "Binary(Literal(Number(1.0)), Plus, Binary(Literal(Number(2.0)), Star, Literal(Number(0.0)))"
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
}

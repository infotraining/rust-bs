#[macro_use]
extern crate assert_float_eq;

use std::cell::RefCell;
use std::rc::Rc;

use assert_float_eq::assert_float_absolute_eq;
use rlox::ast::{AstResult, Expression, Statement, Value};
use rlox::interpreter::{Console, Interpreter, InterpreterError};
use rlox::scanner::TokenType;

fn create_interpreter() -> Interpreter {
    Interpreter::new(Rc::new(RefCell::new(ConsoleMock::new())))
}

#[test]
fn evaluate_literal() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Literal(Value::Number(3.14));
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_number().unwrap(), 3.14);
}

#[test]
fn evaluate_unary_minus() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Unary(
        TokenType::Minus,
        Box::new(Expression::Literal(Value::Number(3.14))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_number().unwrap(), -3.14);
}

#[test]
fn evaluate_unary_bang() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Unary(
        TokenType::Bang,
        Box::new(Expression::Literal(Value::Boolean(false))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_plus_for_double() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::Number(2.71))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_number().unwrap(), 5.85);
}

#[test]
fn evaluate_binary_plus_for_string() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::String("World".to_string()))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_string().unwrap(), "HelloWorld");
}

#[test]
fn evaluate_binary_minus() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Minus,
        Box::new(Expression::Literal(Value::Number(2.71))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_float_absolute_eq!(result.as_number().unwrap(), 0.43, 0.0001);
}

#[test]
fn evaluate_binary_multiplication() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Star,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_float_absolute_eq!(result.as_number().unwrap(), 6.28, 0.0001);
}

#[test]
fn evaluate_binary_division() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Slash,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_float_absolute_eq!(result.as_number().unwrap(), 1.57, 0.0001);
}

#[test]
fn evaluate_binary_comparison_greater() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Greater,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_comparison_less() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Less,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), false);
}

#[test]
fn evaluate_binary_comparison_greater_equal() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::GreaterEqual,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_comparison_less_equal() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::LessEqual,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), false);
}

#[test]
fn evaluate_binary_equal_for_double() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::EqualEqual,
        Box::new(Expression::Literal(Value::Number(3.14))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_equal_for_string() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
        TokenType::EqualEqual,
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_equal_for_nil() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Nil)),
        TokenType::EqualEqual,
        Box::new(Expression::Literal(Value::Nil)),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_equal_for_boolean() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Boolean(true))),
        TokenType::EqualEqual,
        Box::new(Expression::Literal(Value::Boolean(true))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_not_equal_for_double() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::BangEqual,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_binary_not_equal_for_string() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
        TokenType::BangEqual,
        Box::new(Expression::Literal(Value::String("World".to_string()))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluation_unary_minus_with_string_returns_error() {
    let mut interpreter = create_interpreter();

    let expression = Expression::Unary(
        TokenType::Minus,
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
    );

    let result: AstResult<Value> = interpreter.evaluate(&expression);

    match result {
        Ok(_) => panic!("Expected an error"),
        Err(e) => {
            let error = e.downcast_ref::<InterpreterError>().unwrap();
            assert_eq!(
                error.to_string(),
                "Unary operator - is not defined for String(Hello)"
            );
        }
    }
}

#[test]
fn evaluation_of_binary_minus_for_operands_that_are_not_numbers_returns_error() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(2.0))),
        TokenType::Minus,
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
    );
    let result: AstResult<Value> = interpreter.evaluate(&expression);
    match result {
        Ok(_) => panic!("Expected an error"),
        Err(e) => {
            let error = e.downcast_ref::<InterpreterError>().unwrap();
            assert_eq!(
                error.to_string(),
                "Binary operator Minus is not defined for Number(2.00) and String(Hello)"
            );
        }
    }
}

#[test]
fn evaluation_of_binary_plus_for_operands_that_are_not_numbers_returns_error() {
    let mut interpreter = create_interpreter();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(2.0))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::Boolean(true))),
    );
    let result: AstResult<Value> = interpreter.evaluate(&expression);
    match result {
        Ok(_) => panic!("Expected an error"),
        Err(e) => {
            let error = e.downcast_ref::<InterpreterError>().unwrap();
            assert_eq!(error.to_string(), "Operators must be two numebrs or two strings - found Number(2.00) and Boolean(true) instead");
        }
    }
}

struct ConsoleMock {
    output: String,
}

impl ConsoleMock {
    fn get_output(&self) -> &str {
        &self.output
    }

    fn new() -> Self {
        ConsoleMock {
            output: String::new(),
        }
    }
}

impl Console for ConsoleMock {
    fn write(&mut self, value: &str) {
        self.output.push_str(value);
    }
}

#[test]
fn interpreting_expression_prints_value_in_output() {
    // expression: (3.14 + 2.71) * 2.0
    let expression = Expression::Binary(
        Box::new(Expression::Binary(
            Box::new(Expression::Literal(Value::Number(3.14))),
            TokenType::Plus,
            Box::new(Expression::Literal(Value::Number(2.71))),
        )),
        TokenType::Star,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );

    let console_output = Rc::new(RefCell::new(ConsoleMock::new()));
    let mut interpreter = Interpreter::new(console_output.clone());

    interpreter.interpret(&expression);

    assert_eq!(console_output.borrow().get_output(), "Number(11.70)");
}

#[test]
fn interpreting_incorrect_expression_prints_error_in_output() {
    // expression: 3.14 + "Hello"
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
    );

    let console_output = Rc::new(RefCell::new(ConsoleMock::new()));
    let mut interpreter = Interpreter::new(console_output.clone());

    interpreter.interpret(&expression);

    assert_eq!(console_output.borrow().get_output(), "ERROR: Operators must be two numebrs or two strings - found Number(3.14) and String(Hello) instead");
}

#[test]
fn interpret_print_statement() {
    // statement: print 3.14 + 2.71;
    let statements = vec![Statement::PrintStmt(Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::Number(2.71))),
    ))];

    let console_output = Rc::new(RefCell::new(ConsoleMock::new()));
    let mut interpreter = Interpreter::new(console_output.clone());
    
    interpreter.interpret_statements(&statements);

    assert_eq!(console_output.borrow().get_output(), "5.85");   
}

#[test]
fn interpret_multiple_statements() {
    // statements: print 3.14 + 2.71; print "Hello" + "World"; print "!";
    let statements = vec![
        Statement::PrintStmt(Expression::Binary(
            Box::new(Expression::Literal(Value::Number(3.14))),
            TokenType::Plus,
            Box::new(Expression::Literal(Value::Number(2.71))),
        )),
        Statement::PrintStmt(Expression::Binary(
            Box::new(Expression::Literal(Value::String("Hello".to_string()))),
            TokenType::Plus,
            Box::new(Expression::Literal(Value::String("World".to_string()))),
        )),
        Statement::PrintStmt(Expression::Literal(Value::String("!".to_string()))),
    ];


    let console_output = Rc::new(RefCell::new(ConsoleMock::new()));
    let mut interpreter = Interpreter::new(console_output.clone());
    
    interpreter.interpret_statements(&statements);

    assert_eq!(console_output.borrow().get_output(), "5.85HelloWorld!");
}


#[test]
fn parse_and_interpret_multiple_statements() {
    let source_code = r#"print 3.14 + 2.71; print "Hello";"#;
    
    let mut parser = rlox::parser::Parser::new(source_code);
    let statements = parser.parse_source();

    let console_output = Rc::new(RefCell::new(ConsoleMock::new()));
    let mut interpreter = Interpreter::new(console_output.clone());
    
    interpreter.interpret_statements(&statements);

    assert_eq!(console_output.borrow().get_output(), "5.85Hello");
}
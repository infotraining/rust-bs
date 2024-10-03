#[macro_use]
extern crate assert_float_eq;

use rlox::scanner::{Scanner, Token, TokenType, TokenError};
use rlox::ast::{AstResult, Expression, Value, ValueError};
use rlox::interpreter::{EvaluationResult, Interpreter, InterpreterError};
use std::result;
use assert_float_eq::assert_float_absolute_eq;


#[test]
fn evaluating_literal() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Literal(Value::Number(3.14));
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_number().unwrap(), 3.14);
}

#[test]
fn evaluating_unary_minus() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Unary(
        TokenType::Minus,
        Box::new(Expression::Literal(Value::Number(3.14))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_number().unwrap(), -3.14);
}

#[test]
fn evaluating_unary_bang() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Unary(
        TokenType::Bang,
        Box::new(Expression::Literal(Value::Boolean(false))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluating_binary_plus_for_double() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::Number(2.71))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_number().unwrap(), 5.85);
}

#[test]
fn evaluating_binary_plus_for_string() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
        TokenType::Plus,
        Box::new(Expression::Literal(Value::String("World".to_string()))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_string().unwrap(), "HelloWorld");
}

#[test]
fn evaluating_binary_minus() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Minus,
        Box::new(Expression::Literal(Value::Number(2.71))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_float_absolute_eq!(result.as_number().unwrap(), 0.43, 0.0001);
}

#[test]
fn evaluating_binary_multiplication() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Star,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_float_absolute_eq!(result.as_number().unwrap(), 6.28, 0.0001);
}

#[test]
fn evaluating_binary_division() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Slash,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_float_absolute_eq!(result.as_number().unwrap(), 1.57, 0.0001);
}

#[test]
fn evaluating_binary_comparison_greater() {
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::Number(3.14))),
        TokenType::Greater,
        Box::new(Expression::Literal(Value::Number(2.0))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluating_binary_comparison_less() {
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
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
    let mut interpreter = Interpreter::new();
    let expression = Expression::Binary(
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),
        TokenType::BangEqual,
        Box::new(Expression::Literal(Value::String("World".to_string()))),
    );
    let result = interpreter.evaluate(&expression).unwrap();
    assert_eq!(result.as_boolean().unwrap(), true);
}

#[test]
fn evaluate_code_that_fails() {
    let mut interpreter = Interpreter::new();
    
    let expression = Expression::Unary(
        TokenType::Minus,
        Box::new(Expression::Literal(Value::String("Hello".to_string()))),

    );

    let result: AstResult<Value> = interpreter.evaluate(&expression);
    
    match result {
        Ok(_) => panic!("Expected an error"),
        Err(e) => {
            let error = e.downcast_ref::<InterpreterError>().unwrap();
            assert_eq!(error.to_string(), "Unary operator - is not defined for String(Hello)");
        }
    }
}

#[test]
fn evaluation_of_binary_minus_for_operands_that_are_not_numbers() {
    let mut interpreter = Interpreter::new();
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
            assert_eq!(error.to_string(), "Binary operator Minus is not defined for Number(2) and String(Hello)");
        }
    }
}
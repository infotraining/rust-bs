use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{AstResult, Expression, ExpressionVisitor, Statement, StatementVisitor, Value, ValueError};
use crate::scanner::TokenType;


#[derive(Debug, Clone)]
pub struct InterpreterError
{
    pub message: String
}

impl std::error::Error for InterpreterError {}

impl std::fmt::Display for InterpreterError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.message)
    }
}

impl From<ValueError> for InterpreterError
{
    fn from(error: ValueError) -> Self
    {
        InterpreterError { message: error.message }
    }
}

pub type EvaluationResult = Result<Value, InterpreterError>;

pub trait Console
{
    fn write(&mut self, value: &str);
}

pub struct Interpreter
{
    console: Rc<RefCell<dyn Console>>
}

impl Interpreter
{
    pub fn new(console: Rc<RefCell<dyn Console>>) -> Self
    {
        Interpreter { console }
    } 

    pub fn interpret(&mut self, expression: &Expression)
    {        
        match self.evaluate(expression) {
            Ok(value) => { self.console.borrow_mut().write(&format!("{}", value)); }
            Err(e) => {self.console.borrow_mut().write(&format!("ERROR: {}", e)); }
        }    
    }

    pub fn interpret_statements(&mut self, statements: &Vec<Statement>)
    {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, statement: &Statement)
    {
        statement.accept(self);
    }

    pub fn evaluate(&mut self, expression: &Expression) -> AstResult<Value>
    {
        expression.accept(self)
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => { *b },
            Value::String(s) => { !s.is_empty() },
            _ => true
        }
    }

    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Nil, Value::Nil) => true,
            // (Value::Nil, _) => Some(Value::Boolean(false)),
            // (_, Value::Nil) => Some(Value::Boolean(false)),
            (Value::Boolean(left), Value::Boolean(right)) => left == right,
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::String(left), Value::String(right)) => left == right,
            _ => false
        }
    }
}

impl ExpressionVisitor for Interpreter
{
    type VisitResult = AstResult<Value>;

    fn visit_literal(&mut self, value: &Value) -> Self::VisitResult {
        Ok(value.clone())        
    }

    fn visit_grouping(&mut self, expression: &Expression) -> Self::VisitResult {
        self.evaluate(expression)
    }
    
    fn visit_unary(&mut self, operator: &TokenType, expression: &Expression) -> Self::VisitResult {
        
        let value = 
            self.evaluate(expression)?;
        
        match operator {
            TokenType::Minus => {
                let right = value.as_number().map_err(|e| InterpreterError { message: format!("Unary operator - is not defined for {}", e.current_value) })?;
                Ok(Value::Number(-right))
            }
            TokenType::Bang => {
                return Ok(Value::Boolean(!self.is_truthy(&value)));
            }
            _ => { Ok(Value::Nil) }
        }
    }

    fn visit_binary(&mut self, left: &Expression, operator: &TokenType, right: &Expression) -> Self::VisitResult
    {
        let left = self.evaluate(left).unwrap();
        let right: Value = self.evaluate(right).unwrap();

        match operator {
            TokenType::Plus => {                                
                match (left.clone(), right.clone()) {
                    (Value::Number(left), Value::Number(right)) => {
                        return Ok(Value::Number(left + right));
                    }
                    (Value::String(left), Value::String(right)) => {
                        return Ok(Value::String(format!("{}{}", left, right)));
                    }
                    _ => { Err(Box::new(InterpreterError { message: format!("Operators must be two numebrs or two strings - found {} and {} instead", left, right) })) }
                }
            }

            TokenType::Minus => {
                let (left_number, right_number) = get_number_operands(TokenType::Minus, &left, &right)?;
                Ok(Value::Number(left_number - right_number))
            }

            TokenType::Star => {
                let (left_number, right_number) = get_number_operands(TokenType::Star, &left, &right)?;
                Ok(Value::Number(left_number * right_number))
            }

            TokenType::Slash => {
                let (left_number, right_number) = get_number_operands(TokenType::Minus, &left, &right)?;
                Ok(Value::Number(left_number / right_number))
            }

            TokenType::Greater => {
                let (left_number, right_number) = get_number_operands(TokenType::Minus, &left, &right)?;
                Ok(Value::Boolean(left_number > right_number))
            }

            TokenType::Less => {
                let (left_number, right_number) = get_number_operands(TokenType::Minus, &left, &right)?;
                Ok(Value::Boolean(left_number < right_number))
            }

            TokenType::GreaterEqual => {
                let (left_number, right_number) = get_number_operands(TokenType::Minus, &left, &right)?;
                Ok(Value::Boolean(left_number >= right_number))
            }

            TokenType::LessEqual => {
                let (left_number, right_number) = get_number_operands(TokenType::Minus, &left, &right)?;
                Ok(Value::Boolean(left_number <= right_number))
            }

            TokenType::EqualEqual => {                
                Ok(Value::Boolean(self.is_equal(&left, &right)))
            }

            TokenType::BangEqual => {
                Ok(Value::Boolean(!self.is_equal(&left, &right)))
            }

            _ => { Ok(Value::Nil) }
        }        
    }
}

impl StatementVisitor for Interpreter {
    type VisitResult = ();

    fn visit_print_stmt(&mut self, statement: &Statement) -> Self::VisitResult {
        if let Statement::PrintStmt(expression) = statement {
            match self.evaluate(expression) {
                Ok(value) => { self.console.borrow_mut().write(&format!("{}", value)); }
                Err(e) => { println!("ERROR: {}", e); }
            }
        }
    }

    fn visit_expression_stmt(&mut self, statement: &Statement) -> Self::VisitResult {
        if let Statement::ExpressionStmt(expression) = statement {
            match self.evaluate(expression) {
                Ok(_) => {}
                Err(e) => { println!("ERROR: {}", e); }
            }
        }
    }
}

fn get_number_operands(minus: TokenType, left: &Value, right: &Value) -> Result<(f64, f64), InterpreterError> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok((*left, *right)),
        _ => Err(InterpreterError { message: format!("Binary operator {:?} is not defined for {} and {}", minus, left, right) })
    }    
}
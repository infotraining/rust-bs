use crate::parser::accept_visitor;
use crate::parser::AstVisitor;
use crate::parser::Expression;
use crate::parser::Value;
use crate::scanner::TokenType;


pub struct Interpreter
{
}

impl Interpreter
{
    pub fn new() -> Interpreter
    {
        Interpreter {}
    }

    pub fn evaluate(&mut self, expression: &Expression) -> Option<Value>
    {
        accept_visitor(self, expression)
    }

    pub fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => { *b },
            Value::String(s) => { !s.is_empty() },
            _ => true
        }
    }

    pub fn is_equal(&self, left: &Value, right: &Value) -> bool {
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

impl AstVisitor for Interpreter
{
    fn visit_literal(&mut self, value: &Value) -> Option<Value> {
        Some(value.clone())        
    }

    fn visit_grouping(&mut self, expression: &Expression) -> Option<Value>{
        self.evaluate(expression)
    }
    
    fn visit_unary(&mut self, operator: &TokenType, expression: &Expression) -> Option<Value>{
        
        let value = self.evaluate(expression).unwrap();
        
        match operator {
            TokenType::Minus => {
                let right = value.as_number().unwrap();
                Some(Value::Number(-right))
            }
            TokenType::Bang => {
                return Some(Value::Boolean(!self.is_truthy(&value)));
            }
            _ => { None }
        }
    }

    fn visit_binary(&mut self, left: &Expression, operator: &TokenType, right: &Expression) -> Option<Value>
    {
        match operator {
            TokenType::Plus => {
                
                let left: Value = self.evaluate(left).unwrap();
                let right: Value = self.evaluate(right).unwrap();

                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        return Some(Value::Number(left + right));
                    }
                    (Value::String(left), Value::String(right)) => {
                        return Some(Value::String(format!("{}{}", left, right)));
                    }
                    _ => { None }
                }
            }

            TokenType::Minus => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Number(left - right))
            }

            TokenType::Star => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Number(left * right))
            }

            TokenType::Slash => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Number(left / right))
            }

            TokenType::Greater => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Boolean(left > right))
            }

            TokenType::Less => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Boolean(left < right))
            }

            TokenType::GreaterEqual => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Boolean(left >= right))
            }

            TokenType::LessEqual => {
                let left = self.evaluate(left).unwrap().as_number().unwrap();
                let right = self.evaluate(right).unwrap().as_number().unwrap();
                Some(Value::Boolean(left <= right))
            }

            TokenType::EqualEqual => {
                let left = self.evaluate(left).unwrap();
                let right = self.evaluate(right).unwrap();
                Some(Value::Boolean(self.is_equal(&left, &right)))
            }

            TokenType::BangEqual => {
                let left = self.evaluate(left).unwrap();
                let right = self.evaluate(right).unwrap();
                Some(Value::Boolean(!self.is_equal(&left, &right)))
            }

            _ => { None }
        }

        
    }
}
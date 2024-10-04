use std::fmt;

use crate::scanner::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                write!(f, "Number({})", n)
            }
            Value::String(s) => {
                write!(f, "String({})", s)
            }
            Value::Boolean(b) => {
                write!(f, "Boolean({})", b)
            }
            Value::Nil => {
                write!(f, "nil")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueError {
    pub message: String,
    pub current_value: String,
}

impl std::error::Error for ValueError {}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Value {
    pub fn as_number(&self) -> Result<f64, ValueError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(ValueError {
                message: format!("Double expected - found {} instead", self),
                current_value: format! {"{}", self},
            }),
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
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

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type AstResult<T> = Result<T, GenericError>;

pub trait ExpressionVisitor {
    type VisitResult;

    fn visit_literal(&mut self, value: &Value) -> Self::VisitResult;
    fn visit_binary(
        &mut self,
        left: &Expression,
        operator: &TokenType,
        right: &Expression,
    ) -> Self::VisitResult;
    fn visit_unary(&mut self, operator: &TokenType, expression: &Expression) -> Self::VisitResult;
    fn visit_grouping(&mut self, expression: &Expression) -> Self::VisitResult;

    fn accept_visitor<V: ExpressionVisitor>(
        visitor: &mut V,
        expression: &Expression,
    ) -> V::VisitResult {
        match expression {
            Expression::Literal(value) => visitor.visit_literal(value),

            Expression::Binary(left, operator, right) => {
                visitor.visit_binary(left, operator, right)
            }

            Expression::Unary(operator, right) => visitor.visit_unary(operator, right),

            Expression::Grouping(expression) => visitor.visit_grouping(expression),
        }
    }
}

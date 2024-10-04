use std::fmt;

use crate::scanner::TokenType;

/////////////////////////////////////////////////////////////////////////////////////////////////
/// Value

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                write!(f, "Number({:.2})", n)
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                write!(f, "{}", n)
            }
            Value::String(s) => {
                write!(f, "{}", s)
            }
            Value::Boolean(b) => {
                write!(f, "{}", b)
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
                message: format!("Double expected - found {:?} instead", self),
                current_value: format! {"{:?}", self},
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

/////////////////////////////////////////////////////////////////////////////////////////////////
/// Expression

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Value),
    Binary(Box<Expression>, TokenType, Box<Expression>),
    Unary(TokenType, Box<Expression>),
    Grouping(Box<Expression>),
}

impl Expression {
    pub fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) -> V::VisitResult {
        match self {
            Expression::Literal(value) => visitor.visit_literal(value),

            Expression::Binary(left, operator, right) => {
                visitor.visit_binary(left, operator, right)
            }

            Expression::Unary(operator, right) => visitor.visit_unary(operator, right),

            Expression::Grouping(expression) => visitor.visit_grouping(expression),
        }
    }
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

    // fn accept_visitor<V: ExpressionVisitor>(
    //     visitor: &mut V,
    //     expression: &Expression,
    // ) -> V::VisitResult {
    //     match expression {
    //         Expression::Literal(value) => visitor.visit_literal(value),

    //         Expression::Binary(left, operator, right) => {
    //             visitor.visit_binary(left, operator, right)
    //         }

    //         Expression::Unary(operator, right) => visitor.visit_unary(operator, right),

    //         Expression::Grouping(expression) => visitor.visit_grouping(expression),
    //     }
    // }
}


/////////////////////////////////////////////////////////////////////////////////////////////////
/// Statement

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExpressionStmt(Expression),
    PrintStmt(Expression),
}

impl Statement {
    pub fn accept<V: StatementVisitor>(&self, visitor: &mut V) -> V::VisitResult {
            match self {
                Statement::ExpressionStmt(_expression) => visitor.visit_expression_stmt(self),
                Statement::PrintStmt(_expression) => visitor.visit_print_stmt(self),
            }
        }
}

pub trait StatementVisitor {
    type VisitResult;

    fn visit_expression_stmt(&mut self, expression: &Statement) -> Self::VisitResult;
    fn visit_print_stmt(&mut self, expression: &Statement) -> Self::VisitResult;
}
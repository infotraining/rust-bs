
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Add,
    Subtract,
    Divide,
    Multiply,
    Caret,
    LeftParen,
    RightParen,
    Number(f64)
}
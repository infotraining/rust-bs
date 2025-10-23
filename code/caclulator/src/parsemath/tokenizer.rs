use std::iter::Peekable;
use std::str::Chars;
use crate::parsemath::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer {
        Tokenizer{expr: expr.chars().peekable()}
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.expr.next() {
            match c {
                '+' => Some(Token::Add),
                _ => panic!("Unexpected character"),
            }
        }
        else {
            None
        }
    }
}

#[test]
fn tokenizer_simple_expressions() {

    let expr = "+";
    let tokenizer = Tokenizer::new(expr);

    let tokens: Vec<Token> = tokenizer.collect();
    assert_eq!(tokens, vec![Token::Add]);
}
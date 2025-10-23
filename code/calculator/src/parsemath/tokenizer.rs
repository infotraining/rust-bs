use crate::parsemath::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer<'_> {
        Tokenizer {
            expr: expr.chars().peekable(),
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(' ') = self.expr.peek() {
            self.expr.next();
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        if let Some(c) = self.expr.next() {
            match c {
                '+' => Some(Token::Add),
                '-' => Some(Token::Subtract),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                '^' => Some(Token::Caret),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '0'..='9' => {
                    let mut number = c.to_string();
                    while let Some('0'..='9') | Some('.') = self.expr.peek() {
                        number.push(self.expr.next().unwrap());
                    }
                    Some(Token::Number(number.parse::<f64>().unwrap()))
                }
                _ => panic!("Unexpected character"),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    #[case("+", vec![Token::Add])]
    #[case("-", vec![Token::Subtract])]
    #[case("*", vec![Token::Multiply])]
    #[case("/", vec![Token::Divide])]
    #[case("^", vec![Token::Caret])]
    fn tokenizer_operators(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, expected_tokens);
    }

    #[rstest]
    #[case("3", vec![Token::Number(3.0)])]
    #[case("3.14", vec![Token::Number(3.14)])]
    fn tokenizer_numbers(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, expected_tokens);
    }

    #[rstest]
    #[case("(", vec![Token::LeftParen])]
    #[case(")", vec![Token::RightParen])]
    fn tokenizer_parens(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, expected_tokens);
    }

    #[rstest]
    #[case("1+2", vec![Token::Number(1.0), Token::Add, Token::Number(2.0)])]
    #[case("1 + 2", vec![Token::Number(1.0), Token::Add, Token::Number(2.0)])]
    fn tokenizer_expressions(#[case] expr: &str, #[case] expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(expr);

        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, expected_tokens);
    }
}

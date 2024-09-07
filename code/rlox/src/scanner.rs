
use peekmore::PeekMore;
use peekmore::PeekMoreIterator;
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a str,
    chars: PeekMoreIterator<Chars<'a>>,
    start: usize,
    current: usize,
    line: i32,
}

#[derive(Debug, PartialEq)]
pub struct TokenError(String);

pub type TokenResult<'a> = Result<Token<'a>, TokenError>;

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        let chars = source.chars().peekmore();

        Scanner {
            source: source,
            chars: chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> TokenResult<'a> {
        self.skip_whitespaces();

        self.start = self.current;

        match self.advance() {
            Some(c) => match c {
                '(' => self.make_token(TokenType::LEFT_PAREN),
                ')' => self.make_token(TokenType::RIGHT_PAREN),
                '{' => self.make_token(TokenType::LEFT_BRACE),
                '}' => self.make_token(TokenType::RIGHT_BRACE),
                ',' => self.make_token(TokenType::COMMA),
                '.' => self.make_token(TokenType::DOT),
                '-' => self.make_token(TokenType::MINUS),
                '+' => self.make_token(TokenType::PLUS),
                '*' => self.make_token(TokenType::STAR),
                ';' => self.make_token(TokenType::SEMICOLON),
                _ => Err(TokenError(format!(
                    "Unexpected character: {}; Line: {}",
                    c, self.line
                ))),
            },
            None => self.make_token(TokenType::EOF),
        }
    }

    fn make_token(&self, token_type: TokenType) -> TokenResult<'a> {
        Ok(Token {
            token_type: token_type,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        })
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.chars.peek() {
                Some(c) if c.is_whitespace() => {
                    self.chars.next();
                    self.current += 1;
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.current += 1;
                Some(c)
            }
            None => None,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, TokenError> {
        let mut tokens = Vec::new();

        loop {
            match self.scan_token() {
                Ok(token) if token.token_type == TokenType::EOF => {
                    tokens.push(token);
                    break;
                }
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: i32,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    SEMICOLON,
    MINUS,
    PLUS,
    STAR,
    EOF,
}

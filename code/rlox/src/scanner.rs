
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
pub struct TokenError(pub String);

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
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '*' => self.make_token(TokenType::Star),
                ';' => self.make_token(TokenType::Semicolon),

                '!' => self.make_token_if_matches('=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.make_token_if_matches('=', TokenType::EqualEqual, TokenType::Equal),
                '>' => self.make_token_if_matches('=', TokenType::GreaterEqual, TokenType::Greater),
                '<' => self.make_token_if_matches('=', TokenType::LessEqual, TokenType::Less),

                '"' => self.string(),

                _ => Err(TokenError(format!(
                    "Unexpected character: {}; Line: {}",
                    c, self.line
                ))),
            },
            None => self.make_token(TokenType::Eof),
        }
    }

    fn string(&mut self) -> TokenResult<'a> {
        self.start += 1; // Consume the opening ".

        loop {
            match self.peek() {
                Some(c) if *c == '"' => break,
                Some('\n') => self.line += 1,
                None => return Err(TokenError("Unterminated string.".to_string())),
                _ => (),
            }

            self.advance();
        }

        let string_token = self.make_token(TokenType::String);
        self.advance(); // Consume the closing ".
        string_token
    }

    fn make_token(&self, token_type: TokenType) -> TokenResult<'a> {
        Ok(Token {
            token_type: token_type,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        })
    }

    fn make_token_if_matches(&mut self, expected: char, if_token: TokenType, else_token: TokenType) -> TokenResult<'a> {
        match self.peek() {
            Some(c) if *c == expected => { 
                self.advance();
                self.make_token(if_token)
            },
            _ => self.make_token(else_token),
        }
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

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
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
                Ok(token) if token.token_type == TokenType::Eof => {
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
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Star,
    Eof,

    // One or two character tokens.
    Bang,   // !
    BangEqual, // !=
    Equal,  // =
    EqualEqual, // ==
    Greater, // >
    GreaterEqual, // >=
    Less, // <
    LessEqual, // <=

    // String and number tokens.
    String,
}

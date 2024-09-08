use lazy_static::lazy_static;
use peekmore::PeekMore;
use peekmore::PeekMoreIterator;
use std::collections::HashMap;
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
            Some(c) if c.is_digit(10) => self.number(),
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
                '/' => self.make_token(TokenType::Slash),
                ';' => self.make_token(TokenType::Semicolon),

                '!' => self.make_token_if_matches('=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.make_token_if_matches('=', TokenType::EqualEqual, TokenType::Equal),
                '>' => self.make_token_if_matches('=', TokenType::GreaterEqual, TokenType::Greater),
                '<' => self.make_token_if_matches('=', TokenType::LessEqual, TokenType::Less),

                '"' => self.string(),

                'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

                _ => Err(TokenError(format!(
                    "Unexpected character: {}; Line: {}",
                    c, self.line
                ))),
            },
            None => self.make_token(TokenType::Eof),
        }
    }

    fn identifier(&mut self) -> TokenResult<'a> {
        loop {
            match self.peek() {
                Some(c) if c.is_alphanumeric() || *c == '_' => {
                    self.advance();
                }
                _ => break,
            }
        }

        let lexeme = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(lexeme).unwrap_or(&TokenType::Identifier);

        self.make_token(*token_type)
    }

    fn number(&mut self) -> TokenResult<'a> {
        loop {
            match self.peek() {
                Some(c) if c.is_digit(10) => {
                    self.advance();
                }
                Some('.') => {
                    self.advance();
                },
                Some(_) => break,
                None => break,
            }
        }

        self.make_token(TokenType::Number)
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

    fn make_token_if_matches(
        &mut self,
        expected: char,
        if_token: TokenType,
        else_token: TokenType,
    ) -> TokenResult<'a> {
        match self.peek() {
            Some(c) if *c == expected => {
                self.advance();
                self.make_token(if_token)
            }
            _ => self.make_token(else_token),
        }
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.chars.peek() {
                Some('\n') => {
                    self.chars.next();
                    self.line += 1;
                    self.current += 1;
                }
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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token<'a>>, TokenError> {
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

#[derive(Debug, PartialEq, Copy, Clone)]
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
    Slash,
    Eof,

    // One or two character tokens.
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // String, number and identifier tokens.
    String,
    Number,
    Identifier,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}
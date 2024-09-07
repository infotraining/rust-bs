use rlox::scanner::{Scanner, Token, TokenType, TokenError};

#[test]
fn scanning_empty_string() {
    let mut scanner = Scanner::new("");

    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![Token {
            token_type: TokenType::Eof,
            lexeme: "",
            line: 1
        }]
    );
}

#[test]
fn scanning_a_single_character_lexems() {
    let source = "(){},.-+*;";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::LeftParen,
                lexeme: "(",
                line: 1
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: ")",
                line: 1
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{",
                line: 1
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: "}",
                line: 1
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",",
                line: 1
            },
            Token {
                token_type: TokenType::Dot,
                lexeme: ".",
                line: 1
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-",
                line: 1
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+",
                line: 1
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*",
                line: 1
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";",
                line: 1
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "",
                line: 1
            }
        ]
    );
}


#[test]
fn scanning_operators() {
    let source = "= != == > < >= <=";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::Equal,
                lexeme: "=",
                line: 1
            },
            Token {
                token_type: TokenType::BangEqual,
                lexeme: "!=",
                line: 1
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==",
                line: 1
            },
            Token {
                token_type: TokenType::Greater,
                lexeme: ">",
                line: 1
            },
            Token {
                token_type: TokenType::Less,
                lexeme: "<",
                line: 1
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: ">=",
                line: 1
            },
            Token {
                token_type: TokenType::LessEqual,
                lexeme: "<=",
                line: 1
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "",
                line: 1
            }
        ]
    );
}

#[test]
fn scanning_string() {
    let source = "\"Hello, World!\"";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::String,
                lexeme: "Hello, World!",
                line: 1
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "",
                line: 1
            }
        ]
    );
}

#[test]
fn scanning_unterminated_string() {
    let source = "\"Hello, World!";

    let mut scanner = Scanner::new(source);
    let result = scanner.scan_tokens();

    assert_eq!(result.unwrap_err(), TokenError("Unterminated string.".to_string()));
}
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

#[test]
fn scanning_number_literals() {
    let source = "123 456 789.34";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "123",
                line: 1
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "456",
                line: 1
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "789.34",
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

// #[test]
// fn scanning_floating_point_with_two_dots() {
//     let source = "123..45";

//     let mut scanner = Scanner::new(source);
//     let result = scanner.scan_tokens();

//     assert!(result.unwrap_err().0.contains("Unexpected character '.'."));
// }

#[test]
fn scanning_identifiers() {
    let source = "foo bar baz";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::Identifier,
                lexeme: "foo",
                line: 1
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "bar",
                line: 1
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "baz",
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
fn scanning_keywords() {
    let source = "and class else false for fun if nil or print return super this true var while";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::And,
                lexeme: "and",
                line: 1
            },
            Token {
                token_type: TokenType::Class,
                lexeme: "class",
                line: 1
            },
            Token {
                token_type: TokenType::Else,
                lexeme: "else",
                line: 1
            },
            Token {
                token_type: TokenType::False,
                lexeme: "false",
                line: 1
            },
            Token {
                token_type: TokenType::For,
                lexeme: "for",
                line: 1
            },
            Token {
                token_type: TokenType::Fun,
                lexeme: "fun",
                line: 1
            },
            Token {
                token_type: TokenType::If,
                lexeme: "if",
                line: 1
            },
            Token {
                token_type: TokenType::Nil,
                lexeme: "nil",
                line: 1
            },
            Token {
                token_type: TokenType::Or,
                lexeme: "or",
                line: 1
            },
            Token {
                token_type: TokenType::Print,
                lexeme: "print",
                line: 1
            },
            Token {
                token_type: TokenType::Return,
                lexeme: "return",
                line: 1
            },
            Token {
                token_type: TokenType::Super,
                lexeme: "super",
                line: 1
            },
            Token {
                token_type: TokenType::This,
                lexeme: "this",
                line: 1
            },
            Token {
                token_type: TokenType::True,
                lexeme: "true",
                line: 1
            },
            Token {
                token_type: TokenType::Var,
                lexeme: "var",
                line: 1
            },
            Token {
                token_type: TokenType::While,
                lexeme: "while",
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
fn scanning_source_with_many_lines() {
    let source = "var text = \"Text\";\nvar a = 4;\r\n\nvar b = 3.14;";

    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: TokenType::Var,
                lexeme: "var",
                line: 1
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "text",
                line: 1
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=",
                line: 1
            },
            Token {
                token_type: TokenType::String,
                lexeme: "Text",
                line: 1
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";",
                line: 1
            },
            Token {
                token_type: TokenType::Var,
                lexeme: "var",
                line: 2
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "a",
                line: 2
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=",
                line: 2
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "4",
                line: 2
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";",
                line: 2
            },
            Token {
                token_type: TokenType::Var,
                lexeme: "var",
                line: 4
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "b",
                line: 4
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=",
                line: 4
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3.14",
                line: 4
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";",
                line: 4
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "",
                line: 4
            }
        ]
    );
}
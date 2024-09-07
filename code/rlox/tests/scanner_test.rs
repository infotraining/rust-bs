use rlox::scanner::{Scanner, Token, TokenType};

#[test]
fn scanning_empty_string() {
    let mut scanner = Scanner::new("");

    let tokens = scanner.scan_tokens().unwrap();

    assert_eq!(
        tokens,
        vec![Token {
            token_type: TokenType::EOF,
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
                token_type: TokenType::LEFT_PAREN,
                lexeme: "(",
                line: 1
            },
            Token {
                token_type: TokenType::RIGHT_PAREN,
                lexeme: ")",
                line: 1
            },
            Token {
                token_type: TokenType::LEFT_BRACE,
                lexeme: "{",
                line: 1
            },
            Token {
                token_type: TokenType::RIGHT_BRACE,
                lexeme: "}",
                line: 1
            },
            Token {
                token_type: TokenType::COMMA,
                lexeme: ",",
                line: 1
            },
            Token {
                token_type: TokenType::DOT,
                lexeme: ".",
                line: 1
            },
            Token {
                token_type: TokenType::MINUS,
                lexeme: "-",
                line: 1
            },
            Token {
                token_type: TokenType::PLUS,
                lexeme: "+",
                line: 1
            },
            Token {
                token_type: TokenType::STAR,
                lexeme: "*",
                line: 1
            },
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: ";",
                line: 1
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "",
                line: 1
            }
        ]
    );
}

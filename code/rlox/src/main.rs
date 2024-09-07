#[cfg(test)]
mod rlox_tests {

    use super::rlox::{scan_tokens, Token, TokenType};

    #[test]
    fn scanning_the_empty_string() {
        let source = "";
        let tokens = scan_tokens(source);

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
        let tokens = scan_tokens(source);

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
}

mod rlox {
    pub fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut start = 0;
        let mut current = 0;
        let mut line = 1;

        let mut source_chars = source.chars().peekable();

        loop {
            let c = source_chars.next();

            match c {
                Some(c) => {
                    current += 1;
                    match c {
                        '\n' => {
                            line += 1;
                        }
                        '(' => {
                            tokens.push(Token {
                                token_type: TokenType::LEFT_PAREN,
                                lexeme: "(",
                                line: line,
                            });
                        }
                        ')' => {
                            tokens.push(Token {
                                token_type: TokenType::RIGHT_PAREN,
                                lexeme: ")",
                                line: line,
                            });
                        }
                        '{' => {
                            tokens.push(Token {
                                token_type: TokenType::LEFT_BRACE,
                                lexeme: "{",
                                line: line,
                            });
                        }
                        '}' => {
                            tokens.push(Token {
                                token_type: TokenType::RIGHT_BRACE,
                                lexeme: "}",
                                line: line,
                            });
                        }
                        ',' => {
                            tokens.push(Token {
                                token_type: TokenType::COMMA,
                                lexeme: ",",
                                line: line,
                            });
                        }
                        '.' => {
                            tokens.push(Token {
                                token_type: TokenType::DOT,
                                lexeme: ".",
                                line: line,
                            });
                        }
                        '-' => {
                            tokens.push(Token {
                                token_type: TokenType::MINUS,
                                lexeme: "-",
                                line: line,
                            });
                        }
                        '+' => {
                            tokens.push(Token {
                                token_type: TokenType::PLUS,
                                lexeme: "+",
                                line: line,
                            });
                        }
                        '*' => {
                            tokens.push(Token {
                                token_type: TokenType::STAR,
                                lexeme: "*",
                                line: line,
                            });
                        }
                        ';' => {
                            tokens.push(Token {
                                token_type: TokenType::SEMICOLON,
                                lexeme: ";",
                                line: line,
                            });
                        }
                        _ => {}
                    }
                }
                None => {
                    tokens.push(Token {
                        token_type: TokenType::EOF,
                        lexeme: "",
                        line: line,
                    });
                    break;
                }
            }
        }

        tokens
    }

    #[derive(Debug, PartialEq)]
    pub struct Token<'a> {
        pub token_type: TokenType,
        pub lexeme: &'a str,
        pub line: i32,
    }

    #[derive(Debug, PartialEq)]
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
}

fn main() {
    println!("Hello, world!");

    let text = "Hello, world!";

    let tokens = text.chars().enumerate().collect::<Vec<(usize, char)>>();

    println!("{:?}", tokens);
}

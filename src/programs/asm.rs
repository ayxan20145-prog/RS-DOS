use crate::std::{string::String, vec::Vec};

#[derive(PartialEq)]
enum Token {
    Identifier(String),
    Number(i32),

    Mov,

    Comma,
    Newline,

    Eof,
}

enum Statement {
    Mov {
        destination: Operand,
        source: Operand,
    },
}

enum Operand {
    Register(String),
    Number(i32),
}
struct Lexer {
    source: Vec<char>,
    position: usize,
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Lexer {
    fn new(source: &str) -> Self {
        let mut chars = Vec::new();

        for c in source.chars() {
            chars.push(c);
        }

        Self {
            source: chars,
            position: 0,
        }
    }
    fn current(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn next_token(&mut self) -> Token {
        loop {
            match self.current() {
                Some(' ') | Some('\t') | Some('\r') => self.advance(),
                _ => break,
            }
        }

        match self.current() {
            None => Token::Eof,

            Some(',') => {
                self.advance();
                Token::Comma
            }

            Some('\n') => {
                self.advance();
                Token::Newline
            }

            Some(c) if c.is_ascii_digit() => {
                let mut number = String::new();

                loop {
                    match self.current() {
                        Some(c) if c.is_ascii_digit() => {
                            number.push(c as u8);
                            self.advance();
                        }

                        _ => break,
                    }
                }

                Token::Number(number.parse().unwrap())
            }

            Some(c) if c.is_alphabetic() => {
                let mut name = String::new();

                loop {
                    match self.current() {
                        Some(c) => {
                            if c.is_alphanumeric() {
                                name.push(c as u8);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                match name.as_str() {
                    "mov" => Token::Mov,
                    _ => Token::Identifier(name),
                }
            }

            Some(c) => {
                panic!("unexpected char: {}", c);
            }
        }
    }
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::Eof {
                tokens.push(Token::Eof);
                break;
            }

            tokens.push(token);
        }

        tokens
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn parse_statement(&mut self) -> Statement {
        self.advance();

        let destination = self.parse_operand();

        match self.current() {
            Token::Comma => self.advance(),
            _ => panic!("expected ','"),
        }

        let source = self.parse_operand();

        match self.current() {
            Token::Newline | Token::Eof => self.advance(),
            _ => panic!("expected newline"),
        }

        Statement::Mov {
            destination,
            source,
        }
    }
    fn parse_operand(&mut self) -> Operand {
        match self.current() {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Operand::Register(name)
            }

            Token::Number(value) => {
                let value = *value;
                self.advance();
                Operand::Number(value)
            }

            _ => panic!("expected operand"),
        }
    }
}

use crate::std::{string::String, vec::Vec};

enum Token {
    Identifier(String),
    Number(i32),

    Mov,

    Comma,
    Newline,

    Eof,
}
struct Lexer {
    source: Vec<char>,
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
}

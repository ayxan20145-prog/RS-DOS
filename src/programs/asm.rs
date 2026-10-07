use crate::std::{string::String, vec::Vec};

#[derive(PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),

    Mov,
    Ret,

    Comma,
    Newline,

    Eof,
}

enum Statement {
    Mov {
        destination: Operand,
        source: Operand,
    },
    Ret,
}

enum Operand {
    Register(Register),
    Number(i32),
}

enum Register {
    Eax,
    Ebx,
    Ecx,
    Edx,
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
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
                    "ret" => Token::Ret,
                    _ => Token::Identifier(name),
                }
            }

            Some(c) => {
                panic!("unexpected char: {}", c);
            }
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
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
    pub fn new(tokens: Vec<Token>) -> Self {
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
        match self.current() {
            Token::Mov => {
                self.advance();

                let destination = self.parse_operand();

                match self.current() {
                    Token::Comma => self.advance(),
                    _ => panic!("expected ','"),
                }

                let source = self.parse_operand();

                match self.current() {
                    Token::Newline => self.advance(),
                    Token::Eof => {}
                    _ => panic!("expected newline"),
                }

                Statement::Mov {
                    destination,
                    source,
                }
            }
            Token::Ret => {
                self.advance();

                match self.current() {
                    Token::Newline => self.advance(),
                    Token::Eof => {}
                    _ => panic!("expected newline"),
                }

                Statement::Ret
            }
            _ => panic!("expected statement"),
        }
    }
    fn parse_operand(&mut self) -> Operand {
        match self.current() {
            Token::Identifier(name) => {
                let register = match name.as_str() {
                    "eax" => Register::Eax,
                    "ebx" => Register::Ebx,
                    "ecx" => Register::Ecx,
                    "edx" => Register::Edx,
                    _ => panic!("unknown register: {}", name),
                };

                self.advance();
                Operand::Register(register)
            }

            Token::Number(value) => {
                let value = *value;
                self.advance();
                Operand::Number(value)
            }

            _ => panic!("expected operand"),
        }
    }
    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while *self.current() != Token::Eof {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }
}

impl Register {
    fn number(&self) -> u8 {
        match self {
            Self::Eax => 0,
            Self::Ecx => 1,
            Self::Edx => 2,
            Self::Ebx => 3,
        }
    }
}

pub fn assemble(program: &Program) -> Vec<u8> {
    let mut bytes = Vec::new();

    for statement in program.statements.as_slice() {
        match statement {
            Statement::Mov {
                destination,
                source,
            } => match (destination, source) {
                (Operand::Register(reg), Operand::Number(value)) => {
                    let value = *value as u32;

                    bytes.push(0xB8 + reg.number());

                    for byte in value.to_le_bytes() {
                        bytes.push(byte);
                    }
                }

                (Operand::Register(dest), Operand::Register(src)) => {
                    bytes.push(0x89);
                    bytes.push(0xC0 | (src.number() << 3) | dest.number());
                }

                _ => panic!("unsupported mov"),
            },
            Statement::Ret => {
                bytes.push(0xC3);
            }
        }
    }

    bytes
}

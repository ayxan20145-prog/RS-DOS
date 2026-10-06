use crate::std::vec::Vec;

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
}

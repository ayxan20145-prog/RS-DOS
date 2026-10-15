use crate::std::vec::Vec;
use core::{fmt, ops::Deref};

pub struct String {
    bytes: Vec<u8>,
}

impl String {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    pub fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
    pub fn push_str(&mut self, text: &str) {
        for byte in text.as_bytes() {
            self.push(*byte);
        }
    }
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.as_bytes()).expect("String contains invalid utf-8")
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        let mut string = Self::new();
        string.push_str(value);
        string
    }
}
impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
impl fmt::Write for String {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}
impl Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
impl Clone for String {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
        }
    }
}
impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut s = String::new();
        use core::fmt::Write;
        write!(&mut s, $($arg)*).unwrap();
        s
    }};
}

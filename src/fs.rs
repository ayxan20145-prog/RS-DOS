use crate::{print, vga::writer};
use core::fmt::Write;

const MAX_FILES: usize = 32;
const MAX_NAME: usize = 32;

#[derive(Copy, Clone)]
pub struct File {
    pub name: [u8; MAX_NAME],
    pub name_len: usize,
    pub used: bool,
}

pub struct FileSystem {
    pub files: [File; MAX_FILES],
}

impl File {
    pub fn new() -> Self {
        Self {
            name: [0; MAX_NAME],
            name_len: 0,
            used: false,
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: [File::new(); MAX_FILES],
        }
    }
    pub fn create(&mut self, name: &[u8]) -> bool {
        if name.is_empty() || name.len() > MAX_NAME {
            return false;
        }

        for file in &mut self.files {
            if !file.used {
                file.name[..name.len()].copy_from_slice(name);
                file.name_len = name.len();
                file.used = true;

                return true;
            }
        }

        false
    }
    pub fn list(&self) {
        for file in &self.files {
            if file.used {
                print!(
                    "{}",
                    core::str::from_utf8(&file.name[..file.name_len]).unwrap()
                );
            }
        }
    }
}

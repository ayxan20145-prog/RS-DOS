use crate::{drivers::vga::writer, print};
use core::fmt::Write;

const MAX_FILES: usize = 32;
const MAX_NAME: usize = 32;
const MAX_DATA: usize = 1024;

#[derive(Copy, Clone)]
pub struct File {
    pub name: [u8; MAX_NAME],
    pub name_len: usize,

    pub data: [u8; MAX_DATA],
    pub data_len: usize,

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

            data: [0; MAX_DATA],
            data_len: 0,

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
        if name.is_empty() {
            print!("\nfile name cant be empty");
            return false;
        } else if name.len() > MAX_NAME {
            print!("\nfile name too long");
            return false;
        }

        for file in &mut self.files {
            if !file.used {
                file.name[..name.len()].copy_from_slice(name);
                file.name_len = name.len();
                file.data_len = 0;
                file.used = true;

                return true;
            }
        }

        print!("\ncouldnt create file");
        false
    }
    pub fn write(&mut self, name: &[u8], data: &[u8]) -> bool {
        for file in &mut self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                if data.len() > MAX_DATA {
                    print!("\nfile too large");
                    return false;
                }

                file.data[..data.len()].copy_from_slice(data);
                file.data_len = data.len();

                return true;
            }
        }
        print!("\ncouldnt write");
        false
    }
    pub fn read(&self, name: &[u8]) -> Option<&[u8]> {
        for file in &self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                return Some(&file.data[..file.data_len]);
            }
        }

        None
    }
    pub fn remove(&mut self, name: &[u8]) -> bool {
        for file in &mut self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                file.used = false;
                file.name_len = 0;
                file.name = [0; MAX_NAME];

                return true;
            }
        }

        print!("\ncouldnt delete file");
        false
    }
    pub fn list(&self) {
        for file in &self.files {
            if file.used {
                print!(
                    "\n{}",
                    core::str::from_utf8(&file.name[..file.name_len]).unwrap()
                );
            }
        }
    }
}

#[cfg(test)]
mod tests;

use crate::{
    FS,
    drivers::vga::writer,
    print,
    std::{
        error::{Error, Result},
        vec::Vec,
    },
};
use core::fmt::Write;

pub struct File {
    pub name: Vec<u8>,
    pub data: Vec<u8>,
    pub is_dir: bool,
}

pub struct FileSystem {
    pub files: Vec<File>,
}

impl FileSystem {
    pub const fn new() -> Self {
        Self { files: Vec::new() }
    }
    pub fn create(&mut self, name: &[u8]) -> Result<()> {
        if name.is_empty() {
            return Err(Error::FileNameEmpty);
        }

        self.files.push(File {
            name: {
                let mut name_vec = Vec::new();

                for &byte in name {
                    name_vec.push(byte);
                }

                name_vec
            },
            data: Vec::new(),
            is_dir: false,
        });

        Ok(())
    }
    pub fn write(&mut self, name: &[u8], data: &[u8]) -> Result<()> {
        for file in self.files.as_mut_slice() {
            if file.name.as_slice() == name {
                if file.is_dir {
                    return Err(Error::NotAFile);
                }

                file.data.clear();

                for &byte in data {
                    file.data.push(byte);
                }

                return Ok(());
            }
        }

        Err(Error::FileNotFound)
    }
    pub fn read(&self, name: &[u8]) -> Result<&[u8]> {
        for file in self.files.as_slice() {
            if file.name.as_slice() == name {
                if file.is_dir {
                    return Err(Error::NotAFile);
                }

                return Ok(file.data.as_slice());
            }
        }

        Err(Error::FileNotFound)
    }
    pub fn remove_file(&mut self, name: &[u8]) -> Result<()> {
        for i in 0..self.files.len() {
            let file = self.files.get(i).unwrap();

            if file.name.as_slice() == name {
                if file.is_dir {
                    return Err(Error::NotAFile);
                }

                self.files.remove(i);
                return Ok(());
            }
        }

        Err(Error::FileNotFound)
    }
    pub fn list(&self) {
        for file in self.files.as_slice() {
            if file.is_dir {
                print!(
                    "\n<DIR> {}",
                    core::str::from_utf8(file.name.as_slice()).unwrap()
                );
            } else {
                print!(
                    "\n      {}",
                    core::str::from_utf8(file.name.as_slice()).unwrap()
                );
            }
        }
    }
    pub fn create_dir(&mut self, name: &[u8]) -> Result<()> {
        if name.is_empty() {
            return Err(Error::DirectoryNameEmpty);
        }

        self.files.push(File {
            name: {
                let mut name_vec = Vec::new();

                for &byte in name {
                    name_vec.push(byte);
                }

                name_vec
            },
            data: Vec::new(),
            is_dir: true,
        });

        Ok(())
    }
    pub fn remove_dir(&mut self, name: &[u8]) -> Result<()> {
        for i in 0..self.files.len() {
            let file = self.files.get(i).unwrap();

            if file.name.as_slice() == name {
                if !file.is_dir {
                    return Err(Error::NotADirectory);
                }

                self.files.remove(i);
                return Ok(());
            }
        }

        Err(Error::DirectoryNotFound)
    }
}
pub fn fs() -> &'static mut FileSystem {
    unsafe { &mut FS }
}

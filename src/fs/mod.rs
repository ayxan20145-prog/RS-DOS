#[cfg(test)]
mod tests;

use crate::{
    FS,
    drivers::vga::writer,
    print,
    std::error::{Error, Result},
};
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
    pub is_dir: bool,
}

pub struct FileSystem {
    pub files: [File; MAX_FILES],
}

impl File {
    pub const fn new() -> Self {
        Self {
            name: [0; MAX_NAME],
            name_len: 0,

            data: [0; MAX_DATA],
            data_len: 0,

            used: false,
            is_dir: false,
        }
    }
}

impl FileSystem {
    pub const fn new() -> Self {
        Self {
            files: [File::new(); MAX_FILES],
        }
    }
    pub fn create(&mut self, name: &[u8]) -> Result<()> {
        if name.is_empty() {
            return Err(Error::FileNameEmpty);
        } else if name.len() > MAX_NAME {
            return Err(Error::FileNameTooLong);
        }

        for file in &mut self.files {
            if !file.used {
                file.name[..name.len()].copy_from_slice(name);
                file.name_len = name.len();
                file.data_len = 0;
                file.used = true;
                file.is_dir = false;

                return Ok(());
            }
        }

        Err(Error::NoFreeSlot)
    }
    pub fn write(&mut self, name: &[u8], data: &[u8]) -> Result<()> {
        for file in &mut self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                if file.is_dir {
                    return Err(Error::NotAFile);
                }

                if data.len() > MAX_DATA {
                    return Err(Error::FileTooLarge);
                }

                file.data[..data.len()].copy_from_slice(data);
                file.data_len = data.len();

                return Ok(());
            }
        }

        Err(Error::FileNotFound)
    }
    pub fn read(&self, name: &[u8]) -> Result<&[u8]> {
        for file in &self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                if file.is_dir {
                    return Err(Error::NotAFile);
                }
                return Ok(&file.data[..file.data_len]);
            }
        }

        Err(Error::FileNotFound)
    }
    pub fn remove_file(&mut self, name: &[u8]) -> Result<()> {
        for file in &mut self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                if file.is_dir {
                    return Err(Error::NotAFile);
                }

                file.used = false;
                file.name_len = 0;
                file.name = [0; MAX_NAME];

                return Ok(());
            }
        }

        Err(Error::NotAFile)
    }
    pub fn list(&self) {
        for file in &self.files {
            if file.used {
                if file.is_dir {
                    print!(
                        "\n<DIR> {}",
                        core::str::from_utf8(&file.name[..file.name_len]).unwrap()
                    );
                } else {
                    print!(
                        "\n      {}",
                        core::str::from_utf8(&file.name[..file.name_len]).unwrap()
                    );
                }
            }
        }
    }
    pub fn create_dir(&mut self, name: &[u8]) -> Result<()> {
        if name.is_empty() {
            return Err(Error::DirectoryNameEmpty);
        }

        if name.len() > MAX_NAME {
            return Err(Error::DirectoryNameTooLong);
        }

        for file in &mut self.files {
            if !file.used {
                file.name[..name.len()].copy_from_slice(name);
                file.name_len = name.len();
                file.data_len = 0;
                file.used = true;
                file.is_dir = true;

                return Ok(());
            }
        }

        Err(Error::NoFreeSlot)
    }
    pub fn remove_dir(&mut self, name: &[u8]) -> Result<()> {
        for file in &mut self.files {
            if file.used && file.name_len == name.len() && &file.name[..file.name_len] == name {
                if !file.is_dir {
                    return Err(Error::NotADirectory);
                }

                file.used = false;
                file.name_len = 0;
                file.name = [0; MAX_NAME];

                return Ok(());
            }
        }

        Err(Error::DirectoryNotFound)
    }
}
pub fn fs() -> &'static mut FileSystem {
    unsafe { &mut FS }
}

const MAX_FILES: usize = 32;
const MAX_NAME: usize = 32;

#[derive(Copy, Clone)]
pub struct File {
    pub name: [u8; MAX_NAME],
    pub name_len: usize,
}

pub struct FileSystem {
    pub files: [File; MAX_FILES],
}

impl File {
    pub fn new() -> Self {
        Self {
            name: [0; MAX_NAME],
            name_len: 0,
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: [File::new(); MAX_FILES],
        }
    }
}

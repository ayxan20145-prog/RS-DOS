use super::*;

#[test]
fn create_and_read_file() {
    let mut fs = FileSystem::new();
    fs.create(b"hello.txt").unwrap();
    fs.write(b"hello.txt", b"hello world").unwrap();
    assert_eq!(fs.read(b"hello.txt").unwrap(), b"hello world");
}

#[test]
fn empty_name() {
    let mut fs = FileSystem::new();
    assert_eq!(fs.create(b""), Err(Error::FileNameEmpty));
}

#[test]
fn name_too_long() {
    let mut fs = FileSystem::new();
    let name = [b'a'; MAX_NAME + 1];
    assert_eq!(fs.create(&name), Err(Error::FileNameTooLong));
}

#[test]
fn read_missing_file() {
    let fs = FileSystem::new();
    assert_eq!(fs.read(b"hi"), Err(Error::FileNotFound));
}

#[test]
fn write_too_large() {
    let mut fs = FileSystem::new();
    fs.create(b"smth").unwrap();
    let data = [0u8; MAX_DATA + 1];
    assert_eq!(fs.write(b"smth", &data), Err(Error::FileTooLarge));
}

#[test]
fn read_dir_as_file() {
    let mut fs = FileSystem::new();
    fs.create_dir(b"dir").unwrap();
    assert_eq!(fs.read(b"dir"), Err(Error::NotAFile));
}

#[test]
fn no_slots_left() {
    let mut fs = FileSystem::new();

    for _ in 0..MAX_FILES {
        fs.create(b"hi").unwrap();
    }

    assert_eq!(fs.create(b"hi"), Err(Error::NoFreeSlot));
}

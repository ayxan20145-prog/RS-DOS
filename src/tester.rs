use crate::{arch, fs::*, print, writer};
use core::fmt::Write;

pub fn run() {
    if !test_fs() {
        print!("[ERR] test_fs\n");
        arch::cpu::halt();
    }
    print!("[OK] test_fs\n");
}

fn test_fs() -> bool {
    let mut fs = FileSystem::new();
    // create file
    if !fs.create(b"hi.txt").is_ok() {
        return false;
    }

    // empty name
    if !fs.create(b"").is_err() {
        return false;
    }

    // write file
    if !fs.write(b"hi.txt", b"hello world").is_ok() {
        return false;
    }

    // read file
    if !(fs.read(b"hi.txt").unwrap() == b"hello world") {
        return false;
    }

    // read missing file
    if !fs.read(b"hello").is_err() {
        return false;
    }

    // read dir as file
    fs.create_dir(b"hi").unwrap();
    if !fs.read(b"hi").is_err() {
        return false;
    }

    true
}

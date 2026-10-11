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

    fs.create(b"hi.txt").is_ok()
        && fs.create(b"").is_err()
        && fs.write(b"hi.txt", b"hello world").is_ok()
        && fs.read(b"hi.txt").is_ok_and(|d| d == b"hello world")
        && fs.read(b"hello").is_err()
        && fs.create_dir(b"hi").is_ok()
        && fs.read(b"hi").is_err()
}

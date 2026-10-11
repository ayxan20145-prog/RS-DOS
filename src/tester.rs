use crate::{ALLOCATOR, arch, format, fs::*, print, std::string::String, writer};
use core::fmt::Write;

pub fn run() {
    let mark = unsafe { ALLOCATOR.mark() };

    let tests: &[(&str, fn() -> bool)] = &[("test_fs", test_fs), ("test_string", test_string)];

    for (name, test) in tests {
        if !test() {
            print!("[ERR] {}\n", name);
            arch::cpu::halt();
        }
        print!("[OK] {}\n", name);
    }

    unsafe {
        ALLOCATOR.reset(mark);
    }
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

fn test_string() -> bool {
    let mut s = String::new();
    if s.as_str() != "" {
        return false;
    }

    s.push(b'h');
    s.push(b'i');
    if s.as_str() != "hi" {
        return false;
    }

    s.push_str(" there");
    if s.as_str() != "hi there" {
        return false;
    }

    let s2 = s.clone();
    if s2.as_str() != "hi there" {
        return false;
    }

    let s3 = String::from("hello");
    if s3.as_str() != "hello" {
        return false;
    }

    let formatted = format!("{} + {}", 1, 2);
    if formatted.as_str() != "1 + 2" {
        return false;
    }

    true
}

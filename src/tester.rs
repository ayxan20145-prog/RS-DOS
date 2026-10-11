use crate::{
    ALLOCATOR, arch, format,
    fs::*,
    print,
    std::{boxed::Box, string::String, vec::Vec},
    writer,
};
use core::fmt::Write;

pub fn run() {
    let mark = unsafe { ALLOCATOR.mark() };

    let tests: &[(&str, fn() -> bool)] = &[
        ("test_fs", test_fs),
        ("test_string", test_string),
        ("test_box", test_box),
        ("test_vec", test_vec),
    ];

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
fn test_box() -> bool {
    let mut b = Box::new(42u32);
    if *b != 42 {
        return false;
    }

    *b = 43;
    if *b != 43 {
        return false;
    }

    if b.as_ptr().is_null() {
        return false;
    }

    true
}
fn test_vec() -> bool {
    let mut v = Vec::<u32>::new();
    if v.len() != 0 || !v.as_slice().is_empty() {
        return false;
    }

    for i in 0..20 {
        v.push(i);
    }
    if v.len() != 20 {
        return false;
    }

    for i in 0..20 {
        if v.get(i) != Some(&(i as u32)) {
            return false;
        }
    }
    if v.get(20).is_some() {
        return false;
    }

    if v.pop() != Some(19) {
        return false;
    }

    if v.remove(0) != 0 {
        return false;
    }

    if v.as_slice()
        != &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        ]
    {
        return false;
    }

    v.clear();
    if v.len() != 0 || v.pop().is_some() {
        return false;
    }

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    let v3 = v2.clone();
    if v3.as_slice() != &[1, 2] {
        return false;
    }

    true
}

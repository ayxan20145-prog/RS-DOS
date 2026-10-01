use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    // Filesystem
    FileNameEmpty,
    FileNameTooLong,
    FileNotFound,
    FileAlreadyExists,
    NoFreeSlot,
    FileTooLarge,
    NotAFile,
    NotADirectory,
    DirectoryNameEmpty,
    DirectoryNameTooLong,
    DirectoryNotFound,
    DirectoryNotEmpty,
    WriteFailed,
    ReadFailed,
    DeleteFailed,
    CreateFailed,

    // Shell / Parsing
    UnknownCommand,
    MissingArgument,
    InvalidArgument,
    InvalidNumber,
    InvalidColor,
    InvalidAddress,

    // Calc
    DivisionByZero,
    UnknownOperator,
    InvalidOperand,

    // Vi
    BufferFull,
    SaveFailed,

    // Bat
    ScriptNotFound,

    // General
    IoError,
    Unsupported,
    OutOfMemory,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::FileNameEmpty => "file name can't be empty",
            Error::FileNameTooLong => "file name too long",
            Error::FileNotFound => "file not found",
            Error::FileAlreadyExists => "file already exists",
            Error::NoFreeSlot => "no free file slot",
            Error::FileTooLarge => "file too large",
            Error::NotAFile => "not a file",
            Error::NotADirectory => "not a directory",
            Error::DirectoryNameEmpty => "directory name can't be empty",
            Error::DirectoryNameTooLong => "directory name too long",
            Error::DirectoryNotFound => "directory not found",
            Error::DirectoryNotEmpty => "directory not empty",
            Error::WriteFailed => "write failed",
            Error::ReadFailed => "read failed",
            Error::DeleteFailed => "delete failed",
            Error::CreateFailed => "create failed",
            Error::UnknownCommand => "unknown command",
            Error::MissingArgument => "missing argument",
            Error::InvalidArgument => "invalid argument",
            Error::InvalidNumber => "invalid number",
            Error::InvalidColor => "invalid color",
            Error::InvalidAddress => "invalid address",
            Error::DivisionByZero => "can't divide by zero",
            Error::UnknownOperator => "unknown operator",
            Error::InvalidOperand => "invalid operand",
            Error::BufferFull => "buffer full",
            Error::SaveFailed => "save failed",
            Error::ScriptNotFound => "script not found",
            Error::IoError => "I/O error",
            Error::Unsupported => "unsupported operation",
            Error::OutOfMemory => "out of memory",
        };

        write!(f, "{}", msg)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

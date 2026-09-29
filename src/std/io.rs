#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write!(writer(), $($arg)*).unwrap();
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("[ INFO] {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
}

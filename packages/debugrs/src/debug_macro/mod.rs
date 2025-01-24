#[macro_export]
macro_rules! debug {
    // Match when no additional arguments are provided
    ($logger:expr, $message:expr) => {
        $logger.write($message);
    };

    // Match when additional arguments are provided
    ($logger:expr, $fmt:expr, $($args:tt)*) => {
        $logger.write(format!($fmt, $($args)*));
    };
}
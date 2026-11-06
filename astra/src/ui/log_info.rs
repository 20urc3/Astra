#[macro_export]
macro_rules! log_info {
    ($module:expr, $($arg:tt)*) => {
        eprintln!(
            "[{} {}] {}",
            chrono::Utc::now().format("%d %b %H:%M:%S"),
            $module.cyan(),
            format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! log_info {
    ($module:expr, $($arg:tt)*) => {
        eprintln!(
            "[{} {}] {}",
            chrono::Local::now().format("%d-%b-%Y"),
            $module.cyan(),
            format!($($arg)*)
        );
    };
}
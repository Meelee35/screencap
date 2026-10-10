use std::sync::OnceLock;
use std::time::Instant;

pub static START: OnceLock<Instant> = OnceLock::new();

macro_rules! log {
    () => {
        eprintln!(
            "[screencap] [{:>8.4}s]",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64()
        )
    };
    ($($arg:tt)*) => {
        eprintln!(
            "[screencap] [{:>8.4}s] {}",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64(),
            format_args!($($arg)*)
        )
    };
}
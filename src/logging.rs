use std::sync::OnceLock;
use std::time::Instant;

pub static START: OnceLock<Instant> = OnceLock::new();

macro_rules! log {
    () => {
        eprintln!(
            "[screencap:INFO] [{:>8.4}s]",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64()
        )
    };
    ($($arg:tt)*) => {
        eprintln!(
            "[screencap:INFO] [{:>8.4}s] {}",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64(),
            format_args!($($arg)*)
        )
    };
}
macro_rules! err {
    () => {
        eprintln!(
            "[screencap:ERR ] [{:>8.4}s]",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64()
        )
    };
    ($($arg:tt)*) => {
        eprintln!(
            "[screencap:ERR ] [{:>8.4}s] {}",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64(),
            format_args!($($arg)*)
        )
    };
}

macro_rules! warn {
    () => {
        eprintln!(
            "[screencap:WARN] [{:>8.4}s]",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64()
        )
    };
    ($($arg:tt)*) => {
        eprintln!(
            "[screencap:WARN] [{:>8.4}s] {}",
            crate::START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64(),
            format_args!($($arg)*)
        )
    };
}

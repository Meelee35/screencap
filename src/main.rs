use crate::logging::START;
use std::process::ExitCode;
#[macro_use]
mod logging;
pub mod capture;
pub mod ui;
pub mod app;

pub mod output;
pub mod config;
pub mod persistent;
pub mod constants;

fn main() -> ExitCode {
    let mut code = 0;

    log!("Screencap version {}", constants::VERSION);

    app::run::run().unwrap_or_else(|err| {
        output::dialogue::error(&err.to_string());
        code = 1;
    });

    log!("Exiting with code: {code}");
    ExitCode::from(code)
}
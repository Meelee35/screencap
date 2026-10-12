// my IDE really doesn't want this import to be here. It claims its unused. But removing it breaks logging.
use crate::config::cli::Command;
use crate::logging::START;
use std::process::ExitCode;
#[macro_use]
mod logging;
pub mod app;
pub mod capture;
pub mod ui;

pub mod config;
pub mod constants;
pub mod output;
pub mod persistent;

fn main() -> ExitCode {
    let mut code = 0;

    log!("Screencap version {}", constants::VERSION);

    let args = config::cli::args();
    let result = match args.command {
        Some(Command::ConfigWrite(config)) => config::command::write(&config),
        Some(Command::ConfigOverride(config)) => config::command::override_values(&config),
        None => app::run::run(&config::loader::load(&args.config)),
    };

    result.unwrap_or_else(|err| {
        err!("{err:#}");
        output::dialogue::error(&err.to_string());
        code = 1;
    });

    log!("Exiting with code: {code}");
    ExitCode::from(code)
}

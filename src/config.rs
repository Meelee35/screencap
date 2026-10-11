use crate::config::cli::Args;
use std::path::PathBuf;

pub mod cli;
pub mod loader;

pub struct Config {
    pub save_directory: Option<PathBuf>,
    pub clipboard: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            save_directory: args.savedirectory,
            clipboard: args.clipboard,
        }
    }
}

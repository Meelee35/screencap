use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod cli;
pub mod file;
pub mod loader;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub save_directory: Option<PathBuf>,
    pub clipboard: Option<bool>,
}
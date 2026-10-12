use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod cli;
pub mod command;
pub mod file;
pub mod loader;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub save_directory: Option<PathBuf>,
    pub clipboard: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            save_directory: default_save_directory(),
            clipboard: Some(true),
        }
    }
}

fn default_save_directory() -> Option<PathBuf> {
    std::env::var_os("XDG_PICTURES_DIR")
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Pictures")))
}
